"""Translate the engine's typed contract into independent Python values."""

from __future__ import annotations

import json
from dataclasses import fields, is_dataclass
from enum import Enum
from types import MappingProxyType
from typing import Any

from . import _types as t

_ROWS = {
    value.value: getattr(t, value.value.capitalize() + "Row") for value in t.Function
}
_OUTCOMES = {
    value.value: getattr(t, value.value.capitalize() + "Outcome")
    for value in t.Function
    if value not in {t.Function.ANNOTATE, t.Function.RANK}
}


def relation(value: dict[str, Any]) -> t.Relation:
    return t.Relation(
        source=t.RelationName(**value["source"]),
        target=t.RelationName(**value["target"]),
        relation=value["relation"],
        probability=value["probability"],
        either=value.get("either", False),
    )


def detail(function: str, value: Any) -> Any:
    if value is None:
        return None
    if function in {"choose", "tag"}:
        return t.LabelDetail(MappingProxyType(value["labels"]))
    if function in {"score", "rank"}:
        return t.LevelDetail(tuple(value["levels"]))
    if function == "relate":
        return t.RelationDetail(
            tuple(t.Name(**name) for name in value["names"]),
            tuple(relation(item) for item in value["pairs"]),
        )
    if function == "recognize":
        return t.RecognizeDetail(
            tuple(
                t.PieceDetail(
                    item["text"],
                    item["start"],
                    item["end"],
                    MappingProxyType(
                        {
                            t.BoundaryTag(key): probability
                            for key, probability in item["tags"].items()
                        }
                    ),
                )
                for item in value["pieces"]
            ),
            tuple(
                t.KindDetail(
                    item["start"], item["end"], MappingProxyType(item["kinds"])
                )
                for item in value["names"]
            ),
        )
    return None


def result(value: dict[str, Any], *, child: bool = False, item: object = None) -> Any:
    function = value["function"]
    answer = value.get("answer")
    if answer is not None:
        if function == "tag":
            answer = tuple(answer)
        elif function == "find":
            answer = t.Span(**answer)
        elif function == "recognize":
            answer = tuple(t.Name(**name) for name in answer)
        elif function == "relate":
            answer = tuple(relation(pair) for pair in answer)
        elif function == "annotate":
            answer = MappingProxyType(
                {name: result(outcome, child=True) for name, outcome in answer.items()}
            )
    error = value.get("error")
    kwargs: dict[str, Any] = {
        "function": t.Function(function),
        "status": t.Status(value["status"]),
        "answer": answer,
        "probability": value.get("probability"),
        "error": t.RowError(
            t.ErrorCode(error["code"]), error["message"], error.get("http_status")
        )
        if error
        else None,
        "detail": detail(function, value.get("detail")),
    }
    if not child:
        source = value["source"]
        kwargs.update(
            position=value["position"],
            id=value.get("id"),
            source=t.Source(
                source["question_identity"],
                t.Backend(source["backend"]),
                source["model"],
                source.get("name"),
            ),
        )
        if function == "filter":
            kwargs["item"] = item
    return (_OUTCOMES if child else _ROWS)[function](**kwargs)


def row_from_json(text: str, raw_detail: str | None = None, item: object = None) -> Any:
    try:
        value = json.loads(text)
        if raw_detail is not None:
            value["detail"] = json.loads(raw_detail)
        return result(value, item=item)
    except (KeyError, TypeError, ValueError):
        from ._contract import ABI_MESSAGE
        from ._types import ErrorCode, SetupError

        raise SetupError(ErrorCode.USAGE_ABI, ABI_MESSAGE) from None


def plain(value: Any) -> Any:
    """Project typed values for column libraries without losing integer ids."""
    if isinstance(value, Enum):
        return value.value
    if is_dataclass(value) and not isinstance(value, type):
        return {
            field.name: plain(getattr(value, field.name)) for field in fields(value)
        }
    if isinstance(value, (dict, MappingProxyType)):
        return {plain(key): plain(item) for key, item in value.items()}
    if isinstance(value, (list, tuple)):
        return [plain(item) for item in value]
    return value


__all__ = []
