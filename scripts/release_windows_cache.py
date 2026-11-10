"""Exercise Windows cache security through installed public consumers."""
import ctypes
import json
from functools import partial
import os
from pathlib import Path
import signal
import subprocess
import sys
import threading
from contextlib import nullcontext

from long_path_scratch import EXTENDED, extended


# Deep cache checks aim past Windows' 260-character path limit with room to spare.
DEEP_PATH = 270


def deep_name(prefix, parent, suffix):
    """Name a folder under parent so parent/name/suffix passes 260 characters on any runner."""
    padding = DEEP_PATH - len(str(parent)) - len(str(suffix)) - 2 - len(prefix)
    name = prefix + 'x' * max(0, padding)
    if len(name) > 255:
        raise ValueError(f'deep cache folder name passes 255 characters: {len(name)}')
    return name


def cache_environment(root, environment):
    """Keep PowerShell's known-folder fallback out of the checked package."""
    root = Path(root).resolve()
    powershell = root / 'Microsoft/Windows/PowerShell'
    powershell.mkdir(parents=True, exist_ok=True)
    environment = {key: value for key, value in environment.items()
                   if key.upper() not in {'LOCALAPPDATA', 'PSMODULEANALYSISCACHEPATH'}}
    return dict(environment, LOCALAPPDATA=str(root),
                PSModuleAnalysisCachePath=str(powershell / 'ModuleAnalysisCache'))


def run_consumer(command, environment, *, cwd=None, timeout=600):
    """Forward installed consumer output and retain diagnostics for cache assertions."""
    chunks = []
    errors = []
    if os.name == 'nt':
        from windows_caller_process import OwnedJob
        owner = OwnedJob()
    else:
        owner = nullcontext()
    with owner as job:
        options = dict(env=environment, cwd=cwd, stdout=subprocess.PIPE, stderr=subprocess.STDOUT)
        process = (job.start(list(map(str, command)), **options) if job else
                   subprocess.Popen(list(map(str, command)), start_new_session=True, **options))

        def forward():
            try:
                while data := process.stdout.read1(65536):
                    sys.stdout.buffer.write(data)
                    sys.stdout.buffer.flush()
                    chunks.append(data)
            except Exception as error:  # noqa: BLE001 - propagate the reader failure to the owner
                errors.append(error)

        reader = threading.Thread(target=forward, daemon=True)
        reader.start()
        try:
            status = process.wait(timeout=timeout)
        finally:
            # Reap descendants before draining: a grandchild may hold stdout open.
            if job:
                job.close()
            else:
                try:
                    os.killpg(process.pid, signal.SIGKILL)
                except ProcessLookupError:
                    pass
            process.wait()
            reader.join(timeout=30)
            if reader.is_alive():
                raise RuntimeError('Consumer output did not close after process-tree cleanup')
            process.stdout.close()
        if errors:
            raise errors[0]
    return status, b''.join(chunks).decode('utf-8', errors='replace')


def _control(environment, script, path, **values):
    powershell = Path(environment['SYSTEMROOT']) / 'System32/WindowsPowerShell/v1.0/powershell.exe'
    # A pwsh parent's module path hides Windows PowerShell's own Get-Acl,
    # just as in the caller-token probe. Let this runtime select its modules.
    selected = {key: value for key, value in (environment | values).items()
                if key.upper() != 'PSMODULEPATH'}
    result = subprocess.run([powershell, '-NoProfile', '-NonInteractive', '-Command',
        "$ErrorActionPreference='Stop'; " + script],
        env=selected | {'TT_CACHE_PATH': str(path)}, capture_output=True, text=True, check=False,
        timeout=600)
    if result.returncode:
        detail = result.stderr or '<no stderr>'
        for name, value in selected.items():
            if value and any(word in name.upper() for word in ('KEY', 'TOKEN', 'SECRET', 'PASSWORD', 'CREDENTIAL')):
                detail = detail.replace(value, '<redacted>')
        detail = detail.encode('utf-8', errors='replace')[:4096].decode('utf-8', errors='replace')
        raise RuntimeError(f'Windows ACL control failed for {path.name}: exit {result.returncode}; '
                           f'stderr: {detail}') from None
    return result.stdout.strip()


STATE_FUNCTION = """function Get-CacheState {
  $a = Get-Acl -LiteralPath $env:TT_CACHE_PATH;
  [pscustomobject]@{
    owner = $a.GetOwner([System.Security.Principal.SecurityIdentifier]).Value;
    access = $a.GetSecurityDescriptorSddlForm([System.Security.AccessControl.AccessControlSections]::Access)
  }
};
"""


