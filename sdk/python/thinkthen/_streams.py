"""Owned sync and async iteration over one native operation."""

from __future__ import annotations

import asyncio
import logging
from collections.abc import Callable, Iterator, Mapping
from concurrent.futures import ThreadPoolExecutor
from contextvars import copy_context
from dataclasses import replace
from enum import Enum
from types import TracebackType
from typing import TYPE_CHECKING, Any, Generic, Literal, TypeAlias, TypeVar, cast

if TYPE_CHECKING:
    import pandas as pd
    import polars as pl

from . import _native
from ._decode import row_from_json
from ._runtime import binding
from ._types import FilterRow, RecordError

R_co = TypeVar("R_co", covariant=True)
T = TypeVar("T")
T_co = TypeVar("T_co", covariant=True)
_END = object()
Errors: TypeAlias = Literal["raise", "coerce"]


class RowStream(Iterator[R_co], Generic[R_co]):
    def __init__(
        self,
        native: _native.NativeStream,
        failures: dict[int, str],
        remaining: int = 0,
        *,
        function: str,
        question: object,
        members: Mapping[str, Enum] | None = None,
        children: Mapping[str, Mapping[str, Enum]] | None = None,
    ) -> None:
        self._native = native
        self._failures = failures
        self._function = function
        self._question = question
        self._members = members
        self._children = children
        self._remaining = remaining
        self._closed = False

    def __iter__(self) -> RowStream[R_co]:
        return self

    def __next__(self) -> R_co:
        if self._closed:
            raise StopIteration
        try:
            raw = binding(self._native.receive)
            if raw is None:
                self.close()
                raise StopIteration
            self._remaining = max(0, self._remaining - 1)
            row = row_from_json(
                raw[0], raw[1], raw[3], self._members, self._children, raw[2]
            )
            cause = self._failures.pop(row.position, None)
            if cause is not None and row.error is not None:
                row = replace(
                    row,
                    error=replace(row.error, message=f"{row.error.message}: {cause}"),
                )
            if row.error is not None:
                logging.getLogger("thinkthen").debug(
                    "%s %s", row.function.value, row.error.code.value
                )
            return cast(R_co, row)
        except BaseException:
            self.close()
            raise

    def __length_hint__(self) -> int:
        return 0 if self._closed else self._remaining

    def cancel(self) -> None:
        self._native.cancel()

    def close(self) -> None:
        if not self._closed:
            self._closed = True
            self._native.close()
            self._failures.clear()

    def __enter__(self) -> RowStream[R_co]:
        return self

    def __exit__(
        self,
        exc_type: type[BaseException] | None,
        exc_value: BaseException | None,
        traceback: TracebackType | None,
    ) -> None:
        self.close()

    def __del__(self) -> None:
        if hasattr(self, "_native"):
            self._native.cancel()

    def to_list(self) -> list[R_co]:
        try:
            return list(self)
        finally:
            self.close()

    def to_pandas(self) -> pd.DataFrame:
        try:
            from .pandas import frame

            rows = self.to_list()
            return frame(rows, function=self._function)
        finally:
            self.close()

    def to_polars(self) -> pl.DataFrame:
        try:
            from .polars import series

            return series(self.to_list(), self._function, self._question).to_frame()
        finally:
            self.close()


def _next(stream: RowStream[R_co]) -> R_co | object:
    return next(stream, _END)


class AsyncRowStream(Generic[R_co]):
    def __init__(
        self,
        create: Callable[[], RowStream[R_co]],
        cancel_pending: Callable[[], None],
        *,
        function: str,
        question: object,
    ) -> None:
        self._create = create
        self._function = function
        self._question = question
        self._cancel_pending = cancel_pending
        self._stream: RowStream[R_co] | None = None
        self._closed = False
        self._pending: asyncio.Future[Any] | None = None
        self._executor: ThreadPoolExecutor | None = None

    def __aiter__(self) -> AsyncRowStream[R_co]:
        return self

    async def __anext__(self) -> R_co:
        if self._closed:
            raise StopAsyncIteration
        try:
            if self._stream is None:
                self._executor = ThreadPoolExecutor(max_workers=1)
                self._pending = asyncio.get_running_loop().run_in_executor(
                    self._executor, copy_context().run, self._create
                )
                self._stream = await asyncio.shield(self._pending)
            stream = self._stream
            assert stream is not None
            self._pending = asyncio.get_running_loop().run_in_executor(
                self._executor, copy_context().run, _next, stream
            )
            value = await asyncio.shield(self._pending)
            if value is _END:
                await self.aclose()
                raise StopAsyncIteration
            return cast(R_co, value)
        except BaseException:
            await self.aclose()
            raise

    def cancel(self) -> None:
        self._cancel_pending()
        if self._stream is not None:
            self._stream.cancel()

    async def aclose(self) -> None:
        if self._closed:
            return
        self._closed = True
        self.cancel()
        if self._pending is not None:
            try:
                result = await asyncio.shield(self._pending)
                if self._stream is None and isinstance(result, RowStream):
                    self._stream = result
            except BaseException:  # noqa: BLE001, S110 - preserve caller failure during cleanup.
                pass
        if self._executor is not None:
            try:
                if self._stream is not None:
                    await asyncio.get_running_loop().run_in_executor(
                        self._executor, copy_context().run, self._stream.close
                    )
            finally:
                self._executor.shutdown(wait=True)

    async def __aenter__(self) -> AsyncRowStream[R_co]:
        return self

    async def __aexit__(
        self,
        exc_type: type[BaseException] | None,
        exc_value: BaseException | None,
        traceback: TracebackType | None,
    ) -> None:
        await self.aclose()

    def __del__(self) -> None:
        if hasattr(self, "_cancel_pending"):
            self.cancel()

    async def to_list(self) -> list[R_co]:
        try:
            return [row async for row in self]
        finally:
            await self.aclose()

    async def to_pandas(self) -> pd.DataFrame:
        try:
            from .pandas import frame

            return frame(await self.to_list(), function=self._function)
        finally:
            await self.aclose()

    async def to_polars(self) -> pl.DataFrame:
        try:
            from .polars import series

            return series(
                await self.to_list(), self._function, self._question
            ).to_frame()
        finally:
            await self.aclose()


