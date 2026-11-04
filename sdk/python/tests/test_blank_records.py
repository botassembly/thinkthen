"""A blank record is a usage mistake in a one-record call and a row in a stream."""

from __future__ import annotations

import asyncio
from collections.abc import Iterator
from pathlib import Path
from typing import Any

import pytest
from api_support import (
    backend,
    client,
    isolated_environment,  # noqa: F401 - shared pytest fixture.
)
from conformance import Loopback

import thinkthen as tt

pytestmark = pytest.mark.usefixtures("isolated_environment")

BLANK: list[Any] = ["", "   \n", {}]


@pytest.fixture
def server(tmp_path: Path) -> Iterator[Loopback]:
    with backend(tmp_path, [{"probability": 0.9}]) as running:
        yield running


@pytest.mark.parametrize("record", BLANK)
def test_a_blank_one_record_call_raises_record_empty(
    server: Loopback, record: Any
) -> None:
    with client(server) as selected, pytest.raises(tt.RecordError) as raised:
        selected.decide("Ready?", record)
    assert raised.value.code is tt.ErrorCode.RECORD_EMPTY
    assert server.count() == 0


def test_a_blank_async_one_record_call_raises_record_empty(server: Loopback) -> None:
    async def scenario() -> None:
        async with tt.AsyncClient(
            backend="systemone",
            url=server.url,
            api_key="synthetic-loopback-key",
            batch=1,
            retry=tt.RetryPolicy(max_retries=0),
        ) as selected:
            await selected.score("How big?", " ", levels=["small", "large"])

    with pytest.raises(tt.RecordError) as raised:
        asyncio.run(scenario())
    assert raised.value.code is tt.ErrorCode.RECORD_EMPTY
    assert server.count() == 0


def test_a_blank_record_in_a_stream_is_a_failed_row(server: Loopback) -> None:
    with client(server) as selected:
        rows = selected.decide_stream("Ready?", ["", "body"]).to_list()
    by_position = {row.position: row for row in rows}
    assert by_position[1].failed
    assert by_position[1].error is not None
    assert by_position[1].error.code is tt.ErrorCode.RECORD_EMPTY
    assert by_position[2]
