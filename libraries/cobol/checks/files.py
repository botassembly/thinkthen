"""All ten source grammars through the public counted TT-FILES entry point."""
import json
from pathlib import Path
import subprocess
import sys
import tempfile
sys.path.insert(0, str(Path(__file__).resolve().parents[3] / "conformance/children"))
from children import child_env

ROOT = Path(__file__).resolve().parents[3]
HERE = Path(__file__).resolve().parent
FIXTURE = ROOT / "specification/fixtures/files"
questions = [{"decide": "Q?"}, {"choose": "Q?", "options": ["policy", "contract"]},
    {"tag": "Q?", "labels": ["refund", "support"]}, {"score": "Q?", "levels": ["low", "high"]},
    {"filter": "Q?"}, {"rank": "Q?"}, {"find": "Q?"},
    {"annotate": json.loads((FIXTURE / "questions.json").read_text())},
    {"version": 1, "recognize": {"kinds": {"person": None}}},
    {"version": 1, "relate": {"relations": [{"name": "supports", "source": "*", "target": "*"}]}}]
with tempfile.TemporaryDirectory(prefix="thinkthen-cobol-files-") as scratch:
    server = subprocess.Popen([ROOT / "target/debug/conformance-backend"], env=child_env(),
        stdin=subprocess.PIPE, stdout=subprocess.PIPE, text=True)
    try:
        port = int(server.stdout.readline())
        base = f"http://127.0.0.1:{port}/arm/full/v1"
        settings = json.dumps({"base_url": base, "cache": False, "max_retries": 0})
        env = child_env(THINKTHEN_API_KEY="tt-files-loopback", THINKTHEN_BASE_URL=base,
            THINKTHEN_CACHE=str(Path(scratch) / "cache"), HOME=scratch, LD_LIBRARY_PATH=str(HERE / "target"))
        source = {"paths": [str(FIXTURE / "documents")], "unit": "file"}
        for at, question in enumerate(questions):
            run = subprocess.run([HERE / "target/files", json.dumps(question), json.dumps(source), settings],
                env=env, capture_output=True, text=True, timeout=60)
            assert run.returncode == 0, (run.stdout, run.stderr)
            reply = json.loads(run.stdout)
            assert reply["facts"]["requests_sent"] > 0, reply
            rows = [endpoint for edge in reply["value"]["edges"] for endpoint in (edge["source"], edge["target"])] if at == 9 else [reply["value"]] if at == 6 else reply["value"]
            assert rows
            for row in rows:
                assert (row["first_line"], row["last_line"]) == (1, 4) and "\n" in row["record"], row
        server.stdin.write("count\n"); server.stdin.flush()
        before = int(server.stdout.readline())
        refused = subprocess.run([HERE / "target/files", '{"decide":"Q?"}', '{"paths":["missing"],"window":2}', settings],
            env=env, capture_output=True, text=True, timeout=30)
        assert refused.returncode == 1 and int(refused.stdout) == 1, (refused.stdout, refused.stderr)
        server.stdin.write("count\n"); server.stdin.flush()
        assert int(server.stdout.readline()) == before
        print("COBOL all ten source grammars and invalid-option no-send passed")
    finally:
        server.stdin.close()
        server.wait(timeout=30)
