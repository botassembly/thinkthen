"""The typed face of the thinkthen package.

The implementation is a pyo3 extension module; this stub carries the
public signatures the type checkers read, matching ``thinkthen/__init__.py``
(the docstring-carrying wrappers) and the Rust door beneath them.
"""

from typing import Any, Optional, Sequence

__all__ = [
    "Cancelled",
    "DefectError",
    "UsageError",
    "BackendError",
    "DeadlineError",
    "LocalError",
    "ThinkThenError",
    "CancelToken",
    "Question",
    "Edge",
    "Recognized",
    "Relation",
    "Entity",
    "question",
    "decide",
    "decide_many",
    "choose",
    "score",
    "tag",
    "filter",
    "rank",
    "find",
    "annotate",
    "details",
    "recognize",
    "relate",
    "load_questions",
]

class ThinkThenError(Exception):
    kind: str
    retryable: bool

class Cancelled(ThinkThenError): ...
class DefectError(ThinkThenError): ...
class UsageError(ThinkThenError): ...
class BackendError(ThinkThenError): ...
class DeadlineError(ThinkThenError): ...
class LocalError(ThinkThenError): ...

class CancelToken:
    def cancel(self) -> None: ...
    @property
    def cancelled(self) -> bool: ...

class Question: ...

class Entity:
    text: str
    kind: str
    start: int
    end: int
    strength: float

class Relation:
    name: str
    source: str
    target: str
    probability: float

class Edge:
    source: str
    target: str

class Recognized:
    entities: list[Entity]
    relations: list[Relation]

def question(
    *,
    decide: Optional[str] = ...,
    choose: Optional[str] = ...,
    score: Optional[str] = ...,
    tag: Optional[str] = ...,
    threshold: Optional[Any] = ...,
    options: Optional[Sequence[str]] = ...,
    labels: Optional[Sequence[str]] = ...,
    levels: Optional[Sequence[str]] = ...,
    true_: Optional[str] = ...,
    false_: Optional[str] = ...,
    model: Optional[str] = ...,
    file: Optional[str] = ...,
) -> Question: ...

def decide(question: Any, text: Any, *, deadline: Optional[float] = ..., token: Optional[CancelToken] = ...) -> Any: ...
def decide_many(question: Any, records: Sequence[Any], *, deadline: Optional[float] = ..., token: Optional[CancelToken] = ...) -> list[Any]: ...
def choose(question: Any, text: str, *, options: Optional[Sequence[str]] = ..., deadline: Optional[float] = ..., token: Optional[CancelToken] = ...) -> Any: ...
def score(question: Any, text: Any, *, levels: Optional[Sequence[str]] = ..., deadline: Optional[float] = ..., token: Optional[CancelToken] = ...) -> Any: ...
def tag(question: Any, text: str, *, labels: Optional[Sequence[str]] = ..., deadline: Optional[float] = ..., token: Optional[CancelToken] = ...) -> list[str]: ...
def filter(question: Any, records: Sequence[str], *, deadline: Optional[float] = ..., token: Optional[CancelToken] = ...) -> list[str]: ...
def rank(question: Any, records: Sequence[str], *, deadline: Optional[float] = ..., token: Optional[CancelToken] = ...) -> list[Any]: ...
def find(question: Any, units: Sequence[str], *, deadline: Optional[float] = ..., token: Optional[CancelToken] = ...) -> Optional[tuple[int, float]]: ...
def annotate(set_or_path: Any, records: Any, *, on: Optional[str] = ..., deadline: Optional[float] = ..., token: Optional[CancelToken] = ...) -> Any: ...
def details(question: Any, text: str, *, deadline: Optional[float] = ..., token: Optional[CancelToken] = ...) -> dict[str, Any]: ...
def recognize(text: str, *, kinds: Any = ..., relations: Any = ..., threshold: Optional[float] = ..., relation_threshold: Optional[float] = ..., deadline: Optional[float] = ..., token: Optional[CancelToken] = ...) -> Recognized: ...
def relate(records: Any, *, relations: Any = ..., either: Any = ..., threshold: Optional[float] = ..., deadline: Optional[float] = ..., token: Optional[CancelToken] = ...) -> list[Relation]: ...
def usage(question: Any) -> dict[str, Any]: ...
