"""Exercise public convenience and async entry forms with shared typed rows."""

from __future__ import annotations

import asyncio
import json
from pathlib import Path
from typing import Any

import thinkthen as tt
from thinkthen import aio


def shared_entry_forms(directory: Path) -> None:
    from conformance import (
        Loopback,
        admitted,
        assert_expected,
        normalized_row,
        row_json,
    )

    path = directory / "decide-precision.json"
    case = json.loads(path.read_text())
    question = admitted(case["request"]["question"])
    records = case["request"]["records"]

    def check(rows: list[Any]) -> None:
        assert_expected(
            [normalized_row(row_json(row)) for row in rows], case["expected"]
        )

    async def asynchronous(settings: dict[str, Any]) -> None:
        async with tt.AsyncClient(**settings) as client:
            check([await client.decide(question, records[0])])
            async with client.decide_stream(question, records) as stream:
                check([row async for row in stream])
            check([await aio.decide(question, records[0], client=client)])
            async with aio.decide_stream(question, records, client=client) as stream:
                check([row async for row in stream])

    for backend in ("systemone", "openai"):
        with Loopback(path) as server:
            settings: dict[str, Any] = {
                "backend": backend,
                "url": server.url,
                "api_key": "synthetic",
            }
            with tt.Client(**settings) as client:
                check([tt.decide(question, records[0], client=client)])
                with tt.decide_stream(question, records, client=client) as stream:
                    check(list(stream))
            asyncio.run(asynchronous(settings))
            assert server.count() == case["requests"] * 6
    print("Shared public entry forms: top-level, AsyncClient and aio")
