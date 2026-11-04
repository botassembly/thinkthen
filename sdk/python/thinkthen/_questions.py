"""Immutable admitted questions and named selections."""

from __future__ import annotations

import json
from collections.abc import Iterator, Mapping
from dataclasses import asdict, dataclass
from enum import Enum
from pathlib import Path
from types import MappingProxyType
from typing import (
    Any,
    Generic,
    Literal,
    TypeAlias,
    cast,
    get_args,
    get_origin,
    overload,
)

from typing_extensions import TypeForm

from . import _native
from ._runtime import binding, encode
from ._types import (
    ChoiceInput,
    Descriptions,
    ErrorCode,
    RelationRule,
    Strings,
    Threshold,
    UsageError,
    _Label,
)

_MISSING = object()


@dataclass(frozen=True, slots=True)
class _Question:
    _native: _native.NativeQuestion


class _LabelQuestion(_Question, Generic[_Label]):
    __slots__ = ("_members",)
    _members: Mapping[str, Enum]


def choice_values(value: object, name: str) -> tuple[object, ...]:
    if isinstance(value, type) and issubclass(value, Enum):
        return tuple(value)
    if get_origin(value) is Literal:
        return get_args(value)
    if isinstance(value, (list, tuple)):
        return tuple(value)
    raise TypeError(f"{name} must be a list, tuple, Enum or Literal choice set")


def choice_members(value: object, name: str) -> Mapping[str, Enum]:
    return MappingProxyType(
        {
            str(item.value): item
            for item in choice_values(value, name)
            if isinstance(item, Enum)
        }
    )


def annotation_members(value: object) -> Mapping[str, Mapping[str, Enum]]:
    if isinstance(value, Mapping):
        return {
            name: getattr(question, "_members", {}) for name, question in value.items()
        }
    return cast(Mapping[str, Mapping[str, Enum]], getattr(value, "_children", {}))


# The shared question contract names a decide question's meanings by answer.
_MEANINGS = {"yes_means": "true", "no_means": "false"}


def _described(
    values: tuple[object, ...], descriptions: object, name: str
) -> tuple[tuple[object, ...], dict[object, object]]:
    if not isinstance(descriptions, Mapping):
        raise TypeError(f"descriptions must map each of the {name} to its meaning")
    text = {
        key.value if isinstance(key, Enum) else key: value
        for key, value in descriptions.items()
    }
    return (
        tuple(
            {"name": value, "description": text[value]} if value in text else value
            for value in values
        ),
        {key: value for key, value in text.items() if key not in values},
    )


def definition(
    function: str, question: object = None, **fields: object
) -> dict[str, Any]:
    result: dict[str, Any] = {"function": function}
    if question is not None:
        result["questions" if function == "annotate" else "question"] = question
    descriptions = fields.pop("descriptions", None)
    for name, value in fields.items():
        if value is None or value is _MISSING:
            continue
        if name in {"choices", "labels", "levels"}:
            value = tuple(
                item.value if isinstance(item, Enum) else item
                for item in choice_values(value, name)
            )
            if descriptions is not None:
                value, unmatched = _described(value, descriptions, name)
                descriptions = unmatched or None
        elif name in {"kinds", "rules"}:
            if not isinstance(value, (list, tuple)):
                raise TypeError(
                    f"{name} must be a list or tuple, not {type(value).__name__}"
                )
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
        result[_MEANINGS.get(name, name)] = value
    if descriptions is not None:
        # Preserve unconsumed keys for the engine's unknown-field diagnostic.
        result["descriptions"] = descriptions
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
        field: str | None = None,
        yes_means: str | None = None,
        no_means: str | None = None,
    ) -> None:
        """Ask yes or no; yes_means and no_means say what each answer means."""
        object.__setattr__(
            self,
            "_native",
            _admit(
                "decide",
                question,
                threshold=threshold,
                field=field,
                yes_means=yes_means,
                no_means=no_means,
            ),
        )


class FilterQuestion(_Question):
    __slots__ = ()

    def __init__(
        self,
        question: str,
        *,
        threshold: Threshold | None = None,
        field: str | None = None,
        yes_means: str | None = None,
        no_means: str | None = None,
    ) -> None:
        """Keep records answered yes; yes_means and no_means say what each means."""
        object.__setattr__(
            self,
            "_native",
            _admit(
                "filter",
                question,
                threshold=threshold,
                field=field,
                yes_means=yes_means,
                no_means=no_means,
            ),
        )


class ChooseQuestion(_LabelQuestion[_Label]):
    __slots__ = ()

    @overload
    def __init__(
        self,
        question: str,
        *,
        choices: TypeForm[_Label],
        threshold: Threshold | None = None,
        field: str | None = None,
        descriptions: Descriptions[_Label] | None = None,
    ) -> None: ...

    @overload
    def __init__(
        self,
        question: str,
        *,
        choices: list[_Label] | tuple[_Label, ...],
        threshold: Threshold | None = None,
        field: str | None = None,
        descriptions: Descriptions[_Label] | None = None,
    ) -> None: ...

    def __init__(
        self,
        question: str,
        *,
        choices: ChoiceInput[_Label],
        threshold: Threshold | None = None,
        field: str | None = None,
        descriptions: Descriptions[_Label] | None = None,
    ) -> None:
        object.__setattr__(self, "_members", choice_members(choices, "choices"))
        object.__setattr__(
            self,
            "_native",
            _admit(
                "choose",
                question,
                choices=choices,
                threshold=threshold,
                field=field,
                descriptions=descriptions,
            ),
        )


