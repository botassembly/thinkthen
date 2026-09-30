"""Prove settings precedence and pre-send refusal through the public Swift API."""
from pathlib import Path
import json
import os
import tempfile
from backend import Backend
from process_group import run

package = Path(__file__).resolve().parents[2]
logs = package / "target/logs"
logs.mkdir(parents=True, exist_ok=True)
with tempfile.TemporaryDirectory(prefix="swift-settings-", dir=logs) as folder:
    place = Path(folder)
    for name in ("configured", "environment"):
        (place / name).mkdir()
    configured = Backend(place / "configured")
    environment = Backend(place / "environment")
    try:
        env = {"PATH": os.environ.get("PATH", "/usr/bin:/bin"), "HOME": str(place),
               "THINKTHEN_API_KEY": "tt-canary-294", "THINKTHEN_CACHE": str(place / "cache"),
               "THINKTHEN_BASE_URL": f"http://127.0.0.1:{environment.server_port}/generic/v1",
               "LD_LIBRARY_PATH": str(package / "target/native/lib")}
        result = run([str(package / "target/scratch/swift-settings"),
                      f"http://127.0.0.1:{configured.server_port}/generic/v1", str(place / "settings-cache")],
                     cwd=package, env=env, timeout=45)
        assert result.exit == 0 and b"SWIFT_SETTINGS_PASS" in result.stdout, (result.exit, result.stderr[-900:])
        assert configured.arrivals == ["swift-settings"] and environment.arrivals == ["swift-env", "swift-empty"]
        bodies = [json.loads(line) for name in ("configured", "environment")
                  for line in (place / name / "requests.jsonl").read_text().splitlines()]
        assert bodies == [{"state": "Each question quotes the text it asks about.", "model": "jev-1.13.0",
                           "questions": {"q1": {"type": "noul", "instructions": f'The text is "{record}". Is it?'}}}
                          for record in ("swift-settings", "swift-env", "swift-empty")], bodies
        print("Swift settings: configured URL 1, environment URL 2, invalid settings zero PASS")
    finally:
        configured.close(); environment.close()
