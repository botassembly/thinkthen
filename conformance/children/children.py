"""A test child's whole environment, built from nothing (ticket 0127).

The child gets PATH, the parent's value of each name the caller keeps, an
optional owned home, and the values the caller sets. Windows also retains
the system names needed to start. A secret-shaped or THINKTHEN_ name is
never kept: a test sets a THINKTHEN_ value or a fake key explicitly.
"""

import os
from pathlib import Path
import re
import subprocess
import sys

# The names cargo and rustup read, as test_deadline/child.rs lists them.
CARGO = ("HOME", "CARGO_HOME", "CARGO_BUILD_JOBS", "RUSTUP_HOME", "RUSTUP_TOOLCHAIN",
         "CARGO_TARGET_DIR", "RUSTC_WRAPPER", "RUSTFLAGS", "CARGO_ENCODED_RUSTFLAGS",
         "CFLAGS", "CXXFLAGS")
SECRET = re.compile(r"^THINKTHEN_|KEY|TOKEN|SECRET|PASSWORD|CREDENTIAL|AUTH", re.IGNORECASE)


def child_env(keep=(), *, home=None, **values):
    """PATH, kept names, optional owned home/config/cache/state, then overrides.

    Toolchain children can omit home and keep CARGO. Product children supply
    an owned home; explicit values still win for environment behavior cases.
    Windows keeps only the system names needed to load DLLs and open sockets.
    """
    env = {"PATH": os.environ.get("PATH", "/usr/bin:/bin")}
    if os.name == "nt":
        keep = (*keep, "SystemRoot", "SystemDrive", "TEMP", "TMP")
    for name in keep:
        if SECRET.search(name):
            raise ValueError(f"a test child may not keep {name} from the parent: "
                             "set a THINKTHEN_ value or a fake key explicitly")
        if name in os.environ:
            env[name] = os.environ[name]
    if home is not None:
        home = Path(home)
        env.update(HOME=str(home), XDG_CONFIG_HOME=str(home / "config"),
                   XDG_CACHE_HOME=str(home / "cache"), XDG_STATE_HOME=str(home / "state"),
                   APPDATA=str(home / "config"), LOCALAPPDATA=str(home / "local"))
    env.update(values)
    return env


def read_process_line(stream):
    """Read one acknowledgement without buffering a later communicate payload."""
    line = bytearray()
    while True:
        byte = os.read(stream.fileno(), 1)
        if not byte:
            break
        line.extend(byte)
        if byte == b"\n":
            break
    return line.decode(stream.encoding or "utf-8", stream.errors or "strict").replace("\r\n", "\n")


# Ordinary fixture completion only; product deadlines and cancellation probes
# keep their own controls. This bound detects hangs on a busy build host.
FIXTURE_HANG_TIMEOUT = 60


def fixture_stalled(error):
    """Keep the stalled command and captured output in the failing gate log."""
    print(f'fixture hung: {error.cmd!r} after {error.timeout}s', file=sys.stderr, flush=True)
    for name, output in (('stdout', error.stdout), ('stderr', error.stderr)):
        if isinstance(output, bytes):
            output = output.decode('utf-8', errors='replace')
        print(f'{name}:\n{output or "(empty or inherited by the gate log)"}', file=sys.stderr, flush=True)


def fixture_run(command, **options):
    """Run an ordinary fixture with the shared hang guard."""
    try:
        return subprocess.run(command, timeout=FIXTURE_HANG_TIMEOUT, **options)
    except subprocess.TimeoutExpired as error:
        fixture_stalled(error)
        raise


def fixture_communicate(child, input=None):
    """Collect a fixture child without a machine-speed deadline."""
    try:
        return child.communicate(input=input, timeout=FIXTURE_HANG_TIMEOUT)
    except subprocess.TimeoutExpired as error:
        child.kill()
        fixture_stalled(error)
        raise


def fixture_wait(child):
    """Wait for an ordinary fixture, retaining output if it hangs."""
    try:
        return child.wait(timeout=FIXTURE_HANG_TIMEOUT)
    except subprocess.TimeoutExpired as error:
        child.kill()
        try:
            error.stdout, error.stderr = child.communicate(timeout=FIXTURE_HANG_TIMEOUT)
        except subprocess.TimeoutExpired:
            pass
        fixture_stalled(error)
        raise
