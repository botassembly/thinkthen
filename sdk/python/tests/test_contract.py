"""Preserve Python exception transport, string enums and canonical declarations."""

from __future__ import annotations

import json
import pickle
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
        tt.Failed,
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
    "entry",
    json.loads(
        (Path(__file__).resolve().parents[3] / "schemas/contract.json").read_text()
    )["codes"],
    ids=lambda entry: entry["name"],
)
def test_native_error_family_selects_the_public_exception(
    entry: dict[str, Any],
) -> None:
    from thinkthen import _native, _runtime

    def native_failure() -> None:
        raise _native.Error(entry["family"], entry["name"], "fixture error", None)

    expected: type[tt.ThinkThenError] = (
        tt.UsageError if entry["family"] == "usage" else tt.SetupError
    )
    if entry["name"] in {"usage.abi", "usage.key-missing"}:
        expected = tt.SetupError
    elif entry["name"] == "local.cancelled":
        expected = tt.Cancelled
    elif entry["family"] == "internal":
        expected = tt.InternalError
    with pytest.raises(tt.ThinkThenError) as caught:
        _runtime.binding(native_failure)
    assert type(caught.value) is expected
    assert caught.value.code.value == entry["name"]
    assert str(caught.value) == "fixture error"
