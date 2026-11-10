"""Run proof children as the caller or a normal user without changing the source token.

Windows API contracts:
https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-duplicatetokenex
https://learn.microsoft.com/en-us/windows/win32/secauthz/owner-of-a-new-object
https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-createprocessasuserw
https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-createrestrictedtoken
https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-token_default_dacl
"""
from contextlib import nullcontext
import ctypes as C
from ctypes import wintypes as W
import os
from pathlib import Path
import subprocess
import sys
import tempfile
import time
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


class SidAttributes(C.Structure):
    _fields_ = [('sid', C.c_void_p), ('attributes', W.DWORD)]


class BasicLimits(C.Structure):
    _fields_ = [('process_time', C.c_longlong), ('job_time', C.c_longlong), ('flags', W.DWORD),
                ('minimum_working_set', C.c_size_t), ('maximum_working_set', C.c_size_t),
                ('active_process_limit', W.DWORD), ('affinity', C.c_size_t),
                ('priority', W.DWORD), ('scheduling', W.DWORD)]


class ExtendedLimits(C.Structure):
    _fields_ = [('basic', BasicLimits), ('io', C.c_ulonglong * 6),
                ('process_memory', C.c_size_t), ('job_memory', C.c_size_t),
                ('peak_process_memory', C.c_size_t), ('peak_job_memory', C.c_size_t)]


class JobAccounting(C.Structure):
    _fields_ = [('times', C.c_longlong * 4), ('page_faults', W.DWORD), ('total', W.DWORD),
                ('active', W.DWORD), ('terminated', W.DWORD)]


OWNED_JOBS = []


class OwnedJob:
    """Own descendants before their first instruction, then reap before log cleanup."""
    def __init__(self, functions=None):
        self.functions = functions if functions is not None else api()
        self.job = self.functions['CreateJobObjectW'](None, None)
        checked(self.job, 'CreateJobObjectW')
        limits = ExtendedLimits()
        limits.basic.flags = 0x2000  # JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE
        try:
            checked(self.functions['SetInformationJobObject'](self.job, 9, C.byref(limits), C.sizeof(limits)),
                    'SetInformationJobObject')
        except BaseException:
            self.functions['CloseHandle'](self.job)
            raise

        OWNED_JOBS.append(self)

    def attach(self, process):
        checked(self.functions['AssignProcessToJobObject'](self.job, process), 'AssignProcessToJobObject')

    def start(self, command, **options):
        process = subprocess.Popen(command, creationflags=0x00000004 | 0x00000200, **options)
        try:
            self.attach(int(process._handle))
            resume = C.WinDLL('ntdll').NtResumeProcess
            resume.argtypes, resume.restype = [W.HANDLE], W.LONG
            if resume(int(process._handle)):
                raise RuntimeError('Owned Windows process did not resume')
        except BaseException:
            process.kill()
            process.wait()
            raise
        return process

    def close(self):
        if not self.job:
            return
        try:
            checked(self.functions['TerminateJobObject'](self.job, 1), 'TerminateJobObject')
            deadline = time.monotonic() + 30
            while True:
                info = JobAccounting()
                checked(self.functions['QueryInformationJobObject'](self.job, 1, C.byref(info), C.sizeof(info), None),
                        'QueryInformationJobObject')
                if not info.active:
                    break
                if time.monotonic() >= deadline:
                    raise RuntimeError('Owned Windows process tree did not terminate')
                time.sleep(0.01)
        finally:
            self.functions['CloseHandle'](self.job)
            self.job = None
            if self in OWNED_JOBS:
                OWNED_JOBS.remove(self)

    def __enter__(self):
        return self

    def __exit__(self, *_error):
        self.close()


