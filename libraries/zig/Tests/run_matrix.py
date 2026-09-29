"""Run the product Zig consumer matrix against one owned loopback backend."""
from collections import Counter
import json
import os
from pathlib import Path
import tempfile

from backend import Backend
from process_group import run

PACKAGE = Path(__file__).resolve().parent.parent
BIN = PACKAGE / "Tests/zig-out/bin"
ROOT = PACKAGE / "target/logs"
ROOT.mkdir(parents=True, exist_ok=True)
with tempfile.TemporaryDirectory(prefix="zig-matrix-", dir=ROOT) as folder:
    home = Path(folder)
    barrier = home / "barrier"
    barrier.mkdir()
    cache = home / "cache"
    cache.mkdir()
    backend = Backend(barrier)
    receipts = []
    try:
        env = {"PATH": os.environ.get("PATH", "/usr/bin:/bin"), "HOME": str(home),
               "XDG_CONFIG_HOME": str(home), "THINKTHEN_CACHE": str(cache),
               "THINKTHEN_API_KEY": "tt-canary-273", "TT_BARRIER_DIR": str(barrier),
               "THINKTHEN_BASE_URL": f"http://127.0.0.1:{backend.server_port}/generic/v1",
               "LD_LIBRARY_PATH": str(PACKAGE / "target/native/lib")}
        def case(name, command, expected, marker):
            result = run(command, cwd=PACKAGE / "Tests", env=env, timeout=60)
            output = result.stdout + result.stderr
            receipts.append({"case": name, "exit": result.exit, "signals": result.signals,
                             "arrivals": len(backend.arrivals)})
            assert result.exit == 0 and marker.encode() in output, (name, result.exit, output[-1400:])
            assert len(backend.arrivals) == expected, (name, len(backend.arrivals), expected)
        case("example", [str(PACKAGE / "zig-out/bin/thinkthen-example")], 1, "yes 0.90")
        case("matrix", [str(BIN / "matrix")], 32, "matrix: ten verbs")
        case("allocation", [str(BIN / "allocation")], 34, "allocation: bulk indexes")
        case("concurrent", [str(BIN / "concurrent"), "--callers-only"], 37, "concurrent: three callers PASS")
        case("held", [str(BIN / "concurrent"), "--holds-only"], 42, "fresh-token recovery recovery-scalar PASS")
        (ROOT / "observed_requests.json").write_text(json.dumps(backend.arrivals, ensure_ascii=False, indent=2) + "\n")
        expected = json.loads((PACKAGE / "Tests/expected_requests.json").read_text())
        normalize = lambda rows: Counter(json.dumps(row, ensure_ascii=False, sort_keys=True, separators=(",", ":")) for row in rows)
        assert normalize(backend.arrivals) == normalize(expected), "complete normalized Zig request bodies differ"
        assert backend.bulk_completion == [], backend.bulk_completion
        print(f"Zig matrix: {len(backend.arrivals)} exact request bodies, held cancellation and recovery PASS")
    finally:
        (home / "receipts.json").write_text(json.dumps(receipts, indent=2) + "\n")
        backend.close()
