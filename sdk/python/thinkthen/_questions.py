"""Immutable admitted questions and named selections."""

from __future__ import annotations

import json
from collections.abc import Iterator, Mapping
from dataclasses import asdict, dataclass
from types import MappingProxyType
from typing import Any, TypeAlias, cast

from . import _native
from ._runtime import binding, encode
from ._types import ErrorCode, RelationRule, Strings, Threshold, UsageError

_MISSING = object()


@dataclass(frozen=True, slots=True)
class _Question:
    _native: _native.NativeQuestion


def definition(
    function: str, question: object = None, **fields: object
) -> dict[str, Any]:
    result: dict[str, Any] = {"function": function}
    if question is not None:
        result["questions" if function == "annotate" else "question"] = question
    for name, value in fields.items():
        if value is None or value is _MISSING:
            continue
        if name in {"choices", "labels", "levels", "kinds", "rules"}:
            if not isinstance(value, (list, tuple)):
                binding(_native.reject_question_conversion)
            value = tuple(value)
            if name == "rules":
                try:
                    value = [
                        {
                            key: item
                            for key, item in asdict(rule).items()
                            if item is not None
                        }
                        for rule in value
                    ]
                except (TypeError, ValueError):
                    binding(_native.reject_question_conversion)
        if name == "threshold" and isinstance(value, tuple):
            if len(value) != 2:
                binding(_native.reject_question_conversion)
            value = {"low": value[0], "high": value[1]}
        if name in {"threshold", "top"} and (
            isinstance(value, bool)
            or (
                isinstance(value, dict)
                and any(isinstance(item, bool) for item in value.values())
            )
        ):
            binding(_native.reject_question_conversion)
        result[name] = value
    return result


def _admit(
    function: str, question: object = None, **fields: object
) -> _native.NativeQuestion:
    text = encode(
        definition(function, question, **fields), _native.reject_question_conversion
    )
    return binding(lambda: _native.NativeQuestion(text))


class DecideQuestion(_Question):
    __slots__ = ()

    def __init__(
        self,
        question: str,
        *,
        threshold: Threshold | None = None,
        evidence: str | None = None,
    ) -> None:
        object.__setattr__(
            self,
            "_native",
            _admit("decide", question, threshold=threshold, evidence=evidence),
        )


class FilterQuestion(_Question):
    __slots__ = ()

    def __init__(
        self,
        question: str,
        *,
        threshold: Threshold | None = None,
        evidence: str | None = None,
    ) -> None:
        object.__setattr__(
            self,
            "_native",
            _admit("filter", question, threshold=threshold, evidence=evidence),
        )


class ChooseQuestion(_Question):
    __slots__ = ()

    def __init__(
        self,
        question: str,
        *,
        choices: Strings,
        threshold: Threshold | None = None,
        evidence: str | None = None,
    ) -> None:
        object.__setattr__(
            self,
            "_native",
            _admit(
                "choose",
                question,
                choices=choices,
                threshold=threshold,
                evidence=evidence,
            ),
        )


class TagQuestion(_Question):
    __slots__ = ()

    def __init__(
        self,
        question: str,
        *,
        labels: Strings,
        threshold: float | None = None,
        evidence: str | None = None,
    ) -> None:
        object.__setattr__(
            self,
            "_native",
            _admit(
                "tag", question, labels=labels, threshold=threshold, evidence=evidence
            ),
        )


class ScoreQuestion(_Question):
    __slots__ = ()

    def __init__(
        self, question: str, *, levels: Strings, evidence: str | None = None
    ) -> None:
        object.__setattr__(
            self, "_native", _admit("score", question, levels=levels, evidence=evidence)
        )


class RankQuestion(_Question):
    __slots__ = ()

    def __init__(
        self,
        question: str,
        *,
        levels: Strings,
        top: int | None = None,
        evidence: str | None = None,
    ) -> None:
        object.__setattr__(
            self,
            "_native",
            _admit("rank", question, levels=levels, top=top, evidence=evidence),
        )


class FindQuestion(_Question):
    __slots__ = ()

    def __init__(
        self,
        question: str,
        *,
        threshold: float | None = None,
        evidence: str | None = None,
    ) -> None:
        object.__setattr__(
            self,
            "_native",
            _admit("find", question, threshold=threshold, evidence=evidence),
        )


class RecognizeQuestion(_Question):
    __slots__ = ()

    def __init__(
        self,
        *,
        kinds: Strings,
        threshold: float | None = None,
        evidence: str | None = None,
    ) -> None:
        object.__setattr__(
            self,
            "_native",
            _admit(
                "recognize", None, kinds=kinds, threshold=threshold, evidence=evidence
            ),
        )


class RelateQuestion(_Question):
    __slots__ = ()

    def __init__(
        self,
        *,
        rules: list[RelationRule] | tuple[RelationRule, ...],
        threshold: float | None = None,
        evidence: str | None = None,
    ) -> None:
        object.__setattr__(
            self,
            "_native",
            _admit("relate", None, rules=rules, threshold=threshold, evidence=evidence),
        )


class AnnotateQuestion(_Question):
    __slots__ = ()

    def __init__(
        self, questions: AnnotationQuestions, *, evidence: str | None = None
    ) -> None:
        object.__setattr__(
            self,
            "_native",
            _admit("annotate", questions._definitions(), evidence=evidence),
        )


