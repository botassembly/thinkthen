#!/usr/bin/env python3
"""Offline refusal and routing proof; never installs a public package."""
import hashlib
import io
import json
from pathlib import Path
import subprocess
import sys
import tarfile
import tempfile
import unittest
from unittest.mock import patch

from install_check import CHANNELS, Check, Failure, check_index, check_result, clean_environment, extract_archive, validate_version, verify_checksum
from install_check_channels import INSTALLERS, dcf_packages, homebrew, r_universe, selected_formula, sqlite


class RefusalTable(unittest.TestCase):
    def refusal(self, sentence, function, *args):
        with self.assertRaises(Failure) as error:
            function(*args)
        self.assertEqual(str(error.exception), sentence)

    def test_all_channels_answer_and_version_contract(self):
        self.assertEqual(set(CHANNELS), set(INSTALLERS))
        self.assertEqual(len(CHANNELS), 18)
        for channel in CHANNELS:
            with self.subTest(channel=channel):
                check_result('0.1.2', '0.1.2', {'value': True, 'facts': {'requests_sent': 0}})
                self.refusal('installed thinkthen 0.1.3, wanted 0.1.2', check_result, '0.1.2', '0.1.3', {'value': True, 'requests_sent': 0})
                for value in (False, None, 'true', 1, {}, []):
                    self.refusal('replay did not return true', check_result, '0.1.2', '0.1.2', {'value': value, 'requests_sent': 0})
                for requests in (None, 1, False, '0', 0.0):
                    self.refusal('replay must report requests_sent 0', check_result, '0.1.2', '0.1.2', {'value': True, 'requests_sent': requests})
                self.refusal('replay must report requests_sent 0', check_result, '0.1.2', '0.1.2', {'value': True, 'facts': 'placeholder'})
                self.refusal('replay did not return true', check_result, '0.1.2', '0.1.2', 'placeholder')

    def test_invalid_versions_before_side_effects(self):
        for version in ('v0.1.2', '0.1', '0.01.2', '-1.2.3', '0.1.2\n', '$(touch nope)', '0.1.2;echo bad'):
            self.refusal('version must be three whole numbers separated by dots', validate_version, version)
            result = subprocess.run([sys.executable, Path(__file__).with_name('install-check'), 'pip', version], capture_output=True, text=True)
            self.assertEqual(result.returncode, 1)
            self.assertEqual(result.stderr, 'install-check: version must be three whole numbers separated by dots\n')

    def test_indexes(self):
        for channel in ('Go proxy', 'Packagist'):
            check_index(channel, '0.1.2', ['0.1.2'])
            self.refusal(f'{channel} does not list thinkthen 0.1.2 yet; dispatch install-check again later', check_index, channel, '0.1.2', ['0.1.1'])
        for version in (None, '0.1.1'):
            self.refusal('R-universe does not list thinkthen 0.1.2 yet; dispatch install-check again later', check_index, 'R-universe', '0.1.2', version)
        check_index('R-universe', '0.1.2', '0.1.2')
        self.refusal('R-universe no longer offers thinkthen 0.1.2; its binary index lists 0.2.0', check_index, 'R-universe', '0.1.2', '0.2.0')
        self.refusal('R-universe lists thinkthen 0.1.2 only as source; a Linux binary is required', check_index, 'R-universe', '0.1.2', '0.1.2', False)

    def test_clean_environment(self):
        with tempfile.TemporaryDirectory() as own, patch.dict('os.environ', {'THINKTHEN_API_KEY': 'fixture', 'OPENAI_API_KEY': 'fixture', 'THINKTHEN_CACHE': '/unowned', 'THINKTHEN_CONFIG': '/unowned'}):
            env = clean_environment(Path(own))
            self.assertFalse(any(key.endswith('KEY') or key.startswith('THINKTHEN_') for key in env))
            self.assertEqual(env['HOME'], own)
            self.assertEqual(env['HOMEBREW_NO_AUTO_UPDATE'], '1')

    def test_homebrew_history_and_routing(self):
        current = 'class Thinkthen < Formula\n  version "0.2.0"\nend\n'
        old = 'class Thinkthen < Formula\n  version "0.1.2"\nend\n'
        self.assertEqual(selected_formula([current, old], '0.2.0'), current)
        self.assertEqual(selected_formula([current, old], '0.1.2'), old)
        self.refusal('Homebrew tap history does not contain thinkthen 0.1.1', selected_formula, [current, old], '0.1.1')
        with tempfile.TemporaryDirectory() as own:
            check = Fixture(Path(own), 'homebrew')
            check.outputs = {'log': 'new\nold', 'new:Formula/thinkthen.rb': current, 'old:Formula/thinkthen.rb': old, '--prefix': str(check.root / 'installed')}
            with patch.object(check, 'command_replay', return_value=('0.1.2', {'value': True, 'requests_sent': 0})):
                installed, reply, _ = homebrew(check)
            check_result('0.1.2', installed, reply)
            self.assertEqual((check.root / 'selected-tap/Formula/thinkthen.rb').read_text(), old)
            self.assertIn(('brew', 'install', 'installcheck/selected/thinkthen'), check.commands)
            self.assertEqual(check.commands[-1], ('brew', 'untap', 'installcheck/selected'))

    def test_archive_identity_and_escape(self):
        with tempfile.TemporaryDirectory() as own:
            root = Path(own)
            archive = root / 'thinkthen-sqlite-0.1.2-x86_64-unknown-linux-gnu.tar.gz'
            packed(archive, {'libthinkthen0.so': b'fixture extension'})
            digest = hashlib.sha256(archive.read_bytes()).hexdigest()
            verify_checksum(archive, f'{digest}  {archive.name}\n')
            self.refusal(f'checksum for {archive.name} names another file', verify_checksum, archive, f'{digest}  wrong.tar.gz\n')
            self.refusal(f'release asset {archive.name} differs from its checksum', verify_checksum, archive, f'{"0" * 64}  {archive.name}\n')
            destination = root / 'extracted'; destination.mkdir()
            extract_archive(archive, destination)
            self.assertEqual((destination / 'libthinkthen0.so').read_bytes(), b'fixture extension')
            for member in ('../escape', '/escape'):
                packed(archive, {member: b'bad'})
                self.refusal(f'release asset {archive.name} contains an unsafe entry', extract_archive, archive, destination)
            with tarfile.open(archive, 'w:gz') as tar:
                link = tarfile.TarInfo('link'); link.type = tarfile.SYMTYPE; link.linkname = '../escape'; tar.addfile(link)
            self.refusal(f'release asset {archive.name} contains an unsafe entry', extract_archive, archive, destination)

    def test_sqlite_qualified_identity_and_exact_load(self):
        with tempfile.TemporaryDirectory() as own:
            check = Fixture(Path(own), 'sqlite')
            prepare_sample(check)
            archive = check.root / f'thinkthen-sqlite-{check.version}-x86_64-unknown-linux-gnu.tar.gz'
            packed(archive, {'libthinkthen0.so': b'fixture extension'})
            check.outputs = {'--version': '3.53.4 fixture', ':memory:': '0\n{"value":true,"meta":{"requests_sent":0}}'}
            with patch.object(check, 'release', return_value=archive) as release:
                installed, reply, proof = sqlite(check)
            release.assert_called_once_with(archive.name)
            check_result(check.version, installed, reply)
            self.assertEqual(proof, 'release-asset identity')
            self.assertTrue(check.inputs[-1].startswith(f'.load {check.root}/sqlite/libthinkthen0.so\n'))
            self.assertFalse(any('c-' in str(arg) for command in check.commands for arg in command))

    def test_r_binary_only_and_superseded_routes(self):
        with tempfile.TemporaryDirectory() as own:
            check = Fixture(Path(own), 'r-universe'); prepare_sample(check)
            index = 'Package: thinkthen\nVersion: 0.1.2\nFile: thinkthen_0.1.2.tar.gz\n'
            check.text = lambda url, name: index
            archive = check.root / 'thinkthen_0.1.2.tar.gz'
            packed(archive, {'thinkthen/DESCRIPTION': b'Package: thinkthen\nVersion: 0.1.2\nBuilt: R 4.6.1; x86_64-pc-linux-gnu; 2026-10-04; unix\n', 'thinkthen/libs/thinkthen.so': b'fixture'})
            check.fetch = lambda url, destination: archive
            check.outputs = {'Rscript': '0.1.2'}
            check.response = lambda *args: {'value': True, 'requests_sent': 0}
            installed, reply, _ = r_universe(check); check_result(check.version, installed, reply)
            self.assertTrue(any(command[:3] == ('R', 'CMD', 'INSTALL') for command in check.commands))
            self.assertFalse(any('cargo' in command for command in check.commands))
        for listed in ('0.1.1', '0.2.0'):
            with tempfile.TemporaryDirectory() as own:
                check = Fixture(Path(own), 'r-universe')
                check.text = lambda url, name: f'Package: thinkthen\nVersion: {listed}\n'
                with self.assertRaises(Failure): r_universe(check)
                self.assertEqual(check.commands, [])
        with tempfile.TemporaryDirectory() as own:
            check = Fixture(Path(own), 'r-universe')
            check.text = lambda url, name: '' if 'binary' in name else index
            self.refusal('R-universe lists thinkthen 0.1.2 only as source; a Linux binary is required', r_universe, check)
            self.assertEqual(check.commands, [])

    def test_r_binary_address_rejects_source_archive(self):
        with tempfile.TemporaryDirectory() as own:
            check = Fixture(Path(own), 'r-universe')
            check.text = lambda url, name: 'Package: thinkthen\nVersion: 0.1.2\n'
            archive = check.root / 'thinkthen_0.1.2.tar.gz'
            packed(archive, {'thinkthen/DESCRIPTION': b'Package: thinkthen\nVersion: 0.1.2\n', 'thinkthen/src/source.rs': b'fixture'})
            check.fetch = lambda url, destination: archive
            self.refusal('R-universe binary address returned a source archive; a Linux binary is required', r_universe, check)
            self.assertEqual(check.commands, [])

    def test_workflow_cells(self):
        import yaml
        workflow = Path(__file__).resolve().parents[2] / '.github/workflows/install-check.yml'
        doc = yaml.safe_load(workflow.read_text())
        self.assertEqual(doc['permissions'], {})
        self.assertEqual(set(doc.get('on', doc.get(True))), {'workflow_dispatch'})
        cells = set()
        for name, job in doc['jobs'].items():
            self.assertEqual(job['permissions'], {'contents': 'read'})
            self.assertNotIn('environment', job)
            matrix = job.get('strategy', {}).get('matrix', {})
            for runner in matrix.get('runner', [job['runs-on']]):
                for channel in matrix.get('channel', [name]): cells.add((runner, channel))
            for step in job['steps']:
                if 'uses' in step: self.assertRegex(step['uses'], r'@[0-9a-f]{40}$')
                self.assertNotIn('inputs.', step.get('run', ''))
                self.assertNotIn('secrets.', json.dumps(step))
        self.assertEqual(len(cells), 47)
        self.assertEqual({channel for _, channel in cells}, set(CHANNELS))
        all_five = {'ubuntu-24.04', 'ubuntu-24.04-arm', 'macos-15', 'macos-15-intel', 'macos-26'}
        for channel in ('download', 'cargo-install', 'cargo-add', 'pip', 'uv', 'npm', 'rubygems'):
            self.assertEqual({runner for runner, member in cells if member == channel}, all_five)


class Fixture(Check):
    def __init__(self, root, channel):
        super().__init__(root, channel, '0.1.2')
        self.commands, self.inputs, self.outputs = [], [], {}

    def run(self, *args, cwd=None, input=None):
        command = tuple(str(arg) for arg in args)
        self.commands.append(command); self.inputs.append(input)
        for key, output in self.outputs.items():
            if key in command: return output
        return ''


def packed(path, files):
    with tarfile.open(path, 'w:gz') as tar:
        for name, data in files.items():
            item = tarfile.TarInfo(name); item.size = len(data); item.mode = 0o644
            tar.addfile(item, io.BytesIO(data))


def prepare_sample(check):
    check.sample.mkdir(parents=True)
    (check.sample / 'recording').mkdir(mode=0o700)
    (check.sample / 'question.txt').write_text('Recorded question?')
    (check.sample / 'report.txt').write_text('Recorded evidence.')
    (check.root / 'settings.json').write_text(json.dumps({'replay': str(check.sample / 'recording'), 'cache': False}))


if __name__ == '__main__':
    unittest.main()