def coerce_errors(errors: object) -> bool:
    """Admit the failure policy shared with the data frame methods."""
    if errors not in {"raise", "coerce"}:
        raise ValueError('errors must be "raise" or "coerce"')
    return errors == "coerce"


def _filter_failure(rows: list[FilterRow[Any]]) -> RecordError:
    first = rows[0]
    assert first.error is not None
    count = f"{len(rows)} record{'' if len(rows) == 1 else 's'} failed"
    return RecordError(
        first.error.code,
        f"{count}; the first at position {first.position}: {first.error.message}",
        first.error.http_status,
        tuple(rows),
    )


class KeptStream(Iterator[T_co], Generic[T_co]):
    """The kept records of one filter call, as the caller passed them.

    Not-sure records are left out. Under ``errors="raise"`` every kept record is
    delivered first, then one ``RecordError`` lists the failed rows in ``rows``.
    Under ``errors="coerce"`` each failed ``FilterRow`` is yielded in its place.
    Use ``filter_rows`` for every row.
    """

    def __init__(self, rows: RowStream[FilterRow[Any]], coerce: bool) -> None:
        self._rows = rows
        self._coerce = coerce
        self._failed: list[FilterRow[Any]] = []

    def __iter__(self) -> KeptStream[T_co]:
        return self

    def __next__(self) -> T_co:
        for row in self._rows:
            if row.kept:
                return cast(T_co, row.item)
            if row.failed:
                if self._coerce:
                    return cast(T_co, row)
                self._failed.append(row)
        if self._failed:
            failed, self._failed = self._failed, []
            raise _filter_failure(failed)
        raise StopIteration

    def cancel(self) -> None:
        self._rows.cancel()

    def to_list(self) -> list[T_co]:
        """Collect the remaining kept records and close the stream."""
        try:
            return list(self)
        finally:
            self.close()

    def close(self) -> None:
        self._rows.close()
        self._failed.clear()

    def __enter__(self) -> KeptStream[T_co]:
        return self

    def __exit__(
        self,
        exc_type: type[BaseException] | None,
        exc_value: BaseException | None,
        traceback: TracebackType | None,
    ) -> None:
        self.close()


class AsyncKeptStream(Generic[T_co]):
    """The kept records of one async filter call; see ``KeptStream``."""

    def __init__(self, rows: AsyncRowStream[FilterRow[Any]], coerce: bool) -> None:
        self._rows = rows
        self._coerce = coerce
        self._failed: list[FilterRow[Any]] = []

    def __aiter__(self) -> AsyncKeptStream[T_co]:
        return self

    async def __anext__(self) -> T_co:
        async for row in self._rows:
            if row.kept:
                return cast(T_co, row.item)
            if row.failed:
                if self._coerce:
                    return cast(T_co, row)
                self._failed.append(row)
        if self._failed:
            failed, self._failed = self._failed, []
            raise _filter_failure(failed)
        raise StopAsyncIteration

    def cancel(self) -> None:
        self._rows.cancel()

    async def to_list(self) -> list[T_co]:
        """Collect the remaining kept records and close the stream."""
        try:
            return [record async for record in self]
        finally:
            await self.aclose()

    async def aclose(self) -> None:
        self._failed.clear()
        await self._rows.aclose()

    async def __aenter__(self) -> AsyncKeptStream[T_co]:
        return self

    async def __aexit__(
        self,
        exc_type: type[BaseException] | None,
        exc_value: BaseException | None,
        traceback: TracebackType | None,
    ) -> None:
        await self.aclose()


__all__ = ["AsyncKeptStream", "AsyncRowStream", "KeptStream", "RowStream"]
