"""Run proof children with caller ownership without changing the source token.

Windows API contracts:
https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-duplicatetokenex
https://learn.microsoft.com/en-us/windows/win32/secauthz/owner-of-a-new-object
https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-createprocessasuserw
"""
import ctypes as C
from ctypes import wintypes as W
import os
from pathlib import Path
import subprocess
import sys
import tempfile
import traceback


class StartupInfo(C.Structure):
    _fields_ = [('cb', W.DWORD), ('reserved', W.LPWSTR), ('desktop', W.LPWSTR),
                ('title', W.LPWSTR), ('x', W.DWORD), ('y', W.DWORD),
                ('xsize', W.DWORD), ('ysize', W.DWORD), ('xchars', W.DWORD),
                ('ychars', W.DWORD), ('fill', W.DWORD), ('flags', W.DWORD),
                ('show', W.WORD), ('reserved_size', W.WORD), ('reserved_data', C.c_void_p),
                ('stdin', W.HANDLE), ('stdout', W.HANDLE), ('stderr', W.HANDLE)]


class ProcessInfo(C.Structure):
    _fields_ = [('process', W.HANDLE), ('thread', W.HANDLE),
                ('process_id', W.DWORD), ('thread_id', W.DWORD)]


def api():
    kernel = C.WinDLL('kernel32', use_last_error=True)
    security = C.WinDLL('advapi32', use_last_error=True)
    definitions = {
        'GetCurrentProcess': (kernel, [], W.HANDLE),
        'CloseHandle': (kernel, [W.HANDLE], W.BOOL),
        'LocalFree': (kernel, [C.c_void_p], C.c_void_p),
        'WaitForSingleObject': (kernel, [W.HANDLE, W.DWORD], W.DWORD),
        'GetExitCodeProcess': (kernel, [W.HANDLE, C.POINTER(W.DWORD)], W.BOOL),
        'OpenProcessToken': (security, [W.HANDLE, W.DWORD, C.POINTER(W.HANDLE)], W.BOOL),
        'DuplicateTokenEx': (security, [W.HANDLE, W.DWORD, C.c_void_p, C.c_int, C.c_int, C.POINTER(W.HANDLE)], W.BOOL),
        'GetTokenInformation': (security, [W.HANDLE, C.c_int, C.c_void_p, W.DWORD, C.POINTER(W.DWORD)], W.BOOL),
        'SetTokenInformation': (security, [W.HANDLE, C.c_int, C.c_void_p, W.DWORD], W.BOOL),
        'ConvertSidToStringSidW': (security, [C.c_void_p, C.POINTER(W.LPWSTR)], W.BOOL),
        'CreateProcessAsUserW': (security, [W.HANDLE, W.LPCWSTR, W.LPWSTR, C.c_void_p,
            C.c_void_p, W.BOOL, W.DWORD, C.c_void_p, W.LPCWSTR,
            C.POINTER(StartupInfo), C.POINTER(ProcessInfo)], W.BOOL),
    }
    result = {}
    for name, (library, arguments, returns) in definitions.items():
        function = getattr(library, name)
        function.argtypes, function.restype = arguments, returns
        result[name] = function
    return result


def checked(result, operation):
    if not result:
        error = C.get_last_error()
        raise OSError(error, operation + ': ' + C.FormatError(error))


def token_info(functions, token, kind):
    size = W.DWORD()
    functions['GetTokenInformation'](token, kind, None, 0, C.byref(size))
    checked(size.value, 'GetTokenInformation size')
    buffer = C.create_string_buffer(size.value)
    checked(functions['GetTokenInformation'](token, kind, buffer, size, C.byref(size)), 'GetTokenInformation')
    return buffer


def sid(functions, buffer):
    value = W.LPWSTR()
    pointer = C.cast(buffer, C.POINTER(C.c_void_p))[0]
    checked(functions['ConvertSidToStringSidW'](pointer, C.byref(value)), 'ConvertSidToStringSidW')
    try:
        return value.value
    finally:
        functions['LocalFree'](C.cast(value, C.c_void_p))


