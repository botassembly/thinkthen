"""Keep generated public names readable and unambiguous."""
import re
from pathlib import Path
import sys

sys.path.insert(0, str(Path(__file__).resolve().parents[3] / "build-tools/results"))
from sources import tracked_files

ROOT = Path(__file__).resolve().parents[1]


def inspect(name, data):
    # This exact canonical native wait-event arm is ABI, not a schema path.
    identifiers = re.sub(rb"(?<![A-Za-z0-9_-])(?:K_THINKTHEN_SESSION_WAIT_SETTLEMENT_ARM_V1|tt-n-session-wait-settlement-arm-v1-k)(?![A-Za-z0-9_-])", b"", data)
    if re.search(rb"(?i:arm[_-])|Arm[A-Z0-9]|Alternative", identifiers):
        raise ValueError(f"schema alternative marker in {name}")


sources = [path for path in tracked_files(ROOT)
           if path.relative_to(ROOT).as_posix() in {"README.md", "LICENSE"}
           or path.relative_to(ROOT).parts[0] in {"src", "Src", "Sources", "examples", "Examples"}]
for path in sources:
    if path.is_file():
        inspect(str(path), path.read_bytes())

for marker in (b"T_CacheDocument_arm_1", b"CacheDocumentArm2", b"CACHE-DOCUMENT-ARM-1",
               b"T_CacheDocument_arm_Text", b"CacheDocumentAlternative",
               b"K_THINKTHEN_SESSION_WAIT_UNKNOWN_ARM_V1", b"K_THINKTHEN_SESSION_WAIT_SETTLEMENT_ARM_V1_Extra"):
    try:
        inspect('planted identifier', marker)
    except ValueError:
        pass
    else:
        raise AssertionError('accepted schema alternative marker')
print('generated public name checks passed')
