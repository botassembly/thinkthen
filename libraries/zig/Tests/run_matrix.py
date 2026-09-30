"""Run the product Zig consumer matrix against one owned loopback backend."""
from collections import Counter
import json
import os
from pathlib import Path
import tempfile
import sys

from backend import Backend, one_record
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
        if len(sys.argv)==2 and sys.argv[1]=='facts':
            case("facts", [str(BIN / "concurrent"), "--facts"], 4, "ZIG_FACTS_LIFETIME_PASS")
            assert Counter(one_record(row)['state'] for row in backend.arrivals)==Counter(['hold-facts-one','hold-facts-no-usage','status-401','recovery-scalar']),backend.arrivals
            print("Zig facts: two held arrivals, failure and recovery exact=4 PASS")
        elif len(sys.argv)==2 and sys.argv[1]=='facts-allocation':
            case("facts-allocation", [str(BIN / "allocation"), "--facts-allocation"], 4, "facts allocation index 7:")
            assert Counter(one_record(row)['state'] for row in backend.arrivals)==Counter({f'alloc-facts-{i}':1 for i in range(4,8)}),backend.arrivals
            print("Zig post-native facts allocation faults exact=4 PASS")
        else:
            case("example", [str(PACKAGE / "zig-out/bin/thinkthen-example")], 1, "yes 0.90")
            # The repeated first/second bulk call reads the question cache (ADR 0111),
            # so every later count is one lower.
            case("matrix", [str(BIN / "matrix")], 31, "matrix: ten verbs")
            case("allocation", [str(BIN / "allocation")], 33, "allocation: bulk indexes")
            case("concurrent", [str(BIN / "concurrent"), "--callers-only"], 36, "concurrent: three callers PASS")
            case("held", [str(BIN / "concurrent"), "--holds-only"], 41, "fresh-token recovery recovery-scalar PASS")
            (ROOT / "observed_requests.json").write_text(json.dumps(backend.arrivals, ensure_ascii=False, indent=2) + "\n")
            expected = json.loads((PACKAGE / "Tests/expected_requests.json").read_text())
            normalize = lambda rows: Counter(json.dumps(row, ensure_ascii=False, sort_keys=True, separators=(",", ":")) for row in rows)
            assert normalize(backend.arrivals) == normalize(expected), "complete normalized Zig request bodies differ"
            assert backend.bulk_completion == [], backend.bulk_completion
            print(f"Zig matrix: {len(backend.arrivals)} exact request bodies, held cancellation and recovery PASS")
    finally:
        (home / "receipts.json").write_text(json.dumps(receipts, indent=2) + "\n")
        backend.close()
