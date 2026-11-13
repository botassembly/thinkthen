"""Owned clients over the shared admitted engine contract."""

from __future__ import annotations

import atexit
import json
import logging
from collections.abc import Callable, Iterable, Mapping
from dataclasses import fields
from threading import Event, RLock
from typing import Any, Literal, TypeVar, cast, overload
from weakref import WeakSet

from typing_extensions import TypeForm

from . import _native
from ._questions import (
    _MISSING,
    AnnotateQuestion,
    AnnotationInput,
    AnnotationQuestion,
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
    annotation_members,
    choice_members,
    definition,
)
from ._runtime import Cancel, binding, encode, one_row, single_record, source
from ._streams import Errors, KeptStream, RowStream, coerce_errors
from ._types import (
    AnnotateRow,
    Backend,
    BackendInput,
    ChoiceInput,
    ChooseRow,
    ClientSettings,
    DecideRow,
    Descriptions,
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
    Usage,
    _Label,
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
        backend: BackendInput | None = None,
        url: str | None = None,
        model: str | None = None,
        api_key: str | None = None,
        batch: int | None = None,
        max_requests: int | None = None,
        retry: RetryPolicy | None = None,
    ) -> None:
        """Open a client.

        Without ``backend``, the client reads ``THINKTHEN_BACKEND`` as the
        command does, and then uses System One.
        """
        values: dict[str, object] = {
            "url": url,
            "model": model,
            "api_key": api_key,
            "batch": batch,
            "max_requests": max_requests,
        }
        if backend is not None:
            # Omitted, the shared settings read THINKTHEN_BACKEND as the command does.
            values["backend"] = backend
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
                if name == "max_retries":
                    # Retry counts are JSON values with one native admission
                    # rule. Preserve invalid representable values for its exact
                    # redacted diagnostic instead of replacing that contract.
                    policy[name] = value
                else:
                    if isinstance(value, bool) or not isinstance(value, (int, float)):
                        binding(_native.reject_settings_conversion)
                    policy[name + "_ms"] = _milliseconds(value)
            values["retry"] = policy
        settings = encode(values, _native.reject_settings_conversion)
        self._native: _native.NativeEngine | None = binding(
            lambda: _native.NativeEngine(settings)
        )
        self._closed = Event()
        self._final_usage: Usage | None = None
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
            value.get("max_requests"),
        )

    @property
    def usage(self) -> Usage:
        """Cumulative usage; a copied snapshot remains valid after close."""
        with self._lock:
            if self._final_usage is not None:
                return self._final_usage
            engine = self._native
            if engine is None:
                binding(_native.reject_closed_client)
            return Usage._from_json(binding(lambda: engine.usage))

    def __repr__(self) -> str:
        return f"Client(closed={self._native is None})"

    def __reduce__(self) -> tuple[Any, ...]:
        raise TypeError("operational clients cannot be pickled")

    def close(self) -> None:
        with self._lock:
            engine = self._native
            active = list(self._active)
            self._native = None
        if engine is None:
            self._closed.wait()
            return
        try:
            for stream in active:
                stream.cancel()
            for stream in active:
                stream.close()
            snapshot = Usage._from_json(binding(engine.usage_settled))
            with self._lock:
                self._final_usage = snapshot
        finally:
            del engine
            self._closed.set()

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
        members = getattr(question, "_members", {})
        domain_name = {"choose": "choices", "tag": "labels"}.get(function)
        if domain_name is not None:
            domain = options.get(domain_name, _MISSING)
            if domain is not _MISSING:
                members = choice_members(domain, domain_name)
        children = annotation_members(question) if function == "annotate" else {}

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
                if isinstance(question, Mapping):
                    question = AnnotationQuestions(
                        cast(Mapping[str, AnnotationQuestion], question)
                    )
                if not isinstance(question, AnnotationQuestions):
                    raise TypeError(
                        "toolbox must be a dict of questions, a Toolbox, or AnnotationQuestions"
                    )
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
                members=members,
                children=children,
            )
            self._active.add(stream)
        _log.debug("%s", function)
        return stream

    def _single(
        self, function: str, question: object, record: Record, **options: Any
    ) -> Any:
        single_record(function, record)
        with self._rows(function, question, (record,), **options) as rows:
            row = next(rows)
            for _ in rows:
                pass
            return one_row(row)

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
        field: str | None = None,
        yes_means: str | None = None,
        no_means: str | None = None,
        name: str | None = None,
        cancel: Cancel | None = None,
    ) -> DecideRow: ...
    def decide(
        self,
        question: Any,
        record: Any,
        *,
        threshold: Any = None,
        field: Any = None,
        yes_means: Any = None,
        no_means: Any = None,
        name: str | None = None,
        cancel: Cancel | None = None,
    ) -> Any:
        return self._single(
            "decide",
            question,
            record,
            threshold=threshold,
            field=field,
            yes_means=yes_means,
            no_means=no_means,
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
        field: str | None = None,
        yes_means: str | None = None,
        no_means: str | None = None,
        name: str | None = None,
        cancel: Cancel | None = None,
    ) -> RowStream[DecideRow]: ...
    def decide_stream(
        self,
        question: Any,
        records: Any,
        *,
        threshold: Any = None,
        field: Any = None,
        yes_means: Any = None,
        no_means: Any = None,
        name: str | None = None,
        cancel: Cancel | None = None,
    ) -> Any:
        return self._rows(
            "decide",
            question,
            records,
            threshold=threshold,
            field=field,
            yes_means=yes_means,
            no_means=no_means,
            name=name,
            cancel=cancel,
        )

    @overload
    def filter(
        self,
        question: FilterQuestion,
        records: Iterable[T],
        *,
        errors: Literal["raise"] = "raise",
        name: str | None = None,
        cancel: Cancel | None = None,
    ) -> KeptStream[T]: ...
    @overload
    def filter(
        self,
        question: FilterQuestion,
        records: Iterable[T],
        *,
        errors: Literal["coerce"],
        name: str | None = None,
        cancel: Cancel | None = None,
    ) -> KeptStream[T | FilterRow[T]]: ...
    @overload
    def filter(
        self,
        question: str,
        records: Iterable[T],
        *,
        threshold: Threshold | None = None,
        field: str | None = None,
        yes_means: str | None = None,
        no_means: str | None = None,
        errors: Literal["raise"] = "raise",
        name: str | None = None,
        cancel: Cancel | None = None,
    ) -> KeptStream[T]: ...
    @overload
    def filter(
        self,
        question: str,
        records: Iterable[T],
        *,
        threshold: Threshold | None = None,
        field: str | None = None,
        yes_means: str | None = None,
        no_means: str | None = None,
        errors: Literal["coerce"],
        name: str | None = None,
        cancel: Cancel | None = None,
    ) -> KeptStream[T | FilterRow[T]]: ...
    def filter(
        self,
        question: Any,
        records: Any,
        *,
        threshold: Any = None,
        field: Any = None,
        yes_means: Any = None,
        no_means: Any = None,
        errors: Errors = "raise",
        name: str | None = None,
        cancel: Cancel | None = None,
    ) -> Any:
        """Stream the kept records, as passed in; not-sure records are left out.

        A failed record raises one ``RecordError`` after every kept record is
        delivered; ``errors="coerce"`` yields each failed ``FilterRow`` in its
        place instead. ``filter_rows`` keeps every row.
        """
        coerce = coerce_errors(errors)
        return KeptStream(
            self.filter_rows(
                question,
                records,
                threshold=threshold,
                field=field,
                yes_means=yes_means,
                no_means=no_means,
                name=name,
                cancel=cancel,
            ),
            coerce,
        )

    @overload
    def filter_rows(
        self,
        question: FilterQuestion,
        records: Iterable[T],
        *,
        name: str | None = None,
        cancel: Cancel | None = None,
    ) -> RowStream[FilterRow[T]]: ...
    @overload
    def filter_rows(
        self,
        question: str,
        records: Iterable[T],
        *,
        threshold: Threshold | None = None,
        field: str | None = None,
        yes_means: str | None = None,
        no_means: str | None = None,
        name: str | None = None,
        cancel: Cancel | None = None,
    ) -> RowStream[FilterRow[T]]: ...
    def filter_rows(
        self,
        question: Any,
        records: Any,
        *,
        threshold: Any = None,
        field: Any = None,
        yes_means: Any = None,
        no_means: Any = None,
        name: str | None = None,
        cancel: Cancel | None = None,
    ) -> Any:
        """Stream every filter row, kept or not, with its original ``item``."""
        return self._rows(
            "filter",
            question,
            records,
            threshold=threshold,
            field=field,
            yes_means=yes_means,
            no_means=no_means,
            name=name,
            cancel=cancel,
        )

    @overload
    def choose(
        self,
        question: ChooseQuestion[_Label],
        record: Record,
        *,
        name: str | None = None,
        cancel: Cancel | None = None,
    ) -> ChooseRow[_Label]: ...
    @overload
    def choose(
        self,
        question: str,
        record: Record,
        *,
        choices: TypeForm[_Label],
        threshold: float | None = None,
        field: str | None = None,
        descriptions: Descriptions[_Label] | None = None,
        name: str | None = None,
        cancel: Cancel | None = None,
    ) -> ChooseRow[_Label]: ...
    @overload
    def choose(
        self,
        question: str,
        record: Record,
        *,
        choices: list[_Label] | tuple[_Label, ...],
        threshold: float | None = None,
        field: str | None = None,
        descriptions: Descriptions[_Label] | None = None,
        name: str | None = None,
        cancel: Cancel | None = None,
    ) -> ChooseRow[_Label]: ...
    def choose(
        self,
        question: Any,
        record: Any,
        *,
        choices: Any = _MISSING,
        threshold: Any = None,
        field: Any = None,
        descriptions: Any = None,
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
            field=field,
            descriptions=descriptions,
            name=name,
            cancel=cancel,
        )

    @overload
    def choose_stream(
        self,
        question: ChooseQuestion[_Label],
        records: Iterable[Record],
        *,
        name: str | None = None,
        cancel: Cancel | None = None,
    ) -> RowStream[ChooseRow[_Label]]: ...
    @overload
    def choose_stream(
        self,
        question: str,
        records: Iterable[Record],
        *,
        choices: TypeForm[_Label],
        threshold: float | None = None,
        field: str | None = None,
        descriptions: Descriptions[_Label] | None = None,
        name: str | None = None,
        cancel: Cancel | None = None,
    ) -> RowStream[ChooseRow[_Label]]: ...
    @overload
    def choose_stream(
        self,
        question: str,
        records: Iterable[Record],
        *,
        choices: list[_Label] | tuple[_Label, ...],
        threshold: float | None = None,
        field: str | None = None,
        descriptions: Descriptions[_Label] | None = None,
        name: str | None = None,
        cancel: Cancel | None = None,
    ) -> RowStream[ChooseRow[_Label]]: ...
    def choose_stream(
        self,
        question: Any,
        records: Any,
        *,
        choices: Any = _MISSING,
        threshold: Any = None,
        field: Any = None,
        descriptions: Any = None,
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
            field=field,
            descriptions=descriptions,
            name=name,
            cancel=cancel,
        )

    @overload
    def tag(
        self,
        question: TagQuestion[_Label],
        record: Record,
        *,
        name: str | None = None,
        cancel: Cancel | None = None,
    ) -> TagRow[_Label]: ...
    @overload
    def tag(
        self,
        question: str,
        record: Record,
        *,
        labels: TypeForm[_Label],
        threshold: float | None = None,
        field: str | None = None,
        descriptions: Descriptions[_Label] | None = None,
        name: str | None = None,
        cancel: Cancel | None = None,
    ) -> TagRow[_Label]: ...
    @overload
    def tag(
        self,
        question: str,
        record: Record,
        *,
        labels: list[_Label] | tuple[_Label, ...],
        threshold: float | None = None,
        field: str | None = None,
        descriptions: Descriptions[_Label] | None = None,
        name: str | None = None,
        cancel: Cancel | None = None,
    ) -> TagRow[_Label]: ...
    def tag(
        self,
        question: Any,
        record: Any,
        *,
        labels: Any = _MISSING,
        threshold: Any = None,
        field: Any = None,
        descriptions: Any = None,
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
            field=field,
            descriptions=descriptions,
            name=name,
            cancel=cancel,
        )

    @overload
    def tag_stream(
        self,
        question: TagQuestion[_Label],
        records: Iterable[Record],
        *,
        name: str | None = None,
        cancel: Cancel | None = None,
    ) -> RowStream[TagRow[_Label]]: ...
    @overload
    def tag_stream(
        self,
        question: str,
        records: Iterable[Record],
        *,
        labels: TypeForm[_Label],
        threshold: float | None = None,
        field: str | None = None,
        descriptions: Descriptions[_Label] | None = None,
        name: str | None = None,
        cancel: Cancel | None = None,
    ) -> RowStream[TagRow[_Label]]: ...
    @overload
    def tag_stream(
        self,
        question: str,
        records: Iterable[Record],
        *,
        labels: list[_Label] | tuple[_Label, ...],
        threshold: float | None = None,
        field: str | None = None,
        descriptions: Descriptions[_Label] | None = None,
        name: str | None = None,
        cancel: Cancel | None = None,
    ) -> RowStream[TagRow[_Label]]: ...
    def tag_stream(
        self,
        question: Any,
        records: Any,
        *,
        labels: Any = _MISSING,
        threshold: Any = None,
        field: Any = None,
        descriptions: Any = None,
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
            field=field,
            descriptions=descriptions,
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
        levels: ChoiceInput[_Label],
        field: str | None = None,
        descriptions: Descriptions[_Label] | None = None,
        name: str | None = None,
        cancel: Cancel | None = None,
    ) -> ScoreRow: ...
    def score(
        self,
        question: Any,
        record: Any,
        *,
        levels: Any = _MISSING,
        field: Any = None,
        descriptions: Any = None,
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
            field=field,
            descriptions=descriptions,
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
        levels: ChoiceInput[_Label],
        field: str | None = None,
        descriptions: Descriptions[_Label] | None = None,
        name: str | None = None,
        cancel: Cancel | None = None,
    ) -> RowStream[ScoreRow]: ...
    def score_stream(
        self,
        question: Any,
        records: Any,
        *,
        levels: Any = _MISSING,
        field: Any = None,
        descriptions: Any = None,
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
            field=field,
            descriptions=descriptions,
            name=name,
            cancel=cancel,
        )

    @overload
    def rank(
        self,
        question: RankQuestion,
        records: Iterable[T],
        *,
        name: str | None = None,
        cancel: Cancel | None = None,
    ) -> RowStream[RankRow[T]]: ...
    @overload
    def rank(
        self,
        question: str,
        records: Iterable[T],
        *,
        levels: ChoiceInput[_Label],
        top: int | None = None,
        field: str | None = None,
        descriptions: Descriptions[_Label] | None = None,
        name: str | None = None,
        cancel: Cancel | None = None,
    ) -> RowStream[RankRow[T]]: ...
    def rank(
        self,
        question: Any,
        records: Any,
        *,
        levels: Any = _MISSING,
        top: Any = None,
        field: Any = None,
        descriptions: Any = None,
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
            field=field,
            descriptions=descriptions,
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
        field: str | None = None,
        name: str | None = None,
        cancel: Cancel | None = None,
    ) -> FindRow: ...
    def find(
        self,
        question: Any,
        record: Any,
        *,
        threshold: Any = None,
        field: Any = None,
        name: str | None = None,
        cancel: Cancel | None = None,
    ) -> Any:
        return self._single(
            "find",
            question,
            record,
            threshold=threshold,
            field=field,
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
        field: str | None = None,
        name: str | None = None,
        cancel: Cancel | None = None,
    ) -> RowStream[FindRow]: ...
    def find_stream(
        self,
        question: Any,
        records: Any,
        *,
        threshold: Any = None,
        field: Any = None,
        name: str | None = None,
        cancel: Cancel | None = None,
    ) -> Any:
        return self._rows(
            "find",
            question,
            records,
            threshold=threshold,
            field=field,
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
        field: str | None = None,
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
        field: str | None = None,
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
        field: str | None = None,
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
        field: str | None = None,
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
        toolbox: AnnotationInput,
        record: Record,
        *,
        field: str | None = None,
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
        toolbox: AnnotationInput,
        records: Iterable[Record],
        *,
        field: str | None = None,
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


def set_default_client(client: Client) -> None:
    """Use this client for module-level calls."""
    if not isinstance(client, Client):
        raise TypeError("client must be a Client")
    global _default
    with _default_lock:
        _default = client


def close_default_client() -> None:
    global _default
    with _default_lock:
        old, _default = _default, None
    if old is not None:
        old.close()


atexit.register(close_default_client)
__all__ = ["Client", "close_default_client", "set_default_client"]