def _state(environment, path):
    return json.loads(_control(environment, STATE_FUNCTION + 'Get-CacheState | ConvertTo-Json -Compress', path))


def private_root(root, environment):
    """Give the root the owner and access a user's own local app data folder has."""
    root.mkdir(parents=True, exist_ok=True)
    result = json.loads(_control(environment, STATE_FUNCTION + '''$i = [System.Security.Principal.WindowsIdentity]::GetCurrent();
$identity = [pscustomobject]@{ caller = $i.User.Value; defaultOwner = $i.Owner.Value };
$inherited = Get-CacheState;
$sid = $i.User;
$acl = New-Object System.Security.AccessControl.DirectorySecurity;
$acl.SetOwner($sid);
$acl.SetAccessRuleProtection($true, $false);
foreach ($principal in @($sid.Value, 'S-1-5-18', 'S-1-5-32-544')) {
  $principalSid = New-Object System.Security.Principal.SecurityIdentifier($principal);
  $rule = New-Object System.Security.AccessControl.FileSystemAccessRule(
    $principalSid, 'FullControl', 'ContainerInherit,ObjectInherit', 'None', 'Allow');
  $acl.AddAccessRule($rule);
}
Set-Acl -LiteralPath $env:TT_CACHE_PATH -AclObject $acl;
$baseline = Get-CacheState;
$acl = Get-Acl -LiteralPath $env:TT_CACHE_PATH;
$caller = $i.User.Value;
$allowed = @($caller, 'S-1-5-18', 'S-1-5-32-544');
$rules = @($acl.Access);
$valid = $acl.AreAccessRulesProtected -and $rules.Count -eq $allowed.Count;
foreach ($rule in $rules) {
  $sid = $rule.IdentityReference.Translate([System.Security.Principal.SecurityIdentifier]).Value;
  $valid = $valid -and ($sid -in $allowed) -and $rule.AccessControlType -eq 'Allow' -and
    $rule.FileSystemRights -eq 'FullControl' -and $rule.InheritanceFlags -eq 'ContainerInherit,ObjectInherit';
}
[pscustomobject]@{identity=$identity; inherited=$inherited; baseline=$baseline; verified=$valid} |
  ConvertTo-Json -Depth 4 -Compress''', root))
    identity, baseline = result['identity'], result['baseline']
    print('Windows cache baseline identity: ' + json.dumps(identity), flush=True)
    print('Windows cache inherited root: ' + json.dumps(result['inherited']), flush=True)
    assert baseline['owner'] == identity['caller'], 'private cache root owner is not the caller'
    assert result['verified'] is True, 'private cache root DACL was not established'
    print('Windows cache private root verified: ' + json.dumps(baseline), flush=True)
    return identity


def prepare_cache(root, environment):
    """Establish the private cache prerequisite shared by installed consumers."""
    identity = private_root(root, environment)
    state = partial(_state, environment)
    probe = root / '.owner-probe'
    probe.mkdir()
    file = probe / 'file'
    try:
        file.write_bytes(b'owner probe')
        for path in (probe, file):
            actual = state(path)
            print('Windows cache creation prerequisite: ' + json.dumps(actual), flush=True)
            assert actual['owner'] == identity['caller'], ('Windows runner creates cache objects with a distinct owner; '
                'a caller-owned process token is required: ' + str(path))
    finally:
        if file.exists(): file.unlink()
        probe.rmdir()
    return identity


def _token_run(token, *, consumer, env, log):
    """Run a consumer from this token or as a normal user; return its exit and whether the cache refused it."""
    from windows_caller_process import launch
    if token == 'elevated':
        status, output = run_consumer(consumer['command'], env)
    else:
        try:
            launch(consumer['command'], cwd=Path.cwd(), env=env, normal_log=log)
            status = 0
        except SystemExit as error:
            status = error.code
        output = log.read_text(encoding='utf-8', errors='replace')
    refused = 'cache owner or permissions' in output.lower()
    print('Windows elevated cache: ' + json.dumps(dict(consumer=Path(consumer['root']).name, run=token,
                                                       exit=status, cache_refused=refused)), flush=True)
    return status, refused


