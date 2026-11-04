"""Filter returns the caller's kept records, and rank rows carry theirs."""

from __future__ import annotations

import asyncio
import gc
import os
import weakref
from collections.abc import Iterator
from pathlib import Path
from typing import Any, cast

import pytest
from api_support import isolated_environment  # noqa: F401 - shared pytest fixture.
from conformance import Loopback

import thinkthen as tt
from thinkthen import aio

pytestmark = pytest.mark.usefixtures("isolated_environment")

# The shared case answers "refund" yes, "uncertain" not sure and anything else
# no. The missing record fails first, so kept records must still follow it.
RECORDS: list[tt.Record] = [None, "refund", "parcel", "uncertain"]
BAND = (0.2, 0.8)


@pytest.fixture
def server() -> Iterator[Loopback]:
    path = Path(os.environ["THINKTHEN_CASES"]) / "typed/filter-rows.json"
    with Loopback(path) as server:
        yield server


def connect(server: Loopback) -> tt.Client:
    return tt.Client(url=server.url, api_key="synthetic-loopback-key", batch=1)


def test_filter_delivers_kept_records_then_raises_one_error_for_failures(
    server: Loopback,
) -> None:
    kept: list[tt.Record] = []
    with connect(server) as selected, pytest.raises(tt.RecordError) as raised:
        for record in selected.filter("Ready?", RECORDS, threshold=BAND):
            kept.append(record)  # noqa: PERF402 - keep what arrives before the error.
    assert kept == ["refund"]
    assert raised.value.code is tt.ErrorCode.RECORD_MISSING
    assert [row.position for row in raised.value.rows] == [1]
    assert raised.value.rows[0].item is None


def test_filter_errors_coerce_yields_failed_rows_in_place(server: Loopback) -> None:
    with connect(server) as selected:
        found = selected.filter(
            "Ready?", RECORDS, threshold=BAND, errors="coerce"
        ).to_list()
        assert len(found) == 2 and found[1] == "refund"
        failed = found[0]
        assert isinstance(failed, tt.FilterRow) and failed.failed
        assert failed.error is not None
        assert failed.error.code is tt.ErrorCode.RECORD_MISSING
        with pytest.raises(ValueError, match="errors"):
            selected.filter("Ready?", RECORDS, errors=cast(Any, "ignore"))


def test_async_filter_delivers_kept_records_before_the_failure(
    server: Loopback,
) -> None:
    async def scenario() -> list[Any]:
        kept: list[Any] = []
        async with tt.AsyncClient(
            url=server.url, api_key="synthetic-loopback-key", batch=1
        ) as selected:
            with pytest.raises(tt.RecordError) as raised:
                async for record in aio.filter(
                    "Ready?", RECORDS, threshold=BAND, client=selected
                ):
                    kept.append(record)
            assert [row.position for row in raised.value.rows] == [1]
        return kept

    assert asyncio.run(scenario()) == ["refund"]


def test_rank_rows_carry_a_copy_of_their_record() -> None:
    records: list[Any] = [{"id": "a", "text": "first"}, "second", object()]
    with tt.Client(backend="stub") as selected:
        rows = selected.rank("How urgent?", records, levels=["low", "high"]).to_list()
    assert sorted(row.position for row in rows) == [1, 2, 3]
    for row in rows:
        if row.position == 3:
            # A record with no JSON form fails, and its item raises that failure.
            assert row.failed
            with pytest.raises(tt.RecordError) as raised:
                _ = row.item
            assert raised.value.code is tt.ErrorCode.RECORD_INVALID
            assert "object" in str(raised.value)
        else:
            original = records[row.position - 1]
            assert row.item == original
            assert row.item is not original or isinstance(original, str)


class Message(dict[str, str]):
    """A record type that weak references can observe."""


def test_lazy_top_one_rank_keeps_a_bounded_number_of_records() -> None:
    finished: list[int] = []

    def messages(count: int) -> Iterator[Message]:
        for number in range(count):
            message = Message(text=f"message {number}")
            weakref.finalize(message, finished.append, number)
            yield message

    count = 2000
    with (
        tt.Client(backend="stub") as selected,
        selected.rank(
            "How urgent?",
            messages(count),
            levels=["low", "high"],
            top=1,
            field="/text",
        ) as rows,
    ):
        first = next(rows)
        gc.collect()
        assert count - len(finished) <= 2
        assert first.item == {"text": f"message {first.position - 1}"}


def test_rank_exports_leave_out_the_record_copy() -> None:
    # Rank rows join back to their records by id or position, as the guide says
    # only a filter result exports its item.
    with tt.Client(backend="stub") as selected:
        rows = selected.rank("How urgent?", ["first"], levels=["low", "high"])
        exported = rows.to_list()[0].to_dict()
    assert "item" not in exported
    assert exported["position"] == 1
