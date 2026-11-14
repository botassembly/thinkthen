"""Connect the global guide dispatcher to the already prepared SQLite host."""
import importlib.util
import os
from pathlib import Path
import shlex


def run_sqlite(command=None, negative=None):
    """Accept [consumer, extension], or their prepared-artifact environment paths."""
    if command is None:
        command = [os.environ['THINKTHEN_SQLITE_CONSUMER'], os.environ['THINKTHEN_SQLITE_EXTENSION']]
    elif isinstance(command, str):
        command = shlex.split(command)
    if len(command) != 2:
        raise RuntimeError('SQLite guide needs its prepared native consumer and extension')
    path = Path(__file__).resolve().parents[1] / 'extensions/sqlite/tests/ideal.py'
    spec = importlib.util.spec_from_file_location('sqlite_ideal', path)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    module.run(command, negative)
