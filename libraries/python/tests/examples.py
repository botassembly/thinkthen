"""Every example in ``examples.json``, each in a fresh child with its own
cache folder, against the conformance backend's generic arm.

Run as ``python tests/examples.py PORT``. The site's Python tab draws on
the same file, so an example nobody runs cannot reach a page.
"""

import json
import os
import pathlib
import subprocess
import sys
import tempfile

FILE = pathlib.Path(__file__).resolve().parents[1] / "examples.json"


def run(example, port):
    with tempfile.TemporaryDirectory() as folder:
        for name, content in example.get("files", {}).items():
            pathlib.Path(folder, name).write_text(content)
        env = dict(os.environ, THINKTHEN_BASE_URL=f"http://127.0.0.1:{port}/generic/v1",
                   THINKTHEN_CACHE=str(pathlib.Path(folder, "cache")))
        program = f"import thinkthen as tt\nprint(repr({example['python']}))"
        done = subprocess.run([sys.executable, "-c", program], capture_output=True, text=True,
                              cwd=folder, env=env, timeout=60)
        return (done.stdout + done.stderr).strip()


def main(port):
    examples = json.loads(FILE.read_text())["examples"]
    failed = 0
    for name, example in examples.items():
        got = run(example, port)
        if got != example["expected"]:
            print(f"FAIL {name}: want {example['expected']!r}, got {got!r}")
            failed += 1
    print(f"examples: {len(examples) - failed} of {len(examples)} ok")
    return 1 if failed else 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1]))
