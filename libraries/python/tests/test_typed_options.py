"""Installed stubs refuse malformed settings and controls before a call."""
import subprocess
import sys
from conftest import clean_env


def test_installed_type_checker_rejects_malformed_controls(tmp_path):
    caller = tmp_path / "caller.py"
    caller.write_text('''import thinkthen as tt
engine = tt.Engine(throttle="two", replay=12)
engine.decide("Late?", "note", batch=[], attempts="yes", unknown=True)
engine.recognize("Ada", mode="wrong", seed_spans=[{"start": 0}])
engine.choose("Team?", "note", options=9)
engine.decide("Late?", "note", model=None)
async def wrong() -> None:
    await engine.asyncio.decide("Late?", "note", deadline_ms="forever")
''')
    checked = subprocess.run([sys.executable, "-m", "mypy", "--strict", "--no-incremental", str(caller)],
                             capture_output=True, text=True, env=clean_env(home=tmp_path))
    assert checked.returncode == 1, checked.stdout + checked.stderr
    for control in ("throttle", "replay", "batch", "attempts", "unknown", "mode", "end", "options", "deadline_ms", "None"):
        assert control in checked.stdout, checked.stdout
