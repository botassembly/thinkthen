"""Use the prepared Linux Swift toolchain for a bounded Foundation witness."""
import os
from pathlib import Path
import subprocess

root = Path(__file__).resolve().parents[3]
subprocess.run([str(root / 'scripts/linux-binding-gate'), '--abi-binding', 'swift',
                '--scratch', os.environ['TMPDIR'], '--', 'swift',
                str(root / 'sdk/swift/tests/probes/NameInventory.swift')], check=True)
