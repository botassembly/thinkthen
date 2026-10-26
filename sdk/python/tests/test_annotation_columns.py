"""Feature: annotation tables attach child answers and probabilities in one batch."""

from __future__ import annotations

import json
import os
from pathlib import Path
from typing import Any

import pandas as pd
import polars as pl
import pytest
from api_support import isolated_environment  # noqa: F401 - shared fixture.
from conformance import Loopback, admitted

import thinkthen as tt
import thinkthen.pandas
import thinkthen.polars

pytestmark = pytest.mark.usefixtures("isolated_environment")


@pytest.mark.parametrize("surface", ["pandas", "polars"])
def test_annotation_columns_keep_inputs_and_child_values(surface: str) -> None:
    path = Path(os.environ["THINKTHEN_CASES"]) / "typed/annotate-columns.json"
    case = json.loads(path.read_text())
    definition = case["request"]["question"]
    questions = tt.AnnotationQuestions(
        {name: admitted(value) for name, value in definition["questions"].items()}
    )
    source: Any = (
        pd.DataFrame(case["request"]["records"], index=["same"] * 4)
        if surface == "pandas"
        else pl.DataFrame(case["request"]["records"])
    )
    with (
        Loopback(path) as server,
        tt.Client(
            backend="systemone",
            url=server.url,
            api_key="synthetic-loopback-key",
            batch=32,
        ) as client,
    ):
        result = source.thinkthen.annotate(
            questions, evidence=definition["evidence"], client=client
        )
        assert list(result.columns) == [
            "id",
            "body",
            "ready",
            "ready_status",
            "ready_probability",
            "ready_detail",
            "topic",
            "topic_status",
            "topic_probability",
            "topic_detail",
            "urgency",
            "urgency_status",
            "urgency_probability",
            "urgency_detail",
        ]
        assert len(result) == 4 and server.count() == case["requests"]
        if surface == "pandas":
            assert result.index.equals(source.index)
            assert result[["id", "body"]].equals(source)
        else:
            assert result.select("id", "body").equals(source)
        for name in definition["questions"]:
            values = result[name].to_list()
            probabilities = result[name + "_probability"].to_list()
            for index, expected in enumerate(case["expected"]):
                child = expected["answer"][name]
                assert values[index] == child["answer"]
                probability = probabilities[index]
                if "probability" in child:
                    assert probability == child["probability"]
                else:
                    assert probability is None or pd.isna(probability)
            assert result[name + "_status"].to_list() == ["answered"] * 4
        for detail in result["urgency_detail"].to_list():
            levels = detail.levels if surface == "pandas" else detail["levels"]
            assert list(levels) == [0.25, 0.75]
        assert len(source.columns) == 2


@pytest.mark.parametrize("surface", ["pandas", "polars"])
@pytest.mark.parametrize("empty", [False, True])
def test_annotation_columns_retain_schema_for_empty_and_failed_inputs(
    surface: str, empty: bool
) -> None:
    values = {"id": [] if empty else [True], "body": [] if empty else ["text"]}
    source: Any = pd.DataFrame(values) if surface == "pandas" else pl.DataFrame(values)
    questions = tt.AnnotationQuestions({"ready": tt.DecideQuestion("Ready?")})
    with tt.Client(backend="stub") as client:
        result = source.thinkthen.annotate(questions, client=client)
    assert list(result.columns) == [
        "id",
        "body",
        "ready",
        "ready_status",
        "ready_probability",
        "ready_detail",
    ]
    assert len(result) == (0 if empty else 1)
    if not empty:
        assert result["ready_status"].to_list() == ["failed"]
        assert result["ready"].to_list()[0] is None or pd.isna(
            result["ready"].to_list()[0]
        )
