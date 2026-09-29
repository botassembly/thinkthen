"""A swapped packed answer must fail the literal COBOL row assertion."""
import json
import os
from pathlib import Path
import subprocess
import sys

HERE = Path(__file__).resolve().parent
env = os.environ.copy()
env["TT_PLANT_PACKED_SWAP"] = "1"
result = subprocess.run([sys.executable, str(HERE / "run_matrix.py")], cwd=HERE,
                        env=env, capture_output=True, text=True, timeout=80)
assert result.returncode != 0 and "PACKED_NEGATIVE_ASSERTED" in result.stdout, (result.returncode, result.stdout, result.stderr)
line = next(line for line in result.stdout.splitlines() if line.startswith('{"status":'))
outcome = json.loads(line)
assert outcome["arrivals"] == 7, outcome
print("COBOL swapped packed answer rejected after 7 exact arrivals")
