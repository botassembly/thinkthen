"""A test child's whole environment, built from nothing (ticket 0127).

The child gets PATH, the parent's value of each name the caller keeps, and
the values the caller sets. A secret-shaped or THINKTHEN_ name is never kept:
a test sets a THINKTHEN_ value or a fake key explicitly.
"""

import os
import re

SECRET = re.compile(r"^THINKTHEN_|KEY|TOKEN|SECRET|PASSWORD|CREDENTIAL|AUTH", re.IGNORECASE)


def child_env(keep=(), **values):
    """PATH, each kept name the parent has, then the values set here."""
    env = {"PATH": os.environ.get("PATH", "/usr/bin:/bin")}
    for name in keep:
        if SECRET.search(name):
            raise ValueError(f"a test child may not keep {name} from the parent: "
                             "set a THINKTHEN_ value or a fake key explicitly")
        if name in os.environ:
            env[name] = os.environ[name]
    env.update(values)
    return env
