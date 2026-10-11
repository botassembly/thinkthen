"""Host naming, conversion and scheduling over the canonical native session."""
from __future__ import annotations
import asyncio
from collections.abc import Iterator, Mapping
from dataclasses import dataclass
import os
import sys
import time
from typing import Generic, TypeVar
from . import _thinkthen as native
from . import _inputs as c
from .files import FileSelection

T = TypeVar('T')

VERBS = ('decide', 'choose', 'tag', 'score', 'filter', 'rank', 'find', 'annotate', 'recognize', 'relate')


def _json(value):
    if isinstance(value, native._NativeResult):
        return value.to_dict()
    if isinstance(value, Mapping):
        return {key: _json(item) for key, item in value.items()}
    if isinstance(value, (tuple, list)):
        return [_json(item) for item in value]
    return value


def _question(verb, question, controls):
    from ._inputs import QuestionSource
    if verb == 'annotate' and isinstance(question, type):
        from .pydantic import question_set
        question = question_set(question)
    if isinstance(question, QuestionSource):
        if question.none: controls['none'] = True
        return 'selector', question
    if question is None and verb in ('recognize', 'relate'):
        from . import _rules
        plan = {}
        if verb == 'recognize':
            from ._labels import normalize
            kinds = normalize(controls.pop('kinds', []), controls.pop('descriptions', None))
            plan['kinds'] = kinds if isinstance(kinds, dict) else {name: None for name in kinds}
            for key in ('instructions', 'entity_definition'):
                if key in controls: plan[key] = controls.pop(key)
        rules = _rules(controls.pop('relations', None), controls.pop('either', None))
        if rules or verb == 'relate':
            plan['relations'] = [dict(name=n, source=s, target=t, either=e) for n,s,t,e in rules]
        return 'definition', {'version': 1, verb: plan}
    if isinstance(question, os.PathLike):
        return 'file', os.fspath(question)
    if isinstance(question, Mapping):
        return 'definition', _json(question)
    if verb in ('annotate', 'recognize', 'relate'):
        return 'file', question
    if not any(key in controls for key in ('options', 'labels', 'levels', 'true', 'false')):
        return 'text', question
    body = {'decide' if verb in ('filter', 'rank') else verb: question}
    from ._labels import normalize
    for key in ('options', 'labels', 'levels', 'true', 'false'):
        if key in controls:
            value = controls.pop(key)
            body[key] = normalize(value, bare_level_names=key == 'levels') if key in ('options', 'labels', 'levels') else value
    return 'definition', body


def _source(verb, value):
    if isinstance(value, (FileSelection, c.Files)):
        return value, None, False
    if isinstance(value, c.Records): value = value.items
    if isinstance(value, Iterator): return None, value, False
    return value, None, not isinstance(value, (list, tuple))


@dataclass(frozen=True, repr=False)
class Result(Generic[T]):
    """Owned typed results and actual final native settlement."""
    results: tuple
    terminal: object
    function: str
    scalar: bool
    detailed: bool = False
    @property
    def value(self):
        if self.detailed: return self.results[0] if self.scalar else list(self.results)
        return _bare(self.function, self.results, self.scalar)
    @property
    def probability(self):
        values = []
        for row in self.results:
            answer = getattr(row, 'answer', None)
            chance = getattr(answer, 'probability', None)
            if self.function == 'choose' and row.value is not None:
                chance = answer.probabilities.get(answer.pick)
            values.append(chance)
        return values[0] if self.scalar and values else values
    @property
    def details(self): return self.results
    def __bool__(self): return bool(self.value)
    def __eq__(self, other):
        if isinstance(other, Result): return self.to_dict() == other.to_dict()
        return self.value == other
    def __repr__(self): return '<Result: content withheld>'
    @property
    def facts(self): return getattr(self.terminal, 'facts', None)
    def to_dict(self):
        return {'results': [row.to_dict() for row in self.results], 'terminal': self.terminal.to_dict()}


