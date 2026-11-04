"""Run exact authored PowerShell bytes with a loopback-only docs environment."""
import base64
import hashlib
import json
import os
from pathlib import Path
import platform
import subprocess
import sys
import tempfile
import threading
from urllib.parse import urlparse

sys.path.insert(0, str(Path(__file__).resolve().parents[2] / 'scripts'))
from release_windows_cache import cache_environment


def digest(data):
    return hashlib.sha256(data).hexdigest()


def stop_tree(process, system):
    """Bound descendant cleanup, then kill and reap the direct child as fallback."""
    if process.poll() is not None:
        return
    try:
        subprocess.run([system / 'taskkill.exe', '/PID', str(process.pid), '/T', '/F'],
                       stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL, timeout=10, check=False)
    except (OSError, subprocess.TimeoutExpired):
        pass  # The direct child is still killed and reaped below.
    if process.poll() is None:
        process.kill()
    process.wait(timeout=5)


def capture(request):
    if os.name != 'nt':
        raise RuntimeError('PowerShell capture requires native Windows')
    if digest(Path(__file__).read_bytes()) != request['helperHash']:
        raise ValueError('Windows capture helper differs from the requested source')
    cache_helper = Path(__file__).resolve().parents[2] / 'scripts/release_windows_cache.py'
    if digest(cache_helper.read_bytes()) != request['cacheHelperHash']:
        raise ValueError('Windows cache helper differs from the requested source')
    binary = Path(request['binary']).resolve(strict=True)
    if binary.name.lower() != 'thinkthen.exe' or digest(binary.read_bytes()) != request['binaryHash']:
        raise ValueError('Windows CLI differs from its proven build')
    url = urlparse(request['recorderUrl'])
    if url.scheme != 'http' or url.hostname != '127.0.0.1' or not url.port or url.path:
        raise ValueError('Windows capture needs the forwarded loopback recorder')
    system = Path(os.environ['SYSTEMROOT']) / 'System32'
    shell = system / 'WindowsPowerShell/v1.0/powershell.exe'
    with tempfile.TemporaryDirectory(prefix='thinkthen-docs-') as temporary:
        root = Path(temporary)
        for name in ('home', 'cache', 'config', 'tmp', 'program'):
            (root / name).mkdir()
        program = root / 'program'
        hashes = {}
        if set(request['programs']) != {'run.ps1', 'show.ps1'}:
            raise ValueError('Windows capture needs the authored run and show programs')
        for name, entry in request['programs'].items():
            data = base64.b64decode(entry['bytes'], validate=True)
            if digest(data) != entry['sha256']:
                raise ValueError('Authored PowerShell bytes differ')
            (program / name).write_bytes(data)
            hashes[name] = digest(data)
        environment = {name: os.environ[name] for name in ('SYSTEMROOT', 'WINDIR', 'COMSPEC', 'PATHEXT') if name in os.environ}
        environment.update(PATH=os.pathsep.join(map(str, (binary.parent, system, shell.parent))),
                           HOME=str(root / 'home'), USERPROFILE=str(root / 'home'),
                           APPDATA=str(root / 'config'), XDG_CACHE_HOME=str(root / 'cache'),
                           XDG_CONFIG_HOME=str(root / 'config'), XDG_DATA_HOME=str(root / 'home/data'),
                           TEMP=str(root / 'tmp'), TMP=str(root / 'tmp'), TMPDIR=str(root / 'tmp'),
                           LANG='C.UTF-8', TZ='UTC', NO_COLOR='1', TERM='dumb', COLUMNS='100',
                           THINKTHEN_BACKEND='systemone', THINKTHEN_BASE_URL=request['recorderUrl'],
                           THINKTHEN_API_KEY='docs-loopback')
        if request.get('model') is not None:
            environment['THINKTHEN_MODEL'] = request['model']
        environment = cache_environment(root / 'cache', environment)
        observed = subprocess.check_output([shell, '-NoProfile', '-NonInteractive', '-Command',
            '[pscustomobject]@{version=$PSVersionTable.PSVersion.ToString();'
            'outputCodePage=[Console]::OutputEncoding.CodePage;'
            'pipelineCodePage=$OutputEncoding.CodePage}|ConvertTo-Json -Compress'], env=environment, timeout=10)
        command = [str(shell), '-NoProfile', '-NonInteractive', '-ExecutionPolicy', 'Bypass', '-File', str(program / 'run.ps1')]
        process = subprocess.Popen(command, cwd=program, env=environment, stdin=subprocess.DEVNULL,
                                   stdout=subprocess.PIPE, stderr=subprocess.PIPE)
        cleanup_lock = threading.Lock()
        finished = threading.Event()

        def cleanup():
            with cleanup_lock:
                stop_tree(process, system)

        def disconnected():
            # The host keeps stdin open until SSH exits. EOF cancels its child.
            # Abrupt interpreter loss is cleaned by the VM owner's shutdown.
            os.read(sys.stdin.fileno(), 1)
            if not finished.is_set():
                cleanup()

        threading.Thread(target=disconnected, daemon=True).start()
        timed_out = False
        try:
            stdout, stderr = process.communicate(timeout=request['timeoutMs'] / 1000)
        except subprocess.TimeoutExpired:
            timed_out = True
            cleanup()
            stdout, stderr = process.communicate(timeout=5)
        finally:
            try:
                cleanup()
            finally:
                finished.set()
        return {'stdout': base64.b64encode(stdout).decode('ascii'),
                'stderr': base64.b64encode(stderr).decode('ascii'),
                'exit': 124 if timed_out else process.returncode, 'timedOut': timed_out,
                'provenance': {'source': request['source'], 'program_sha256': hashes,
                               'binary_source': request['binarySource'], 'binary_sha256': request['binaryHash'],
                               'os': platform.platform(), 'shell': str(shell),
                               'powershell': json.loads(observed.decode('utf-8-sig'))}}


if __name__ == '__main__':
    result = capture(json.loads(sys.stdin.readline()))
    sys.stdout.write(json.dumps(result, ensure_ascii=True) + '\n')
