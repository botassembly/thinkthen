"""Give every script started by name the standard tomllib, or a Python that has it.

This file carries a standard-library name on purpose. Python puts scripts/ first
on the path for a script started by name, so `import tomllib` reaches this file
before the standard library, and no script has to change to choose its Python.
Python 3.11 or newer gets the real module. Older Python, where the real module
does not exist, restarts the script under scripts/native-tools-env or stops with
one clear line. Keep the syntax here readable by old Python.
"""
import importlib.machinery
import importlib.util
import os
import sys

PROBLEM = ('Python {}.{} has no tomllib: put Python 3.11 or newer first on PATH, '
           'or prepare CPython 3.13 for scripts/native-tools-env'.format(*sys.version_info))
RESTARTED = 'THINKTHEN_TOMLLIB_RESTARTED'
# The sourced helper can exit, and its own Python calls can fail on this same old
# Python, so its output stays hidden and the EXIT trap keeps the failure to one line.
RESTART = r'''
trap 'printf "%s\n" "$1" >&2' EXIT
cd "$2" && . scripts/native-tools-env >/dev/null 2>&1
cd "$3"
trap - EXIT
shift 3
exec python3 "$@"
'''


def standard_library():
    spec = importlib.machinery.PathFinder.find_spec('tomllib', [os.path.dirname(os.__file__)])
    module = importlib.util.module_from_spec(spec)
    sys.modules['tomllib'] = module
    spec.loader.exec_module(module)


def restart():
    main = sys.modules['__main__']
    script = getattr(main, '__file__', None) or ''
    # -c, stdin and -m leave no script file to start again, and Windows has no sh to source.
    if (os.name == 'nt' or not os.path.isfile(script) or getattr(main, '__spec__', None)
            or RESTARTED in os.environ):
        raise SystemExit(PROBLEM)
    os.environ[RESTARTED] = '1'
    sys.stdout.flush()
    sys.stderr.flush()
    root = os.path.dirname(os.path.dirname(os.path.realpath(__file__)))
    os.execv('/bin/sh', ['sh', '-c', RESTART, 'tomllib-restart', PROBLEM, root, os.getcwd(),
                         os.path.abspath(script)] + sys.argv[1:])


if sys.version_info >= (3, 11):
    standard_library()
else:
    restart()
