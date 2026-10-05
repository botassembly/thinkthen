"""A bounded mypy fixture for the installed public stub."""

from typing import Any, assert_type
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
    assert_type(tt.choose("Which?", "one", options=["a", "b"]), tt.Call[str | None])
    assert_type(engine.choose("Which?", ["one"], options=Labels),
                tt.Call[list[str | None]])
    assert_type(engine.choose("Which?", ["one"], options=Literal["first", "second"]),
                tt.Call[list[str | None]])
    judge = engine.decide("Ready?")
    assert_type(judge, tt.Judge[bool | None])
    assert_type(judge(iter(["one"])), tt.Stream[bool | None])
    assert_type(tt.plan(judge, ["one"]), dict[str, Any])


def source_types(engine: tt.Engine) -> None:
    source = tt.read_files("documents", unit="file")
    assert_type(source, tt.FileSelection)
    assert_type(next(iter(source)), tt.SourceRecord)
    assert_type(engine.decide("Ready?", source), tt.Call[list[tt.Located[bool | None]]])
    assert_type(engine.find("Which?", source), tt.Call[tt.Located[dict[str, Any]] | None])
