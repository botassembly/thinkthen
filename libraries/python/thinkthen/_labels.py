"""Lower Python label types to the existing ordered question-file shapes."""

from enum import Enum
from typing import Annotated, Literal, get_args, get_origin

from ._thinkthen import UsageError


def _literal(value):
    if get_origin(value) is not Literal:
        raise UsageError("labels take a direct Literal of strings")
    labels = get_args(value)
    from ._thinkthen import _label_names
    return _label_names("Literal", list(labels))


def _enum(value):
    labels = []
    meanings = {}
    from ._thinkthen import _label_names
    _label_names("Enum", [member.name for member in value.__members__.values()])
    for member in value:
        label = member.value if isinstance(member.value, str) else member.name
        labels.append(label)
        meaning = getattr(member, "description", None)
        if meaning is None:
            doc = member.__doc__
            if doc and doc != value.__doc__:
                meaning = doc
        if meaning is not None:
            meanings[label] = meaning
    _label_names("EnumValues", labels)
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
    from ._thinkthen import _label_values
    return _label_values(labels, meanings, descriptions, bare_level_names)
