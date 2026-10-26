"""Exchange the shared saved-toolbox cases through native consumers."""
import json
from pathlib import Path
import subprocess
import sys
from admission import check as check_admission

consumer, root, *others = sys.argv[1:]
sys.path.insert(0, str(Path(root) / "databases"))
from testing import child_environment

env = child_environment()
fixtures = Path(root) / "conformance/toolbox"
saved = (fixtures / "questions.json").read_text()
expected = json.loads(saved)
malformed = json.loads((fixtures / "malformed.json").read_text())
consumers = [consumer, *others]
# Close the exchange so the first surface also loads the last surface's output.
for executable in [*consumers, consumer]:
    saved = subprocess.check_output([executable, "toolbox", saved], text=True, env=env)
    assert json.loads(saved) == expected, (executable, saved)
for executable in consumers:
    for invalid in malformed:
        result = subprocess.check_output([executable, "toolbox-invalid", invalid], text=True, env=env)
        assert result.splitlines()[0] == "usage.question", (executable, result)
    surface = subprocess.check_output([executable, "admission-surface"], text=True, env=env).strip()
    check_admission(executable, root, surface, env)
print("saved toolbox: JSON exchange, native lookup, owned results and shared errors pass")
