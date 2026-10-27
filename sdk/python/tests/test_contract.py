"""Preserve Python exception transport, string enums and canonical declarations."""

from __future__ import annotations

import importlib.util
import json
import pickle
import shutil
import sys
from pathlib import Path
from typing import Any

import pytest

import thinkthen as tt


@pytest.mark.parametrize(
    "kind",
    [
        tt.ThinkThenError,
        tt.UsageError,
        tt.SetupError,
        tt.Cancelled,
        tt.InternalError,
        tt.RecordError,
    ],
)
def test_exceptions_pickle_with_code_message_and_http_status(
    kind: type[tt.ThinkThenError],
) -> None:
    error = kind(tt.ErrorCode.BACKEND_STATUS, "backend refused the request", 503)
    restored = pickle.loads(pickle.dumps(error))
    assert type(restored) is kind
    assert restored.code is tt.ErrorCode.BACKEND_STATUS
    assert str(restored) == "backend refused the request"
    assert restored.http_status == 503


@pytest.mark.parametrize(
    "kind", [tt.Function, tt.Status, tt.Backend, tt.BoundaryTag, tt.ErrorCode]
)
def test_enums_format_as_their_wire_values(kind: Any) -> None:
    for value in kind:
        assert str(value) == value.value
        assert f"{value}" == value.value
        assert format(value, ">30") == format(value.value, ">30")


@pytest.mark.parametrize(
    "source",
    ["functions", "statuses", "backends", "codes", "BoundaryTag", "stale_codes"],
)
def test_contract_check_rejects_drift(tmp_path: Path, source: str) -> None:
    root = Path(__file__).resolve().parents[3]
    for name in (
        "sdk/python/thinkthen/_types.py",
        "sdk/python/thinkthen/_codes.py",
        "schemas/contract.json",
        "crates/thinkthen/src/core/function/recognize.rs",
    ):
        target = tmp_path / name
        target.parent.mkdir(parents=True, exist_ok=True)
        shutil.copyfile(root / name, target)
    sys.path.insert(0, str(root / "sdk/python"))
    try:
        spec = importlib.util.spec_from_file_location(
            "check_contract", root / "sdk/python/check_contract.py"
        )
        assert spec is not None and spec.loader is not None
        checker = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(checker)
        checker.check(tmp_path)
        if source == "BoundaryTag":
            path = tmp_path / "crates/thinkthen/src/core/function/recognize.rs"
            path.write_text(path.read_text().replace('"BEGIN"', '"NEW_TAG"', 1))
        elif source == "stale_codes":
            path = tmp_path / "sdk/python/thinkthen/_codes.py"
            path.write_text(path.read_text() + "# stale declaration\n")
        else:
            path = tmp_path / "schemas/contract.json"
            contract = json.loads(path.read_text())
            contract[source].append({"name": "new-value", "value": 999})
            path.write_text(json.dumps(contract))
        with pytest.raises(AssertionError):
            checker.check(tmp_path)
    finally:
        sys.path.pop(0)