def owned_processes():
    """Snapshot only live processes in this harness's currently tracked jobs.

    These are owned processes, not proven scratch writers. Empty jobs do not
    establish that no untracked process exists.
    """
    result = []
    kernel = C.WinDLL('kernel32', use_last_error=True)
    kernel.OpenProcess.argtypes, kernel.OpenProcess.restype = [W.DWORD, W.BOOL, W.DWORD], W.HANDLE
    kernel.GetProcessTimes.argtypes = [W.HANDLE, *([C.POINTER(W.FILETIME)] * 4)]
    kernel.QueryFullProcessImageNameW.argtypes = [W.HANDLE, W.DWORD, W.LPWSTR, C.POINTER(W.DWORD)]
    kernel.CloseHandle.argtypes = [W.HANDLE]
    for job in list(OWNED_JOBS):
        data = (C.c_size_t * 66)()
        checked(job.functions['QueryInformationJobObject'](job.job, 3, data, C.sizeof(data), None),
                'QueryInformationJobObject process list')
        count = C.cast(data, C.POINTER(W.DWORD))[1]
        for pid in list(data)[1:1 + min(count, 64)]:
            row = dict(pid=int(pid))
            handle = kernel.OpenProcess(0x1000, False, pid)
            if not handle:
                row['query_error'] = C.get_last_error()
            else:
                try:
                    created, exited, system, user = (W.FILETIME() for _ in range(4))
                    if kernel.GetProcessTimes(handle, C.byref(created), C.byref(exited), C.byref(system), C.byref(user)):
                        row['created_filetime'] = (created.dwHighDateTime << 32) | created.dwLowDateTime
                    size = W.DWORD(32768)
                    image = C.create_unicode_buffer(size.value)
                    if kernel.QueryFullProcessImageNameW(handle, 0, image, C.byref(size)):
                        row['executable'] = image.value[:1024]
                        row['executable_truncated'] = len(image.value) > 1024
                finally:
                    kernel.CloseHandle(handle)
            result.append(row)
    return dict(scope='currently tracked owned jobs; not writer attribution', processes=result[:64],
                truncated=len(result) > 64)


