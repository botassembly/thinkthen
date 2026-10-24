"""Named Python calls that translate once into shared engine requests."""

from __future__ import annotations

from collections.abc import Iterable, Iterator, Sequence
from threading import Lock
from typing import Literal, cast

from . import _native
from ._relation import Relation
from ._runtime import (
    SKIP,
    Cancel,
    Question,
    Raw,
    Stream,
    Toolbox,
    binding,
    encode,
    row_from_json,
    source,
)
from ._types import (
    Adapter,
    Found,
    JSONValue,
    Name,
    Record,
    RecordT,
    RelationRule,
    Row,
    Threshold,
)


class Client:
    """One engine whose backend settings and connections survive each call."""

    def __init__(
        self,
        *,
        adapter: Adapter = "systemone",
        mode: Literal["embedded", "proxy"] = "embedded",
        key_source: str | None = None,
        url: str | None = None,
        model: str | None = None,
        batch: int | None = None,
    ) -> None:
        settings = encode(
            {
                "adapter": adapter,
                "mode": mode,
                "key_source": key_source,
                "url": url,
                "model": model,
                "batch": batch,
            }
        )
        self._native = binding(lambda: _native.NativeEngine(settings))

    def _start(
        self,
        function: str,
        question: object,
        records: Iterable[Record],
        *,
        cancel: Cancel | None = None,
        loaded: Question[object] | None = None,
        toolbox: Toolbox | None = None,
        keep_originals: bool,
        **options: object,
    ) -> _native.NativeStream:
        if loaded is None and isinstance(question, Question):
            loaded, question = question, None
        options = {key: value for key, value in options.items() if value is not None}
        handle = cancel if cancel is not None else Cancel()
        request = encode(
            {
                "function": function,
                "question": {"type": "inline", "value": question},
                "options": options,
            }
        )
        encoded_options = encode(options)
        return binding(
            lambda: self._native.start(
                function,
                request,
                encoded_options,
                source(records),
                handle._native,
                keep_originals,
                loaded._native if loaded is not None else None,
                toolbox._native if toolbox is not None else None,
            )
        )

    def _single(
        self,
        function: str,
        question: object,
        text: Record,
        *,
        cancel: Cancel | None = None,
        loaded: Question[object] | None = None,
        toolbox: Toolbox | None = None,
        **options: object,
    ) -> Row[JSONValue]:
        native = self._start(
            function,
            question,
            (text,),
            keep_originals=False,
            cancel=cancel,
            loaded=loaded,
            toolbox=toolbox,
            **options,
        )
        try:
            received = cast(Iterator[Raw], iter(lambda: binding(native.receive), None))
            raw = next(received)
            row = row_from_json(raw[0], raw[1])
            for _ in received:
                pass
            return row
        finally:
            native.close()

    def _rows(
        self,
        function: str,
        question: object,
        records: Iterable[Record],
        *,
        cancel: Cancel | None = None,
        loaded: Question[object] | None = None,
        toolbox: Toolbox | None = None,
        **options: object,
    ) -> Stream[Row[JSONValue]]:
        native = self._start(
            function,
            question,
            records,
            keep_originals=False,
            cancel=cancel,
            loaded=loaded,
            toolbox=toolbox,
            **options,
        )
        return Stream(native, lambda raw: row_from_json(raw[0], raw[1]))

    def decide(
        self,
        question: str | Question[object],
        text: Record,
        *,
        threshold: Threshold | None = None,
        on: str | None = None,
        cancel: Cancel | None = None,
    ) -> Row[bool]:
        return cast(
            Row[bool],
            self._single(
                "decide", question, text, threshold=threshold, on=on, cancel=cancel
            ),
        )

    def choose(
        self,
        question: str | Question[object],
        text: Record,
        *,
        options: Sequence[str] | None = None,
        threshold: Threshold | None = None,
        on: str | None = None,
        cancel: Cancel | None = None,
    ) -> Row[str]:
        return cast(
            Row[str],
            self._single(
                "choose",
                question,
                text,
                options=options,
                threshold=threshold,
                on=on,
                cancel=cancel,
            ),
        )

    def tag(
        self,
        question: str | Question[object],
        text: Record,
        *,
        labels: Sequence[str] | None = None,
        threshold: Threshold | None = None,
        on: str | None = None,
        cancel: Cancel | None = None,
    ) -> Row[list[str]]:
        return cast(
            Row[list[str]],
            self._single(
                "tag",
                question,
                text,
                labels=labels,
                threshold=threshold,
                on=on,
                cancel=cancel,
            ),
        )

    def score(
        self,
        question: str | Question[object],
        text: Record,
        *,
        levels: Sequence[str] | None = None,
        on: str | None = None,
        cancel: Cancel | None = None,
    ) -> Row[float]:
        return cast(
            Row[float],
            self._single("score", question, text, levels=levels, on=on, cancel=cancel),
        )

    def filter(
        self,
        question: str | Question[object],
        records: Iterable[RecordT],
        *,
        threshold: Threshold | None = None,
        on: str | None = None,
        cancel: Cancel | None = None,
    ) -> Stream[RecordT]:
        native = self._start(
            "filter",
            question,
            records,
            keep_originals=True,
            threshold=threshold,
            on=on,
            cancel=cancel,
        )
        return Stream(native, lambda raw: cast(RecordT, raw[3]) if raw[2] else SKIP)

    def rank(
        self,
        question: str | Question[JSONValue],
        records: Iterable[Record],
        *,
        levels: Sequence[str] | None = None,
        top: int | None = None,
        on: str | None = None,
        cancel: Cancel | None = None,
    ) -> Stream[Row[int]]:
        loaded = question if isinstance(question, Question) else None
        text = question if isinstance(question, str) else None
        return cast(
            Stream[Row[int]],
            self._rows(
                "rank",
                text,
                records,
                levels=levels,
                top=top,
                on=on,
                cancel=cancel,
                loaded=loaded,
            ),
        )

    def find(
        self,
        question: str | Question[object],
        text: Record,
        *,
        threshold: Threshold | None = None,
        on: str | None = None,
        cancel: Cancel | None = None,
    ) -> Row[Found]:
        return cast(
            Row[Found],
            self._single(
                "find", question, text, threshold=threshold, on=on, cancel=cancel
            ),
        )

    def annotate(
        self,
        toolbox: Toolbox | dict[str, JSONValue],
        text: Record,
        *,
        on: str | None = None,
        cancel: Cancel | None = None,
    ) -> Row[dict[str, JSONValue]]:
        loaded = toolbox if isinstance(toolbox, Toolbox) else None
        return cast(
            Row[dict[str, JSONValue]],
            self._single(
                "annotate",
                None if loaded is not None else toolbox,
                text,
                on=on,
                cancel=cancel,
                toolbox=loaded,
            ),
        )

    def recognize(
        self,
        text: Record,
        *,
        kinds: Sequence[str] | Question[object] = (),
        threshold: Threshold | None = None,
        on: str | None = None,
        cancel: Cancel | None = None,
    ) -> Row[list[Name]]:
        return cast(
            Row[list[Name]],
            self._single(
                "recognize", kinds, text, threshold=threshold, on=on, cancel=cancel
            ),
        )

    def relate(
        self,
        text: Record,
        *,
        rules: Sequence[RelationRule] | Question[object],
        threshold: Threshold | None = None,
        on: str | None = None,
        cancel: Cancel | None = None,
    ) -> Row[list[Relation]]:
        return cast(
            Row[list[Relation]],
            self._single(
                "relate", rules, text, threshold=threshold, on=on, cancel=cancel
            ),
        )

    def decide_stream(
        self,
        question: str | Question[object],
        records: Iterable[Record],
        *,
        threshold: Threshold | None = None,
        on: str | None = None,
        cancel: Cancel | None = None,
    ) -> Stream[Row[bool]]:
        return cast(
            Stream[Row[bool]],
            self._rows(
                "decide", question, records, threshold=threshold, on=on, cancel=cancel
            ),
        )

    def choose_stream(
        self,
        question: str | Question[object],
        records: Iterable[Record],
        *,
        options: Sequence[str] | None = None,
        threshold: Threshold | None = None,
        on: str | None = None,
        cancel: Cancel | None = None,
    ) -> Stream[Row[str]]:
        return cast(
            Stream[Row[str]],
            self._rows(
                "choose",
                question,
                records,
                options=options,
                threshold=threshold,
                on=on,
                cancel=cancel,
            ),
        )

    def tag_stream(
        self,
        question: str | Question[object],
        records: Iterable[Record],
        *,
        labels: Sequence[str] | None = None,
        threshold: Threshold | None = None,
        on: str | None = None,
        cancel: Cancel | None = None,
    ) -> Stream[Row[list[str]]]:
        return cast(
            Stream[Row[list[str]]],
            self._rows(
                "tag",
                question,
                records,
                labels=labels,
                threshold=threshold,
                on=on,
                cancel=cancel,
            ),
        )

    def score_stream(
        self,
        question: str | Question[object],
        records: Iterable[Record],
        *,
        levels: Sequence[str] | None = None,
        on: str | None = None,
        cancel: Cancel | None = None,
    ) -> Stream[Row[float]]:
        return cast(
            Stream[Row[float]],
            self._rows("score", question, records, levels=levels, on=on, cancel=cancel),
        )

    def find_stream(
        self,
        question: str | Question[object],
        records: Iterable[Record],
        *,
        threshold: Threshold | None = None,
        on: str | None = None,
        cancel: Cancel | None = None,
    ) -> Stream[Row[Found]]:
        return cast(
            Stream[Row[Found]],
            self._rows(
                "find", question, records, threshold=threshold, on=on, cancel=cancel
            ),
        )

    def annotate_stream(
        self,
        toolbox: Toolbox | dict[str, JSONValue],
        records: Iterable[Record],
        *,
        on: str | None = None,
        cancel: Cancel | None = None,
    ) -> Stream[Row[dict[str, JSONValue]]]:
        loaded = toolbox if isinstance(toolbox, Toolbox) else None
        return cast(
            Stream[Row[dict[str, JSONValue]]],
            self._rows(
                "annotate",
                None if loaded is not None else toolbox,
                records,
                on=on,
                cancel=cancel,
                toolbox=loaded,
            ),
        )

    def recognize_stream(
        self,
        records: Iterable[Record],
        *,
        kinds: Sequence[str] | Question[object] = (),
        threshold: Threshold | None = None,
        on: str | None = None,
        cancel: Cancel | None = None,
    ) -> Stream[Row[list[Name]]]:
        return cast(
            Stream[Row[list[Name]]],
            self._rows(
                "recognize", kinds, records, threshold=threshold, on=on, cancel=cancel
            ),
        )

    def relate_stream(
        self,
        records: Iterable[Record],
        *,
        rules: Sequence[RelationRule] | Question[object],
        threshold: Threshold | None = None,
        on: str | None = None,
        cancel: Cancel | None = None,
    ) -> Stream[Row[list[Relation]]]:
        return cast(
            Stream[Row[list[Relation]]],
            self._rows(
                "relate", rules, records, threshold=threshold, on=on, cancel=cancel
            ),
        )


_default: Client | None = None
_default_lock = Lock()


def default_client() -> Client:
    global _default
    with _default_lock:
        if _default is None:
            _default = Client()
        return _default
