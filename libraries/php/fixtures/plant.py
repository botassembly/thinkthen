"""Prove a changed packed answer fails the existing public PHP row assertion."""

import os
from pathlib import Path
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[3]
env = dict(os.environ, TT_PLANT_BAD_PACKED="1")
result = subprocess.run([sys.executable, "libraries/php/fixtures/run_matrix.py"], cwd=ROOT, env=env,
                        capture_output=True, text=True, timeout=120)
assert result.returncode == 1 and "FAILED:matrix" in result.stdout, (result.returncode, result.stdout, result.stderr)
folder = Path(result.stdout.split()[0])
assert "reverse completion reorders" in (folder / "matrix.log").read_text(), folder
print("PHP_PACKED_PLANT_REJECTED reverse completion reorders")
