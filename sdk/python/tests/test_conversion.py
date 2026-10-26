"""Classify Python conversion failures at their public call or record boundary."""

from __future__ import annotations

import os
from collections.abc import Iterator
from pathlib import Path
from typing import Any

import pytest
from conformance import Loopback

import thinkthen as tt


def invalid_value(kind: str) -> Any:
    if kind == "object":
        return object()
    if kind == "surrogate":
        return "\ud800"
    if kind == "cycle":
        value: list[Any] = []
        value.append(value)
        return value
    return float(kind)


@pytest.mark.parametrize("kind", ["object", "cycle", "nan", "inf", "surrogate"])
@pytest.mark.parametrize("context", ["settings", "question", "toolbox"])
def test_unconvertible_call_values_have_native_usage_codes_before_any_record(
    kind: str,
    context: str,
) -> None:
    secret = "synthetic-key-that-must-stay-private"
    value = invalid_value(kind)
    consumed: list[str] = []

    def records() -> Iterator[str]:
        consumed.append("input")
        yield "body"

    with pytest.raises(tt.UsageError) as failure:
        if context == "settings":
            tt.Client(backend="stub", api_key=secret, model=value)
        elif context == "question":
            tt.Client(backend="stub").decide_stream(value, records())
        else:
            tt.Toolbox({"bad": value})
    assert failure.value.code == (
        "usage.settings" if context == "settings" else "usage.question"
    )
    assert consumed == []
    field = "settings" if context == "settings" else "question"
    assert str(failure.value) == (
        f"{field}: expected a JSON-representable {field} object; "
        "received an incompatible host value (contents redacted)"
    )
    assert secret not in str(failure.value)
    assert secret not in repr(failure.value.__context__)
    assert not isinstance(failure.value.__context__, (TypeError, ValueError))
    assert failure.value.__cause__ is None


@pytest.mark.parametrize("kind", ["object", "cycle", "nan", "inf", "surrogate"])
@pytest.mark.parametrize("backend", ["systemone", "openai"])
def test_record_conversion_failures_keep_positions_without_backend_requests(
    kind: str,
    backend: str,
) -> None:
    path = Path(os.environ["THINKTHEN_CASES"]) / "typed/decide-precision.json"
    with Loopback(path) as server:
        client = tt.Client(
            backend=tt.Backend(backend), url=server.url, api_key="synthetic", batch=1
        )
        single = client.decide("Ready?", invalid_value(kind))
        assert single.status == "failed" and single.position == 1
        assert single.error is not None and single.error.code == "record.invalid"
        assert server.count() == 0
        with client.decide_stream(
            "Ready?", [invalid_value(kind), "evidence"]
        ) as stream:
            rows = list(stream)
        assert [row.position for row in rows] == [1, 2]
        assert [row.status for row in rows] == ["failed", "answered"]
        assert rows[0].error is not None and rows[0].error.code == "record.invalid"
        assert rows[1].answer is True and rows[1].probability == 0.8123456789012345
        assert server.count() == 1


@pytest.mark.parametrize("kind", [TypeError, ValueError, RuntimeError])
def test_caller_iterator_exceptions_keep_identity_and_close_the_operation(
    kind: type[Exception],
) -> None:
    original = kind("caller iterator failed")

    def records() -> Iterator[str]:
        yield "first"
        raise original

    client = tt.Client(backend="stub")
    with (
        client.decide_stream("Ready?", records()) as stream,
        pytest.raises(kind) as failure,
    ):
        list(stream)
    assert failure.value is original
    assert list(stream) == []
    assert client.decide("Ready?", "another").position == 1
