"""Run community build tools with an installed Python that has stdlib TOML support."""
import os
import shutil
import subprocess
import sys


def require_tomllib():
    if sys.version_info >= (3, 11):
        return
    # Community Linux images carry these CPython hosts outside PATH.
    candidates = ['python3.13', 'python3.12', 'python3.11'] + [
        f'/opt/python/cp3{minor}-cp3{minor}/bin/python' for minor in (13, 12, 11)]
    for candidate in candidates:
        python = shutil.which(candidate)
        if not python:
            continue
        try:
            checked = subprocess.run([python, '-c',
                'import sys,tomllib; sys.exit(sys.version_info < (3, 11))'],
                capture_output=True, timeout=5)
        except (OSError, subprocess.TimeoutExpired):
            continue
        if checked.returncode == 0:
            os.execv(python, [python, *sys.argv])
    raise SystemExit('community: install Python 3.11 or later for the build tools')
