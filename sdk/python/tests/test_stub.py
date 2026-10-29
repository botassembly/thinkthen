"""The existing facade needs neither credentials nor HTTP for the stub."""

from __future__ import annotations

import json
import os
from pathlib import Path
from typing import Any

import pytest
from conformance import Loopback, request_stream

import thinkthen as tt


def test_all_functions_use_the_stub_without_backend_requests() -> None:
    directory = Path(os.environ["THINKTHEN_CASES"]) / "typed"
    manifest = json.loads((directory / "manifest.json").read_text())
    with Loopback(directory / "decide-precision.json") as server:
        functions: set[str] = set()
        for filename in manifest["cases"]:
            case = json.loads((directory / filename).read_text())
            request = case["request"]
            functions.add(request["question"]["function"])
            settings = manifest["client"] | case.get("client", {})
            cancel = tt.Cancel()
            if case.get("cancelled"):
                cancel.cancel()

            def consume(
                settings: dict[str, Any] = settings,
                request: dict[str, Any] = request,
                cancel: tt.Cancel = cancel,
            ) -> list[Any]:
                with (
                    tt.Client(
                        backend="stub",
                        url=server.url,
                        batch=settings["batch"],
                        retry=tt.RetryPolicy(
                            max_retries=settings["retry"]["max_retries"]
                        ),
                    ) as client,
                    request_stream(client, request, cancel) as stream,
                ):
                    return list(stream)

            if isinstance(case["expected"], dict):
                with pytest.raises(tt.ThinkThenError) as failure:
                    consume()
                assert failure.value.code == case["expected"]["error"]
                continue
            rows = consume()
            assert all(row.source is not None for row in rows)
            assert all(row.position >= 1 for row in rows)
        assert len(functions) == 10 and server.count() == 0
