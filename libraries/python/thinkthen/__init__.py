"""Ten named typed calls over owned native requests and sessions."""
import json
import os
from typing import NamedTuple as _NamedTuple
from . import _thinkthen
from ._thinkthen import BackendError, Cancelled, CancelToken, DeadlineError, DefectError, LocalError, ThinkThenError, UsageError
from ._native_results import NativeUsagePersistence
from ._calls import Result
from ._inputs import ABSENT, Image, Item, Records, Files, QuestionSource
from .files import FileSelection, SourceRecord, read_files

__all__ = ['BackendError', 'Cancelled', 'CancelToken', 'DeadlineError', 'DefectError', 'LocalError', 'ThinkThenError', 'UsageError', 'Engine', 'Result', 'UsageStatus', 'NativeUsagePersistence', 'ABSENT', 'Image', 'Item', 'Records', 'Files', 'QuestionSource', 'FileSelection', 'SourceRecord', 'read_files', 'decide', 'choose', 'score', 'tag', 'filter', 'rank', 'find', 'annotate', 'recognize', 'relate', 'usage']

class UsageStatus(_NamedTuple):
    """Live native durability state and optional fixed advice; no call facts."""
    state: NativeUsagePersistence
    advice: str | None

def _pandas(value):
    """The pandas class a value is, by name, found through its type's method
    resolution order so a subclass counts, or ``None``. It imports nothing."""
    for kind in type(value).__mro__:
        if kind.__module__.partition(".")[0] == "pandas":
            return kind.__name__
    return None


def _on(frame, on, new):
    """``df[on]`` of a pandas frame, after decision 4's checks in order."""
    try:
        hash(on)
    except TypeError:
        raise UsageError('on= takes one column label, such as "body"') from None
    if frame.columns.nlevels != 1:
        raise UsageError("on= reads a frame whose column labels have one level")
    if on not in frame.columns:
        raise UsageError(f"the frame has no column named {on!r}")
    if not isinstance(frame.columns.get_loc(on), int):
        raise UsageError(f"the frame has more than one column named {on!r}")
    for name in new:
        if name in frame.columns:
            raise UsageError(f"the frame already has a column named {name!r}; rename it first")
    return frame[on]


def _rules(relations, either):
    if isinstance(relations, dict):
        relations = [(name, *ends) for name, ends in relations.items()]
    both = set(either or ())
    return [(name, source, target, name in both) for name, source, target in relations or ()]

from ._named_generated import NamedCalls

class Engine(NamedCalls):
    """An engine with its own settings, each keyword-only.

    ``backend``, ``base_url``, ``model``, ``throttle`` (1 to 32 requests in flight), ``batch``,
    ``max_requests``, ``max_request_bytes``, ``refresh_cache``, ``cache`` (a folder, ``False`` for none, or ``True``
    for the default folder), ``timeout``, ``max_retries``, ``record``, ``replay``, and ``profile``. An omitted setting comes
    from the environment. ``backend`` selects its captured named key; an explicit
    ``base_url`` receives that key and retains its setup path and limits. The throttle is one per loaded copy of this
    package: a second, different throttle raises ``UsageError``.
    ``proxy`` is reserved; supplying it raises ``UsageError`` before any request.

    Each verb takes an authored question dictionary, an explicit question selector or its text. Beside a text,
    ``choose`` takes ``options``, ``score`` takes ``levels``, and ``tag``
    takes ``labels``.
    """

    __slots__ = ("_engine", "_settings_json", "_sessions")

    def __setattr__(self, name, value):
        if hasattr(self, name):
            raise AttributeError("an Engine's validated settings cannot be changed")
        object.__setattr__(self, name, value)

    def _configure(self, given):
        self._sessions = set()
        self._settings_json = json.dumps({key: value for key, value in given.items()
                                          if value is not None}, default=os.fspath)
        self._engine = _thinkthen._Engine(self._settings_json)

    def __repr__(self):
        return "Engine()"

    @property
    def asyncio(self):
        from ._calls import AsyncCalls
        return AsyncCalls(self)

    def usage_persistence(self) -> UsageStatus:
        """Observe current durability without waiting for pending writes."""
        return UsageStatus(*self._engine.usage_persistence())

    def finish_usage_status(self) -> UsageStatus:
        """Drain current deltas; only native usage-lock acquisition has a deadline."""
        return UsageStatus(*self._engine.finish_usage_status())

    def close(self):
        failure = None
        for operation in tuple(self._sessions):
            try: operation.cancel()
            except Exception as error:
                if failure is None: failure = error
        if failure is not None: raise failure

    def __enter__(self): return self
    def __exit__(self, *args): self.close()
    async def __aenter__(self): return self
    async def __aexit__(self, *args): self.close()

    def _named(self, verb, question, value, **controls):
        from ._calls import call
        return call(self, verb, question, value, controls)

    def iterate(self, function, question, records, **controls):
        from ._calls import Session
        return Session(self, function, question, records, controls)

    def plan(self, function, question, records, **controls):
        """Preview a complete native request without a key or a send."""
        from ._calls import _question, _source
        fields = dict(controls)
        asked = _question(function, question, fields)
        source, producer, _ = _source(function, records)
        if producer is not None:
            raise UsageError('plan reads a complete input, not an iterator')
        return self._engine._plan(function, asked, source, fields)

    def usage(self):
        """This engine's totals: requests sent, cache answers, and tokens."""
        return self._engine.usage()


_process = None
def _engine():
    global _process
    if _process is None:
        engine = Engine.__new__(Engine)
        engine._engine = _thinkthen._Engine("{}")
        engine._settings_json = None
        engine._sessions = set()
        _process = engine
    return _process


def decide(question, text, **controls):
    return _engine().decide(question, text, **controls)

def choose(question, text, **controls):
    return _engine().choose(question, text, **controls)

def score(question, text, **controls):
    return _engine().score(question, text, **controls)

def tag(question, text, **controls):
    return _engine().tag(question, text, **controls)

def filter(question, records, **controls):
    return _engine().filter(question, records, **controls)

def rank(question, records, **keywords):
    return _engine().rank(question, records, **keywords)

def find(question, units, **keywords):
    return _engine().find(question, units, **keywords)

def annotate(questions, records, **keywords):
    return _engine().annotate(questions, records, **keywords)

def recognize(text, ask=None, **keywords):
    return _engine().recognize(text, ask, **keywords)

def relate(entities, ask=None, **keywords):
    return _engine().relate(entities, ask, **keywords)

def usage():
    """The process engine's totals."""
    return _engine().usage()
