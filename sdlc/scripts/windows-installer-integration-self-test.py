#!/usr/bin/env python3
"""Prove Windows smoke routing offline with fake hosts; never claim native proof."""

import hashlib
import io
from contextlib import redirect_stdout
import importlib.util
import json
from pathlib import Path
import tempfile
import urllib.request
from unittest.mock import patch
import zipfile

REPO = Path(__file__).resolve().parents[2]
SPEC = importlib.util.spec_from_file_location('installer_smoke', REPO / 'sdlc/scripts/release-windows-smoke.py')
SMOKE = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(SMOKE)
VERSION = '0.2.0'


def route(root, *, missing=False, receipt=False, extra=False):
    platform = root / 'platform'
    platform.mkdir()
    name = f'thinkthen-{VERSION}-{SMOKE.COMMAND.TARGET}.zip'
    with zipfile.ZipFile(platform / name, 'w') as output:
        output.writestr('thinkthen.exe', b'offline routing fixture, never executed')
    data = (platform / name).read_bytes()
    (platform / (name + '.sha256')).write_text(hashlib.sha256(data).hexdigest() + '  ' + name)
    calls, replays = [], []

    def fake_run(args, env):
        assert args[1:5] == ['-NoProfile', '-NonInteractive', '-File', str(REPO / 'install.ps1')]
        assert args[5:] == ['-Version', VERSION]
        assert env['THINKTHEN_INSTALL_BASE'] == env['THINKTHEN_INSTALL_API']
        base = env['THINKTHEN_INSTALL_BASE']
        assert base.startswith('http://127.0.0.1:')
        prefix = f'/botassembly/thinkthen/releases/download/v{VERSION}/'
        for file in (name, name + '.sha256'):
            with urllib.request.urlopen(base + prefix + file, timeout=5) as response:
                assert response.read() == (platform / file).read_bytes()
        target = Path(env['THINKTHEN_INSTALL_DIR'])
        target.mkdir()
        (target / 'thinkthen.exe').write_bytes(b'not executable')
        (target / '.thinkthen-install.lock').write_bytes(b'')
        (target / 'thinkthen.install.json').write_text(json.dumps(dict(state='pending' if receipt else 'installed', version=VERSION)))
        if extra:
            (target / 'unexpected').write_text('plant')
        calls.append(args[0])

    def replay(binary, sample, env, version):
        assert version == VERSION and binary.name == 'thinkthen.exe'
        assert binary.parent.name.startswith('installed-')
        replays.append(binary)

    with patch.object(SMOKE.shutil, 'which', side_effect=lambda host: None if missing and host == 'powershell' else host), \
         patch.object(SMOKE, 'run', side_effect=fake_run), patch.object(SMOKE, 'smoke', side_effect=replay), redirect_stdout(io.StringIO()):
        SMOKE.installed_smoke(platform, root / 'sample', {}, VERSION, root)
    assert calls == ['powershell', 'pwsh'] and len(replays) == 2


def main():
    with tempfile.TemporaryDirectory(prefix='thinkthen-installer-routing-') as temporary:
        root = Path(temporary)
        for name, options in (('good', {}), ('missing-host', dict(missing=True)),
                              ('pending-receipt', dict(receipt=True)), ('unexpected-file', dict(extra=True))):
            case = root / name
            case.mkdir()
            try:
                route(case, **options)
            except RuntimeError:
                if name == 'good':
                    raise
            else:
                if name != 'good':
                    raise RuntimeError(f'routing accepted {name}')
    print('Windows installer routing: both hosts install before replay; 3 refusal plants pass; native Windows proof NOT RUN')


if __name__ == '__main__':
    main()