def _check_elevated_consumer(consumer):
    root = Path(consumer['root'])
    filename = consumer['filename']
    env = cache_environment(root, consumer['environment'])
    identity = private_root(root, env)
    assert identity['defaultOwner'] == 'S-1-5-32-544', 'not a real elevated token: ' + json.dumps(identity)
    run = partial(_token_run, consumer=consumer, env=env, log=root.with_name(root.name + '-normal.log'))
    assert run('elevated')[0] == 0, 'elevated run failed'
    for path in [root / 'thinkthen', *sorted(root.glob('thinkthen/**/*'))]:
        print('Windows elevated cache object ' + str(path) + ': ' + json.dumps(_state(env, path)), flush=True)
    asset, = root.glob('thinkthen/native/*/' + filename)
    written = asset.stat().st_mtime_ns
    assert _state(env, asset)['owner'] == 'S-1-5-32-544', 'elevated run did not leave an Administrators-owned library'
    assert run('normal')[0] == 0, 'normal run failed on the elevated cache'
    # A plain file error falls back to temp, which would answer without the elevated cache.
    assert asset.stat().st_mtime_ns == written, 'normal run replaced the elevated library'
    assert _state(env, asset)['owner'] == 'S-1-5-32-544', 'normal run changed the library owner'
    copies = list(Path(env['TEMP']).glob('thinkthen-*/thinkthen/native/*/' + filename))
    assert not copies, 'normal run answered from temp: ' + str(copies)
    subprocess.run([Path(env['SYSTEMROOT']) / 'System32/icacls.exe', asset, '/grant', '*S-1-1-0:(M)'],
                   env=env, capture_output=True, text=True, check=True)
    for token in ('elevated', 'normal'):
        status, refused = run(token)
        assert status and refused, token + ' run accepted an Administrators-owned library Everyone can modify'


def check_elevated(consumers):
    """Prove each consumer from this real elevated token, then as a normal user on the same cache."""
    failures = []
    for consumer in consumers:
        try:
            _check_elevated_consumer(consumer)
        except Exception as error:  # noqa: BLE001 - report every consumer before failing the step
            failures.append(Path(consumer['root']).name + ': ' + str(error))
            print('Windows elevated cache: FAIL: ' + failures[-1], flush=True)
    if failures:
        raise RuntimeError('Windows elevated cache runs failed: ' + '; '.join(failures))


def restore_cache(path, saved, environment):
    subprocess.run([Path(environment['SYSTEMROOT']) / 'System32/icacls.exe', path,
                    '/setowner', '*' + saved['owner']], env=environment,
                   capture_output=True, text=True, check=True, timeout=600)
    restored = json.loads(_control(environment, STATE_FUNCTION + '''$a = Get-Acl -LiteralPath $env:TT_CACHE_PATH;
$a.SetSecurityDescriptorSddlForm($env:TT_CACHE_ACCESS, [System.Security.AccessControl.AccessControlSections]::Access);
Set-Acl -LiteralPath $env:TT_CACHE_PATH -AclObject $a;
Get-CacheState | ConvertTo-Json -Compress''', path, TT_CACHE_ACCESS=saved['access']))
    assert restored == saved, 'cache security was not restored: ' + str(path)