NonAnnotateQuestion: TypeAlias = (
    DecideQuestion
    | FilterQuestion
    | ChooseQuestion
    | TagQuestion
    | ScoreQuestion
    | RankQuestion
    | FindQuestion
    | RecognizeQuestion
    | RelateQuestion
)
AnnotationQuestion: TypeAlias = (
    DecideQuestion
    | FilterQuestion
    | ChooseQuestion
    | TagQuestion
    | ScoreQuestion
    | FindQuestion
    | RecognizeQuestion
    | RelateQuestion
)


class Toolbox(Mapping[str, NonAnnotateQuestion]):
    __slots__ = ("_native",)

    def __init__(self, questions: Mapping[str, NonAnnotateQuestion]) -> None:
        try:
            definitions = {
                name: json.loads(question._native.definition)
                for name, question in questions.items()
            }
        except (AttributeError, TypeError):
            binding(_native.reject_question_conversion)
        text = encode(definitions, _native.reject_question_conversion)
        self._native = binding(lambda: _native.NativeToolbox(text))

    @classmethod
    def from_json(cls, text: str) -> Toolbox:
        value = object.__new__(cls)
        value._native = binding(lambda: _native.NativeToolbox.from_json(text))
        return value

    def to_json(self) -> str:
        return binding(self._native.to_json)

    def __getitem__(self, name: str) -> NonAnnotateQuestion:
        try:
            return self.question(name)
        except UsageError as error:
            if error.code is ErrorCode.USAGE_NAME:
                raise KeyError(name) from error
            raise

    def __iter__(self) -> Iterator[str]:
        return iter(self._definitions())

    def __len__(self) -> int:
        return len(self._definitions())

    def _definitions(self) -> dict[str, Any]:
        return cast(dict[str, Any], json.loads(self._native.definition))

    def question(self, name: str) -> NonAnnotateQuestion:
        native = binding(lambda: self._native.question(name))
        cls = _CLASSES[native.function]
        value = object.__new__(cls)
        object.__setattr__(value, "_native", native)
        return cast(NonAnnotateQuestion, value)

    def decide(self, name: str) -> DecideQuestion:
        value = self.question(name)
        if not isinstance(value, DecideQuestion):
            binding(_native.reject_question_conversion)
        return value

    def filter(self, name: str) -> FilterQuestion:
        value = self.question(name)
        if not isinstance(value, FilterQuestion):
            binding(_native.reject_question_conversion)
        return value

    def choose(self, name: str) -> ChooseQuestion:
        value = self.question(name)
        if not isinstance(value, ChooseQuestion):
            binding(_native.reject_question_conversion)
        return value

    def tag(self, name: str) -> TagQuestion:
        value = self.question(name)
        if not isinstance(value, TagQuestion):
            binding(_native.reject_question_conversion)
        return value

    def score(self, name: str) -> ScoreQuestion:
        value = self.question(name)
        if not isinstance(value, ScoreQuestion):
            binding(_native.reject_question_conversion)
        return value

    def rank(self, name: str) -> RankQuestion:
        value = self.question(name)
        if not isinstance(value, RankQuestion):
            binding(_native.reject_question_conversion)
        return value

    def find(self, name: str) -> FindQuestion:
        value = self.question(name)
        if not isinstance(value, FindQuestion):
            binding(_native.reject_question_conversion)
        return value

    def recognize(self, name: str) -> RecognizeQuestion:
        value = self.question(name)
        if not isinstance(value, RecognizeQuestion):
            binding(_native.reject_question_conversion)
        return value

    def relate(self, name: str) -> RelateQuestion:
        value = self.question(name)
        if not isinstance(value, RelateQuestion):
            binding(_native.reject_question_conversion)
        return value


class AnnotationQuestions(Mapping[str, AnnotationQuestion]):
    __slots__ = ("_native", "_questions")

    def __init__(self, questions: Mapping[str, AnnotationQuestion]) -> None:
        try:
            copied = dict(questions)
            definitions = {
                name: json.loads(question._native.definition)
                for name, question in copied.items()
            }
        except (TypeError, AttributeError):
            binding(_native.reject_question_conversion)
        self._native = _admit("annotate", definitions)
        self._questions: Mapping[str, AnnotationQuestion] = MappingProxyType(copied)

    def _definitions(self) -> dict[str, Any]:
        return cast(dict[str, Any], json.loads(self._native.definition)["questions"])

    def __getitem__(self, name: str) -> AnnotationQuestion:
        return self._questions[name]

    def __iter__(self) -> Iterator[str]:
        return iter(self._questions)

    def __len__(self) -> int:
        return len(self._questions)


_CLASSES: dict[str, type[_Question]] = {
    "decide": DecideQuestion,
    "filter": FilterQuestion,
    "choose": ChooseQuestion,
    "tag": TagQuestion,
    "score": ScoreQuestion,
    "rank": RankQuestion,
    "find": FindQuestion,
    "recognize": RecognizeQuestion,
    "relate": RelateQuestion,
    "annotate": AnnotateQuestion,
}
__all__ = [
    "AnnotateQuestion",
    "AnnotationQuestion",
    "AnnotationQuestions",
    "ChooseQuestion",
    "DecideQuestion",
    "FilterQuestion",
    "FindQuestion",
    "NonAnnotateQuestion",
    "RankQuestion",
    "RecognizeQuestion",
    "RelateQuestion",
    "ScoreQuestion",
    "TagQuestion",
    "Toolbox",
]
