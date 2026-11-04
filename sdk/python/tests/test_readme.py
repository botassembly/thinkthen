"""Run the documented quickstart against the installed wheel."""

import os
import re
import subprocess
import sys
from pathlib import Path


def test_readme_quickstart(tmp_path: Path) -> None:
    readme = (Path(__file__).parents[1] / "README.md").read_text()
    example = re.search(r"```python\n(.*?)```", readme, re.DOTALL)
    assert example is not None
    code = tmp_path / "example.py"
    code.write_text(example[1])
    # Readers see the default client; the example runs on the stub.
    environment = {**os.environ, "THINKTHEN_BACKEND": "stub"}
    subprocess.run([sys.executable, str(code)], check=True, env=environment)
