"""Manually call all ten functions through the compiled Zig public consumer."""
import json
from pathlib import Path
import subprocess
import sys

bundle = Path(sys.argv[1]).resolve()
settings = {"adapter": "stub"} if sys.argv[2:] == ["--stub"] else {}
for case in json.loads((bundle / "stub.json").read_text()):
    request = case["request"]
    actual = json.loads(subprocess.check_output(
        [str(bundle / "consumer"), "run", json.dumps(settings), json.dumps(request)], text=True))
    assert isinstance(actual, list) and actual, (request["function"], actual)
    assert all(row["status"] in ("answered", "unsure") for row in actual), actual
    print(request["function"], json.dumps(actual, ensure_ascii=False))
