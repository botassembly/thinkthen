#!/usr/bin/env python3
"""Offline private-process startup, identity, shutdown and retention proof (0398B)."""
import io
import json
from pathlib import Path
import shutil
import subprocess
import sys
import tarfile
import tempfile
import unittest
from unittest.mock import patch
from install_check import Check, Failure, check_result, clean_environment, main
from install_check_channels import postgresql
from install_check_postgresql import stop_postgres


@unittest.skipUnless(sys.platform.startswith('linux'), 'PostgreSQL install cell runs only on Linux')
class PostgresLifecycle(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.seed = tempfile.TemporaryDirectory(prefix='thinkthen-pg-fixture-')
        root = Path(cls.seed.name)
        source = root / 'postmaster.c'
        source.write_text('#include <unistd.h>\nint main(void) { for (;;) pause(); }\n')
        cls.server = root / 'postgres'
        subprocess.run(['cc', source, '-o', cls.server], env=clean_environment(root), check=True, capture_output=True)

    @classmethod
    def tearDownClass(cls):
        cls.seed.cleanup()

    def run_fixture(self, check):
        with patch('install_check_tools.postgres_tools', check.tools):
            return postgresql(check)

    def test_successful_query_stops_exact_private_child(self):
        with tempfile.TemporaryDirectory() as own:
            check = PgFixture(Path(own), self.server)
            try:
                installed, reply, proof = self.run_fixture(check)
                check_result(check.version, installed, reply)
                self.assertEqual(proof, 'installed SQL extension version')
                self.assertIsNotNone(check.child.poll())
                self.assertFalse(check.retain_scratch)
                self.assertTrue(any(Path(command[0]).name == 'psql' for command in check.commands))
            finally: check.close()

    def test_spawn_then_startup_timeout_still_stops_private_child(self):
        with tempfile.TemporaryDirectory() as own:
            check = PgFixture(Path(own), self.server); check.failure = 'startup'
            try:
                with self.assertRaises(subprocess.TimeoutExpired) as error: self.run_fixture(check)
                self.assertIs(error.exception, check.start_error)
                self.assertIsNotNone(check.child.poll())
                self.assertFalse(check.retain_scratch)
                self.assertFalse(any(Path(command[0]).name == 'psql' for command in check.commands))
            finally: check.close()

    def test_primary_and_cleanup_failures_remain_separate(self):
        for primary in ('startup', 'query', 'version'):
            with self.subTest(primary=primary), tempfile.TemporaryDirectory() as own:
                check = PgFixture(Path(own), self.server); check.failure = primary
                stderr = io.StringIO()
                try:
                    with patch('install_check_channels.stop_postgres', side_effect=Failure('fixture cleanup failed')), patch('sys.stderr', stderr):
                        if primary == 'startup':
                            with self.assertRaises(subprocess.TimeoutExpired) as error: self.run_fixture(check)
                            self.assertIs(error.exception, check.start_error)
                        else:
                            with self.assertRaises(Failure) as error: self.run_fixture(check)
                            self.assertEqual(str(error.exception), 'fixture query failed' if primary == 'query' else 'installed thinkthen 0.1.3, wanted 0.1.2')
                    self.assertEqual(stderr.getvalue(), 'install-check: PostgreSQL cleanup also failed: fixture cleanup failed\n')
                    self.assertTrue(check.retain_scratch)
                    self.assertIsNone(check.child.poll())
                finally: check.close()

    def test_cleanup_failure_after_success_refuses_success(self):
        with tempfile.TemporaryDirectory() as own:
            check = PgFixture(Path(own), self.server)
            try:
                with patch('install_check_channels.stop_postgres', side_effect=Failure('fixture cleanup failed')):
                    with self.assertRaises(Failure) as error: self.run_fixture(check)
                self.assertEqual(str(error.exception), 'PostgreSQL cleanup failed: fixture cleanup failed')
                self.assertTrue(check.retain_scratch)
            finally: check.close()

    def test_wrong_pid_and_private_path_identity_never_signals_child(self):
        for identity in ('pid-path', 'socket', 'runtime', 'arguments', 'missing-pid'):
            with self.subTest(identity=identity), tempfile.TemporaryDirectory() as own:
                check = PgFixture(Path(own), self.server); check.bad_identity = identity
                try:
                    with self.assertRaises(Failure): self.run_fixture(check)
                    self.assertTrue(check.retain_scratch)
                    self.assertIsNone(check.child.poll())
                finally: check.close()

    def test_exited_pid_never_signals_another_process(self):
        with tempfile.TemporaryDirectory() as own:
            check = PgFixture(Path(own), self.server)
            try:
                check.tools(check, check.runtime)
                check.start()
                check.child.terminate(); check.child.wait()
                with patch('install_check_postgresql.signal.pidfd_send_signal') as send:
                    stop_postgres(check, check.data, check.socket, check.binary)
                send.assert_not_called()
            finally: check.close()


class ScratchRetention(unittest.TestCase):
    def test_main_retains_only_when_shutdown_is_unverified(self):
        original = tempfile.TemporaryDirectory
        for retained in (False, True):
            with self.subTest(retained=retained):
                containers = []
                def make(*args, **kwargs):
                    container = original(*args, **kwargs); containers.append(container); return container
                def install(check):
                    check.retain_scratch = retained
                    (check.root / 'marker').write_text('owned')
                    if retained: raise Failure('fixture primary failed')
                    return check.version, {'value': True, 'requests_sent': 0}, 'fixture'
                stdout, stderr = io.StringIO(), io.StringIO()
                try:
                    with patch('install_check.tempfile.TemporaryDirectory', make), patch('install_check_tools.prepare'), \
                         patch.object(Check, 'prepare_sample'), patch('install_check_channels.install', install), \
                         patch('sys.stdout', stdout), patch('sys.stderr', stderr):
                        code = main(['postgresql', '0.1.2'])
                    root = Path(containers[0].name)
                    self.assertEqual(root.exists(), retained)
                    self.assertEqual(code, int(retained))
                    if retained:
                        self.assertEqual((root / 'marker').read_text(), 'owned')
                        self.assertEqual(stdout.getvalue(), '')
                        self.assertIn('retained PostgreSQL scratch at ' + str(root), stderr.getvalue())
                        self.assertTrue(stderr.getvalue().endswith('install-check: fixture primary failed\n'))
                finally:
                    for container in containers: container.cleanup()


class PgFixture(Check):
    def __init__(self, root, server):
        super().__init__(root, 'postgresql', '0.1.2')
        self.server, self.child = server, None
        self.failure = self.bad_identity = None
        self.commands = []
        self.runtime = root / 'pg-runtime'
        self.binary = self.runtime / 'usr/lib/postgresql/16/bin'
        self.data, self.socket = root / 'pg-data', root / 'pg-socket'
        self.sample.mkdir(parents=True)
        (self.sample / 'question.txt').write_text('Recorded question?')
        (self.sample / 'report.txt').write_text('Recorded evidence.')

    def release(self, name):
        archive = self.root / name
        with tarfile.open(archive, 'w:gz') as tar:
            for name in ('extension/thinkthen.control', 'lib/thinkthen.so'):
                entry = tarfile.TarInfo(name); entry.size = 7; tar.addfile(entry, io.BytesIO(b'fixture'))
        return archive

    def tools(self, check, runtime):
        (runtime / 'usr/share/postgresql/16/extension').mkdir(parents=True)
        (runtime / 'usr/lib/postgresql/16/lib').mkdir(parents=True)
        self.binary.mkdir(parents=True)
        shutil.copy2(self.server, self.binary / 'postgres')

    def start(self):
        self.data.mkdir(mode=0o700, exist_ok=True)
        self.socket.mkdir(mode=0o700, exist_ok=True)
        command = self.binary / 'postgres'
        if self.bad_identity == 'runtime': command = self.server
        data = self.data if self.bad_identity != 'arguments' else self.root / 'unowned'
        self.child = subprocess.Popen([command, '-D', data, '-k', self.socket], env=self.env, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
        fields = [str(self.child.pid), str(self.data), '1', '5432', str(self.socket), '', '0', 'ready']
        if self.bad_identity == 'pid-path': fields[1] = str(self.root / 'unowned')
        if self.bad_identity == 'socket': fields[4] = str(self.root / 'unowned')
        if self.bad_identity != 'missing-pid':
            (self.data / 'postmaster.pid').write_text('\n'.join(fields) + '\n')
            (self.data / 'postmaster.pid').chmod(0o600)

    def run(self, *args, cwd=None, input=None):
        command = tuple(str(arg) for arg in args); self.commands.append(command)
        tool = Path(command[0]).name
        if tool == 'initdb': self.data.mkdir(mode=0o700)
        if tool == 'pg_ctl':
            self.start()
            if self.failure == 'startup':
                self.start_error = subprocess.TimeoutExpired(command, 0.1)
                raise self.start_error
        if tool == 'psql':
            if self.failure == 'query': raise Failure('fixture query failed')
            version = '0.1.3' if self.failure == 'version' else self.version
            return version + '\n' + json.dumps({'value': True, 'meta': {'requests_sent': 0}})
        return ''

    def close(self):
        if self.child is not None:
            if self.child.poll() is None: self.child.terminate()
            self.child.wait()


if __name__ == '__main__':
    unittest.main()
