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
_MISSING = object()

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

class Engine:
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

    def __init__(self, *, backend=None, base_url=None, model=None, throttle=None,
                 batch=None,
                 max_requests=None, max_requests_total=None, max_request_bytes=None, cache=None, timeout=None, max_retries=None,
                 record=None, replay=None, profile=None, refresh_cache=None, proxy=None):
        self._sessions = set()
        given = dict(backend=backend, base_url=base_url, model=model, throttle=throttle, batch=batch,
                     max_requests=max_requests, max_requests_total=max_requests_total,
                     max_request_bytes=max_request_bytes, cache=cache, timeout=timeout,
                     max_retries=max_retries, record=record, replay=replay, profile=profile,
                     refresh_cache=refresh_cache, proxy=proxy)
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

    def decide(self, question, text, **controls):
        return self._named('decide', question, text, **controls)

    def choose(self, question, text, **controls):
        return self._named('choose', question, text, **controls)

    def score(self, question, text, **controls):
        return self._named('score', question, text, **controls)

    def tag(self, question, text, **controls):
        return self._named('tag', question, text, **controls)

    def filter(self, question, records, **controls):
        return self._named('filter', question, records, **controls)

    def iterate(self, function, question, records, **controls):
        from ._calls import Session
        return Session(self, function, question, records, controls)

    def plan(self, function, question, records, **controls):
        """Preview a complete native request without a key or a send."""
        from ._calls import _question, _source, _dump
        fields = dict(controls)
        asked = _question(function, question, fields)
        source, producer, _ = _source(function, records)
        if producer is not None:
            raise UsageError('plan reads a complete input, not an iterator')
        return self._engine._plan(_dump({'schema': 'thinkthen.request/1', 'call': {
            'function': function, 'question': asked, 'input': source, 'options': fields}}))

    def rank(self, question, records, *, top=None, batch=None, context=None,
             deadline_ms=_MISSING, token=None, **legacy):
        """Records most likely first, as ``{"index", "record", "probability"}``.

        ``question`` is the question text. ``top`` keeps the first entries.
        """
        controls = dict(legacy)
        for key, item in dict(top=top, batch=batch, context=context).items():
            if item is not None: controls[key] = item
        if deadline_ms is not _MISSING: controls["deadline_ms"] = deadline_ms
        return self._named('rank', question, records, token=token, **controls)

    def find(self, question, units, *, none=False, deadline_ms=_MISSING, token=None, **legacy):
        """The unit that answers the question best, as ``{"index", "unit",
        "probability"}``, or ``None`` when nothing fits. ``none=True`` offers
        a none candidate, as ``find --none`` does."""
        controls = dict(legacy, none=none, token=token)
        if deadline_ms is not _MISSING: controls['deadline_ms'] = deadline_ms
        return self._named('find', question, units, **controls)

    def annotate(self, questions, records, *, on=None, batch=None, context=None,
                 deadline_ms=_MISSING, token=None, **legacy):
        """Ask every question in a named set of every record.

        ``questions`` is a question-set file path or the file's ``dict``.
        A member whose ``on`` names a part reads it from each record as JSON
        text. One ``dict`` comes back per record. A question the backend failed
        reads ``{"failed": {"kind": "backend", "cause": ...}}``. With ``on=``,
        ``records`` is a Polars or pandas ``DataFrame``, and the frame comes
        back with one typed column per question. The last column, ``failed``,
        holds a question-to-failure map for partial rows and null otherwise.
        A pandas frame keeps its index. A question named as a column is refused first.
        """
        if on is not None:
            from ._frame_calls import library
            if library(records) is None:
                raise UsageError('annotate with on= takes a Polars or pandas DataFrame; a list of str takes no on=')
        controls = dict(legacy, token=token)
        if on is not None: controls['on'] = on
        for key, item in dict(batch=batch, context=context).items():
            if item is not None: controls[key] = item
        if deadline_ms is not _MISSING: controls['deadline_ms'] = deadline_ms
        return self._named('annotate', questions, records, **controls)

    def recognize(self, text, ask=None, *, kinds=None, relations=None, either=None,
                  threshold=None, relation_threshold=None, on=None, deadline_ms=_MISSING,
                  token=None, descriptions=None, instructions=None, entity_definition=None, **legacy):
        """Find every name in a text and say what kind it is.

        ``kinds`` lists kind words, or maps each to a description.
        ``relations`` maps a name to a ``(source, target)`` pair of kinds,
        and ``either`` names the rules that read both ways. ``ask`` is a
        file path or the file's ``dict`` in place of the keywords. Offsets
        count Python string positions, so ``text[e.start:e.end]`` is the
        name and ``e.length`` is ``e.end - e.start``. With no kinds, every
        name has the kind ``ENTITY``. With ``on=``, ``text`` is a Polars
        ``DataFrame``, and a frame comes back with one row per name: ``row``
        (counted from 1), ``text``, ``start``, ``end``, ``length``, ``kind``,
        and ``strength``. A pandas ``DataFrame`` comes back with a new
        ``names`` column: one list per row of ``dict`` with those fields but
        ``row``. Relations take one text.
        """
        if on is not None and relations is not None:
            raise UsageError("recognize with on= takes no relations; ask them of one text")
        controls = dict(legacy, token=token)
        if on is not None: controls['on'] = on
        if ask is None:
            controls.update(kinds=kinds or [], relations=relations, either=either, descriptions=descriptions)
            if instructions is not None: controls['instructions'] = instructions
            if entity_definition is not None: controls['entity_definition'] = entity_definition
        elif instructions is not None or entity_definition is not None or descriptions is not None:
            raise UsageError("recognize task keywords take an inline declaration")
        for key, item in dict(threshold=threshold, relation_threshold=relation_threshold).items():
            if item is not None: controls[key] = item
        if deadline_ms is not _MISSING: controls['deadline_ms'] = deadline_ms
        return self._named('recognize', ask, text, **controls)

    def relate(self, entities, ask=None, *, relations=None, either=None, threshold=None,
               deadline_ms=_MISSING, token=None, **legacy):
        """Say how the entities relate: a list of ``Edge``.

        ``entities`` holds ``(name, kind)`` pairs, dicts with ``name`` and
        ``kind``, ``Entity`` values, or what ``recognize`` found. A
        ``RecognizedEntity``, or a dict with ``text`` and no ``name``, is
        named by its ``text``. ``relations`` and ``either`` read as for
        ``recognize``.
        """
        controls = dict(legacy, token=token)
        if ask is None: controls.update(relations=relations, either=either)
        if threshold is not None: controls['threshold'] = threshold
        if deadline_ms is not _MISSING: controls['deadline_ms'] = deadline_ms
        return self._named('relate', ask, entities, **controls)

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

def recognize(text, ask=None, *, kinds=None, relations=None, either=None, threshold=None,
              relation_threshold=None, on=None, deadline_ms=_MISSING, token=None,
              descriptions=None, instructions=None, entity_definition=None, **legacy):
    return _engine().recognize(text, ask, kinds=kinds, relations=relations, either=either,
                               threshold=threshold, relation_threshold=relation_threshold,
                               on=on, deadline_ms=deadline_ms, token=token,
                               descriptions=descriptions, instructions=instructions,
                               entity_definition=entity_definition, **legacy)

def relate(entities, ask=None, *, relations=None, either=None, threshold=None,
           deadline_ms=_MISSING, token=None, **legacy):
    return _engine().relate(entities, ask, relations=relations, either=either,
                            threshold=threshold, deadline_ms=deadline_ms, token=token, **legacy)

def usage():
    """The process engine's totals."""
    return _engine().usage()
