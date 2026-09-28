"""Optional Pydantic v2 authoring and explicit validation of annotate rows.

Import this module only when using the ``thinkthen[pydantic]`` extra.
"""

import json
import os
import types
from typing import Annotated, Literal, get_args, get_origin

try:
    from pydantic import BaseModel, ConfigDict, Field, create_model
    from pydantic.fields import FieldInfo
except ImportError as error:
    raise ImportError("thinkthen.pydantic needs the thinkthen[pydantic] extra") from error

from . import _thinkthen
from ._thinkthen import LocalError, UsageError


def _literal(value):
    from ._labels import _literal as direct
    return direct(value)


def labels(value):
    """Ordered label names and Field meanings for one Pydantic label form."""
    if isinstance(value, type) and issubclass(value, BaseModel) and value is not BaseModel:
        names = list(value.model_fields)
        meanings = {name: field.description for name, field in value.model_fields.items()
                    if field.description is not None}
        return names, meanings
    if get_origin(value) is Annotated:
        base, *metadata = get_args(value)
        fields = [item for item in metadata if isinstance(item, FieldInfo)]
        if len(fields) != 1:
            raise UsageError("an Annotated label set takes one Field")
        names = _literal(base)
        return names, {name: fields[0].description for name in names
                       if fields[0].description is not None}
    if isinstance(value, tuple):
        names = []
        meanings = {}
        for entry in value:
            one, described = labels(entry)
            if len(one) != 1 or one[0] in names:
                raise UsageError("each annotated tuple entry names one distinct label")
            names.extend(one)
            meanings.update(described)
        return names, meanings
    raise UsageError("Pydantic labels take a model class or Annotated Literal")


def _field_kind(annotation):
    """The four supported authoring types and their labels, if any."""
    args = get_args(annotation)
    optional = False
    if get_origin(annotation) in (types.UnionType, __import__("typing").Union):
        if len(args) != 2 or type(None) not in args:
            raise UsageError("question-set fields take one supported answer type")
        annotation = next(item for item in args if item is not type(None))
        optional = True
    if annotation is bool:
        if not optional:
            raise UsageError("decide question fields take bool | None")
        return "decide", None
    if annotation is float:
        if optional:
            raise UsageError("score question fields take float without None")
        return "score", None
    if get_origin(annotation) is Literal:
        if not optional:
            raise UsageError("choose question fields take Literal[...] | None")
        return "choose", _literal(annotation)
    if get_origin(annotation) is list:
        if optional:
            raise UsageError("tag question fields take list[Literal[...]] without None")
        members = get_args(annotation)
        if len(members) != 1:
            raise UsageError("tag question fields take list[Literal[...]]")
        (member,) = members
        return "tag", _literal(member)
    raise UsageError("question-set fields take bool, Literal, float or list[Literal]")


def question_set(model):
    """Lower a BaseModel class to the existing version-one question-set file."""
    if not isinstance(model, type) or not issubclass(model, BaseModel) or model is BaseModel:
        raise UsageError("a question set takes a Pydantic model class")
    questions = {}
    allowed = {"decide": {"true", "false", "threshold", "on"},
               "choose": {"options", "threshold", "on"},
               "score": {"levels", "on"},
               "tag": {"labels", "threshold", "on"}}
    for name, field in model.model_fields.items():
        kind, names = _field_kind(field.annotation)
        if not field.description:
            raise UsageError("each question-set field needs Field(description=...)")
        body = {kind: field.description}
        if names is not None:
            body["options" if kind == "choose" else "labels"] = names
        extra = field.json_schema_extra or {}
        if not isinstance(extra, dict) or any(key not in allowed[kind] for key in extra):
            raise UsageError("question-set Field settings must be existing question keys")
        if names is not None:
            key = "options" if kind == "choose" else "labels"
            if key in extra:
                offered = extra[key]
                if not isinstance(offered, (list, dict)) or list(offered) != names:
                    raise UsageError("question-set Field labels must match its Literal")
        body.update(extra)
        questions[name] = body
    return {"version": 1, "questions": questions}


class _Failure(BaseModel):
    model_config = ConfigDict(strict=True, extra="forbid")
    kind: Literal["usage", "backend", "deadline", "local", "cancelled", "defect"]
    cause: str


class _Failed(BaseModel):
    model_config = ConfigDict(strict=True, extra="forbid")
    failed: _Failure


def row_model(question_set_value) -> type[BaseModel]:
    """Build a strict row validator from a native-validated question set."""
    if isinstance(question_set_value, type) and issubclass(question_set_value, BaseModel):
        source = question_set(question_set_value)
        text = json.dumps(source)
        _thinkthen._QuestionSet._from_json(text)
    elif isinstance(question_set_value, dict):
        source = question_set_value
        text = json.dumps(source)
        _thinkthen._QuestionSet._from_json(text)
    else:
        try:
            with open(os.fspath(question_set_value), encoding="utf-8") as stream:
                text = stream.read()
        except (OSError, TypeError, ValueError) as error:
            raise LocalError("the question set could not be read") from error
        try:
            _thinkthen._QuestionSet._from_json(text)
        except UsageError as error:
            raise LocalError(str(error)) from error
        source = json.loads(text)
    fields = {}
    for name, question in source["questions"].items():
        kind = next(verb for verb in ("decide", "choose", "score", "tag") if verb in question)
        if kind in ("choose", "tag"):
            key = "options" if kind == "choose" else "labels"
            named = question[key]
            label_type = Literal.__getitem__(tuple(named))
            answer = label_type if kind == "choose" else list[label_type]
        else:
            answer = bool if kind == "decide" else float
        if kind in ("decide", "choose"):
            answer = answer | None
        fields[name] = (answer | _Failed, Field(...))
    return create_model("ThinkThenRow", __config__=ConfigDict(strict=True, extra="forbid"),
                        **fields)