class TagQuestion(_LabelQuestion[_Label]):
    __slots__ = ()

    @overload
    def __init__(
        self,
        question: str,
        *,
        labels: TypeForm[_Label],
        threshold: float | None = None,
        field: str | None = None,
        descriptions: Descriptions[_Label] | None = None,
    ) -> None: ...

    @overload
    def __init__(
        self,
        question: str,
        *,
        labels: list[_Label] | tuple[_Label, ...],
        threshold: float | None = None,
        field: str | None = None,
        descriptions: Descriptions[_Label] | None = None,
    ) -> None: ...

    def __init__(
        self,
        question: str,
        *,
        labels: ChoiceInput[_Label],
        threshold: float | None = None,
        field: str | None = None,
        descriptions: Descriptions[_Label] | None = None,
    ) -> None:
        object.__setattr__(self, "_members", choice_members(labels, "labels"))
        object.__setattr__(
            self,
            "_native",
            _admit(
                "tag",
                question,
                labels=labels,
                threshold=threshold,
                field=field,
                descriptions=descriptions,
            ),
        )


class ScoreQuestion(_Question):
    __slots__ = ()

    def __init__(
        self,
        question: str,
        *,
        levels: ChoiceInput[_Label],
        field: str | None = None,
        descriptions: Descriptions[_Label] | None = None,
    ) -> None:
        object.__setattr__(
            self,
            "_native",
            _admit(
                "score",
                question,
                levels=levels,
                field=field,
                descriptions=descriptions,
            ),
        )


class RankQuestion(_Question):
    __slots__ = ()

    def __init__(
        self,
        question: str,
        *,
        levels: ChoiceInput[_Label],
        top: int | None = None,
        field: str | None = None,
        descriptions: Descriptions[_Label] | None = None,
    ) -> None:
        object.__setattr__(
            self,
            "_native",
            _admit(
                "rank",
                question,
                levels=levels,
                top=top,
                field=field,
                descriptions=descriptions,
            ),
        )


class FindQuestion(_Question):
    __slots__ = ()

    def __init__(
        self,
        question: str,
        *,
        threshold: float | None = None,
        field: str | None = None,
    ) -> None:
        object.__setattr__(
            self,
            "_native",
            _admit("find", question, threshold=threshold, field=field),
        )


class RecognizeQuestion(_Question):
    __slots__ = ()

    def __init__(
        self,
        *,
        kinds: Strings,
        threshold: float | None = None,
        field: str | None = None,
    ) -> None:
        object.__setattr__(
            self,
            "_native",
            _admit("recognize", None, kinds=kinds, threshold=threshold, field=field),
        )


class RelateQuestion(_Question):
    __slots__ = ()

    def __init__(
        self,
        *,
        rules: list[RelationRule] | tuple[RelationRule, ...],
        threshold: float | None = None,
        field: str | None = None,
    ) -> None:
        object.__setattr__(
            self,
            "_native",
            _admit("relate", None, rules=rules, threshold=threshold, field=field),
        )


class AnnotateQuestion(_Question):
    __slots__ = ("_children",)
    _children: Mapping[str, Mapping[str, Enum]]

    def __init__(self, questions: AnnotationInput, *, field: str | None = None) -> None:
        if not isinstance(questions, AnnotationQuestions):
            questions = AnnotationQuestions(
                cast(Mapping[str, AnnotationQuestion], questions)
            )
        object.__setattr__(self, "_children", annotation_members(questions))
        object.__setattr__(
            self,
            "_native",
            _admit("annotate", questions._definitions(), field=field),
        )


NonAnnotateQuestion: TypeAlias = (
    DecideQuestion
    | FilterQuestion
    | ChooseQuestion[Any]
    | TagQuestion[Any]
    | ScoreQuestion
    | RankQuestion
    | FindQuestion
    | RecognizeQuestion
    | RelateQuestion
)
AnnotationQuestion: TypeAlias = (
    DecideQuestion
    | FilterQuestion
    | ChooseQuestion[Any]
    | TagQuestion[Any]
    | ScoreQuestion
    | FindQuestion
    | RecognizeQuestion
    | RelateQuestion
)


class Toolbox(Mapping[str, NonAnnotateQuestion]):
    __slots__ = ("_native", "_questions")

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
        self._questions = MappingProxyType(dict(questions))

    @classmethod
    def from_json(cls, text: str) -> Toolbox:
        value = object.__new__(cls)
        value._native = binding(lambda: _native.NativeToolbox.from_json(text))
        value._questions = MappingProxyType({})
        return value

    @classmethod
    def load(cls, path: str | Path) -> Toolbox:
        """Read a saved toolbox from a UTF-8 JSON file."""
        return cls.from_json(Path(path).read_text(encoding="utf-8"))

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
        if name in self._questions:
            return self._questions[name]
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

    def choose(self, name: str) -> ChooseQuestion[Any]:
        value = self.question(name)
        if not isinstance(value, ChooseQuestion):
            binding(_native.reject_question_conversion)
        return value

    def tag(self, name: str) -> TagQuestion[Any]:
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


AnnotationInput: TypeAlias = (
    AnnotationQuestions | Mapping[str, AnnotationQuestion] | Toolbox
)


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
