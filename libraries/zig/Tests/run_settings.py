"""Prove Zig settings precedence and invalid-settings refusal at the public API."""
import json
import os
from pathlib import Path
import tempfile
from backend import Backend
from process_group import run

package = Path(__file__).resolve().parent.parent
logs = package / "target/logs"
logs.mkdir(parents=True, exist_ok=True)
with tempfile.TemporaryDirectory(prefix="zig-settings-", dir=logs) as folder:
    place = Path(folder)
    for name in ("configured", "environment"):
        (place / name).mkdir()
    configured = Backend(place / "configured")
    environment = Backend(place / "environment")
    try:
        env = {"PATH": os.environ.get("PATH", "/usr/bin:/bin"), "HOME": str(place),
               "THINKTHEN_API_KEY": "tt-canary-273", "THINKTHEN_CACHE": str(place / "cache"),
               "THINKTHEN_BASE_URL": f"http://127.0.0.1:{environment.server_port}/generic/v1",
               "LD_LIBRARY_PATH": str(package / "target/native/lib")}
        result = run([str(package / "Tests/zig-out/bin/settings"),
                      f"http://127.0.0.1:{configured.server_port}/generic/v1", str(place / "settings-cache")],
                     cwd=package, env=env, timeout=45)
        assert result.exit == 0 and b"ZIG_SETTINGS_PASS" in result.stderr, (result.exit, result.stderr[-900:])
        expected = lambda record: {"state": "Each question quotes the text it asks about.", "model": "jev-1.13.0",
                                   "questions": {"q1": {"type": "noul", "instructions": f'The text is "{record}". Is it?'}}}
        assert configured.arrivals + environment.arrivals == [expected(state) for state in ("zig-settings", "zig-env", "zig-empty")]
        print("Zig settings: configured URL 1, environment URL 2, invalid settings zero PASS")
    finally:
        configured.close(); environment.close()
