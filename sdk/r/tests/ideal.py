"""Installed-package entrypoint for the shared R literal guide runner."""
from pathlib import Path
import os
import sys

sys.path.insert(0, str(Path(__file__).resolve().parents[3] / 'scripts'))
from r_guide_examples import run_r

run_r(negative=os.environ.get('THINKTHEN_GUIDE_NEGATIVE'))
