"""Existing Python option names map once into canonical question fields."""

from __future__ import annotations

import os
from pathlib import Path

import pytest
from conformance import Loopback, admitted

import thinkthen as tt


@pytest.mark.parametrize(
    "function,domain", [("choose", "choices"), ("tag", "labels"), ("score", "levels")]
)
def test_saved_questions_keep_function_domains(function: str, domain: str) -> None:
    canonical = "choices" if domain == "options" else domain
    question = tt.Toolbox(
        {
            "entry": admitted(
                {"function": function, "question": "Which?", canonical: ["a", "b"]}
            )
        }
    ).question("entry")
    client = tt.Client(backend="stub")
    expected = getattr(client, function)("Which?", "alpha", **{domain: ["a", "b"]})
    assert getattr(client, function)(question, "alpha") == expected
    with getattr(client, function + "_stream")(question, ["alpha"]) as rows:
        assert list(rows) == [expected]


def test_evidence_pointer_maps_to_the_shared_record_selection() -> None:
    path = Path(os.environ["THINKTHEN_CASES"]) / "typed/decide-precision.json"
    with Loopback(path) as server:
        client = tt.Client(url=server.url, api_key="synthetic")
        selected = client.decide(
            "Ready?", {"id": "r", "body": "evidence"}, evidence="/body"
        )
        inline = client.decide("Ready?", "evidence")
        assert selected.answer == inline.answer is True
        assert selected.probability == inline.probability == 0.8123456789012345
        assert selected.id == "r" and inline.id is None


@pytest.mark.parametrize("stream", [False, True])
@pytest.mark.parametrize("typed", [False, True])
def test_score_forwards_execution_name(stream: bool, typed: bool) -> None:
    with tt.Client(backend="stub") as client:
        question = tt.ScoreQuestion("Urgency?", levels=["low", "high"])
        kwargs = {} if typed else {"levels": ["low", "high"]}
        source = question if typed else "Urgency?"
        method = "score_stream" if stream else "score"
        call = getattr(client, method)
        records = ["body"] if stream else "body"
        result = call(source, records, name="execution", **kwargs)
        if stream:
            with result as rows:
                row = rows.to_list()[0]
        else:
            row = result
        assert row.source.name == "execution"
        with pytest.raises(tt.UsageError):
            call(source, records, name="", **kwargs)
