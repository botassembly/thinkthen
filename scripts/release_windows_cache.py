"""Exercise Windows cache security through installed public consumers."""
import json
from functools import partial
import os
from pathlib import Path
import subprocess


def _control(environment, script, path, **values):
    powershell = Path(environment['SYSTEMROOT']) / 'System32/WindowsPowerShell/v1.0/powershell.exe'
    result = subprocess.run([powershell, '-NoProfile', '-NonInteractive', '-Command',
        "$ErrorActionPreference='Stop'; " + script],
        env=environment | {'TT_CACHE_PATH': str(path)} | values, capture_output=True, text=True, check=True)
    return result.stdout.strip()


def _state(environment, path):
    return json.loads(_control(environment, """$a = Get-Acl -LiteralPath $env:TT_CACHE_PATH;
[pscustomobject]@{
  owner = $a.GetOwner([System.Security.Principal.SecurityIdentifier]).Value;
  access = $a.GetSecurityDescriptorSddlForm([System.Security.AccessControl.AccessControlSections]::Access)
} | ConvertTo-Json -Compress""", path))


def prepare_cache(root, environment):
    """Establish the private cache prerequisite shared by installed consumers."""
    root.mkdir(parents=True, exist_ok=True)
    control = partial(_control, environment)
    state = partial(_state, environment)
    identity = json.loads(control('''$i = [System.Security.Principal.WindowsIdentity]::GetCurrent();
[pscustomobject]@{ caller = $i.User.Value; defaultOwner = $i.Owner.Value } | ConvertTo-Json -Compress''', root))
    print('Windows cache baseline identity: ' + json.dumps(identity), flush=True)
    print('Windows cache inherited root: ' + json.dumps(state(root)), flush=True)
    control('''$sid = [System.Security.Principal.WindowsIdentity]::GetCurrent().User;
$acl = New-Object System.Security.AccessControl.DirectorySecurity;
$acl.SetOwner($sid);
$acl.SetAccessRuleProtection($true, $false);
foreach ($principal in @($sid.Value, 'S-1-5-18', 'S-1-5-32-544')) {
  $identity = New-Object System.Security.Principal.SecurityIdentifier($principal);
  $rule = New-Object System.Security.AccessControl.FileSystemAccessRule(
    $identity, 'FullControl', 'ContainerInherit,ObjectInherit', 'None', 'Allow');
  $acl.AddAccessRule($rule);
}
Set-Acl -LiteralPath $env:TT_CACHE_PATH -AclObject $acl''', root)
    baseline = state(root)
    assert baseline['owner'] == identity['caller'], 'private cache root owner is not the caller'
    verified = control('''$acl = Get-Acl -LiteralPath $env:TT_CACHE_PATH;
$caller = [System.Security.Principal.WindowsIdentity]::GetCurrent().User.Value;
$allowed = @($caller, 'S-1-5-18', 'S-1-5-32-544');
$rules = @($acl.Access);
$valid = $acl.AreAccessRulesProtected -and $rules.Count -eq $allowed.Count;
foreach ($rule in $rules) {
  $sid = $rule.IdentityReference.Translate([System.Security.Principal.SecurityIdentifier]).Value;
  $valid = $valid -and ($sid -in $allowed) -and $rule.AccessControlType -eq 'Allow' -and
    $rule.FileSystemRights -eq 'FullControl' -and $rule.InheritanceFlags -eq 'ContainerInherit,ObjectInherit';
}
$valid''', root)
    assert verified == 'True', 'private cache root DACL was not established'
    print('Windows cache private root verified: ' + json.dumps(baseline), flush=True)
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


def check_cache(command, root, environment, filename):
    root.mkdir()
    env = dict(environment, LOCALAPPDATA=str(root), LocalAppData=str(root))
    system = Path(env['SYSTEMROOT']) / 'System32'

    control = partial(_control, env)
    state = partial(_state, env)

    def icacls(path, *args):
        subprocess.run([system / 'icacls.exe', path, *args], env=env,
                       capture_output=True, text=True, check=True)

    def restore(path, saved):
        icacls(path, '/setowner', '*' + saved['owner'])
        control('''$a = Get-Acl -LiteralPath $env:TT_CACHE_PATH;
$a.SetSecurityDescriptorSddlForm($env:TT_CACHE_ACCESS, [System.Security.AccessControl.AccessControlSections]::Access);
Set-Acl -LiteralPath $env:TT_CACHE_PATH -AclObject $a''', path, TT_CACHE_ACCESS=saved['access'])
        assert state(path) == saved, 'cache security was not restored: ' + str(path)

    def consumer(rejected=False):
        result = subprocess.run(list(map(str, command)), env=env, capture_output=True, text=True)
        output = result.stdout + result.stderr
        if rejected:
            assert result.returncode != 0, 'consumer accepted unsafe cache: ' + output
            assert 'cache owner or permissions' in output.lower(), output
        elif result.returncode:
            raise RuntimeError('installed cache consumer failed: ' + output)

    identity = prepare_cache(root, env)
    try:
        consumer()
    except RuntimeError:
        for path in sorted(root.rglob('*')):
            print('Windows cache baseline child ' + str(path) + ': ' + json.dumps(state(path)), flush=True)
        raise
    asset, = root.glob('thinkthen/native/*/' + filename)
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
            consumer(rejected=True)
        finally:
            restore(path, saved)
        consumer()
        print('Windows cache: Everyone Modify refused and restored: ' + str(path), flush=True)

        try:
            try:
                icacls(path, '/setowner', '*S-1-5-32-544')
            except subprocess.CalledProcessError as error:
                raise RuntimeError('Windows cache owner proof unavailable: icacls cannot set the job-owned path owner to BUILTIN Administrators; runner ownership privilege is required: ' + str(path) + '\n' + error.stdout + error.stderr) from error
            owner = state(path)['owner']
            assert owner == 'S-1-5-32-544' and owner != caller, 'distinct owner control was not installed: ' + str(path)
            consumer(rejected=True)
        finally:
            restore(path, saved)
        consumer()
        print('Windows cache: distinct owner refused and restored: ' + str(path), flush=True)

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
