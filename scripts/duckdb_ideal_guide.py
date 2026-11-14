"""Dispatch the SQL guide consumers using an already built DuckDB package."""
from pathlib import Path
import os
import sys

ROOT = Path(__file__).resolve().parent.parent


def run(package=None, loopback=None, negative=None, guides=('duckdb', 'sql')):
    package = package or os.environ.get('THINKTHEN_DUCKDB_PACKAGE')
    loopback = loopback or os.environ.get('THINKTHEN_LOOPBACK')
    if not package or not loopback:
        raise RuntimeError('Use scripts/duckdb-gate --development --ideal-guides to prepare the DuckDB package and fixture once')
    sys.path.insert(0, str(ROOT / 'databases/duckdb/tests'))
    from ideal_guides import run as consume
    consume(Path(package), Path(loopback), negative, guides)


def run_duckdb(package, negative=None):
    run(package, negative=negative, guides=('duckdb',))


def run_sql(package, negative=None):
    run(package, negative=negative, guides=('sql',))
