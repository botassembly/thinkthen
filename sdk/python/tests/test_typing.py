"""Reject invalid calls through an installed package's public declarations."""

from __future__ import annotations

import subprocess
import sys
from pathlib import Path

import pytest


@pytest.mark.parametrize(
    "statement",
    [
        'client.choose("Which?", "body", choices="not-a-list")',
        (
            'tt.AnnotationQuestions({"ranking": '
            'tt.RankQuestion("How much?", levels=["low", "high"])})'
        ),
        'client.annotate(tt.Toolbox({"ready": tt.DecideQuestion("Ready?")}), "body")',
        'client.choose(tt.DecideQuestion("Ready?"), "body")',
    ],
)
def test_static_consumers_reject_wrong_domains(statement: str, tmp_path: Path) -> None:
    source = tmp_path / "consumer.py"
    source.write_text(
        'import thinkthen as tt\nclient = tt.Client(backend="stub")\n'
        + statement
        + "\n"
    )
    checked = subprocess.run(
        [
            "mypy",
            "--strict",
            "--no-incremental",
            "--python-executable",
            sys.executable,
            str(source),
        ],
        text=True,
        capture_output=True,
        check=False,
    )
    assert checked.returncode == 1, checked.stdout + checked.stderr
    assert "consumer.py:3: error:" in checked.stdout
