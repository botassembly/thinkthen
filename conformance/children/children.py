"""A test child's whole environment, built from nothing (ticket 0127).

The child gets PATH, the parent's value of each name the caller keeps, an
optional owned home, and the values the caller sets. Windows also retains
the system names needed to start. A secret-shaped or THINKTHEN_ name is
never kept: a test sets a THINKTHEN_ value or a fake key explicitly.
"""

import os
from pathlib import Path
import re

# The names cargo and rustup read, as test_deadline/child.rs lists them.
CARGO = ("HOME", "CARGO_HOME", "RUSTUP_HOME", "RUSTUP_TOOLCHAIN", "CARGO_TARGET_DIR", "RUSTC_WRAPPER")
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
