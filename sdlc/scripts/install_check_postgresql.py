"""Stop only the Linux private postmaster created by one install check."""
import ctypes
import os
from pathlib import Path
import select
import signal
from install_check import Failure


def libc_pidfd(name, *args):
    # Some standalone Python builds omit these optional Linux wrappers even
    # when libc and the kernel provide the same process-bound operations.
    libc = ctypes.CDLL(None, use_errno=True)
    try:
        function = getattr(libc, name)
    except AttributeError as error:
        raise Failure('PostgreSQL cleanup requires Linux pidfd support') from error
    function.argtypes = ([ctypes.c_int, ctypes.c_uint] if name == 'pidfd_open'
                         else [ctypes.c_int, ctypes.c_int, ctypes.c_void_p, ctypes.c_uint])
    function.restype = ctypes.c_int
    result = function(*args)
    if result == -1:
        code = ctypes.get_errno()
        raise OSError(code, os.strerror(code))
    return result


def open_pidfd(pid):
    native = getattr(os, 'pidfd_open', None)
    return native(pid) if native else libc_pidfd('pidfd_open', pid, 0)


def signal_pidfd(handle, number):
    native = getattr(signal, 'pidfd_send_signal', None)
    if native:
        native(handle, number)
    else:
        libc_pidfd('pidfd_send_signal', handle, number, None, 0)


def stop_postgres(check, data, socket, binary):
    root = check.root.resolve()
    for path, name in ((data, 'pg-data'), (socket, 'pg-socket')):
        if path != root / name or path.is_symlink() or not path.is_dir():
            raise Failure('PostgreSQL cleanup cannot verify this run\'s private cluster paths')
        info = path.stat()
        if info.st_uid != os.geteuid() or info.st_mode & 0o077:
            raise Failure('PostgreSQL cleanup cannot verify this run\'s private cluster ownership')
    pid_file = data / 'postmaster.pid'
    if pid_file.is_symlink() or not pid_file.is_file():
        raise Failure('PostgreSQL cleanup cannot verify this run\'s postmaster PID')
    info = pid_file.stat()
    if info.st_uid != os.geteuid() or info.st_mode & 0o022:
        raise Failure('PostgreSQL cleanup cannot verify this run\'s postmaster PID ownership')
    fields = pid_file.read_text().splitlines()
    if (len(fields) < 6 or not fields[0].isdigit() or int(fields[0]) <= 1
            or fields[1] != str(data) or fields[3] != '5432'
            or fields[4] != str(socket) or fields[5] != ''):
        raise Failure('PostgreSQL cleanup cannot verify this run\'s postmaster PID identity')
    try:
        handle = open_pidfd(int(fields[0]))
    except ProcessLookupError:
        return  # This exact PID has already exited; no process gets a signal.
    try:
        if select.select([handle], [], [], 0)[0]:
            return
        process = Path('/proc') / fields[0]
        major = binary.parent.name
        if major not in ('16', '17', '18'):
            raise Failure('PostgreSQL cleanup refuses an unsupported server major')
        expected = root / f'pg-runtime/usr/lib/postgresql/{major}/bin/postgres'
        if binary / 'postgres' != expected or (process / 'exe').resolve() != expected:
            raise Failure('PostgreSQL cleanup refuses a process outside this run\'s runtime')
        args = (process / 'cmdline').read_bytes().rstrip(b'\0').split(b'\0')
        pairs = list(zip(args, args[1:]))
        if (b'-D', os.fsencode(data)) not in pairs or (b'-k', os.fsencode(socket)) not in pairs:
            raise Failure('PostgreSQL cleanup refuses a process outside this run\'s private cluster')
        # A pidfd binds the signal to this process even if its numeric PID is reused.
        try:
            signal_pidfd(handle, signal.SIGINT)
        except ProcessLookupError:
            return
        if not select.select([handle], [], [], 30)[0]:
            raise Failure('PostgreSQL cleanup timed out waiting for this run\'s postmaster')
    finally:
        os.close(handle)