def check_cache(command, root, environment, filename):
    root.mkdir()
    env = cache_environment(root, environment)
    system = Path(env['SYSTEMROOT']) / 'System32'

    control = partial(_control, env)
    state = partial(_state, env)

    def icacls(path, *args):
        subprocess.run([system / 'icacls.exe', path, *args], env=env,
                       capture_output=True, text=True, check=True)

    restore = partial(restore_cache, environment=env)

    def consumer(rejected=False):
        status, output = run_consumer(command, env)
        if rejected:
            assert status != 0, 'consumer accepted unsafe cache: ' + output
            assert 'cache owner or permissions' in output.lower(), output
        elif status:
            raise RuntimeError('installed cache consumer failed: ' + output)

    def root_fallback(label):
        """Check that a consumer leaves an unsafe root for temp: a safe temp answers, an unsafe one refuses."""
        # A deep safe temp puts the library past 260 characters on every runner, as a deep %TEMP% does.
        # The consumer's temp folder is named thinkthen-*, so thinkthen-x is its shortest spelling.
        safe = root.with_name(deep_name(label[0] + 's', root.parent, Path('thinkthen-x') / asset.relative_to(root)))
        unsafe = root.with_name(label[0] + 'u')
        private_root(safe, env)
        private_root(unsafe, env)
        icacls(unsafe, '/grant', '*S-1-1-0:(OI)(CI)(M)')
        status, output = run_consumer(command, env | {'TEMP': str(safe), 'TMP': str(safe)})
        assert status == 0, 'consumer did not answer from a safe temp: ' + output
        copies = list(Path(extended(safe)).glob('thinkthen-*/thinkthen/native/*/' + filename))
        assert len(copies) == 1, f'consumer answered without the temp cache: {copies}'
        assert len(str(copies[0]).removeprefix(EXTENDED)) > 260, f'the temp cache is not past 260 characters: {copies[0]}'
        status, output = run_consumer(command, env | {'TEMP': str(unsafe), 'TMP': str(unsafe)})
        assert status != 0, 'consumer accepted an unsafe temp: ' + output
        assert 'cache owner or permissions' in output.lower(), output
        print('Windows cache: unsafe root answered from safe temp and refused in unsafe temp: ' + label, flush=True)

    def messy_deep_root():
        """Check that a deep cache root spelled with `/`, `.` and `..` still loads the engine."""
        deep = root.with_name(deep_name('d', root.parent, asset.relative_to(root)))
        private_root(deep, env)
        messy = deep.parent.as_posix() + '/./detour/../' + deep.name
        status, output = run_consumer(command, env | {'LOCALAPPDATA': messy})
        assert status == 0, 'consumer did not answer from a messy deep cache root: ' + output
        copies = list(Path(extended(deep)).glob('thinkthen/native/*/' + filename))
        assert len(copies) == 1, f'consumer answered without the messy deep cache: {copies}'
        assert len(str(copies[0]).removeprefix(EXTENDED)) > 260, f'the deep cache is not past 260 characters: {copies[0]}'
        print('Windows cache: deep root spelled with /, . and .. answered: ' + messy, flush=True)

    def refused(path, label):
        # The root belongs to other programs too, so every consumer falls back there.
        if path == root:
            root_fallback(label)
        else:
            consumer(rejected=True)

    identity = prepare_cache(root, env)
    try:
        consumer()
    except RuntimeError:
        for path in sorted(root.rglob('*')):
            print('Windows cache baseline child ' + str(path) + ': ' + json.dumps(state(path)), flush=True)
        raise
    asset, = root.glob('thinkthen/native/*/' + filename)
    messy_deep_root()
    paths = (asset, asset.parent, asset.parent.parent, root / 'thinkthen', root)
    caller = identity['caller']
    for path in paths:
        saved = state(path)
        try:
            icacls(path, '/grant', '*S-1-1-0:(M)')
            allowed = control('''$a = Get-Acl -LiteralPath $env:TT_CACHE_PATH;
@($a.Access | Where-Object {
  $_.IdentityReference.Translate([System.Security.Principal.SecurityIdentifier]).Value -eq 'S-1-1-0' -and
  $_.AccessControlType -eq 'Allow' -and
  ($_.FileSystemRights -band [System.Security.AccessControl.FileSystemRights]::Modify) -eq [System.Security.AccessControl.FileSystemRights]::Modify
}).Count''', path)
            assert int(allowed) > 0, 'Everyone Modify allow ACE was not installed: ' + str(path)
            refused(path, 'everyone')
        finally:
            restore(path, saved)
        consumer()
        print('Windows cache: Everyone Modify refused and restored: ' + str(path), flush=True)

        try:
            try:
                icacls(path, '/setowner', '*S-1-5-19')
            except subprocess.CalledProcessError as error:
                raise RuntimeError('Windows cache owner proof unavailable: icacls cannot set the job-owned path owner to LOCAL SERVICE; runner ownership privilege is required: ' + str(path) + '\n' + error.stdout + error.stderr) from error
            owner = state(path)['owner']
            assert owner == 'S-1-5-19' and owner != caller, 'distinct owner control was not installed: ' + str(path)
            refused(path, 'owner')
        finally:
            restore(path, saved)
        consumer()
        print('Windows cache: distinct owner refused and restored: ' + str(path), flush=True)

    saved = state(asset)
    try:
        # SE_FILE_OBJECT, with DACL and PROTECTED_DACL. PowerShell and icacls cannot
        # write a null access list, which lets everyone in.
        status = ctypes.WinDLL('advapi32').SetNamedSecurityInfoW(str(asset), 1, ctypes.c_uint32(0x80000004),
                                                                 None, None, None, None)
        assert status == 0, 'null access list control was not installed: ' + str(status)
        consumer(rejected=True)
    finally:
        restore(asset, saved)
    consumer()
    print('Windows cache: null access list refused and restored: ' + str(asset), flush=True)

    directory = asset.parent
    target = directory.with_name(directory.name + '.junction-target')
    directory.rename(target)
    try:
        subprocess.run([env['COMSPEC'], '/d', '/c', 'mklink', '/J', directory, target],
                       env=env, capture_output=True, text=True, check=True)
        kind = control('(Get-Item -LiteralPath $env:TT_CACHE_PATH -Force).LinkType', directory)
        assert kind == 'Junction', 'directory junction control was not installed'
        consumer(rejected=True)
    finally:
        if directory.exists(): os.rmdir(directory)
        target.rename(directory)
    consumer()
    print('Windows cache: junction refused and restored; installed consumer passes', flush=True)
