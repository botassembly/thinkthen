"""Every public constructor uses the same built-in backend and expected rows."""

from __future__ import annotations

import json
import os
from pathlib import Path
from typing import Any, cast

import pytest
from conformance import Loopback, row_json

import thinkthen as tt


def test_all_functions_match_stub_rows_without_http(
    tmp_path: Path, monkeypatch: pytest.MonkeyPatch
) -> None:
    for name in ("THINKTHEN_API_KEY", "OPENAI_API_KEY", "THINKTHEN_BATCH"):
        monkeypatch.delenv(name, raising=False)
    cases = cast(
        list[dict[str, Any]],
        json.loads(
            (
                Path(os.environ["THINKTHEN_CASES"]).parent / "construction/stub.json"
            ).read_text()
        ),
    )
    monitor = tmp_path / "monitor.json"
    monitor.write_text('{"replies":[]}')
    with Loopback(monitor) as server:
        monkeypatch.setenv("THINKTHEN_BASE_URL", server.url)
        for case in cases:
            request = case["request"]
            client = tt.Client(adapter="stub", url=server.url, model="ignored", batch=1)
            function = request["function"]
            question = request["question"]["value"]
            options = request.get("options", {})
            records = request["records"]
            if function == "filter":
                with client.filter(question, records, **options) as stream:
                    actual = list(stream)
                expected = [
                    record
                    for record, row in zip(records, case["expected"])
                    if row["status"] == "answered" and row["answer"] is True
                ]
            else:
                rows: tt.Stream[tt.Row[object]]
                if function == "annotate":
                    toolbox = tmp_path / "questions.json"
                    toolbox.write_text(json.dumps(question))
                    rows = client.annotate_stream(question, records)
                elif function == "recognize":
                    rows = client.recognize_stream(records, kinds=question, **options)
                elif function == "relate":
                    rows = client.relate_stream(records, rules=question, **options)
                elif function == "rank":
                    rows = client.rank(question, records, **options)
                else:
                    rows = getattr(client, function + "_stream")(
                        question, records, **options
                    )
                with rows:
                    actual = [row_json(row) for row in rows]
                expected = case["expected"]
            assert actual == expected, (function, actual, expected)
        assert server.count() == 0
