#!/usr/bin/env python3
"""Compile and run the downloaded Windows C archive's consumer, without a source DLL."""
import argparse
import http.server
import importlib.util
import json
import os
from pathlib import Path
import shutil
import tempfile
import threading

SPEC = importlib.util.spec_from_file_location('windows_c', Path(__file__).with_name('release-windows-c.py'))
C = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(C)


def consume(archive):
    contents = C.check(archive)
    with tempfile.TemporaryDirectory(prefix='thinkthen-c-consumer-') as temporary:
        root = Path(temporary)
        for name, data in zip(C.MEMBERS, contents):
            path = root / name
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_bytes(data)
        binary = root / 'consumer.exe'
        shutil.copyfile(root / 'bin/thinkthen.dll', root / 'thinkthen.dll')
        C.run(['cl.exe', '/nologo', '/TC', '/std:c11', '/W4', '/WX', '/MD', '/D_CRT_SECURE_NO_WARNINGS',
               '/I' + str(root / 'include'), str(C.REPO / 'libraries/c/tests/c/driver.c'),
               '/Fo' + str(root / 'consumer.obj'), '/Fe' + str(binary), '/link',
               str(root / 'lib/thinkthen.dll.lib')])
        C.inspect(root, binary)
        env = {name: value for name, value in C.environment().items()
               if name not in ('INCLUDE', 'LIB', 'LIBPATH')}
        # Only the public DLL beside the executable can satisfy the C import.
        env.update(PATH=str(root), HOME=str(root / 'home'), APPDATA=str(root / 'Roaming'),
                   LOCALAPPDATA=str(root / 'Local'), THINKTHEN_API_KEY='sk-c-archive-loopback')
        requests = []

        class Backend(http.server.BaseHTTPRequestHandler):
            def do_POST(self):
                body = self.rfile.read(int(self.headers['Content-Length']))
                request = json.loads(body)
                requests.append(request)
                response = json.dumps({'model': 'jev-1.13.0', 'answers': {
                    name: {'type': 'noul', 'noul': 0.9} for name in request['questions']}}).encode()
                self.send_response(200)
                self.send_header("Connection", "close")
                self.send_header('Content-Length', str(len(response)))
                self.end_headers()
                self.wfile.write(response)

            def log_message(self, *_args):
                pass

        with http.server.ThreadingHTTPServer(('127.0.0.1', 0), Backend) as server:
            thread = threading.Thread(target=server.serve_forever, daemon=True)
            thread.start()
            try:
                base = f'http://127.0.0.1:{server.server_port}/generic/v1'
                fields = (base, '{"decide":"asks for a refund"}', 'Refund me.')
                payload = ('decide 3\n' + ''.join(f'{len(field.encode())}\n{field}\n' for field in fields)).encode()
                refusal = "max_estimated_input_tokens_total=10 (encoded-body-bytes-908-v1) would be exceeded before this call's first request"
                for capped in (True, False):
                    child_env = env | {'THINKTHEN_CACHE': str(root / ('capped' if capped else 'open'))}
                    if capped:
                        child_env['THINKTHEN_MAX_ESTIMATED_INPUT_TOKENS_TOTAL'] = '10'
                    result = C.CAPTURE([str(binary)], input=payload, env=child_env, timeout=60)
                    if result.returncode != 0 or result.stderr or b'sk-c-archive-loopback' in result.stdout:
                        raise ValueError('downloaded C consumer failed or exposed its key')
                    expected = f'1 {len(refusal)}\n{refusal}\n'.encode() if capped else b'0 21\n1 0.90000000000000002\n'
                    if result.stdout != expected or len(requests) != (0 if capped else 1):
                        raise ValueError('downloaded C consumer refusal/result or counted requests differ')
            finally:
                server.shutdown()
                thread.join(timeout=10)
        print('Windows C archive smoke: exact spend refusal sent 0 requests; successful call sent 1')


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('archive', type=Path)
    args = parser.parse_args()
    if os.name != 'nt':
        parser.error('Windows C archive execution requires Windows')
    consume(args.archive)

if __name__ == '__main__':
    main()
