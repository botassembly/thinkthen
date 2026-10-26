"""Single rows explain how callers obtain a closable stream."""

from __future__ import annotations

from pathlib import Path
from typing import Any, cast

import pytest
from api_support import (
    backend,
    isolated_environment,  # noqa: F401 - shared pytest fixture.
    yes,
)

import thinkthen as tt

pytestmark = pytest.mark.usefixtures("isolated_environment")


@pytest.mark.parametrize("function", [value.value for value in tt.Function])
def test_rows_name_the_matching_stream_when_used_as_context_managers(
    tmp_path: Path, function: str
) -> None:
    domains: dict[str, dict[str, Any]] = {
        "choose": {"choices": ["first", "second"]},
        "tag": {"labels": ["first", "second"]},
        "score": {"levels": ["low", "high"]},
        "rank": {"levels": ["low", "high"]},
        "recognize": {"kinds": ["name"]},
        "relate": {"rules": [tt.RelationRule("name", "name", "knows")]},
    }
    options = domains.get(function, {})
    with (
        backend(tmp_path, yes()) as server,
        tt.Client(backend="stub", url=server.url) as selected,
    ):
        call = getattr(selected, function)
        if function == "annotate":
            row = call(
                tt.AnnotationQuestions({"ready": tt.DecideQuestion("Ready?")}), "body"
            )
            assert row.answer is not None
            assert not hasattr(row.answer["ready"], "__enter__")
        elif function in {"filter", "rank"}:
            with call("Ready?", ["body"], **options) as stream:
                row = next(stream)
        elif function in {"recognize", "relate"}:
            row = call("body", **options)
        else:
            row = call("Ready?", "body", **options)
        stream_name = (
            function if function in {"filter", "rank"} else function + "_stream"
        )
        with pytest.raises(TypeError) as failure, cast(Any, row):
            raise AssertionError("a row must not enter a with block")
        assert str(failure.value) == (
            f"this is one {function} row; use {stream_name}(...) for a stream"
        )
        assert server.count() == 0


def test_single_call_keeps_a_json_array_as_one_record(tmp_path: Path) -> None:
    with (
        backend(tmp_path, yes()) as server,
        tt.Client(backend="stub", url=server.url) as selected,
    ):
        row = selected.decide("Ready?", ["first", "second"])
        assert isinstance(row, tt.DecideRow)
        assert row.status is tt.Status.ANSWERED and row.position == 1
        assert server.count() == 0
