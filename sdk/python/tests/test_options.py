"""Public option mappings use the same requests as every language."""

from __future__ import annotations

import json
import os
from pathlib import Path
from typing import Any

import pytest
from conformance import row_json

import thinkthen as tt


def execute(client: tt.Client, request: dict[str, Any], single: bool) -> list[object]:
    tagged = request["question"]
    question = (
        tt.Toolbox(tagged["value"]).question(tagged["name"])
        if tagged["type"] == "toolbox"
        else tagged["value"]
    )
    function = request["function"]
    options = request.get("options", {})
    if single and function != "rank":
        return [
            row_json(getattr(client, function)(question, record, **options))
            for record in request["records"]
        ]
    method = function if function == "rank" else function + "_stream"
    with getattr(client, method)(question, request["records"], **options) as rows:
        return [row_json(row) for row in rows]


def test_shared_option_combinations() -> None:
    path = Path(os.environ["THINKTHEN_CASES"]).parent / "construction/options.json"
    for case in json.loads(path.read_text()):
        client = tt.Client(**case["client"])
        for single in (False, True):
            if "expected_error" in case:
                # Python excludes these options in its public signatures.
                with pytest.raises(TypeError, match="threshold"):
                    execute(client, case["request"], single)
            else:
                actual = execute(client, case["request"], single)
                expected = execute(client, case["equivalent_request"], single)
                assert actual == expected, case["name"]
                assert len(actual) == len(case["request"]["records"])
