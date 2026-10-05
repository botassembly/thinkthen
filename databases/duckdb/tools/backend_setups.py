"""Setup profile captured by the SQL engine: refusal then explicit override."""
import json
import subprocess
import sys
import tempfile
from pathlib import Path
from harness import Backend, CHILD, EXTENSION, case, child_env, expect, main


@case
def setup_profile_refuses_before_send_and_sql_profile_wins():
    with Backend() as backend, tempfile.TemporaryDirectory(prefix="thinkthen-0400-") as scratch:
        root = Path(scratch)
        env = child_env(backend.base(), root)
        env.update(LOCAL_SETUP_KEY="fake-0400")
        folder = Path(env["HOME"]) / "Library/Application Support/thinkthen" if sys.platform == "darwin" else Path(env["XDG_CONFIG_HOME"]) / "thinkthen"
        folder.mkdir(parents=True, exist_ok=True)
        (folder / "config.json").write_text(json.dumps({"schema": "thinkthen.config/1", "backends": {"small": {
            "url": backend.base(), "key_env": "LOCAL_SETUP_KEY", "model": "m",
            "profile": {"schema": "thinkthen.backend-profile/1", "name": "small", "max_evidence_bytes": 3}}}}))
        sql = "SELECT thinkthen_tag('Which labels fit?', 'alpha', ['one', 'two'])"
        done = subprocess.run([sys.executable, "-c", CHILD, str(EXTENSION), json.dumps(["SET thinkthen_backend = 'small'", sql])], env=env, capture_output=True, text=True, timeout=60, check=False)
        expect(done.returncode, 0, "the child reports SQL failure as a result")
        assert "profile small" in done.stdout, done.stdout
        assert "fake-0400" not in done.stdout + done.stderr
        expect(backend.count(), 0, "setup refusal sends nothing")
        profile = json.dumps({"schema": "thinkthen.backend-profile/1", "name": "explicit", "max_evidence_bytes": 100})
        done = subprocess.run([sys.executable, "-c", CHILD, str(EXTENSION), json.dumps(["SET thinkthen_backend = 'small'", f"SET thinkthen_profile = '{profile}'", "SELECT thinkthen_decide('a refund?', 'alpha')"])], env=env, capture_output=True, text=True, timeout=60, check=False)
        expect(done.returncode, 0, "explicit SQL profile succeeds")
        assert '"rows": [[true]]' in done.stdout, done.stdout
        expect(backend.count(), 1, "explicit profile permits one request")


if __name__ == "__main__":
    main()