def api():
    kernel = C.WinDLL('kernel32', use_last_error=True)
    security = C.WinDLL('advapi32', use_last_error=True)
    definitions = {
        'GetCurrentProcess': (kernel, [], W.HANDLE),
        'CloseHandle': (kernel, [W.HANDLE], W.BOOL),
        'LocalFree': (kernel, [C.c_void_p], C.c_void_p),
        'WaitForSingleObject': (kernel, [W.HANDLE, W.DWORD], W.DWORD),
        'GetExitCodeProcess': (kernel, [W.HANDLE, C.POINTER(W.DWORD)], W.BOOL),
        'CreateJobObjectW': (kernel, [C.c_void_p, W.LPCWSTR], W.HANDLE),
        'SetInformationJobObject': (kernel, [W.HANDLE, C.c_int, C.c_void_p, W.DWORD], W.BOOL),
        'AssignProcessToJobObject': (kernel, [W.HANDLE, W.HANDLE], W.BOOL),
        'TerminateJobObject': (kernel, [W.HANDLE, W.UINT], W.BOOL),
        'QueryInformationJobObject': (kernel, [W.HANDLE, C.c_int, C.c_void_p, W.DWORD, C.c_void_p], W.BOOL),
        'ResumeThread': (kernel, [W.HANDLE], W.DWORD),
        'TerminateProcess': (kernel, [W.HANDLE, W.UINT], W.BOOL),
        'OpenProcessToken': (security, [W.HANDLE, W.DWORD, C.POINTER(W.HANDLE)], W.BOOL),
        'DuplicateTokenEx': (security, [W.HANDLE, W.DWORD, C.c_void_p, C.c_int, C.c_int, C.POINTER(W.HANDLE)], W.BOOL),
        'GetTokenInformation': (security, [W.HANDLE, C.c_int, C.c_void_p, W.DWORD, C.POINTER(W.DWORD)], W.BOOL),
        'SetTokenInformation': (security, [W.HANDLE, C.c_int, C.c_void_p, W.DWORD], W.BOOL),
        'ConvertSidToStringSidW': (security, [C.c_void_p, C.POINTER(W.LPWSTR)], W.BOOL),
        'ConvertStringSidToSidW': (security, [W.LPCWSTR, C.POINTER(C.c_void_p)], W.BOOL),
        'GetLengthSid': (security, [C.c_void_p], W.DWORD),
        'InitializeAcl': (security, [C.c_void_p, W.DWORD, W.DWORD], W.BOOL),
        'AddAccessAllowedAce': (security, [C.c_void_p, W.DWORD, W.DWORD, C.c_void_p], W.BOOL),
        'CreateRestrictedToken': (security, [W.HANDLE, W.DWORD, W.DWORD, C.c_void_p, W.DWORD,
            C.c_void_p, W.DWORD, C.c_void_p, C.POINTER(W.HANDLE)], W.BOOL),
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


def string_sid(functions, text):
    value = C.c_void_p()
    checked(functions['ConvertStringSidToSidW'](text, C.byref(value)), 'ConvertStringSidToSidW')
    return value


def user_default_dacl(functions, token):
    """Grant new objects to the user and SYSTEM, as a normal shell's token does.

    An elevated token's default grants Administrators, which this token holds deny-only,
    so without this its child processes lack full access to their own process and token.
    """
    info = token_info(functions, token, 1)  # TokenUser; the SID lives inside this buffer
    user = C.cast(info, C.POINTER(C.c_void_p))[0]
    system = string_sid(functions, 'S-1-5-18')
    try:
        sids = (user, system)
        size = 8 + sum(8 + functions['GetLengthSid'](value) for value in sids)  # ACL, then each ACE
        acl = (W.DWORD * (size // 4))()
        checked(functions['InitializeAcl'](acl, size, 2), 'InitializeAcl')
        for value in sids:
            checked(functions['AddAccessAllowedAce'](acl, 2, 0x10000000, value), 'AddAccessAllowedAce')  # GENERIC_ALL
        default = C.c_void_p(C.addressof(acl))
        checked(functions['SetTokenInformation'](token, 6, C.byref(default), C.sizeof(default)),
                'SetTokenInformation(TokenDefaultDacl)')
    finally:
        functions['LocalFree'](system)


def normal_token(functions):
    """Return the current user's token as a normal shell has it: no Administrators, medium integrity."""
    source, token = W.HANDLE(), W.HANDLE()
    # The new token takes this handle's access. A restricted token must descend
    # from the caller's own, or starting a process with it needs a privilege.
    checked(functions['OpenProcessToken'](functions['GetCurrentProcess'](), 0x0008 | 0x0002 | 0x0001 | 0x0080,
                                         C.byref(source)), 'OpenProcessToken')
    administrators = string_sid(functions, 'S-1-5-32-544')
    medium = string_sid(functions, 'S-1-16-8192')
    try:
        disabled = SidAttributes(administrators, 0)
        # DISABLE_MAX_PRIVILEGE, with Administrators made deny-only.
        checked(functions['CreateRestrictedToken'](source, 0x1, 1, C.byref(disabled), 0, None, 0, None,
                                                   C.byref(token)), 'CreateRestrictedToken')
        label = SidAttributes(medium, 0x20)  # SE_GROUP_INTEGRITY
        size = C.sizeof(label) + functions['GetLengthSid'](medium)
        checked(functions['SetTokenInformation'](token, 25, C.byref(label), size), 'SetTokenInformation(TokenIntegrityLevel)')
        user_default_dacl(functions, token)
        integrity = sid(functions, token_info(functions, token, 25))  # TokenIntegrityLevel
        if integrity != 'S-1-16-8192':
            raise RuntimeError('Normal token is not medium integrity: ' + integrity)
        print('Windows proof token: normal integrity=' + integrity, flush=True)
        return token
    finally:
        functions['LocalFree'](medium)
        functions['LocalFree'](administrators)
        functions['CloseHandle'](source)


def launch(command, *, cwd, env, probe=False, normal_log=None, timeout=3600, log_path=None):
    """Run a child whose default owner is the caller, and wait.

    The child has the current token, or with normal_log a normal user's token and
    that log file, which the normal user can write and a private folder would refuse.
    """
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
        if normal_log:
            duplicate = normal_token(functions)
        else:
            access = 0x0008 | 0x0002 | 0x0001 | 0x0080  # QUERY, DUPLICATE, ASSIGN_PRIMARY, ADJUST_DEFAULT
            checked(functions['DuplicateTokenEx'](source, access, None, 2, 1, C.byref(duplicate)), 'DuplicateTokenEx')
        user = token_info(functions, duplicate, 1)
        owner = C.c_void_p(C.cast(user, C.POINTER(C.c_void_p))[0])
        checked(functions['SetTokenInformation'](duplicate, 4, C.byref(owner), C.sizeof(owner)), 'SetTokenInformation(TokenOwner)')
        assert sid(functions, token_info(functions, duplicate, 4)) == caller, 'duplicate default owner differs from caller'
        print('Windows proof token: parent owner=' + original + '; child owner=' + caller, flush=True)
        folder = nullcontext() if normal_log or log_path else tempfile.TemporaryDirectory(prefix='thinkthen-caller-process-')
        with folder as temporary:
            with OwnedJob(functions) as job:
                log = (Path(normal_log or log_path) if normal_log or log_path else Path(temporary) / 'child.log').resolve()
                log.parent.mkdir(parents=True, exist_ok=True)
                child = [sys.executable, str(Path(__file__).resolve()), '--child', str(log),
                         '90' if probe else str(timeout), *map(str, command)]
                line = subprocess.list2cmdline(child)
                if len(line) >= 32767:
                    raise RuntimeError('Caller-owned proof command exceeds CreateProcessAsUserW limit')
                environment = C.create_unicode_buffer('\0'.join(key + '=' + value for key, value in
                    sorted(env.items(), key=lambda item: item[0].upper())) + '\0\0')
                startup = StartupInfo()
                startup.cb = C.sizeof(startup)
                result = functions['CreateProcessAsUserW'](duplicate, sys.executable,
                    C.create_unicode_buffer(line), None, None, False, 0x00000400 | 0x00000004, environment,
                    str(Path(cwd).resolve()), C.byref(startup), C.byref(process))
                checked(result, 'CreateProcessAsUserW')
                try:
                    job.attach(process.process)
                    if functions['ResumeThread'](process.thread) == 0xffffffff:
                        checked(False, 'ResumeThread')
                except BaseException:
                    checked(functions['TerminateProcess'](process.process, 1), 'TerminateProcess')
                    functions['WaitForSingleObject'](process.process, 30000)
                    raise
                deadline = time.monotonic() + (90 if probe else timeout) + 30 if probe or timeout is not None else None

                def wait():
                    status = functions['WaitForSingleObject'](process.process, 100)
                    if status == 0x102 and deadline is not None and time.monotonic() >= deadline:
                        raise subprocess.TimeoutExpired(command, timeout)
                    return status

                stream(log, wait)
                status = W.DWORD()
                checked(functions['GetExitCodeProcess'](process.process, C.byref(status)), 'GetExitCodeProcess')
                if status.value:
                    print('Windows proof child exited ' + str(status.value), flush=True)
                    raise SystemExit(status.value)
    finally:
        try:
            if original is not None:
                assert sid(functions, token_info(functions, source, 4)) == original, 'source token default owner changed'
                print('Windows proof token: parent owner unchanged', flush=True)
        finally:
            for handle in (process.thread, process.process, duplicate, source):
                if handle: functions['CloseHandle'](handle)


def stream(log, wait):
    """Forward child output while it runs, then drain its final output."""
    position = 0
    while True:
        status = wait()
        if log.exists():
            with log.open('rb') as source:
                source.seek(position)
                data = source.read()
                position = source.tell()
            if data:
                sys.stdout.buffer.write(data)
                sys.stdout.buffer.flush()
        if status == 0:
            return
        checked(status == 0x102, 'WaitForSingleObject')


def probe(cwd, env, normal=None):
    """Check isolated token ownership without a compiler or engine.

    Given a folder, check there that a normal user's token works and is not an administrator's.
    """
    powershell = Path(env['SYSTEMROOT']) / 'System32/WindowsPowerShell/v1.0/powershell.exe'
    with nullcontext(normal) if normal else tempfile.TemporaryDirectory(prefix='thinkthen-owner-probe-') as temporary:
        script = """$ErrorActionPreference='Stop';
$i=[System.Security.Principal.WindowsIdentity]::GetCurrent();
if ($i.User.Value -ne $i.Owner.Value) { throw 'Child token default owner is not the caller' };
if ($env:TT_NORMAL_PROBE) {
  $administrators=New-Object System.Security.Principal.SecurityIdentifier('S-1-5-32-544');
  if ((New-Object System.Security.Principal.WindowsPrincipal($i)).IsInRole($administrators)) { throw 'Normal token is an administrator' };
  function Start-Probe($program, [string[]]$arguments) {
    $ErrorActionPreference='Continue';
    try {
      $output=@(& $program @arguments 2>&1 | ForEach-Object { [string]$_ });
      [Console]::WriteLine('Normal token started ' + $program + ': exit ' + $LASTEXITCODE + ': ' + ($output -join ' | '));
    } catch {
      [Console]::WriteLine('Normal token could not start ' + $program + ': ' + $_.Exception.Message);
    }
  };
  function Find-Programs($name) {
    @(Get-Command $name -All -CommandType Application -ErrorAction SilentlyContinue | ForEach-Object { $_.Source })
  };
  # These only report. The real proof starts its consumers from Python, so the
  # consumer runs decide; launch reads the integrity level from the token itself.
  $system=Join-Path $env:SYSTEMROOT 'System32';
  Start-Probe (Join-Path $system 'whoami.exe') @('/groups');
  Start-Probe (Join-Path $system 'cmd.exe') @('/c','exit 0');
  $found=@(Find-Programs 'whoami.exe');
  [Console]::WriteLine('Normal token PATH finds whoami.exe at: ' + ($found -join '; '));
  if ($found) { Start-Probe $found[0] @('/?') };
  foreach ($tool in @(@('go.exe','version'),@('java.exe','-version'))) {
    $path=@(Find-Programs $tool[0]);
    if ($path) { Start-Probe $path[0] @($tool[1]) }
  }
};
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
        # A pwsh parent's module path hides Windows PowerShell's own Get-Acl.
        modules = {key: value for key, value in env.items() if key.upper() != 'PSMODULEPATH'}
        launch([powershell, '-NoProfile', '-NonInteractive', '-ExecutionPolicy', 'Bypass', '-File', source],
               cwd=cwd, env=modules | {'TT_OWNER_PROBE': str(temporary)} | ({'TT_NORMAL_PROBE': '1'} if normal else {}),
               probe=True, normal_log=Path(temporary) / 'child.log' if normal else None)



def child():
    log, timeout, *command = sys.argv[2:]
    with open(log, 'w', encoding='utf-8') as output:
        try:
            result = subprocess.run(command, stdout=output, stderr=subprocess.STDOUT,
                                    timeout=None if timeout == 'None' else int(timeout), check=False)
            return result.returncode
        except Exception:  # noqa: BLE001 - preserve the child failure in its streamed log
            traceback.print_exc(file=output)
            return 1


if __name__ == '__main__':
    if sys.argv[1:2] != ['--child']:
        raise SystemExit('Use the Windows proof entry point')
    raise SystemExit(child())
