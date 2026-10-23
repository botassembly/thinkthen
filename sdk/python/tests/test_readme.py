"""Check the documented dataframe entry points against the installed wheel."""

import re
import subprocess
import sys
from pathlib import Path

import pytest


@pytest.mark.parametrize("library", ["pandas", "Polars"])
def test_readme_wrapper_returns_a_typed_dataframe(library: str, tmp_path: Path) -> None:
    readme = (Path(__file__).parents[1] / "README.md").read_text()
    section = readme.split(f"### {library} example\n", 1)[1].split("### ", 1)[0]
    example = re.search(r"```python\n(.*?)```", section, re.DOTALL)
    assert example is not None
    code = tmp_path / "example.py"
    alias = "pd" if library == "pandas" else "pl"
    code.write_text(
        "from typing_extensions import assert_type\n"
        + example[1]
        + f"\nassert_type(result, {alias}.DataFrame)\n"
    )
    checked = subprocess.run(
        [
            "mypy",
            "--strict",
            "--no-incremental",
            "--python-executable",
            sys.executable,
            str(code),
        ],
        text=True,
        capture_output=True,
        check=False,
    )
    assert checked.returncode == 0, checked.stdout + checked.stderr
