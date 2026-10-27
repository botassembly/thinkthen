"""Owned clients over the shared admitted engine contract."""

from __future__ import annotations

import atexit
import json
import logging
from collections.abc import Callable, Iterable
from dataclasses import fields
from threading import RLock
from typing import Any, TypeVar, overload
from weakref import WeakSet

from . import _native
from ._questions import (
    _MISSING,
    AnnotateQuestion,
    AnnotationQuestions,
    ChooseQuestion,
    DecideQuestion,
    FilterQuestion,
    FindQuestion,
    RankQuestion,
    RecognizeQuestion,
    RelateQuestion,
    ScoreQuestion,
    TagQuestion,
    _Question,
    definition,
)
from ._runtime import Cancel, binding, encode, source
from ._streams import RowStream
from ._types import (
    AnnotateRow,
    Backend,
    BackendInput,
    ChooseRow,
    ClientSettings,
    DecideRow,
    FilterRow,
    FindRow,
    RankRow,
    RecognizeRow,
    Record,
    RelateRow,
    RelationRule,
    RetryPolicy,
    ScoreRow,
    Strings,
    TagRow,
    Threshold,
)

T = TypeVar("T")
_log = logging.getLogger("thinkthen")
_log.addHandler(logging.NullHandler())


def _milliseconds(value: float) -> int:
    try:
        seconds = float(value)
        exact = seconds == value
    except Exception:  # noqa: BLE001 - host conversion must use the static engine error.
        binding(_native.reject_settings_conversion)
    if not exact:
        binding(_native.reject_settings_conversion)
    return binding(lambda: _native.seconds_to_millis(seconds))


