"""Questions say what each answer means, and the request carries it."""

from __future__ import annotations

import json
from enum import Enum
from pathlib import Path
from typing import Any, Literal

import pytest
from api_support import (
    backend,
    client,
    isolated_environment,  # noqa: F401 - shared pytest fixture.
)

import thinkthen as tt
from thinkthen import _native
from thinkthen._runtime import binding

pytestmark = pytest.mark.usefixtures("isolated_environment")


class Team(Enum):
    BILLING = "billing"
    SHIPPING = "shipping"


def test_yes_and_no_meanings_reach_the_backend_request(tmp_path: Path) -> None:
    # The fixture answers yes only when the request carries a meaning for yes.
    replies: list[dict[str, Any]] = [
        {"criterion": "true", "probability": 0.9},
        {"probability": 0.1},
    ]
    with backend(tmp_path, replies) as server, client(server) as selected:
        assert not selected.decide("Ready?", "body")
        assert selected.decide("Ready?", "body", yes_means="ships today")
        question = tt.DecideQuestion(
            "Ready?", yes_means="ships today", no_means="waits"
        )
        with selected.decide_stream(question, ["body"]) as rows:
            assert next(rows)


def test_choice_descriptions_change_the_question_sent() -> None:
    with tt.Client(backend="stub") as selected:
        plain = selected.choose("Which team?", "body", choices=Team)
        described = selected.choose(
            "Which team?",
            "body",
            choices=Team,
            descriptions={Team.BILLING: "money back"},
        )
    assert plain.source.question_identity != described.source.question_identity


@pytest.mark.parametrize(
    "choices", [Team, Literal["billing", "shipping"], ["billing", "shipping"]]
)
def test_every_choice_set_carries_descriptions(choices: Any) -> None:
    saved = tt.Toolbox(
        {
            "route": tt.ChooseQuestion(
                "Which team?", choices=choices, descriptions={"billing": "money back"}
            ),
            "flags": tt.TagQuestion(
                "Which teams?", labels=choices, descriptions={"shipping": "parcels"}
            ),
            "size": tt.ScoreQuestion(
                "How big?", levels=["small", "large"], descriptions={"large": "many"}
            ),
        }
    )
    entries = json.loads(saved.to_json())["entries"]
    assert entries["route"]["question"]["choices"] == [
        {"name": "billing", "description": "money back"},
        "shipping",
    ]
    assert entries["flags"]["question"]["labels"][1] == {
        "name": "shipping",
        "description": "parcels",
    }
    assert entries["size"]["question"]["levels"][1] == {
        "name": "large",
        "description": "many",
    }


def test_a_description_for_an_unknown_choice_is_refused() -> None:
    saved = json.loads(
        tt.Toolbox({"route": tt.ChooseQuestion("Which team?", choices=Team)}).to_json()
    )
    saved["entries"]["route"]["question"]["descriptions"] = {"sales": "selling things"}
    with pytest.raises(tt.UsageError) as shared:
        binding(
            lambda: _native.NativeQuestion(
                json.dumps(saved["entries"]["route"]["question"])
            )
        )
    with pytest.raises(tt.UsageError) as raised:
        tt.ChooseQuestion(
            "Which team?", choices=Team, descriptions={"sales": "selling things"}
        )
    assert raised.value.code is tt.ErrorCode.USAGE_QUESTION
    assert str(raised.value) == str(shared.value)


def test_filter_meanings_reach_the_backend_request(tmp_path: Path) -> None:
    replies: list[dict[str, Any]] = [
        {"criterion": "true", "probability": 0.9},
        {"probability": 0.1},
    ]
    with backend(tmp_path, replies) as server, client(server) as selected:
        assert selected.filter("Ready?", ["body"]).to_list() == []
        kept = selected.filter("Ready?", ["body"], yes_means="ships today")
        assert kept.to_list() == ["body"]
        question = tt.FilterQuestion(
            "Ready?", yes_means="ships today", no_means="waits"
        )
        assert selected.filter(question, ["body"]).to_list() == ["body"]


def test_rank_levels_carry_descriptions() -> None:
    saved = tt.Toolbox(
        {
            "urgency": tt.RankQuestion(
                "How urgent?", levels=["low", "high"], descriptions={"high": "today"}
            )
        }
    )
    levels = json.loads(saved.to_json())["entries"]["urgency"]["question"]["levels"]
    assert levels == ["low", {"name": "high", "description": "today"}]
    with tt.Client(backend="stub") as selected:
        plain = selected.rank("How urgent?", ["body"], levels=["low", "high"])
        described = selected.rank(
            "How urgent?",
            ["body"],
            levels=["low", "high"],
            descriptions={"high": "today"},
        )
        before, after = plain.to_list()[0], described.to_list()[0]
    assert before.source.question_identity != after.source.question_identity
