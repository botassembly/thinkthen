"""Host naming, conversion and scheduling over the canonical native session."""
from __future__ import annotations
import asyncio
from collections.abc import Iterator, Mapping
from dataclasses import dataclass
import os
import sys
from typing import Generic, TypeVar
from . import _thinkthen as native
from . import _inputs as c
from .files import FileSelection

T = TypeVar('T')

VERBS = ('decide', 'choose', 'tag', 'score', 'filter', 'rank', 'find', 'annotate', 'recognize', 'relate')


def _json(value):
    if isinstance(value, native._NativeResult):
        return value._to_dict()
    if isinstance(value, Mapping):
        return {key: _json(item) for key, item in value.items()}
    if isinstance(value, (tuple, list)):
        return [_json(item) for item in value]
    return value


def _original(value, text=False):
    return {'kind': 'text', 'text': value} if text else {'kind': 'json', 'value': _json(value)}


def _item(value, verb):
    if isinstance(value, c.Item):
        import base64
        descriptor = {'images': [{'kind': 'bytes', 'media': image.media, 'bytes': base64.b64encode(image.data).decode('ascii')} for image in value.images]}
        if not value.image_only: descriptor['original'] = _original(value.value, value.text)
        if value.context is not c.ABSENT: descriptor['context'] = value.context
        if value.options is not c.ABSENT:
            descriptor['options'] = [dict(name=name, **({} if description is c.ABSENT else {'description': description})) for name, description in value.options]
        return descriptor
    if verb == 'relate':
        if isinstance(value, tuple): value = dict(zip(('name', 'kind'), value, strict=True))
        elif isinstance(value, native._NativeResult): value = {'name': value.text, 'kind': value.kind}
    return {'original': _original(value, isinstance(value, str))}


def _input(value, verb):
    if isinstance(value, (c.Files, FileSelection)):
        reading = {'unit': value.unit}
        if value.window is not None and value.window is not c.ABSENT: reading['window'] = value.window
        source = {'paths': [os.fspath(path) for path in value.paths], 'reading': reading, 'media': getattr(value, 'media', 'text')}
        if getattr(value, 'jsonl', False): source['framing'] = 'jsonl'
        return {'kind': 'source', 'source': source}
    if isinstance(value, c.Records): value = value.items
    values = value if isinstance(value, (list, tuple)) else (value,)
    return {'kind': 'records', 'items': [_item(item, verb) for item in values]}


def _question(verb, question, controls, frame=False):
    if verb == 'annotate' and isinstance(question, type):
        from .pydantic import question_set
        question = question_set(question)
    if verb in ('recognize', 'relate'):
        for key in tuple(controls):
            if controls[key] is None and key not in ('deadline_ms', 'model', 'batch', 'context', 'field', 'context_field', 'mode', 'snippet_pieces', 'stage_context', 'seed_spans', 'examples', 'seed_spans_field', 'examples_field', 'max_requests_total', 'on'):
                controls.pop(key)
        if question is None and verb == 'recognize':
            controls.setdefault('kinds', [])
    elif verb == 'annotate':
        for key in ('batch', 'context', 'on'):
            if controls.get(key, object()) is None: controls.pop(key)
    from ._labels import normalize
    for key in ('options', 'labels', 'levels', 'kinds'):
        if key in controls:
            controls[key] = normalize(controls[key], controls.pop('descriptions', None), bare_level_names=key == 'levels')
    return native._build_question(verb, question, controls, frame)


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
        if isinstance(other, Result): return self._to_dict() == other._to_dict()
        return self.value == other
    def __repr__(self): return '<Result: content withheld>'
    @property
    def facts(self): return getattr(self.terminal, 'facts', None)
    def _to_dict(self):
        return {'results': [row._to_dict() for row in self.results], 'terminal': self.terminal._to_dict()}


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
        _, self.producer, _ = _source(verb, value)
        self.session = None
        try:
            asked = _question(verb, question, fields, frame_surface is not None)
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
                raise _cancelled()
            self.session = engine._engine._request_session(verb, (asked, source, fields), self.producer is not None, self.surface)
            self.engine = engine
            self.verb = verb
            self.pending = None
            self.producer_error = None
            self.results = []
            self.terminal = None
            self.closed = False
            engine._sessions.add(self)
        except BaseException:
            try: self._close_producer(preserve_failure=True)
            finally:
                if self.session is not None:
                    try: self.session.close()
                    except BaseException: pass
            raise

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
                    error.results, error.terminal = tuple(self.results), packet
                    if self.producer_error is None: raise error
                    raise error from self.producer_error
                return True
            if kind == 'row': self.results.append(packet.value)
            if kind == 'aggregate':
                value = packet.value
                chunk = list(value) if isinstance(value, list) else [value]
                self.results.extend(chunk)
        if self.producer is not None:
            if self.pending is None:
                try:
                    self.pending = native._RequestItem(next(self.producer), self.verb)
                except StopIteration:
                    self.session._finish()
                    self._close_producer()
                except Exception as error:
                    self.producer_error = error
                    self.session._finish(True)
                    self._close_producer()
            if self.pending is not None:
                status = self.session._push(self.pending)
                if status == 'accepted': self.pending = None
                elif status == 'closed': self._close_producer(preserve_failure=True); self.pending = None
        return False

    def wait(self):
        self.session._wait(self.producer is not None)

    def _close_producer(self, preserve_failure=False):
        producer, self.producer = self.producer, None
        if producer is not None:
            active_failure = preserve_failure or sys.exc_info()[0] is not None
            try:
                close = getattr(producer, 'close', None)
                if close is not None: close()
            except BaseException:
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
        while not operation.step(): operation.wait()
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
            while not operation.step(): await asyncio.to_thread(operation.wait)
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
                operation.wait()
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
