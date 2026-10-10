"""Run focused public calls from an extracted native-bearing PHP archive."""
import argparse
import hashlib
import json
from pathlib import Path
import shutil
import subprocess
import sys
import tarfile
import tempfile

ROOT = Path(__file__).resolve().parents[3]
sys.path.insert(0, str(ROOT / 'conformance/children'))
from children import child_env
from backend import Backend


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--archive', type=Path, required=True)
    parser.add_argument('--composer', type=Path, required=True)
    parser.add_argument('--parity', action='store_true')
    args = parser.parse_args()
    with tempfile.TemporaryDirectory(prefix='thinkthen-php-session-') as folder:
        work = Path(folder)
        app = work / 'app'; app.mkdir()
        with tarfile.open(args.archive) as archive:
            metadata = json.load(archive.extractfile('composer.json'))
        metadata.update(version='0.2.0', dist={'type': 'tar', 'url': args.archive.resolve().as_uri()})
        (app / 'composer.json').write_text(json.dumps({'repositories': [{'type': 'package', 'package': metadata}],
            'require': {'botassembly/thinkthen': '0.2.0'}}))
        (work / 'barrier').mkdir()
        (work / 'home').mkdir()
        install_env = child_env(home=work / 'home', LANG='C.UTF-8', LC_ALL='C.UTF-8',
            COMPOSER_DISABLE_NETWORK='1', COMPOSER_HOME=str(work / 'composer-home'))
        installed = subprocess.run(['/usr/bin/php8.3', '-n', '-d', 'extension=ffi', '-d', 'extension=phar',
            '-d', 'extension=iconv', str(args.composer.resolve()),
            'install', '--no-plugins', '--no-scripts', '--no-dev', '--no-interaction', '--no-progress'],
            cwd=app, env=install_env, capture_output=True, timeout=5)
        assert installed.returncode == 0, (installed.stdout, installed.stderr)
        shutil.copyfile(Path(__file__).with_name('session_consumer.php'), work / 'consumer.php')
        server = Backend(work / 'barrier')
        try:
            env = child_env(home=work / 'home', LANG='C.UTF-8', LC_ALL='C.UTF-8',
                TT_AUTOLOAD=str(app / 'vendor/autoload.php'), TT_BARRIER=str(work / 'barrier'),
                THINKTHEN_BASE_URL=f'http://127.0.0.1:{server.server_port}/generic/v1',
                THINKTHEN_API_KEY='tt-canary-291')
            for mode in (() if args.parity else ('surface', 'poll', 'named', 'presence', 'failure', 'zero', 'destroy')):
                before = server.attempts
                result = subprocess.run(['/usr/bin/php8.3', '-n', '-d', 'extension=ffi', '-d', 'ffi.enable=1',
                    str(work / 'consumer.php'), mode], capture_output=True, env=env, timeout=5)
                assert result.returncode == 0, (mode, result.stdout, result.stderr)
                assert ('PHP_SESSION_PASS ' + mode).encode() in result.stdout
                if mode == 'surface':
                    assert server.attempts == before + 1, 'attributed call request count'
                    agents = [json.loads(line) for line in (work / 'barrier/user-agents.jsonl').read_text().splitlines()]
                    assert agents == ['thinkthen/0.2.0 (php)'], agents
                if mode == 'poll': assert server.attempts == before + 3, 'poll generator request count'
                if mode == 'zero': assert server.attempts == before, 'rejected input sent a request'
                print(result.stdout.decode(), end='')
            env = child_env(home=work / 'home', keep=('PATH', 'LANG', 'LC_ALL'),
                THINKTHEN_PARITY_PACKAGE=str(app / 'vendor/botassembly/thinkthen'),
                THINKTHEN_COMPLETE_LIBRARY=str(app / 'vendor/botassembly/thinkthen/native/libthinkthen.so'),
                THINKTHEN_TEST_PROFILE='full' if args.parity else 'routine')
            subprocess.run([sys.executable, str(Path(__file__).with_name('complete_parity.py')), 'php'],
                env=env, check=True)
        finally:
            for file in (work / 'barrier').glob('arrived-*'):
                file.with_name(file.name.replace('arrived-', 'release-', 1)).touch()
            server.close()
    print('Installed archive SHA256', hashlib.sha256(args.archive.read_bytes()).hexdigest())


if __name__ == '__main__':
    main()
