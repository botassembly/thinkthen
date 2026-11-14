"""Check transport bytes, bounded child cleanup and portable build inputs."""
import builtins
import base64
import hashlib
import importlib.util
import json
import os
from pathlib import Path
import shlex
import shutil
import subprocess
import sys
import tempfile
from types import SimpleNamespace
from urllib.request import Request, urlopen
from unittest.mock import patch

REQUEST = {'model': 'test', 'questions': {'q1': {'instructions': 'Refund?', 'type': 'noul'}},
           'state': 'Please refund my order.'}


def load_helper(path):
    spec = importlib.util.spec_from_file_location('windows_example', path)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


if len(sys.argv) > 2 and sys.argv[1] in ('--native-fixture', '--local-native-fixture'):
    # The real runner and adapter invoke this SSH actor. Execute the real Python
    # helper, replacing only the native OS/process boundary; never start a VM.
    assert 'THINKTHEN_API_KEY' not in os.environ and 'OPENAI_API_KEY' not in os.environ
    local = sys.argv[1] == '--local-native-fixture'
    if local:
        assert os.environ['SystemRoot'] == '/simulated-system-root'
        assert sys.argv[-2:] == ['-I', sys.argv[2]]
    else:
        command = sys.argv[-1].split(' ')
        assert command[:-1] == ['powershell.exe', '-NoProfile', '-NonInteractive', '-ExecutionPolicy', 'Bypass', '-EncodedCommand']
        payload = base64.b64decode(command[-1], validate=True).decode('utf-16-le')
        quoted_helper = "'" + sys.argv[2].replace("'", "''") + "'"
        assert payload == "& 'python' -I " + quoted_helper + '; exit $LASTEXITCODE'
        port = int(sys.argv[sys.argv.index('-R') + 1].rsplit(':', 1)[1])
        print(f'Allocated port {port} for remote forward', file=sys.stderr, flush=True)
    request = json.loads(sys.stdin.readline())
    helper = load_helper(Path(sys.argv[2]))
    if local:
        assert request['id'] == 'start/decide-branch-ps'
        assert request['source'] == request['binarySource']
        assert request['model'] is None
        assert base64.b64decode(request['programs']['show.ps1']['bytes']) == (Path(sys.argv[2]).parents[1] / 'examples/start/decide-branch-ps/show.ps1').read_bytes()

    class NativeProcess:
        returncode = 0

        def __init__(self, command, *, env, **options):
            assert command[-1].endswith('run.ps1')
            assert env['THINKTHEN_BASE_URL'] == request['recorderUrl']
            assert 'THINKTHEN_URL' not in env and 'OPENAI_API_KEY' not in env
            assert env['THINKTHEN_API_KEY'] == 'docs-loopback'
            self.environment = env

        def communicate(self, **options):
            call = Request(self.environment['THINKTHEN_BASE_URL'] + '/systemone',
                           data=json.dumps(REQUEST).encode(), headers={'Content-Type': 'application/json'})
            with urlopen(call, timeout=5) as response:
                assert json.load(response)['answers']['q1']['type'] == 'noul'
            return (b'send to the refunds queue\r\n' if local else b'refund\r\n'), b''

        def poll(self):
            return self.returncode

    native_os = SimpleNamespace(name='nt', environ={'SYSTEMROOT': '/simulated-windows'},
                                pathsep=os.pathsep, read=os.read)
    with patch.object(helper, 'os', native_os), \
         patch.object(helper.platform, 'platform', return_value='simulated-native-boundary'), \
         patch.object(helper.subprocess, 'Popen', NativeProcess), \
         patch.object(helper.subprocess, 'check_output', return_value=b'{"version":"simulated-native-boundary"}'):
        result = helper.capture(request)
    print(json.dumps(result), flush=True)
    sys.exit(0)


subprocess.run(['node', str(Path(__file__).with_name('windows-example-test.mjs'))], check=True)

adapter = load_helper(Path(__file__).with_name('windows-example.py'))
with subprocess.Popen([sys.executable, '-c', 'import threading; threading.Event().wait()'],
                      stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL) as child:
    with patch.object(adapter.subprocess, 'run', side_effect=subprocess.TimeoutExpired('taskkill', 10)):
        adapter.stop_tree(child, Path('unused-system'))
    assert child.poll() is not None, 'failed taskkill left the direct child alive'

original_import = builtins.__import__


def windows_import(name, *arguments, **options):
    if name == 'fcntl':
        raise ImportError('fcntl is unavailable on Windows')
    return original_import(name, *arguments, **options)


