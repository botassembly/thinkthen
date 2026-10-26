"""Saved toolboxes hydrate Python question objects through shared admission."""

import json
import os
from pathlib import Path

import pytest
from api_support import isolated_environment  # noqa: F401 - shared pytest fixture.

import thinkthen as tt

pytestmark = pytest.mark.usefixtures("isolated_environment")


def test_saved_toolbox_loads_typed_questions_and_maps_names() -> None:
    root = Path(os.environ["THINKTHEN_CASES"]) / "toolbox"
    loaded = tt.Toolbox.from_json((root / "questions.json").read_text())
    assert {"ready", "topic", "names"}.issubset(loaded)
    assert len(loaded) == 9
    assert isinstance(loaded["ready"], tt.FilterQuestion)
    assert isinstance(loaded["topic"], tt.ChooseQuestion)
    assert isinstance(loaded["names"], tt.RelateQuestion)
    assert json.loads(loaded.to_json()) == json.loads(
        (root / "questions.json").read_text()
    )
    with tt.Client(backend="stub") as client:
        original = tt.ChooseQuestion(
            "Which topic?",
            choices=["billing", "shipping"],
            threshold=0.7,
            evidence="/body",
        )
        assert client.choose(loaded.choose("topic"), {"body": "Bill"}) == client.choose(
            original, {"body": "Bill"}
        )
    with pytest.raises(KeyError):
        loaded["missing"]
    assert loaded.get("missing") is None
    for malformed in json.loads((root / "malformed.json").read_text()):
        with pytest.raises(tt.UsageError) as error:
            tt.Toolbox.from_json(malformed)
        assert error.value.code == tt.ErrorCode.USAGE_QUESTION
