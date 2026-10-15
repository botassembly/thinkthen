"""The source-host entry prepares a real backend without a caller's selector."""
from __future__ import annotations

import http.client
import json
import os
from pathlib import Path
import tempfile
from unittest.mock import patch

import harness
import source_hosts


def check(root, target, selection):
    # Only the costly host matrix is omitted. The actual entry, Cargo build,
    # artifact selection, harness and loopback process all run unchanged.
    env = source_hosts.tool_environment() | {
        'CARGO_TARGET_DIR': selection, 'CARGO_BUILD_JOBS': '2',
        'THINKTHEN_BACKEND_BIN': '/missing/stale-caller-backend'}
    with patch.object(os, 'environ', env), \
            patch.object(source_hosts, 'hosts', return_value=iter(())), \
            patch.object(harness, 'BACKEND', ''):
        assert source_hosts.check_hosts(root / 'cache', False) == 0
        executable = Path(harness.BACKEND)
        # A changed cwd must not break a relative target-directory selection.
        previous = Path.cwd()
        try:
            os.chdir(root)
            with harness.Backend() as backend:
                assert executable.is_absolute(), executable
                assert executable.is_relative_to(target.resolve()), executable
                assert executable.is_file() and os.access(executable, os.X_OK)
                assert backend.count() == 0
                connection = http.client.HTTPConnection('127.0.0.1', backend.port, timeout=10)
                body = '{"state":"x","model":"m","questions":{"q1":{"type":"noul","instructions":"Is it?"}}}'
                try:
                    connection.request('POST', '/arm/full/capture/v1/systemone', body=body)
                    response = connection.getresponse()
                    assert response.status == 200
                    assert json.loads(response.read())['answers']['q1']['noul'] == 0.9
                finally:
                    connection.close()
                assert backend.count() == 1
                assert backend.capture() == [body]
                observed = backend.snapshot('bearers')
                assert observed['absent'] == 1 and observed['unknown'] == 0, observed
            assert executable.is_file(), 'fixture disappeared before its owner finished'
        finally:
            os.chdir(previous)
    return executable


def main():
    with tempfile.TemporaryDirectory(prefix='thinkthen-source-backend-') as folder:
        root = Path(folder)
        target = root / 'target with spaces'
        target.mkdir()
        alias = root / 'target link'
        alias.symlink_to(target, target_is_directory=True)
        for selection in (str(target), os.path.relpath(target, source_hosts.REPO), str(alias)):
            executable = check(root, target, selection)
    assert not executable.exists(), 'fixture survived its owned target cleanup'
    print('ok   source hosts build, select, run and clean their real loopback backend')


if __name__ == '__main__':
    main()