with patch('builtins.__import__', windows_import):
    import release_native
    assert callable(release_native.source_inputs)
    assert callable(release_native.stage_outputs)
print('Windows capture cleanup reaps its child after failed taskkill; source helpers import without fcntl')

# Run the actual runner, recorder, adapter and helper together. The runner alone
# creates THINKTHEN_BASE_URL; fixtures supply no URL or native execution.
repository = Path(__file__).resolve().parents[2]
with tempfile.TemporaryDirectory(prefix="windows recorder O'Brien-") as temporary:
    root = Path(temporary)
    checkout = root / 'checkout'
    site = checkout / 'site'
    for name in ('scripts/run-examples.mjs', 'scripts/windows-example.mjs', 'scripts/windows-example.py',
                 'scripts/capture-backend.mjs', 'src/lib/binary.mjs', 'src/lib/json-values.mjs',
                 'src/lib/examples.mjs', 'src/lib/prepared-examples.mjs', 'src/lib/xlsx.mjs'):
        target = site / name
        target.parent.mkdir(parents=True, exist_ok=True)
        shutil.copyfile(repository / 'site' / name, target)
    (checkout / 'scripts').mkdir()
    for name in ('release_windows_cache.py', 'long_path_scratch.py'):
        shutil.copyfile(repository / 'scripts' / name, checkout / 'scripts' / name)
    (site / 'product.config.mjs').write_text('export default {binary:"thinkthen",binDir:"../bin",versionArgs:["--version"],exampleEnv:{THINKTHEN_BACKEND:"stub"}};')
    (site / 'src/data').mkdir()
    (site / 'src/data/site.json').write_text('{"version":"0.2.0"}')
    (checkout / 'bin').mkdir()
    (checkout / 'bin/thinkthen').write_text('#!/bin/sh\nprintf "thinkthen 0.2.0\\n"\n')
    (checkout / 'bin/thinkthen').chmod(0o700)
    binary = checkout / 'bin/thinkthen.exe'
    binary.write_bytes(b'simulated native executable')
    example = site / 'examples/start/ps'
    example.mkdir(parents=True)
    (example / 'run.ps1').write_bytes(b'& ./show.ps1\r\n')
    (example / 'show.ps1').write_bytes(b"'refund'\r\n")
    subprocess.run(['git', 'init', '--quiet', '--template=', str(checkout)], check=True)
    subprocess.run(['git', '-C', str(checkout), '-c', 'user.name=Fixture', '-c', 'user.email=fixture@example.invalid',
                    'commit', '--quiet', '--allow-empty', '-m', 'Initialize recorder contract fixture'], check=True)
    ssh = root / 'ssh'
    ssh.write_text('#!/bin/sh\nexec ' + ' '.join(map(shlex.quote, [sys.executable, str(Path(__file__).resolve()),
                   '--native-fixture', str(site / 'scripts/windows-example.py')])) + ' "$@"\n')
    ssh.chmod(0o700)
    cache = root / 'responses.json'
    digest = hashlib.sha256(json.dumps(REQUEST, sort_keys=True, separators=(',', ':')).encode()).hexdigest()
    cache.write_text(json.dumps({digest: {'request': REQUEST, 'response': {'answers': {'q1': {'type': 'noul', 'noul': 0.99}}}}}))
    artifacts = root / 'artifacts'
    result = subprocess.run(['node', 'scripts/run-examples.mjs', '--replay', 'start/ps'], cwd=site,
                            env={'PATH': os.environ['PATH'], 'HOME': str(root),
                                 'THINKTHEN_API_KEY': 'upstream-sentinel-stays-on-host',
                                 'OPENAI_API_KEY': 'upstream-sentinel-stays-on-host',
                                 'SITE_CAPTURE_FILE': str(cache), 'SITE_CAPTURE_ARTIFACT_DIR': str(artifacts),
                                 'SITE_WINDOWS_HOST': 'simulated-native', 'SITE_WINDOWS_SSH': str(ssh),
                                 'SITE_WINDOWS_HELPER': str(site / 'scripts/windows-example.py'),
                                 'SITE_WINDOWS_BINARY': str(binary),
                                 'SITE_WINDOWS_BINARY_SHA256': hashlib.sha256(binary.read_bytes()).hexdigest(),
                                 'SITE_WINDOWS_BINARY_SOURCE': 'b' * 40}, capture_output=True, timeout=30)
    diagnostic = artifacts / 'failures/start/ps/stderr.txt'
    assert result.returncode == 0, (result.stdout, result.stderr,
                                  diagnostic.read_bytes() if diagnostic.exists() else b'')
    manifest = json.loads((artifacts / 'manifest.json').read_text())
    assert manifest['passed_ids'] == ['start/ps'] and manifest['failures'] == []
    assert manifest['calls'] == 0 and manifest['bytes'] == 0
    assert (example / 'output.txt').read_bytes() == b'refund\r\n'
    assert (example / 'exit').read_text() == '0\n'
    assert json.loads((example / 'capture.json').read_text())['answers'] == 1
    print('Real runner -> adapter -> helper/cache environment replay serves one cached loopback answer with zero paid calls/bytes; native process boundary simulated.')

    # Exercise the native-local qualification through the existing helper. Only
    # its OS/process boundary is simulated; these disposable replies are NOT docs evidence.
    native_example = site / 'examples/start/decide-branch-ps'
    shutil.copytree(repository / 'site/examples/start/decide-branch-ps', native_example)
    expected = site / 'examples/start/decide-branch'
    expected.mkdir()
    (expected / 'capture.json').write_text('{"backend":"systemone","answers":1,"date":"fixture"}')
    (expected / 'output.txt').write_bytes(b'send to the refunds queue\n')
    (expected / 'stderr.txt').write_bytes(b'')
    (expected / 'exit').write_text('0\n')
    python = root / 'native-python'
    python.write_text('#!/bin/sh\nexec ' + ' '.join(map(shlex.quote, [sys.executable, str(Path(__file__).resolve()),
                      '--local-native-fixture', str(site / 'scripts/windows-example.py')])) + ' "$@"\n')
    python.chmod(0o700)
    source = subprocess.check_output(['git', '-C', str(checkout), 'rev-parse', 'HEAD'], text=True).strip()
    def qualify(cache_path=cache, revision=source):
        destination = root / ('native-' + str(len(list(root.glob('native-*')))))
        result = subprocess.run(['node', 'scripts/windows-example.mjs', '--binary', str(binary),
            '--source', revision, '--cache', str(cache_path), '--evidence', str(destination)], cwd=site,
            env={'PATH': os.environ['PATH'], 'SystemRoot': '/simulated-system-root', 'SITE_WINDOWS_PYTHON': str(python),
                 'THINKTHEN_API_KEY': 'sentinel-must-not-travel', 'OPENAI_API_KEY': 'sentinel-must-not-travel'},
            capture_output=True)
        return result, destination
    original = cache.read_bytes()
    result, destination = qualify()
    assert result.returncode == 0, result.stderr
    evidence = json.loads((destination / 'windows-example.json').read_text())
    assert evidence['served'] == 1 and evidence['failure'] is None
    assert [evidence[key] for key in ('calls', 'bytes', 'inputBytes')] == [0, 0, 0]
    assert evidence['exit'] == 0 and evidence['timedOut'] is False
    assert evidence['provenance']['binary_sha256'] == hashlib.sha256(binary.read_bytes()).hexdigest()
    assert evidence['provenance']['binary_source'] == source
    assert evidence['provenance']['program_sha256']['show.ps1'] == hashlib.sha256((native_example / 'show.ps1').read_bytes()).hexdigest()
    assert (destination / 'stdout.txt').read_bytes() == b'send to the refunds queue\r\n'
    assert cache.read_bytes() == original
    result, _ = qualify(root / 'absent-real-cache.json')
    assert result.returncode != 0 and b'ENOENT' in result.stderr
    result, _ = qualify(revision='a' * 40)
    assert result.returncode != 0 and b'differs from the candidate source' in result.stderr
    missing = root / 'missing-entry.json'
    missing.write_text('{}')
    result, destination = qualify(missing)
    assert result.returncode != 0
    evidence = json.loads((destination / 'windows-example.json').read_text())
    assert evidence['failure'] == 'saved response is missing' and evidence['served'] == 0
    assert [evidence[key] for key in ('calls', 'bytes', 'inputBytes')] == [0, 0, 0]
    assert missing.read_text() == '{}'
    (expected / 'output.txt').write_bytes(b'wrong captured branch\n')
    result, _ = qualify()
    assert result.returncode != 0 and b'differs from the recorded capture' in result.stderr
    print('Native-local replay hook preserves payload/identities, refuses missing source/cache/entry and wrong output; zero paid calls, simulated native process only.')
