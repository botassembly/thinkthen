"""Lower Python label types to the existing ordered question-file shapes."""

from enum import Enum
from typing import Annotated, Literal, get_args, get_origin

from ._thinkthen import UsageError


def _literal(value):
    if get_origin(value) is not Literal:
        raise UsageError("labels take a direct Literal of strings")
    labels = get_args(value)
    if any(not isinstance(label, str) for label in labels):
        raise UsageError("Literal labels are strings")
    if len(set(labels)) != len(labels):
        raise UsageError("Literal labels must be distinct")
    return list(labels)


def _enum(value):
    labels = []
    meanings = {}
    if len(value.__members__) != len(list(value)):
        raise UsageError("Enum labels must not have aliases")
    for member in value:
        label = member.value if isinstance(member.value, str) else member.name
        if label in labels:
            raise UsageError("Enum labels must be distinct")
        labels.append(label)
        meaning = getattr(member, "description", None)
        if meaning is None:
            doc = member.__doc__
            if doc and doc != value.__doc__:
                meaning = doc
        if meaning is not None:
            meanings[label] = meaning
    return labels, meanings


def normalize(value, descriptions=None, *, bare_level_names=False):
    """Return an old list/map unchanged, or an ordered new list/map."""
    if isinstance(value, type) and issubclass(value, Enum):
        labels, meanings = _enum(value)
    elif get_origin(value) is Literal:
        labels, meanings = _literal(value), {}
    elif get_origin(value) is Annotated or (
        isinstance(value, tuple) and value and all(get_origin(item) is Annotated for item in value)
    ) or (
        isinstance(value, type) and any(base.__module__.startswith("pydantic.")
                                        for base in value.__mro__)
    ):
        from . import pydantic as adapter
        labels, meanings = adapter.labels(value)
    else:
        if isinstance(value, Enum) or any(base.__module__.startswith("pydantic.")
                                          for base in type(value).__mro__):
            raise UsageError("labels take an Enum or Pydantic model class, not an instance")
        if descriptions is not None:
            raise UsageError("descriptions= takes an Enum, Literal or Pydantic label set")
        if get_origin(value) is not None:
            raise UsageError("labels take a direct Literal of strings")
        return value
    if descriptions is not None:
        if not isinstance(descriptions, dict):
            raise UsageError("descriptions= maps selected labels to meanings")
        unknown = descriptions.keys() - set(labels)
        if unknown:
            raise UsageError("descriptions= names an unknown label")
        meanings.update(descriptions)
    if not meanings:
        return labels
    return {label: meanings[label] if label in meanings else
            (label if bare_level_names else None) for label in labels}
