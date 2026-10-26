"""The existing facade needs neither credentials nor HTTP for the stub."""

from __future__ import annotations

import json
import os
from pathlib import Path

import pytest
from conformance import Loopback, request_stream

import thinkthen as tt


def test_all_functions_use_the_stub_without_backend_requests() -> None:
    directory = Path(os.environ["THINKTHEN_CASES"]) / "typed"
    manifest = json.loads((directory / "manifest.json").read_text())
    with Loopback(directory / "decide-precision.json") as server:
        client = tt.Client(backend="stub", url=server.url)
        functions: set[str] = set()
        for filename in manifest["cases"]:
            case = json.loads((directory / filename).read_text())
            request = case["request"]
            functions.add(request["question"]["function"])
            if isinstance(case["expected"], dict):
                with (
                    pytest.raises(tt.ThinkThenError) as failure,
                    request_stream(client, request) as stream,
                ):
                    assert list(stream) == []
                assert failure.value.code == case["expected"]["error"]
                continue
            with request_stream(client, request) as stream:
                rows = list(stream)
            assert all(row.source is not None for row in rows)
            assert all(row.position >= 1 for row in rows)
        assert len(functions) == 10 and server.count() == 0
