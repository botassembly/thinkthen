"""Async streams own their worker and release it at their public boundary."""

from __future__ import annotations

import asyncio
import threading
from collections.abc import AsyncIterator, Iterator
from contextvars import ContextVar

import pytest
from api_support import isolated_environment  # noqa: F401 - shared pytest fixture.

import thinkthen as tt

pytestmark = pytest.mark.usefixtures("isolated_environment")


@pytest.fixture
def started_threads(monkeypatch: pytest.MonkeyPatch) -> list[threading.Thread]:
    started: list[threading.Thread] = []
    original = threading.Thread.start

    def start(thread: threading.Thread) -> None:
        original(thread)
        started.append(thread)

    monkeypatch.setattr(threading.Thread, "start", start)
    return started


def test_async_stream_starts_one_thread_for_a_thousand_rows(
    started_threads: list[threading.Thread],
) -> None:
    async def scenario() -> None:
        async with tt.AsyncClient(backend="stub") as client:
            for _ in range(2):
                started_threads.clear()
                stream = client.decide_stream("Ready?", ("body" for _ in range(1000)))
                rows = await stream.to_list()
                assert [row.position for row in rows] == list(range(1, 1001))
                assert all(row.error is None for row in rows)
                assert len(started_threads) == 1
                assert not started_threads[0].is_alive()

    asyncio.run(scenario())


def test_a_hundred_async_streams_do_not_share_an_executor_limit(
    started_threads: list[threading.Thread],
) -> None:
    async def scenario() -> None:
        entered = 0
        ready = asyncio.Event()

        async def records() -> AsyncIterator[str]:
            nonlocal entered
            entered += 1
            if entered == 100:
                ready.set()
            await ready.wait()
            yield "body"

        async with tt.AsyncClient(backend="stub") as client:
            results = await asyncio.wait_for(
                asyncio.gather(
                    *(
                        client.decide_stream("Ready?", records()).to_list()
                        for _ in range(100)
                    )
                ),
                timeout=30,
            )
            assert all(len(rows) == 1 and rows[0].error is None for rows in results)
            assert len(started_threads) == 100
            assert all(not thread.is_alive() for thread in started_threads)

    asyncio.run(scenario())


@pytest.mark.parametrize("cancel_task", [False, True])
def test_async_worker_exits_after_early_stop_or_pending_input_cancellation(
    started_threads: list[threading.Thread], cancel_task: bool
) -> None:
    async def scenario() -> None:
        entered = asyncio.Event()
        released = asyncio.Event()

        async def records() -> AsyncIterator[str]:
            entered.set()
            await released.wait()
            yield "body"

        async with tt.AsyncClient(backend="stub") as client:
            token = tt.Cancel()
            stream = client.decide_stream(
                "Ready?",
                records() if cancel_task else ["first", "second"],
                cancel=token,
            )
            if cancel_task:
                pending = asyncio.create_task(anext(stream))
                await asyncio.wait_for(entered.wait(), timeout=10)
                pending.cancel()
                with pytest.raises(asyncio.CancelledError):
                    await pending
            else:
                assert (await anext(stream)).position == 1
            await stream.aclose()
            assert await stream.to_list() == []
            assert len(started_threads) == 1
            assert not started_threads[0].is_alive()
            assert (await client.decide("Ready?", "again", cancel=token)).error is None

    asyncio.run(scenario())


def test_async_input_keeps_the_callers_context() -> None:
    current = ContextVar("stream-input", default="missing")

    def records() -> Iterator[str]:
        assert current.get() == "caller"
        yield "body"

    async def scenario() -> None:
        current.set("caller")
        async with tt.AsyncClient(backend="stub") as client:
            rows = await client.decide_stream("Ready?", records()).to_list()
            assert len(rows) == 1 and rows[0].error is None

    asyncio.run(scenario())
