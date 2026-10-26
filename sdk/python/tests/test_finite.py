"""Keep known finite batches intact without consuming general iterators early."""

from __future__ import annotations

import os
from collections.abc import Iterator
from pathlib import Path
from typing import Any

import pytest
from conformance import Loopback

import thinkthen as tt


@pytest.mark.parametrize("backend", ["systemone", "openai"])
@pytest.mark.parametrize("container", [list, tuple])
@pytest.mark.parametrize("batch,requests", [(32, 1), (2, 4)])
def test_known_finite_inputs_keep_batches_bounded_and_invalid_neighbors_visible(
    backend: str,
    container: type[list[Any] | tuple[Any, ...]],
    batch: int,
    requests: int,
) -> None:
    records = container(
        [
            {"id": "one", "body": "refund"},
            object(),
            {"id": "three", "body": "ordinary"},
            {"id": True, "body": "refund"},
            None,
            {"id": "six", "body": "refund"},
            {"id": "seven", "body": "ordinary"},
        ]
    )
    path = Path(os.environ["THINKTHEN_CASES"]) / "typed/filter-rows.json"
    with Loopback(path) as server:
        client = tt.Client(
            backend=tt.Backend(backend),
            url=server.url,
            api_key="synthetic",
            batch=batch,
        )
        with client.decide_stream("Ready?", records, evidence="/body") as stream:
            rows = list(stream)
        assert [row.position for row in rows] == [1, 2, 3, 4, 5, 6, 7]
        assert [row.status for row in rows] == [
            "answered",
            "failed",
            "answered",
            "failed",
            "failed",
            "answered",
            "answered",
        ]
        assert [row.answer for row in rows] == [
            True,
            None,
            False,
            None,
            None,
            True,
            False,
        ]
        assert [row.id for row in rows] == [
            "one",
            None,
            "three",
            None,
            None,
            "six",
            "seven",
        ]
        assert [row.error.code if row.error else None for row in rows] == [
            None,
            "record.invalid",
            None,
            "record.invalid",
            "record.missing",
            None,
            None,
        ]
        assert [row.probability for row in rows] == [
            0.9,
            None,
            0.1,
            None,
            None,
            0.9,
            0.1,
        ]
        assert server.count() == requests


def test_finite_question_admission_precedes_record_conversion() -> None:
    consumed: list[str] = []

    class RecordList(list[str]):
        def __iter__(self) -> Iterator[str]:
            consumed.append("converted")
            return super().__iter__()

    client = tt.Client(backend="stub")
    records = RecordList(["body"])
    invalid: Any = "not-a-numeric-threshold"
    with pytest.raises(tt.UsageError) as failure:
        client.decide_stream("Ready?", records, threshold=invalid)
    assert failure.value.code == "usage.threshold" and consumed == []


def test_general_iterables_stay_lazy_until_the_first_pull() -> None:
    consumed: list[str] = []
    original = ValueError("caller source failed")

    class Records:
        def __iter__(self) -> Iterator[str]:
            consumed.append("iteration")
            yield "first"
            raise original

    client = tt.Client(backend="stub")
    stream = client.decide_stream("Ready?", Records())
    assert consumed == []
    with stream, pytest.raises(ValueError) as failure:
        list(stream)
    assert failure.value is original and consumed == ["iteration"]
    assert list(stream) == []


@pytest.mark.parametrize("container", [list, tuple])
def test_empty_finite_inputs_send_no_requests(
    container: type[list[Any] | tuple[Any, ...]],
) -> None:
    path = Path(os.environ["THINKTHEN_CASES"]) / "typed/decide-precision.json"
    with Loopback(path) as server:
        client = tt.Client(url=server.url, api_key="synthetic")
        with client.decide_stream("Ready?", container()) as stream:
            assert list(stream) == []
        assert server.count() == 0
