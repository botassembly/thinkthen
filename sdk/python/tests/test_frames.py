"""Feature: frame adapters map every public function without losing row alignment."""

import os
from pathlib import Path

from frames import check_frames


def test_every_function_on_pandas_and_polars_frames() -> None:
    check_frames(Path(os.environ["THINKTHEN_CASES"]))
