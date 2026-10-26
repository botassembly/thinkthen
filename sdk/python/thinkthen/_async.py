"""Async ownership around the ordinary native client."""

from __future__ import annotations

import asyncio
from collections.abc import AsyncIterable, Iterable
from concurrent.futures import CancelledError as InputCancelled
from threading import Event
from typing import Any, TypeVar, overload
from weakref import WeakSet

from . import _native
from ._client import Client
from ._questions import (
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
)
from ._runtime import Cancel, binding
from ._streams import AsyncRowStream
from ._types import (
    AnnotateRow,
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


class AsyncClient:
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
        self._client = Client(
            backend=backend,
            url=url,
            model=model,
            api_key=api_key,
            batch=batch,
            retry=retry,
        )
        self._active: WeakSet[AsyncRowStream[Any]] = WeakSet()

    @property
    def settings(self) -> ClientSettings:
        return self._client.settings

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
    ) -> AsyncClient:
        return cls(
            backend=backend,
            url=url,
            model=model,
            api_key=api_key,
            batch=batch,
            retry=retry,
        )

    def __repr__(self) -> str:
        return f"AsyncClient({self._client!r})"

    def __reduce__(self) -> tuple[Any, ...]:
        raise TypeError("operational clients cannot be pickled")

    async def __aenter__(self) -> AsyncClient:  # noqa: PYI034 - Python 3.10.
        return self

    async def __aexit__(self, *exception: object) -> None:
        await self.aclose()

    async def aclose(self) -> None:
        active = list(self._active)
        for stream in active:
            stream.cancel()
        for stream in active:
            await stream.aclose()
        await asyncio.to_thread(self._client.close)

    def _rows(
        self,
        function: str,
        question: object,
        records: Iterable[Any] | AsyncIterable[Any],
        *,
        name: str | None = None,
        cancel: Cancel | None = None,
        **options: object,
    ) -> AsyncRowStream[Any]:
        loop = asyncio.get_running_loop()
        handle = cancel._child() if cancel is not None else Cancel()
        stopped = Event()
        pending_input: list[Any] = [None]
        if isinstance(records, AsyncIterable):
            iterator = aiter(records)

            async def pull_one() -> Any:
                return await anext(iterator)

            def pull() -> Iterable[Any]:
                while not stopped.is_set():
                    future = asyncio.run_coroutine_threadsafe(pull_one(), loop)
                    pending_input[0] = future
                    if stopped.is_set():
                        future.cancel()
                    try:
                        yield future.result()
                    except StopAsyncIteration:
                        return
                    except InputCancelled:
                        if stopped.is_set():
                            binding(_native.reject_cancelled)
                        raise

            inputs: Iterable[Any] = pull()
        else:
            inputs = records

        def stop() -> None:
            stopped.set()
            future = pending_input[0]
            if future is not None:
                future.cancel()
            handle.cancel()

        stream: AsyncRowStream[Any] = AsyncRowStream(
            lambda: self._client._rows(
                function, question, inputs, name=name, cancel=handle, **options
            ),
            stop,
            function=function,
            question=question,
        )
        self._active.add(stream)
        if cancel is not None:
            cancel._watch(stream.cancel)
        return stream

    async def _single(
        self, function: str, question: object, record: Record, **options: Any
    ) -> Any:
        async with self._rows(function, question, (record,), **options) as rows:
            row = await anext(rows)
            async for _ in rows:
                pass
            return row

    def _arguments(
        self,
        function: str,
        collection: bool,
        args: tuple[Any, ...],
        options: dict[str, Any],
    ) -> tuple[Any, Any]:
        key = "records" if collection else "record"
        if function in {"recognize", "relate"} and len(args) == 1:
            return None, args[0]
        question = (
            args[0] if args else options.pop("question", options.pop("toolbox", None))
        )
        if len(args) < 2 and key not in options:
            raise TypeError(f"{key} is required")
        records = args[1] if len(args) > 1 else options.pop(key)
        return question, records

    @overload
    async def decide(
        self,
        question: DecideQuestion,
        record: Record,
        *,
        name: str | None = None,
        cancel: Cancel | None = None,
    ) -> DecideRow: ...
    @overload
    async def decide(
        self,
        question: str,
        record: Record,
        *,
        threshold: Threshold | None = None,
        evidence: str | None = None,
        name: str | None = None,
        cancel: Cancel | None = None,
    ) -> DecideRow: ...
    async def decide(self, *args: Any, **kwargs: Any) -> Any:
        question, records = self._arguments("decide", False, args, kwargs)
        return await self._single("decide", question, records, **kwargs)

    @overload
    def decide_stream(
        self,
        question: DecideQuestion,
        records: Iterable[Record] | AsyncIterable[Record],
        *,
        name: str | None = None,
        cancel: Cancel | None = None,
    ) -> AsyncRowStream[DecideRow]: ...
    @overload
    def decide_stream(
        self,
        question: str,
        records: Iterable[Record] | AsyncIterable[Record],
        *,
        threshold: Threshold | None = None,
        evidence: str | None = None,
        name: str | None = None,
        cancel: Cancel | None = None,
    ) -> AsyncRowStream[DecideRow]: ...
    def decide_stream(self, *args: Any, **kwargs: Any) -> Any:
        question, records = self._arguments("decide", True, args, kwargs)
        return self._rows("decide", question, records, **kwargs)

    @overload
    def filter(
        self,
        question: FilterQuestion,
        records: Iterable[T] | AsyncIterable[T],
        *,
        name: str | None = None,
        cancel: Cancel | None = None,
    ) -> AsyncRowStream[FilterRow[T]]: ...
    @overload
    def filter(
        self,
        question: str,
        records: Iterable[T] | AsyncIterable[T],
        *,
        threshold: Threshold | None = None,
        evidence: str | None = None,
        name: str | None = None,
        cancel: Cancel | None = None,
    ) -> AsyncRowStream[FilterRow[T]]: ...
    def filter(self, *args: Any, **kwargs: Any) -> Any:
        question, records = self._arguments("filter", True, args, kwargs)
        return self._rows("filter", question, records, **kwargs)

    @overload
    async def choose(
        self,
        question: ChooseQuestion,
        record: Record,
        *,
        name: str | None = None,
        cancel: Cancel | None = None,
    ) -> ChooseRow: ...
    @overload
    async def choose(
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
    async def choose(self, *args: Any, **kwargs: Any) -> Any:
        question, records = self._arguments("choose", False, args, kwargs)
        return await self._single("choose", question, records, **kwargs)

    @overload
    def choose_stream(
        self,
        question: ChooseQuestion,
        records: Iterable[Record] | AsyncIterable[Record],
        *,
        name: str | None = None,
        cancel: Cancel | None = None,
    ) -> AsyncRowStream[ChooseRow]: ...
    @overload
    def choose_stream(
        self,
        question: str,
        records: Iterable[Record] | AsyncIterable[Record],
        *,
        choices: Strings,
        threshold: Threshold | None = None,
        evidence: str | None = None,
        name: str | None = None,
        cancel: Cancel | None = None,
    ) -> AsyncRowStream[ChooseRow]: ...
    def choose_stream(self, *args: Any, **kwargs: Any) -> Any:
        question, records = self._arguments("choose", True, args, kwargs)
        return self._rows("choose", question, records, **kwargs)

    @overload
    async def tag(
        self,
        question: TagQuestion,
        record: Record,
        *,
        name: str | None = None,
        cancel: Cancel | None = None,
    ) -> TagRow: ...
    @overload
    async def tag(
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
    async def tag(self, *args: Any, **kwargs: Any) -> Any:
        question, records = self._arguments("tag", False, args, kwargs)
        return await self._single("tag", question, records, **kwargs)

    @overload
    def tag_stream(
        self,
        question: TagQuestion,
        records: Iterable[Record] | AsyncIterable[Record],
        *,
        name: str | None = None,
        cancel: Cancel | None = None,
    ) -> AsyncRowStream[TagRow]: ...
    @overload
    def tag_stream(
        self,
        question: str,
        records: Iterable[Record] | AsyncIterable[Record],
        *,
        labels: Strings,
        threshold: float | None = None,
        evidence: str | None = None,
        name: str | None = None,
        cancel: Cancel | None = None,
    ) -> AsyncRowStream[TagRow]: ...
    def tag_stream(self, *args: Any, **kwargs: Any) -> Any:
        question, records = self._arguments("tag", True, args, kwargs)
        return self._rows("tag", question, records, **kwargs)

    @overload
    async def score(
        self,
        question: ScoreQuestion,
        record: Record,
        *,
        name: str | None = None,
        cancel: Cancel | None = None,
    ) -> ScoreRow: ...
    @overload
    async def score(
        self,
        question: str,
        record: Record,
        *,
        levels: Strings,
        evidence: str | None = None,
        name: str | None = None,
        cancel: Cancel | None = None,
    ) -> ScoreRow: ...
    async def score(self, *args: Any, **kwargs: Any) -> Any:
        question, records = self._arguments("score", False, args, kwargs)
        return await self._single("score", question, records, **kwargs)

    @overload
    def score_stream(
        self,
        question: ScoreQuestion,
        records: Iterable[Record] | AsyncIterable[Record],
        *,
        name: str | None = None,
        cancel: Cancel | None = None,
    ) -> AsyncRowStream[ScoreRow]: ...
    @overload
    def score_stream(
        self,
        question: str,
        records: Iterable[Record] | AsyncIterable[Record],
        *,
        levels: Strings,
        evidence: str | None = None,
        name: str | None = None,
        cancel: Cancel | None = None,
    ) -> AsyncRowStream[ScoreRow]: ...
    def score_stream(self, *args: Any, **kwargs: Any) -> Any:
        question, records = self._arguments("score", True, args, kwargs)
        return self._rows("score", question, records, **kwargs)

    @overload
    def rank(
        self,
        question: RankQuestion,
        records: Iterable[Record] | AsyncIterable[Record],
        *,
        name: str | None = None,
        cancel: Cancel | None = None,
    ) -> AsyncRowStream[RankRow]: ...
    @overload
    def rank(
        self,
        question: str,
        records: Iterable[Record] | AsyncIterable[Record],
        *,
        levels: Strings,
        top: int | None = None,
        evidence: str | None = None,
        name: str | None = None,
        cancel: Cancel | None = None,
    ) -> AsyncRowStream[RankRow]: ...
    def rank(self, *args: Any, **kwargs: Any) -> Any:
        question, records = self._arguments("rank", True, args, kwargs)
        return self._rows("rank", question, records, **kwargs)

    @overload
    async def find(
        self,
        question: FindQuestion,
        record: Record,
        *,
        name: str | None = None,
        cancel: Cancel | None = None,
    ) -> FindRow: ...
    @overload
    async def find(
        self,
        question: str,
        record: Record,
        *,
        threshold: float | None = None,
        evidence: str | None = None,
        name: str | None = None,
        cancel: Cancel | None = None,
    ) -> FindRow: ...
    async def find(self, *args: Any, **kwargs: Any) -> Any:
        question, records = self._arguments("find", False, args, kwargs)
        return await self._single("find", question, records, **kwargs)

    @overload
    def find_stream(
        self,
        question: FindQuestion,
        records: Iterable[Record] | AsyncIterable[Record],
        *,
        name: str | None = None,
        cancel: Cancel | None = None,
    ) -> AsyncRowStream[FindRow]: ...
    @overload
    def find_stream(
        self,
        question: str,
        records: Iterable[Record] | AsyncIterable[Record],
        *,
        threshold: float | None = None,
        evidence: str | None = None,
        name: str | None = None,
        cancel: Cancel | None = None,
    ) -> AsyncRowStream[FindRow]: ...
    def find_stream(self, *args: Any, **kwargs: Any) -> Any:
        question, records = self._arguments("find", True, args, kwargs)
        return self._rows("find", question, records, **kwargs)

    @overload
    async def recognize(
        self,
        question: RecognizeQuestion,
        record: Record,
        *,
        name: str | None = None,
        cancel: Cancel | None = None,
    ) -> RecognizeRow: ...
    @overload
    async def recognize(
        self,
        record: Record,
        *,
        kinds: Strings,
        threshold: float | None = None,
        evidence: str | None = None,
        name: str | None = None,
        cancel: Cancel | None = None,
    ) -> RecognizeRow: ...
    async def recognize(self, *args: Any, **kwargs: Any) -> Any:
        question, records = self._arguments("recognize", False, args, kwargs)
        return await self._single("recognize", question, records, **kwargs)

    @overload
    def recognize_stream(
        self,
        question: RecognizeQuestion,
        records: Iterable[Record] | AsyncIterable[Record],
        *,
        name: str | None = None,
        cancel: Cancel | None = None,
    ) -> AsyncRowStream[RecognizeRow]: ...
    @overload
    def recognize_stream(
        self,
        records: Iterable[Record] | AsyncIterable[Record],
        *,
        kinds: Strings,
        threshold: float | None = None,
        evidence: str | None = None,
        name: str | None = None,
        cancel: Cancel | None = None,
    ) -> AsyncRowStream[RecognizeRow]: ...
    def recognize_stream(self, *args: Any, **kwargs: Any) -> Any:
        question, records = self._arguments("recognize", True, args, kwargs)
        return self._rows("recognize", question, records, **kwargs)

    @overload
    async def relate(
        self,
        question: RelateQuestion,
        record: Record,
        *,
        name: str | None = None,
        cancel: Cancel | None = None,
    ) -> RelateRow: ...
    @overload
    async def relate(
        self,
        record: Record,
        *,
        rules: list[RelationRule] | tuple[RelationRule, ...],
        threshold: float | None = None,
        evidence: str | None = None,
        name: str | None = None,
        cancel: Cancel | None = None,
    ) -> RelateRow: ...
    async def relate(self, *args: Any, **kwargs: Any) -> Any:
        question, records = self._arguments("relate", False, args, kwargs)
        return await self._single("relate", question, records, **kwargs)

    @overload
    def relate_stream(
        self,
        question: RelateQuestion,
        records: Iterable[Record] | AsyncIterable[Record],
        *,
        name: str | None = None,
        cancel: Cancel | None = None,
    ) -> AsyncRowStream[RelateRow]: ...
    @overload
    def relate_stream(
        self,
        records: Iterable[Record] | AsyncIterable[Record],
        *,
        rules: list[RelationRule] | tuple[RelationRule, ...],
        threshold: float | None = None,
        evidence: str | None = None,
        name: str | None = None,
        cancel: Cancel | None = None,
    ) -> AsyncRowStream[RelateRow]: ...
    def relate_stream(self, *args: Any, **kwargs: Any) -> Any:
        question, records = self._arguments("relate", True, args, kwargs)
        return self._rows("relate", question, records, **kwargs)

    @overload
    async def annotate(
        self,
        question: AnnotateQuestion,
        record: Record,
        *,
        name: str | None = None,
        cancel: Cancel | None = None,
    ) -> AnnotateRow: ...
    @overload
    async def annotate(
        self,
        toolbox: AnnotationQuestions,
        record: Record,
        *,
        evidence: str | None = None,
        name: str | None = None,
        cancel: Cancel | None = None,
    ) -> AnnotateRow: ...
    async def annotate(self, *args: Any, **kwargs: Any) -> Any:
        question, records = self._arguments("annotate", False, args, kwargs)
        return await self._single("annotate", question, records, **kwargs)

    @overload
    def annotate_stream(
        self,
        question: AnnotateQuestion,
        records: Iterable[Record] | AsyncIterable[Record],
        *,
        name: str | None = None,
        cancel: Cancel | None = None,
    ) -> AsyncRowStream[AnnotateRow]: ...
    @overload
    def annotate_stream(
        self,
        toolbox: AnnotationQuestions,
        records: Iterable[Record] | AsyncIterable[Record],
        *,
        evidence: str | None = None,
        name: str | None = None,
        cancel: Cancel | None = None,
    ) -> AsyncRowStream[AnnotateRow]: ...
    def annotate_stream(self, *args: Any, **kwargs: Any) -> Any:
        question, records = self._arguments("annotate", True, args, kwargs)
        return self._rows("annotate", question, records, **kwargs)


__all__ = ["AsyncClient"]
