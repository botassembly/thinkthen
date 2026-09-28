"""A bounded mypy fixture for the installed public stub."""

from typing import assert_type
from enum import Enum
from typing import Literal

import thinkthen as tt
from thinkthen import AnnotatedRow, ErrorKind, QuestionKind


class Labels(Enum):
    first = "first"
    second = "second"


def check(engine: tt.Engine, question: tt.Question, error: tt.ThinkThenError,
          completion: tt.Completion) -> None:
    assert_type(question.kind, QuestionKind)
    assert_type(error.kind, ErrorKind)
    assert_type(completion.kind, ErrorKind | None)
    call = engine.annotate({"version": 1, "questions": {"open": {"decide": "Open?"}}},
                           ["one"])
    assert_type(call, tt.Call[list[AnnotatedRow]])
    assert_type(call.value[0]["open"], tt.AnnotatedValue)
    assert_type(tt.choose("Which?", "one", options=["a", "b"]),
                tt.Call[str | None | tt.Column])
    assert_type(engine.choose_many("Which?", ["one"], options=Labels),
                tt.Call[list[str | None]])
    assert_type(engine.choose_many("Which?", ["one"], options=Literal["first", "second"]),
                tt.Call[list[str | None]])