def _bare(verb, results, scalar):
    if verb == 'relate': return results[0].value if results else []
    if verb == 'filter': return [row.input for row in results]
    if verb == 'rank':
        return [{'index': row.index, 'record': row.input, 'probability': getattr(row.answer, 'probability', None)}
                for row in results]
    if verb == 'find':
        row = results[0]
        if row.index is None: return None
        return {'index': row.index, 'unit': row.value, 'probability': row.answer.probabilities[row.answer.pick]}
    values = [row.value for row in results]
    return values[0] if scalar and values else values


def _cancelled():
    error = native.Cancelled('the call was cancelled')
    error.kind, error.retryable, error.facts = 'cancelled', False, None
    return error


class Operation:
    """A bounded producer and one nonblocking native session."""
    def __init__(self, engine, verb, question, value, controls):
        from . import _pandas, _frames, _frame_calls
        frame_surface = _frame_calls.library(value)
        self.on = None
        self.surface = frame_surface or ('pandas' if _pandas(value) == 'Series' else 'python-polars' if _frames.is_series(value) else None)
        self.frame = (value.copy() if self.surface == 'pandas' else value.clone()) if self.surface else None
        self.present = ()
        self.members = ()
        fields = dict(controls)
        asked = _question(verb, question, fields)
        if frame_surface is not None:
            if verb == 'annotate':
                self.members = _frame_calls.members(asked)
            value, self.present, self.on = _frame_calls.records(self.frame, verb, fields, self.surface, self.members)
        elif self.frame is not None:
            if self.surface == 'pandas':
                from ._pandas_calls import records
            else:
                from ._polars_calls import records
            value, self.present = records(self.frame, verb)
        self.details = fields.pop('details', False)
        self.token = fields.pop('token', None)
        if 'on' in fields:
            on = fields.pop('on')
            fields['field'] = [on] if isinstance(on, str) else list(on)
        source, self.producer, self.scalar = _source(verb, value)
        if self.token is not None and self.token.cancelled:
            self._close_producer(preserve_failure=True)
            raise _cancelled()
        try:
            self.session = engine._engine._request_session(verb, asked, source, fields, self.producer is not None, self.surface)
        except native.ThinkThenError as error:
            self._close_producer()
            raise
        self.engine = engine
        self.verb = verb
        self.pending = None
        self.results = []
        self.terminal = None
        self.closed = False
        engine._sessions.add(self)

    def step(self):
        if self.closed:
            raise _cancelled()
        if self.token is not None and self.token.cancelled:
            error = _cancelled()
            error.results = tuple(self.results)
            error.terminal = self.terminal
            try: self.cancel()
            except Exception: pass
            raise error
        # Poll before advancing the producer. Native closure stops all further reads.
        packet = self.session._poll_typed()
        if packet is not None:
            kind = packet['kind']
            if kind == 'end':
                return True
            if kind == 'terminal':
                self.terminal = packet
                if 'failure' in packet:
                    failure = packet.failure
                    cls = {'usage': native.UsageError, 'backend': native.BackendError, 'local': native.LocalError,
                           'cancelled': native.Cancelled, 'deadline': native.DeadlineError, 'defect': native.DefectError}[failure.error.kind]
                    error = cls(failure.error.message)
                    error.complete = failure
                    error.kind = failure.error.kind
                    error.retryable = failure.error.retryable
                    error.facts = getattr(failure, 'facts', None)
                    error.results = tuple(self.results)
                    error.terminal = packet
                    raise error
                return True
            if kind == 'row': self.results.append(packet.value)
            if kind == 'aggregate':
                value = packet.value
                chunk = list(value) if isinstance(value, list) else [value]
                if self.verb == 'recognize':
                    self.results.extend(chunk)
                else:
                    self.results = chunk
        if self.producer is not None:
            if self.pending is None:
                try:
                    self.pending = native._RequestItem(next(self.producer), self.verb)
                except StopIteration:
                    self.session._finish()
                    self._close_producer()
                except Exception:
                    self.session._finish(True)
                    self._close_producer()
            if self.pending is not None:
                status = self.session._push(self.pending)
                if status == 'accepted': self.pending = None
                elif status == 'closed': self._close_producer(preserve_failure=True); self.pending = None
        return False

    def _close_producer(self, preserve_failure=False):
        producer, self.producer = self.producer, None
        if producer is not None:
            close = getattr(producer, 'close', None)
            if close is not None:
                active_failure = preserve_failure or sys.exc_info()[0] is not None
                try: close()
                except Exception:
                    if not active_failure: raise

    def cancel(self):
        self.session.cancel()
        self.close()

    def close(self):
        if not self.closed:
            self.closed = True
            try: self._close_producer()
            finally:
                self.session.close()
                self.engine._sessions.discard(self)

    def result(self):
        if self.on is not None:
            from ._frame_calls import FrameResult, PolarsFrameResult
            Frame = FrameResult if self.surface == 'pandas' else PolarsFrameResult
            return Frame(tuple(self.results), self.terminal, self.verb, self.scalar,
                         self.details, self.frame, self.present, self.on, self.members)
        if self.frame is not None:
            if self.surface == 'pandas':
                from ._pandas_calls import PandasResult as ColumnResult
            else:
                from ._polars_calls import PolarsResult as ColumnResult
            return ColumnResult(tuple(self.results), self.terminal, self.verb, self.scalar,
                                self.details, self.frame, self.present)
        return Result(tuple(self.results), self.terminal, self.verb, self.scalar, self.details)


