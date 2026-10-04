#!/usr/bin/env python3
"""Exercise actual PowerShell installer boundaries offline; native proof needs Windows."""

import argparse
import hashlib
import http.server
import io
import json
import os
from pathlib import Path
import shutil
import struct
import subprocess
import tempfile
import threading
import zipfile
import warnings

REPO = Path(__file__).resolve().parents[2]
NAME = 'thinkthen-0.2.0-x86_64-pc-windows-msvc.zip'


def pe():
    data = bytearray(128)
    data[:2] = b'MZ'
    struct.pack_into('<I', data, 60, 64)
    data[64:68] = b'PE\0\0'
    struct.pack_into('<H', data, 68, 0x8664)
    struct.pack_into('<H', data, 84, 2)
    struct.pack_into('<H', data, 86, 2)
    struct.pack_into('<H', data, 88, 0x20B)
    return bytes(data)


def archive(names=('thinkthen.exe',), *, mode=0o100755, dos=0, binary=None):
    output = io.BytesIO()
    with warnings.catch_warnings(), zipfile.ZipFile(output, 'w') as target:
        warnings.filterwarnings('ignore', message="Duplicate name: 'thinkthen.exe'", category=UserWarning)
        for name in names:
            entry = zipfile.ZipInfo(name)
            entry.external_attr = mode << 16 | dos
            entry.compress_type = zipfile.ZIP_DEFLATED
            target.writestr(entry, pe() if binary is None else binary)
    return output.getvalue()


def cases(root, base):
    result = []

    def case(kind, label, ok, **fields):
        result.append(dict(kind=kind, label=label, ok=ok, **fields))

    def blob(kind, label, ok, data):
        name = f'case-{len(result)}.bin'
        (root / name).write_bytes(data)
        case(kind, label, ok, file=name)

    for value in ('0.2.0', 'v0.2.0', '1.2.3'):
        case('version', 'valid version ' + value, True, value=value)
    for value in ('0.1.2', 'v0.1.9', '0.2', '0.2.0-pre', ' 0.2.0', '0.2.0\n',
                  '../0.2.0', '0.2.0/other', '0.+2.0', '0.2.0.1', '9999999999.2.0'):
        case('version', 'refuse version ' + repr(value), False, value=value)
    for value in ('https://github.com', 'http://127.0.0.1:80', 'http://[::1]:80'):
        case('transport', 'allowed transport ' + value, True, value=value)
    for value in ('http://example.com', 'http://localhost:80', 'ftp://127.0.0.1',
                  'https://user:secret@example.com', 'https://example.com/#fragment'):
        case('transport', 'refused transport', False, value=value)
    case('base', 'base query refused', False, value='https://example.com/?q=a')
    good = pe()
    blob('pe', 'PE64 x86 executable', True, good)
    for label, change in (('x86', (68, '<H', 0x14C)), ('ARM64', (68, '<H', 0xAA64)),
                          ('DLL', (86, '<H', 0x2002)), ('not executable', (86, '<H', 0)),
                          ('PE32', (88, '<H', 0x10B)), ('overflow offset', (60, '<I', 0xFFFFFFFF)),
                          ('outside offset', (60, '<I', 127)), ('short optional', (84, '<H', 0)),
                          ('optional outside bytes', (84, '<H', 1000))):
        data = bytearray(good)
        struct.pack_into(change[1], data, change[0], change[2])
        blob('pe', label, False, data)
    for length in (0, 2, 63, 65, 89):
        blob('pe', f'truncated PE {length}', False, good[:length])
    good_zip = archive()
    blob('zip', 'one regular command ZIP', True, good_zip)
    blob('zip', 'unspecified Unix type', True, archive(mode=0o755))
    for names in ((), ('thinkthen.exe', 'thinkthen.exe'), ('thinkthen.exe', 'extra'),
                  ('folder/thinkthen.exe',), ('thinkthen.exe/',), ('../thinkthen.exe',),
                  ('/thinkthen.exe',), ('C:thinkthen.exe',), ('thinkthen.exe:stream',),
                  ('folder\\thinkthen.exe',), ('THINKTHEN.EXE',)):
        blob('zip', 'refused inventory ' + repr(names), False, archive(names))
    for mode in (0o120777, 0o040755, 0o060600):
        blob('zip', 'special Unix type', False, archive(mode=mode))
    for dos in (0x10, 0x400):
        blob('zip', 'DOS directory or reparse', False, archive(dos=dos))
    blob('zip', 'non-PE command', False, archive(binary=b'not an executable'))
    blob('zip', 'truncated ZIP', False, good_zip[:-10])
    central = good_zip.index(b'PK\x01\x02')
    for label, offset, form, value in (('encrypted', central + 8, '<H', 1),
                                       ('unsupported compression', central + 10, '<H', 99),
                                       ('declared expanded oversize', central + 24, '<I', 134217729),
                                       ('declared compressed oversize', central + 20, '<I', 134217729),
                                       ('actual length mismatch', central + 24, '<I', 129)):
        data = bytearray(good_zip)
        struct.pack_into(form, data, offset, value)
        blob('zip', label, False, data)
    (root / NAME).write_bytes(good_zip)
    digest = hashlib.sha256(good_zip).hexdigest()
    for text in (digest, digest + '  ' + NAME, digest + ' *' + NAME, '\n' + digest + '\r\n\n'):
        case('checksum', 'valid checksum', True, file=NAME, name=NAME, value=text)
    for text in ('', digest + '\n' + digest, digest[:-1], 'g' * 64,
                 digest + ' wrong.zip', digest + ' ' + NAME + ' extra', '0' * 64):
        case('checksum', 'refused checksum', False, file=NAME, name=NAME, value=text)
    for path, limit, ok in (('/good', 20, True), ('/large-length', 20, False),
                            ('/large-stream', 20, False), ('/partial', 20, False),
                            ('/redirect-unsafe', 20, False), ('/redirect-loop', 20, False),
                            ('/retry', 20, True), ('/missing', 20, False)):
        case('download', path, ok, value=base + path, limit=limit)
    case('latest', 'latest skips old draft prerelease and incomplete assets', True,
         value=base + '/available', expected='0.2.3')
    case('latest', 'no Windows release refuses', False, value=base + '/unavailable', expected='')
    return result