def launch(command, *, cwd, env, probe=False):
    """Duplicate the current token, change only its default owner, and wait."""
    if os.name != 'nt':
        raise RuntimeError('Caller-owned proof processes require native Windows')
    sys.stdout.reconfigure(encoding='utf-8')
    functions = api()
    source, duplicate = W.HANDLE(), W.HANDLE()
    process = ProcessInfo()
    original = None
    try:
        # Source access is deliberately QUERY | DUPLICATE, never ADJUST_DEFAULT.
        checked(functions['OpenProcessToken'](functions['GetCurrentProcess'](), 0x0008 | 0x0002,
                                             C.byref(source)), 'OpenProcessToken')
        original = sid(functions, token_info(functions, source, 4))  # TokenOwner
        caller = sid(functions, token_info(functions, source, 1))  # TokenUser
        access = 0x0008 | 0x0002 | 0x0001 | 0x0080  # QUERY, DUPLICATE, ASSIGN_PRIMARY, ADJUST_DEFAULT
        checked(functions['DuplicateTokenEx'](source, access, None, 2, 1, C.byref(duplicate)), 'DuplicateTokenEx')
        user = token_info(functions, duplicate, 1)
        owner = C.c_void_p(C.cast(user, C.POINTER(C.c_void_p))[0])
        checked(functions['SetTokenInformation'](duplicate, 4, C.byref(owner), C.sizeof(owner)), 'SetTokenInformation(TokenOwner)')
        assert sid(functions, token_info(functions, duplicate, 4)) == caller, 'duplicate default owner differs from caller'
        print('Windows proof token: parent owner=' + original + '; child owner=' + caller, flush=True)
        with tempfile.TemporaryDirectory(prefix='thinkthen-caller-process-') as temporary:
            log = Path(temporary) / 'child.log'
            child = [sys.executable, str(Path(__file__).resolve()), '--child', str(log),
                     '90' if probe else '0', *map(str, command)]
            line = subprocess.list2cmdline(child)
            if len(line) >= 32767:
                raise RuntimeError('Caller-owned proof command exceeds CreateProcessAsUserW limit')
            environment = C.create_unicode_buffer('\0'.join(key + '=' + value for key, value in
                sorted(env.items(), key=lambda item: item[0].upper())) + '\0\0')
            startup = StartupInfo()
            startup.cb = C.sizeof(startup)
            result = functions['CreateProcessAsUserW'](duplicate, sys.executable,
                C.create_unicode_buffer(line), None, None, False, 0x00000400, environment,
                str(Path(cwd).resolve()), C.byref(startup), C.byref(process))
            checked(result, 'CreateProcessAsUserW')
            # The wrapper owns the command timeout, including killing and waiting
            # for PowerShell. Wait for it so cleanup cannot orphan its command.
            waited = functions['WaitForSingleObject'](process.process, 0xffffffff)
            checked(waited == 0, 'WaitForSingleObject')
            status = W.DWORD()
            checked(functions['GetExitCodeProcess'](process.process, C.byref(status)), 'GetExitCodeProcess')
            if log.exists():
                print('\n'.join(log.read_text(encoding='utf-8', errors='replace').splitlines()[-200:]), flush=True)
            if status.value:
                print('Caller-owned proof child exited ' + str(status.value), flush=True)
                raise SystemExit(status.value)
    finally:
        try:
            if original is not None:
                assert sid(functions, token_info(functions, source, 4)) == original, 'source token default owner changed'
                print('Windows proof token: parent owner unchanged', flush=True)
        finally:
            for handle in (process.thread, process.process, duplicate, source):
                if handle: functions['CloseHandle'](handle)


def probe(cwd, env):
    """Check isolated token ownership without a compiler or engine."""
    powershell = Path(env['SYSTEMROOT']) / 'System32/WindowsPowerShell/v1.0/powershell.exe'
    with tempfile.TemporaryDirectory(prefix='thinkthen-owner-probe-') as temporary:
        script = """$ErrorActionPreference='Stop';
$i=[System.Security.Principal.WindowsIdentity]::GetCurrent();
if ($i.User.Value -ne $i.Owner.Value) { throw 'Child token default owner is not the caller' };
$directory=Join-Path $env:TT_OWNER_PROBE 'directory';
$file=Join-Path $directory 'file';
New-Item -ItemType Directory -Path $directory | Out-Null;
Set-Content -LiteralPath $file -Value 'ownership probe';
$owners=@($directory,$file) | ForEach-Object {
  $owner=(Get-Acl -LiteralPath $_).GetOwner([System.Security.Principal.SecurityIdentifier]).Value;
  if ($owner -ne $i.User.Value) { throw ('Child object owner differs from caller: ' + $_) };
  $owner
};
[pscustomobject]@{caller=$i.User.Value;defaultOwner=$i.Owner.Value;objectOwners=$owners} | ConvertTo-Json -Compress
"""
        source = Path(temporary) / 'probe.ps1'
        source.write_text(script, encoding='utf-8')
        launch([powershell, '-NoProfile', '-NonInteractive', '-ExecutionPolicy', 'Bypass', '-File', source],
               cwd=cwd, env=env | {'TT_OWNER_PROBE': temporary}, probe=True)



def child():
    log, timeout, *command = sys.argv[2:]
    with open(log, 'w', encoding='utf-8') as output:
        try:
            result = subprocess.run(command, stdout=output, stderr=subprocess.STDOUT,
                                    timeout=int(timeout) or None, check=False)
            return result.returncode
        except Exception:
            traceback.print_exc(file=output)
            return 1


if __name__ == '__main__':
    if sys.argv[1:2] != ['--child']:
        raise SystemExit('Use the Windows proof entry point')
    raise SystemExit(child())
