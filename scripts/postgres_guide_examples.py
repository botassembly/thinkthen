"""Consume the PostgreSQL guide in its already installed PostgreSQL runtime."""
import importlib.util
import os
from pathlib import Path
import sys
from types import SimpleNamespace

ROOT = Path(__file__).resolve().parent.parent


def run_postgresql(loopback=None, negative=None):
    loopback = loopback or os.environ.get('THINKTHEN_LOOPBACK')
    if not loopback:
        raise RuntimeError('PostgreSQL needs its prepared loopback; use scripts/postgres-gate --development ideal')
    here = ROOT / 'extensions/postgres/tests'
    sys.path.insert(0, str(here))
    spec = importlib.util.spec_from_file_location('postgres_guide_contract', here / 'typed_api.py')
    contract = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(contract)
    guide_spec = importlib.util.spec_from_file_location('postgres_ideal', here / 'ideal.py')
    guide = importlib.util.module_from_spec(guide_spec)
    guide_spec.loader.exec_module(guide)
    guide.run(SimpleNamespace(loopback=Path(loopback)), contract.server, contract.query, contract.failure, negative)