class Server(http.server.BaseHTTPRequestHandler):
    calls = []
    command_zip = None
    release_version = None

    def log_message(self, *_args):
        pass

    def do_GET(self):
        self.calls.append(self.path)
        if self.path == '/missing':
            self.send_error(404)
            return
        if self.path == '/retry' and self.calls.count('/retry') % 3 != 0:
            self.send_error(503)
            return
        if self.path.startswith('/redirect'):
            self.send_response(302)
            self.send_header('Location', 'http://example.com/no' if self.path.endswith('unsafe') else '/redirect-loop')
            self.send_header('Content-Length', '0')
            self.send_header("Connection", "close")
            self.end_headers()
            return
        body = b'good'
        name = f'thinkthen-{self.release_version}-x86_64-pc-windows-msvc.zip'
        asset_path = f'/botassembly/thinkthen/releases/download/v{self.release_version}/{name}'
        if self.path == asset_path and self.command_zip is not None:
            body = self.command_zip
        elif self.path == asset_path + '.sha256' and self.command_zip is not None:
            body = (hashlib.sha256(self.command_zip).hexdigest() + '  ' + name).encode()
        if self.path.endswith('/repos/botassembly/thinkthen/releases'):
            def release(version, **extra):
                name = f'thinkthen-{version}-x86_64-pc-windows-msvc.zip'
                return dict(tag_name='v' + version, draft=False, prerelease=False,
                            assets=[dict(name=name), dict(name=name + '.sha256')]) | extra
            releases = [release('0.1.2'), release('0.2.5', draft=True), release('0.2.4', prerelease=True),
                        release('0.2.9', assets=[])]
            if self.path.startswith('/available/'):
                releases.append(release('0.2.3'))
            body = json.dumps(releases).encode()
        elif self.path in ('/large-length', '/large-stream'):
            body = b'x' * 21
        self.send_response(200)
        self.send_header("Connection", "close")
        if self.path != '/large-stream':
            self.send_header('Content-Length', str(10 if self.path == '/partial' else len(body)))
        self.end_headers()
        self.wfile.write(body)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--self-test', action='store_true', help='verify fixture construction and identical installer copies; does not prove Windows execution')
    parser.add_argument('--powershell', action='append', help='host executable; repeat to check both hosts')
    parser.add_argument('--binary', type=Path, help='freshly packed real executable for native Windows boundaries')
    parser.add_argument('--version', default='0.2.0')
    args = parser.parse_args()
    if (REPO / 'install.ps1').read_bytes() != (REPO / 'site/public/install.ps1').read_bytes():
        parser.error('site installer copy differs from root source')
    if args.self_test:
        with tempfile.TemporaryDirectory(prefix='thinkthen-installer-construction-') as temporary:
            root = Path(temporary)
            table = cases(root, 'http://127.0.0.1:1')
            json.loads(json.dumps(table))
            with zipfile.ZipFile(io.BytesIO(archive())) as source:
                assert source.namelist() == ['thinkthen.exe'] and source.read('thinkthen.exe') == pe()
            assert sum(case['kind'] == 'download' for case in table) == 8
            assert any(case['label'] == 'encrypted' and not case['ok'] for case in table)
        print(f'Windows installer fixtures: {len(table)} cases constructed; root/site copies match; Windows execution NOT RUN')
        return
    hosts = args.powershell or ([shutil.which('powershell'), shutil.which('pwsh')] if os.name == 'nt' else [shutil.which('pwsh')])
    if not all(hosts):
        if os.name == 'nt':
            parser.error('native fixtures require both Windows PowerShell 5.1 and PowerShell 7')
        print('Windows installer execution NOT RUN: no PowerShell host. Use --powershell for portable boundary proof.')
        return
    if args.binary and os.name != 'nt':
        parser.error('--binary requires Windows; Linux cannot prove native installation')
    if os.name == 'nt' and not args.binary:
        parser.error('Windows boundary suite requires --binary from a freshly checked release ZIP')
    with tempfile.TemporaryDirectory(prefix='thinkthen-installer-fixtures-') as temporary:
        root = Path(temporary)
        with http.server.ThreadingHTTPServer(('127.0.0.1', 0), Server) as server:
            thread = threading.Thread(target=server.serve_forever, daemon=True)
            thread.start()
            try:
                base = f'http://127.0.0.1:{server.server_port}'
                if args.binary:
                    Server.command_zip = archive(binary=args.binary.read_bytes())
                    Server.release_version = args.version
                table = cases(root, base)
                case_file = root / 'cases.json'
                case_file.write_text(json.dumps(table), encoding='utf-8')
                env = {key: value for key, value in os.environ.items()
                       if not key.upper().startswith('THINKTHEN_') and not key.upper().endswith('_API_KEY')}
                for host in hosts:
                    for mode in ('transaction', 'replacement', 'committed', 'uncommitted'):
                        recovery = subprocess.run([host, '-NoProfile', '-NonInteractive', '-File',
                                                   str(REPO / 'sdlc/scripts/windows-installer-recovery-test.ps1'),
                                                   '-Installer', str(REPO / 'install.ps1'), '-Mode', mode,
                                                   '-FixtureRoot', str(root)], env=env, timeout=30)
                        expected = 1 if mode == 'uncommitted' else 0
                        if recovery.returncode != expected:
                            raise RuntimeError(f'actual recovery AST {mode} exited {recovery.returncode}, expected {expected}')
                    print('Actual entry point AST: committed output/diagnostic failure exits 0; uncommitted failure exits 1; native Windows proof NOT RUN')
                    call = [host, '-NoProfile', '-NonInteractive', '-File', str(REPO / 'sdlc/scripts/windows-installer-boundaries.ps1'),
                            '-Installer', str(REPO / 'install.ps1'), '-Cases', str(case_file), '-FixtureRoot', str(root)]
                    if args.binary:
                        call += ['-NativeBinary', str(args.binary.resolve()), '-ReleaseVersion', args.version, '-ReleaseBase', base]
                    subprocess.run(call, env=env, check=True, timeout=180)
            finally:
                server.shutdown()
                thread.join()


if __name__ == '__main__':
    main()
