"""Isolate native database test subprocesses from developer credentials."""
import os
from pathlib import Path
import sys

sys.path.insert(0, str(Path(__file__).resolve().parents[1] / 'scripts'))
from test_control import read_line


def child_environment():
    names = ('PATH', 'HOME', 'SYSTEMROOT', 'TMPDIR', 'TEMP', 'TMP', 'LLVM_PROFILE_FILE')
    return {name: os.environ[name] for name in names if name in os.environ}
