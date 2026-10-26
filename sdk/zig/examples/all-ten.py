"""Manually call all ten functions through the compiled Zig public consumer."""
import json
from pathlib import Path
import subprocess
import sys

bundle = Path(sys.argv[1]).resolve()
settings = {"backend": "stub"} if sys.argv[2:] == ["--stub"] else {}
manifest = json.loads((bundle / "typed/manifest.json").read_text())
for filename in manifest["cases"]:
    case = json.loads((bundle / "typed" / filename).read_text())
    request = case["request"]
    actual = json.loads(subprocess.check_output(
        [str(bundle / "consumer"), "run", json.dumps(settings), json.dumps(request)], text=True))
    if isinstance(case["expected"], dict):
        assert actual == case["expected"], actual
        continue
    assert isinstance(actual, list) and actual, (request["question"]["function"], actual)
    assert all(row["status"] in ("answered", "unsure", "failed") for row in actual), actual
    print(request["question"]["function"], json.dumps(actual, ensure_ascii=False))
