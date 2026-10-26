"""Export each public function's rows without losing typed result values."""

from __future__ import annotations

import json
import os
from pathlib import Path
from typing import Any

import pytest
from api_support import isolated_environment  # noqa: F401 - shared pytest fixture.
from conformance import Loopback, plain, request_stream

import thinkthen as tt

pytestmark = pytest.mark.usefixtures("isolated_environment")


@pytest.mark.parametrize("failed", [False, True])
def test_rows_export_all_functions_as_independent_json_values(failed: bool) -> None:
    directory = Path(os.environ["THINKTHEN_CASES"]) / "typed"
    manifest = json.loads((directory / "manifest.json").read_text())
    functions: set[str] = set()
    for filename in manifest["cases"]:
        path = directory / filename
        case = json.loads(path.read_text())
        function = case["request"]["question"]["function"]
        if function in functions or not isinstance(case["expected"], list):
            continue
        if not any(row["status"] == "answered" for row in case["expected"]):
            continue
        request = case["request"]
        if failed:
            request = request | {"records": [None]}
        with (
            Loopback(path) as server,
            tt.Client(
                url=server.url, api_key="synthetic-export-key", batch=1
            ) as client,
            request_stream(client, request) as stream,
        ):
            rows = list(stream)
        assert rows
        for row in rows:
            exported = row.to_dict()
            assert type(exported) is dict
            assert exported["function"] == function
            assert exported["status"] == ("failed" if failed else row.status.value)
            assert json.loads(json.dumps(exported)) == exported
            assert exported["probability"] == row.probability
            if failed:
                assert exported["answer"] is None
            assert "synthetic-export-key" not in json.dumps(exported)
            assert not {"raw", "backend_response", "api_key"} & exported.keys()
            if failed:
                assert exported["error"]["code"] == "record.missing"
                if function == "filter":
                    assert exported["item"] is None
            assert_independent(row, exported)
        functions.add(function)
    assert functions == {function.value for function in tt.Function}


def assert_independent(row: Any, exported: dict[str, Any]) -> None:
    if row.function is not tt.Function.ANNOTATE:
        # The independent conformance projection omits optional false relation flags.
        expected = plain(row.answer)
        if row.function is tt.Function.RELATE and expected is not None:
            for relation in expected:
                relation.setdefault("either", False)
        assert exported["answer"] == expected
    if row.detail is not None:
        assert type(exported["detail"]) is dict
        if hasattr(row.detail, "labels"):
            assert exported["detail"]["labels"] == dict(row.detail.labels)
            exported["detail"]["labels"].clear()
            assert row.detail.labels
        elif hasattr(row.detail, "levels"):
            assert exported["detail"]["levels"] == list(row.detail.levels)
    if row.function is tt.Function.ANNOTATE and row.answer is not None:
        assert type(exported["answer"]) is dict
        for name, outcome in row.answer.items():
            assert exported["answer"][name] == outcome.to_dict()
    if row.function is tt.Function.RECOGNIZE and row.detail is not None:
        assert all(type(key) is str for key in exported["detail"]["pieces"][0]["tags"])


def test_export_keeps_integer_ids_and_copies_filter_items() -> None:
    record = {"id": 2**64 - 1, "text": "body"}
    with (
        tt.Client(backend="stub") as client,
        client.filter("Ready?", [record]) as stream,
    ):
        row = next(stream)
    exported = row.to_dict()
    assert json.loads(json.dumps(exported))["id"] == 2**64 - 1
    assert exported["item"] == record
    exported["item"]["text"] = "changed"
    assert row.item["text"] == "body"