def call(engine, verb, question, value, controls):
    operation = Operation(engine, verb, question, value, controls)
    try:
        while not operation.step(): time.sleep(.001)
        return operation.result()
    finally: operation.close()


class AsyncCalls:
    """Await the same named calls without blocking the event loop."""
    def __init__(self, engine): self._engine = engine
    def __getattr__(self, verb):
        if verb not in VERBS: raise AttributeError(verb)
        if verb in ('recognize', 'relate'):
            async def named(value, ask=None, **controls):
                return await self._run(verb, ask, value, controls)
        else:
            async def named(question, value, **controls):
                return await self._run(verb, question, value, controls)
        return named

    async def _run(self, verb, question, value, controls):
        import inspect
        arguments = (value, question) if verb in ('recognize', 'relate') else (question, value)
        inspect.signature(getattr(self._engine, verb)).bind(*arguments, **controls)
        operation = Operation(self._engine, verb, question, value, controls)
        try:
            while not operation.step(): await asyncio.sleep(.001)
            return operation.result()
        except asyncio.CancelledError:
            try: operation.cancel()
            except Exception: pass  # Producer cleanup cannot replace task cancellation.
            raise
        finally: operation.close()

class Session:
    """Lazy typed results; final facts become available only after native settlement."""
    def __init__(self, engine, function, question, source, controls):
        self._arguments = (engine, function, question, source, dict(controls))
        self._operation = None
        self._position = 0
        self._ended = False

    def __iter__(self): return self
    def __next__(self):
        if self._ended: raise StopIteration
        if self._operation is None:
            self._operation = Operation(*self._arguments)
        operation = self._operation
        try:
            while self._position >= len(operation.results):
                if operation.step():
                    self._ended = True
                    operation.close()
                    raise StopIteration
                time.sleep(.001)
            value = operation.results[self._position]
            self._position += 1
            return value
        except StopIteration: raise
        except BaseException:
            self._ended = True
            operation.close()
            raise

    @property
    def facts(self):
        return None if self._operation is None else getattr(self._operation.terminal, 'facts', None)

    @property
    def terminal(self):
        return None if self._operation is None else self._operation.terminal

    def cancel(self):
        if self._operation is None:
            self._ended = True
        else:
            self._operation.session.cancel()
            self._operation._close_producer(preserve_failure=True)
            self._operation.token = None

    def close(self):
        self._ended = True
        if self._operation is not None: self._operation.close()
    def __enter__(self): return self
    def __exit__(self, *args): self.close()
