"""Exercise installed CLI diagnostics against the shared fault arms."""
import json
from pathlib import Path
import subprocess
import sys
import tempfile

from children import fixture_run, fixture_wait


def main():
    binary, server = map(str, map(Path, sys.argv[1:]))
    backend = subprocess.Popen([server], stdin=subprocess.PIPE, stdout=subprocess.PIPE,
                               stderr=subprocess.PIPE, text=True, env={})
    try:
        port = backend.stdout.readline().strip()
        assert port.isdigit(), port
        arms = [(f'status/{code}', ['decide', 'Is it?'], 'status', code)
                for code in (401, 402, 403, 404)]
        arms.append(('refuse', ['decide', 'Is it?'], 'status', 422))
        arms.extend((f'malformed/{cause}', ['choose', 'Which?', 'a', 'b'], 'reply', None)
                    for cause in ('missing_answer', 'wrong_kind', 'missing_probability',
                                  'invalid_probability', 'invalid_distribution',
                                  'unexpected_probability'))
        with tempfile.TemporaryDirectory() as home:
            for arm, command, cause, status in arms:
                result = fixture_run([binary, *command, '--url',
                                         f'http://127.0.0.1:{port}/arm/{arm}/v1',
                                         '--no-cache', '--max-retries', '0', '--facts'],
                                        input='private evidence', capture_output=True, text=True,
                                        env={'HOME': home, 'THINKTHEN_API_KEY': 'sk-loopback-arms'})
                assert result.returncode == 4, (arm, result)
                assert result.stdout == '', (arm, result.stdout)
                lines = result.stderr.splitlines()
                assert len(lines) == 2, (arm, lines)
                facts = json.loads(lines[1])
                assert facts['stopped']['cause'] == cause, (arm, facts)
                assert facts['stopped'].get('status') == status, (arm, facts)
                for marker in ('sk-loopback-arms', 'c2stbG9vcGJhY2stYXJtcw',
                               'sk%2Dloopback%2Darms', '736b2d6c6f6f706261636b2d61726d73',
                               'Authorization', 'fixture-header', 'fixturelongtoken',
                               'private evidence', 'provider message'):
                    assert marker not in result.stderr, (arm, marker)
                prefix = 'thinkthen: the reply was refused:' if cause == 'reply' else 'thinkthen: the backend '
                assert lines[0].startswith(prefix), (arm, lines)
        backend.stdin.close()
        assert fixture_wait(backend) == 0
        assert backend.stderr.read() == ''
        print(f'Installed provider diagnostic boundary: {len(arms)} fault arms passed')
    finally:
        if backend.poll() is None:
            backend.kill()
            backend.wait()


if __name__ == '__main__':
    main()
