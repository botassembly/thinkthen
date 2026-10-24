"""Isolate native database test subprocesses from developer credentials."""
import os


def child_environment():
    names = ('PATH', 'HOME', 'SYSTEMROOT', 'TMPDIR', 'TEMP', 'TMP', 'LLVM_PROFILE_FILE')
    return {name: os.environ[name] for name in names if name in os.environ}