class Client:
    def __init__(
        self,
        *,
        backend: BackendInput = "systemone",
        url: str | None = None,
        model: str | None = None,
        api_key: str | None = None,
        batch: int | None = None,
        retry: RetryPolicy | None = None,
    ) -> None:
        values: dict[str, object] = {
            "backend": backend,
            "url": url,
            "model": model,
            "api_key": api_key,
            "batch": batch,
        }
        if isinstance(batch, bool):
            binding(_native.reject_settings_conversion)
        if retry is not None:
            if not isinstance(retry, RetryPolicy):
                binding(_native.reject_settings_conversion)
            policy: dict[str, object] = {}
            for field in fields(retry):
                name, value = field.name, getattr(retry, field.name)
                if value is None:
                    continue
                if isinstance(value, bool) or not isinstance(value, (int, float)):
                    binding(_native.reject_settings_conversion)
                if name == "max_retries":
                    policy[name] = value
                else:
                    policy[name + "_ms"] = _milliseconds(value)
            values["retry"] = policy
        settings = encode(values, _native.reject_settings_conversion)
        self._native: _native.NativeEngine | None = binding(
            lambda: _native.NativeEngine(settings)
        )
        self._lock = RLock()
        self._active: WeakSet[RowStream[Any]] = WeakSet()

    @property
    def settings(self) -> ClientSettings:
        with self._lock:
            engine = self._native
            if engine is None:
                binding(_native.reject_closed_client)
            value = json.loads(binding(lambda: engine.settings))
        retry = value["retry"]
        return ClientSettings(
            Backend(value["backend"]),
            value["model"],
            value["url"],
            value["batch"],
            RetryPolicy(
                retry["max_retries"],
                retry["initial_backoff_ms"] / 1000,
                retry["attempt_timeout_ms"] / 1000,
            ),
        )

    @classmethod
    def from_env(
        cls,
        *,
        backend: BackendInput = "systemone",
        url: str | None = None,
        model: str | None = None,
        api_key: str | None = None,
        batch: int | None = None,
        retry: RetryPolicy | None = None,
    ) -> Client:
        return cls(
            backend=backend,
            url=url,
            model=model,
            api_key=api_key,
            batch=batch,
            retry=retry,
        )

    def __repr__(self) -> str:
        return f"Client(closed={self._native is None})"

    def __reduce__(self) -> tuple[Any, ...]:
        raise TypeError("operational clients cannot be pickled")

    def close(self) -> None:
        with self._lock:
            active = list(self._active)
            self._native = None
        for stream in active:
            stream.cancel()
        for stream in active:
            stream.close()

    def __enter__(self) -> Client:  # noqa: PYI034 - standard library on Python 3.10.
        return self

    def __exit__(self, *exception: object) -> None:
        self.close()

    def _rows(
        self,
        function: str,
        question: object,
        records: Iterable[Any],
        convert: Callable[[object], object] | None = None,
        /,
        *,
        name: str | None = None,
        cancel: Cancel | None = None,
        **options: object,
    ) -> RowStream[Any]:
        shape_question = question
        loaded: _native.NativeQuestion | None = None
        if isinstance(question, _Question):
            if question._native.function != function or any(
                value is not None and value is not _MISSING
                for value in options.values()
            ):
                binding(_native.reject_question_conversion)
            loaded = question._native
            payload: dict[str, object] = {"name": name}
        else:
            if function == "annotate":
                if not isinstance(question, AnnotationQuestions):
                    binding(_native.reject_question_conversion)
                question = question._definitions()
            admitted = definition(function, question, **options)
            payload = {"question": admitted, "name": name}
        request = encode(payload, _native.reject_question_conversion)
        handle = cancel if cancel is not None else Cancel()
        with self._lock:
            engine = self._native
            if engine is None:
                binding(_native.reject_closed_client)
            start = (
                engine.start_finite
                if isinstance(records, (list, tuple))
                else engine.start
            )
            failures: dict[int, str] = {}
            native = binding(
                lambda: start(
                    request,
                    source(records, failures, convert),
                    handle._native,
                    function == "filter",
                    loaded,
                )
            )
            stream: RowStream[Any] = RowStream(
                native,
                failures,
                len(records) if isinstance(records, (list, tuple)) else 0,
                function=function,
                question=shape_question,
            )
            self._active.add(stream)
        _log.debug("%s", function)
        return stream

    def _single(
        self, function: str, question: object, record: Record, **options: Any
    ) -> Any:
        with self._rows(function, question, (record,), **options) as rows:
            row = next(rows)
            for _ in rows:
                pass
            return row

    def _invoke(
        self,
        function: str,
        collection: bool,
        args: tuple[Any, ...],
        options: dict[str, Any],
    ) -> Any:
        key = "records" if collection else "record"
        if function in {"recognize", "relate"} and len(args) == 1:
            question, records = None, args[0]
        else:
            question = (
                args[0]
                if args
                else options.pop("question", options.pop("toolbox", None))
            )
            if len(args) < 2 and key not in options:
                raise TypeError(f"{key} is required")
            records = args[1] if len(args) > 1 else options.pop(key)
        domain = {"recognize": "kinds", "relate": "rules"}.get(function)
        if (
            domain is not None
            and not isinstance(question, _Question)
            and domain not in options
        ):
            raise TypeError(f"{domain} is required")
        method = self._rows if collection else self._single
        return method(function, question, records, **options)

    @overload
    def decide(
        self,
        question: DecideQuestion,
        record: Record,
        *,
        name: str | None = None,
        cancel: Cancel | None = None,
    ) -> DecideRow: ...
    @overload
    def decide(
        self,
        question: str,
        record: Record,
        *,
        threshold: Threshold | None = None,
        evidence: str | None = None,
        name: str | None = None,
        cancel: Cancel | None = None,
    ) -> DecideRow: ...
    def decide(
        self,
        question: Any,
        record: Any,
        *,
        threshold: Any = None,
        evidence: Any = None,
        name: str | None = None,
        cancel: Cancel | None = None,
    ) -> Any:
        return self._single(
            "decide",
            question,
            record,
            threshold=threshold,
            evidence=evidence,
            name=name,
            cancel=cancel,
        )

    @overload
    def decide_stream(
        self,
        question: DecideQuestion,
        records: Iterable[Record],
        *,
        name: str | None = None,
        cancel: Cancel | None = None,
    ) -> RowStream[DecideRow]: ...
    @overload
    def decide_stream(
        self,
        question: str,
        records: Iterable[Record],
        *,
        threshold: Threshold | None = None,
        evidence: str | None = None,
        name: str | None = None,
        cancel: Cancel | None = None,
    ) -> RowStream[DecideRow]: ...
    def decide_stream(
        self,
        question: Any,
        records: Any,
        *,
        threshold: Any = None,
        evidence: Any = None,
        name: str | None = None,
        cancel: Cancel | None = None,
    ) -> Any:
        return self._rows(
            "decide",
            question,
            records,
            threshold=threshold,
            evidence=evidence,
            name=name,
            cancel=cancel,
        )

    @overload
    def filter(
        self,
        question: FilterQuestion,
        records: Iterable[T],
        *,
        name: str | None = None,
        cancel: Cancel | None = None,
    ) -> RowStream[FilterRow[T]]: ...
    @overload
    def filter(
        self,
        question: str,
        records: Iterable[T],
        *,
        threshold: Threshold | None = None,
        evidence: str | None = None,
        name: str | None = None,
        cancel: Cancel | None = None,
    ) -> RowStream[FilterRow[T]]: ...
    def filter(
        self,
        question: Any,
        records: Any,
        *,
        threshold: Any = None,
        evidence: Any = None,
        name: str | None = None,
        cancel: Cancel | None = None,
    ) -> Any:
        return self._rows(
            "filter",
            question,
            records,
            threshold=threshold,
            evidence=evidence,
            name=name,
            cancel=cancel,
        )

    @overload
    def choose(
        self,
        question: ChooseQuestion,
        record: Record,
        *,
        name: str | None = None,
        cancel: Cancel | None = None,
    ) -> ChooseRow: ...
    @overload
    def choose(
        self,
        question: str,
        record: Record,
        *,
        choices: Strings,
        threshold: Threshold | None = None,
        evidence: str | None = None,
        name: str | None = None,
        cancel: Cancel | None = None,
    ) -> ChooseRow: ...
    def choose(
        self,
        question: Any,
        record: Any,
        *,
        choices: Any = _MISSING,
        threshold: Any = None,
        evidence: Any = None,
        name: str | None = None,
        cancel: Cancel | None = None,
    ) -> Any:
        if not isinstance(question, _Question) and choices is _MISSING:
            raise TypeError("choices is required")
        return self._single(
            "choose",
            question,
            record,
            choices=choices,
            threshold=threshold,
            evidence=evidence,
            name=name,
            cancel=cancel,
        )

    @overload
    def choose_stream(
        self,
        question: ChooseQuestion,
        records: Iterable[Record],
        *,
        name: str | None = None,
        cancel: Cancel | None = None,
    ) -> RowStream[ChooseRow]: ...
    @overload
    def choose_stream(
        self,
        question: str,
        records: Iterable[Record],
        *,
        choices: Strings,
        threshold: Threshold | None = None,
        evidence: str | None = None,
        name: str | None = None,
        cancel: Cancel | None = None,
    ) -> RowStream[ChooseRow]: ...
    def choose_stream(
        self,
        question: Any,
        records: Any,
        *,
        choices: Any = _MISSING,
        threshold: Any = None,
        evidence: Any = None,
        name: str | None = None,
        cancel: Cancel | None = None,
    ) -> Any:
        if not isinstance(question, _Question) and choices is _MISSING:
            raise TypeError("choices is required")
        return self._rows(
            "choose",
            question,
            records,
            choices=choices,
            threshold=threshold,
            evidence=evidence,
            name=name,
            cancel=cancel,
        )

    @overload
    def tag(
        self,
        question: TagQuestion,
        record: Record,
        *,
        name: str | None = None,
        cancel: Cancel | None = None,
    ) -> TagRow: ...
    @overload
    def tag(
        self,
        question: str,
        record: Record,
        *,
        labels: Strings,
        threshold: float | None = None,
        evidence: str | None = None,
        name: str | None = None,
        cancel: Cancel | None = None,
    ) -> TagRow: ...
    def tag(
        self,
        question: Any,
        record: Any,
        *,
        labels: Any = _MISSING,
        threshold: Any = None,
        evidence: Any = None,
        name: str | None = None,
        cancel: Cancel | None = None,
    ) -> Any:
        if not isinstance(question, _Question) and labels is _MISSING:
            raise TypeError("labels is required")
        return self._single(
            "tag",
            question,
            record,
            labels=labels,
            threshold=threshold,
            evidence=evidence,
            name=name,
            cancel=cancel,
        )

    @overload
    def tag_stream(
        self,
        question: TagQuestion,
        records: Iterable[Record],
        *,
        name: str | None = None,
        cancel: Cancel | None = None,
    ) -> RowStream[TagRow]: ...
    @overload
    def tag_stream(
        self,
        question: str,
        records: Iterable[Record],
        *,
        labels: Strings,
        threshold: float | None = None,
        evidence: str | None = None,
        name: str | None = None,
        cancel: Cancel | None = None,
    ) -> RowStream[TagRow]: ...
    def tag_stream(
        self,
        question: Any,
        records: Any,
        *,
        labels: Any = _MISSING,
        threshold: Any = None,
        evidence: Any = None,
        name: str | None = None,
        cancel: Cancel | None = None,
    ) -> Any:
        if not isinstance(question, _Question) and labels is _MISSING:
            raise TypeError("labels is required")
        return self._rows(
            "tag",
            question,
            records,
            labels=labels,
            threshold=threshold,
            evidence=evidence,
            name=name,
            cancel=cancel,
        )

    @overload
    def score(
        self,
        question: ScoreQuestion,
        record: Record,
        *,
        name: str | None = None,
        cancel: Cancel | None = None,
    ) -> ScoreRow: ...
    @overload
    def score(
        self,
        question: str,
        record: Record,
        *,
        levels: Strings,
        evidence: str | None = None,
        name: str | None = None,
        cancel: Cancel | None = None,
    ) -> ScoreRow: ...
    def score(
        self,
        question: Any,
        record: Any,
        *,
        levels: Any = _MISSING,
        evidence: Any = None,
        name: str | None = None,
        cancel: Cancel | None = None,
    ) -> Any:
        if not isinstance(question, _Question) and levels is _MISSING:
            raise TypeError("levels is required")
        return self._single(
            "score",
            question,
            record,
            levels=levels,
            evidence=evidence,
            name=name,
            cancel=cancel,
        )

    @overload
    def score_stream(
        self,
        question: ScoreQuestion,
        records: Iterable[Record],
        *,
        name: str | None = None,
        cancel: Cancel | None = None,
    ) -> RowStream[ScoreRow]: ...
    @overload
    def score_stream(
        self,
        question: str,
        records: Iterable[Record],
        *,
        levels: Strings,
        evidence: str | None = None,
        name: str | None = None,
        cancel: Cancel | None = None,
    ) -> RowStream[ScoreRow]: ...
    def score_stream(
        self,
        question: Any,
        records: Any,
        *,
        levels: Any = _MISSING,
        evidence: Any = None,
        name: str | None = None,
        cancel: Cancel | None = None,
    ) -> Any:
        if not isinstance(question, _Question) and levels is _MISSING:
            raise TypeError("levels is required")
        return self._rows(
            "score",
            question,
            records,
            levels=levels,
            evidence=evidence,
            name=name,
            cancel=cancel,
        )

    @overload
    def rank(
        self,
        question: RankQuestion,
        records: Iterable[Record],
        *,
        name: str | None = None,
        cancel: Cancel | None = None,
    ) -> RowStream[RankRow]: ...
    @overload
    def rank(
        self,
        question: str,
        records: Iterable[Record],
        *,
        levels: Strings,
        top: int | None = None,
        evidence: str | None = None,
        name: str | None = None,
        cancel: Cancel | None = None,
    ) -> RowStream[RankRow]: ...
    def rank(
        self,
        question: Any,
        records: Any,
        *,
        levels: Any = _MISSING,
        top: Any = None,
        evidence: Any = None,
        name: str | None = None,
        cancel: Cancel | None = None,
    ) -> Any:
        if not isinstance(question, _Question) and levels is _MISSING:
            raise TypeError("levels is required")
        return self._rows(
            "rank",
            question,
            records,
            levels=levels,
            top=top,
            evidence=evidence,
            name=name,
            cancel=cancel,
        )

    @overload
    def find(
        self,
        question: FindQuestion,
        record: Record,
        *,
        name: str | None = None,
        cancel: Cancel | None = None,
    ) -> FindRow: ...
    @overload
    def find(
        self,
        question: str,
        record: Record,
        *,
        threshold: float | None = None,
        evidence: str | None = None,
        name: str | None = None,
        cancel: Cancel | None = None,
    ) -> FindRow: ...
    def find(
        self,
        question: Any,
        record: Any,
        *,
        threshold: Any = None,
        evidence: Any = None,
        name: str | None = None,
        cancel: Cancel | None = None,
    ) -> Any:
        return self._single(
            "find",
            question,
            record,
            threshold=threshold,
            evidence=evidence,
            name=name,
            cancel=cancel,
        )

    @overload
    def find_stream(
        self,
        question: FindQuestion,
        records: Iterable[Record],
        *,
        name: str | None = None,
        cancel: Cancel | None = None,
    ) -> RowStream[FindRow]: ...
    @overload
    def find_stream(
        self,
        question: str,
        records: Iterable[Record],
        *,
        threshold: float | None = None,
        evidence: str | None = None,
        name: str | None = None,
        cancel: Cancel | None = None,
    ) -> RowStream[FindRow]: ...
    def find_stream(
        self,
        question: Any,
        records: Any,
        *,
        threshold: Any = None,
        evidence: Any = None,
        name: str | None = None,
        cancel: Cancel | None = None,
    ) -> Any:
        return self._rows(
            "find",
            question,
            records,
            threshold=threshold,
            evidence=evidence,
            name=name,
            cancel=cancel,
        )

    @overload
    def recognize(
        self,
        question: RecognizeQuestion,
        record: Record,
        *,
        name: str | None = None,
        cancel: Cancel | None = None,
    ) -> RecognizeRow: ...
    @overload
    def recognize(
        self,
        record: Record,
        *,
        kinds: Strings,
        threshold: float | None = None,
        evidence: str | None = None,
        name: str | None = None,
        cancel: Cancel | None = None,
    ) -> RecognizeRow: ...
    def recognize(self, *args: Any, **options: Any) -> Any:
        return self._invoke("recognize", False, args, options)

    @overload
    def recognize_stream(
        self,
        question: RecognizeQuestion,
        records: Iterable[Record],
        *,
        name: str | None = None,
        cancel: Cancel | None = None,
    ) -> RowStream[RecognizeRow]: ...
    @overload
    def recognize_stream(
        self,
        records: Iterable[Record],
        *,
        kinds: Strings,
        threshold: float | None = None,
        evidence: str | None = None,
        name: str | None = None,
        cancel: Cancel | None = None,
    ) -> RowStream[RecognizeRow]: ...
    def recognize_stream(self, *args: Any, **options: Any) -> Any:
        return self._invoke("recognize", True, args, options)

    @overload
    def relate(
        self,
        question: RelateQuestion,
        record: Record,
        *,
        name: str | None = None,
        cancel: Cancel | None = None,
    ) -> RelateRow: ...
    @overload
    def relate(
        self,
        record: Record,
        *,
        rules: list[RelationRule] | tuple[RelationRule, ...],
        threshold: float | None = None,
        evidence: str | None = None,
        name: str | None = None,
        cancel: Cancel | None = None,
    ) -> RelateRow: ...
    def relate(self, *args: Any, **options: Any) -> Any:
        return self._invoke("relate", False, args, options)

    @overload
    def relate_stream(
        self,
        question: RelateQuestion,
        records: Iterable[Record],
        *,
        name: str | None = None,
        cancel: Cancel | None = None,
    ) -> RowStream[RelateRow]: ...
    @overload
    def relate_stream(
        self,
        records: Iterable[Record],
        *,
        rules: list[RelationRule] | tuple[RelationRule, ...],
        threshold: float | None = None,
        evidence: str | None = None,
        name: str | None = None,
        cancel: Cancel | None = None,
    ) -> RowStream[RelateRow]: ...
    def relate_stream(self, *args: Any, **options: Any) -> Any:
        return self._invoke("relate", True, args, options)

    @overload
    def annotate(
        self,
        question: AnnotateQuestion,
        record: Record,
        *,
        name: str | None = None,
        cancel: Cancel | None = None,
    ) -> AnnotateRow: ...
    @overload
    def annotate(
        self,
        toolbox: AnnotationQuestions,
        record: Record,
        *,
        evidence: str | None = None,
        name: str | None = None,
        cancel: Cancel | None = None,
    ) -> AnnotateRow: ...
    def annotate(self, *args: Any, **options: Any) -> Any:
        return self._invoke("annotate", False, args, options)

    @overload
    def annotate_stream(
        self,
        question: AnnotateQuestion,
        records: Iterable[Record],
        *,
        name: str | None = None,
        cancel: Cancel | None = None,
    ) -> RowStream[AnnotateRow]: ...
    @overload
    def annotate_stream(
        self,
        toolbox: AnnotationQuestions,
        records: Iterable[Record],
        *,
        evidence: str | None = None,
        name: str | None = None,
        cancel: Cancel | None = None,
    ) -> RowStream[AnnotateRow]: ...
    def annotate_stream(self, *args: Any, **options: Any) -> Any:
        return self._invoke("annotate", True, args, options)


_default: Client | None = None
_default_lock = RLock()


def default_client() -> Client:
    global _default
    with _default_lock:
        if _default is None:
            _default = Client()
        return _default


def close_default_client() -> None:
    global _default
    with _default_lock:
        old, _default = _default, None
    if old is not None:
        old.close()


atexit.register(close_default_client)
__all__ = ["Client", "close_default_client"]
