"""ThinkThen from Python: ``import thinkthen as tt``.

Ten verbs, ``decide_many``, ``details``, ``question``, and ``usage``. A call
takes one ``str`` or a list, tuple, or other iterable of ``str``. ``None``
means "not sure". ``decide``, ``choose``, ``score``, ``tag``, and
``decide_many`` also take a Polars ``Series`` and give one back, and
``annotate`` and ``recognize`` take a Polars ``DataFrame`` with ``on=``. The
column crosses in place through the Arrow stream form, and the engine makes one
call over it. This package never imports Polars. Every call reaches the real engine on its own worker
thread, so Ctrl-C stops it at once and raises ``Cancelled``, a subclass of
both ``KeyboardInterrupt`` and ``ThinkThenError``.

The module functions use one engine configured by the environment
(``THINKTHEN_BASE_URL``, ``THINKTHEN_CACHE``, and the rest). ``tt.Engine``
takes the settings as keywords and has the same methods.

Every verb takes ``deadline``, in seconds from the call, and ``token``, a
``CancelToken`` any thread can set to stop the call.
No deadline is spelled ``deadline=None`` or ``deadline=-1`` (ADR 0041).
Zero is a spent deadline: the call sends nothing and raises
``DeadlineError``. Any other negative, a bool, and a non-number raise
``UsageError``, so compute a budget as ``max(0, end - now)``.
"""

import json
import os

from . import _thinkthen
from ._thinkthen import (
    BackendError,
    Cancelled,
    CancelToken,
    DeadlineError,
    DefectError,
    Edge,
    Entity,
    LocalError,
    Question,
    Recognized,
    ThinkThenError,
    UsageError,
)

__all__ = [
    "BackendError", "Cancelled", "CancelToken", "DeadlineError", "DefectError",
    "Edge", "Engine", "Entity", "LocalError", "Question", "Recognized",
    "ThinkThenError", "UsageError", "annotate", "choose", "decide",
    "decide_many", "details", "filter", "find", "question", "rank",
    "recognize", "relate", "score", "tag", "usage",
]

_VERBS = ("decide", "choose", "score", "tag")
_PARTS = {"threshold": "threshold", "options": "options", "labels": "labels",
          "levels": "levels", "true_": "true", "false_": "false", "model": "model"}


def question(*, decide=None, choose=None, score=None, tag=None, threshold=None,
             options=None, labels=None, levels=None, true_=None, false_=None,
             model=None, file=None):
    """Build one question from its parts, or read it from ``file``.

    Give exactly one of ``decide``, ``choose``, ``score``, or ``tag``. A cut
    is a number, and a band for ``decide`` is a ``(low, high)`` pair or the
    file's ``"low:high"`` text. Parts
    and files make the same question, since both go through the file form.
    """
    verbs = [(verb, text) for verb, text in zip(_VERBS, (decide, choose, score, tag))
             if text is not None]
    if len(verbs) > 1:
        raise TypeError(f"question() takes one verb, and both {verbs[0][0]!r} and "
                        f"{verbs[1][0]!r} are given")
    given = {key: value for key, value in dict(
        threshold=threshold, options=options, labels=labels, levels=levels,
        true_=true_, false_=false_, model=model).items() if value is not None}
    if file is not None:
        if verbs or given:
            raise TypeError("question(file=...) takes nothing beside the file")
        return Question._load(os.fspath(file))
    if not verbs:
        raise TypeError("question() takes one of decide, choose, score, or tag")
    body = {verbs[0][0]: verbs[0][1]}
    for key, value in given.items():
        body[_PARTS[key]] = _threshold(value) if key == "threshold" else value
    return Question._from_json(json.dumps(body))


def _threshold(value):
    if isinstance(value, (tuple, list)) and len(value) == 2:
        return f"{value[0]}:{value[1]}"
    return value


def _asked(value, verb, **parts):
    """A built question, or one built from its text and the call's parts."""
    parts = {name: part for name, part in parts.items() if part is not None}
    if isinstance(value, str):
        return question(**{verb if verb in _VERBS else "decide": value}, **parts)
    if not isinstance(value, Question):
        raise UsageError(f"{verb} takes a question text or tt.question(), "
                         f"not a {type(value).__name__}")
    if parts:
        raise TypeError(f"{verb} takes {' and '.join(parts)} only beside a question text")
    return value


def _rebuilt(value, answer):
    """A Polars input gets its answers back through its own class."""
    if isinstance(answer, _thinkthen._Arrow):
        return type(value)(answer)
    return answer


class _Stream:
    """A frame answer offered only as a stream, since Polars reads an object
    with ``__arrow_c_array__`` as one array."""

    def __init__(self, held):
        self._held = held

    def __arrow_c_stream__(self, requested_schema=None):
        return self._held.__arrow_c_stream__(requested_schema)


def _ordering(value, verb):
    if isinstance(value, str):
        return Question._ordering(verb, value)
    return _asked(value, verb)


def _spec(maker, ask):
    if isinstance(ask, dict):
        return maker._from_json(json.dumps(ask))
    return maker._load(os.fspath(ask))


def _rules(relations, either):
    if isinstance(relations, dict):
        relations = [(name, *ends) for name, ends in relations.items()]
    both = set(either or ())
    return [(name, source, target, name in both) for name, source, target in relations or ()]


class Engine:
    """An engine with its own settings, each keyword-only.

    ``base_url``, ``model``, ``throttle`` (1 to 32 requests in flight),
    ``max_requests``, ``cache`` (a folder, ``False`` for none, or ``True``
    for the default folder), and ``cache_bytes``. An omitted setting comes
    from the environment. The throttle is one per loaded copy of this
    package: a second, different throttle raises ``UsageError``.

    Each verb takes a ``tt.question()`` or its text. Beside a text,
    ``choose`` takes ``options``, ``score`` takes ``levels``, and ``tag``
    takes ``labels``.
    """

    __slots__ = ("_engine",)

    def __init__(self, *, base_url=None, model=None, throttle=None,
                 max_requests=None, cache=None, cache_bytes=None):
        self._engine = _thinkthen._Engine(
            base_url=base_url, model=model, throttle=throttle,
            max_requests=max_requests, cache=cache, cache_bytes=cache_bytes)

    def __repr__(self):
        return "Engine()"

    def decide(self, question, text, *, deadline=None, token=None):
        """``True``, ``False``, or ``None`` when the rule says "not sure"."""
        return _rebuilt(text, self._engine.ask("decide", _asked(question, "decide"), text,
                                                deadline, token))

    def decide_many(self, question, records, *, deadline=None, token=None):
        """One ``decide`` answer per record, in input order."""
        return _rebuilt(records, self._engine.many("decide_many", _asked(question, "decide_many"),
                                                   records, deadline, token))

    def choose(self, question, text, *, options=None, deadline=None, token=None):
        """The option picked, or ``None`` when the rule says "not sure"."""
        return _rebuilt(text, self._engine.ask("choose", _asked(question, "choose", options=options),
                                                text, deadline, token))

    def score(self, question, text, *, levels=None, deadline=None, token=None):
        """The position on the question's levels, from 0 to K-1."""
        return _rebuilt(text, self._engine.ask("score", _asked(question, "score", levels=levels),
                                                text, deadline, token))

    def tag(self, question, text, *, labels=None, deadline=None, token=None):
        """The labels that reach the cut."""
        return _rebuilt(text, self._engine.ask("tag", _asked(question, "tag", labels=labels),
                                                text, deadline, token))

    def details(self, question, text, *, deadline=None, token=None):
        """The command's ``--details`` document, as a ``dict``."""
        return json.loads(self._engine.ask("details", _asked(question, "details"), text,
                                            deadline, token))

    def filter(self, question, records, *, deadline=None, token=None):
        """The records the ``decide`` question passes, in input order."""
        return self._engine.many("filter", _asked(question, "filter"), records, deadline, token)

    def rank(self, question, records, *, top=None, deadline=None, token=None):
        """Records most likely first, as ``{"index", "record", "probability"}``.

        ``question`` is the question text. ``top`` keeps the first entries.
        """
        ranked = self._engine.order("rank", _ordering(question, "rank"), records, deadline, token)
        ordered = [{"index": index, "record": record, "probability": probability}
                   for index, record, probability in ranked]
        return ordered if top is None else ordered[:top]

    def find(self, question, units, *, deadline=None, token=None):
        """The unit that answers the question best, as ``{"index", "unit",
        "probability"}``, or ``None`` when nothing fits."""
        found = self._engine.order("find", _ordering(question, "find"), units, deadline, token)
        if not found:
            return None
        [(index, unit, probability)] = found
        return {"index": index, "unit": unit, "probability": probability}

    def annotate(self, questions, records, *, on=None, deadline=None, token=None):
        """Ask every question in a named set of every record.

        ``questions`` is a question-set file path or the file's ``dict``.
        One ``dict`` comes back per record. A question the backend failed
        reads ``{"failed": {"kind": "backend", "cause": ...}}``. With ``on=``,
        ``records`` is a Polars ``DataFrame``, and the frame comes back with
        one new column per question. A failed question's column holds text.
        """
        asked = _spec(_thinkthen._QuestionSet, questions)
        if on is None:
            return self._engine.annotate(asked, records, deadline, token)
        return type(records)(_Stream(_thinkthen._annotate_frame(self._engine, asked, records,
                                                                on, deadline, token)))

    def recognize(self, text, ask=None, *, kinds=None, relations=None, either=None,
                  threshold=None, relation_threshold=None, on=None, deadline=None, token=None):
        """Find every name in a text and say what kind it is.

        ``kinds`` lists kind words, or maps each to a description.
        ``relations`` maps a name to a ``(source, target)`` pair of kinds,
        and ``either`` names the rules that read both ways. ``ask`` is a
        file path or the file's ``dict`` in place of the keywords. Offsets
        count Python string positions, so ``text[e.start:e.end]`` is the
        name. With ``on=``, ``text`` is a Polars ``DataFrame``, and a frame
        comes back with one row per name: ``row`` (counted from 1), ``text``,
        ``kind``, ``start``, ``end``, and ``strength``. Relations take one text.
        """
        if on is not None and relations is not None:
            raise UsageError("recognize with on= takes no relations; ask them of one text")
        if ask is not None:
            spec = _spec(_thinkthen._Recognize, ask)
        else:
            named = kinds.items() if isinstance(kinds, dict) else [(k, None) for k in kinds or ()]
            spec = _thinkthen._Recognize._build(list(named), _rules(relations, either),
                                               threshold, relation_threshold)
        if on is not None:
            return type(text)(_Stream(_thinkthen._recognize_frame(self._engine, spec, text, on,
                                                                  deadline, token)))
        return self._engine.recognize(spec, text, deadline, token)

    def relate(self, entities, ask=None, *, relations=None, either=None, threshold=None,
               deadline=None, token=None):
        """Say how the entities relate: a list of ``Edge``.

        ``entities`` holds ``(name, kind)`` pairs, dicts with ``name`` and
        ``kind``, or ``Entity`` values. ``relations`` and ``either`` read as
        for ``recognize``.
        """
        if ask is not None:
            spec = _spec(_thinkthen._Relate, ask)
        else:
            spec = _thinkthen._Relate._build(_rules(relations, either), threshold)
        return self._engine.relate(spec, entities, deadline, token)

    def usage(self):
        """This engine's totals: requests sent, cache answers, and tokens."""
        return self._engine.usage()


_process = None


def _engine():
    global _process
    if _process is None:
        engine = Engine.__new__(Engine)
        engine._engine = _thinkthen._Engine._process()
        _process = engine
    return _process


def decide(question, text, *, deadline=None, token=None):
    return _engine().decide(question, text, deadline=deadline, token=token)


def decide_many(question, records, *, deadline=None, token=None):
    return _engine().decide_many(question, records, deadline=deadline, token=token)


def choose(question, text, *, options=None, deadline=None, token=None):
    return _engine().choose(question, text, options=options, deadline=deadline, token=token)


def score(question, text, *, levels=None, deadline=None, token=None):
    return _engine().score(question, text, levels=levels, deadline=deadline, token=token)


def tag(question, text, *, labels=None, deadline=None, token=None):
    return _engine().tag(question, text, labels=labels, deadline=deadline, token=token)


def details(question, text, *, deadline=None, token=None):
    return _engine().details(question, text, deadline=deadline, token=token)


def filter(question, records, *, deadline=None, token=None):
    return _engine().filter(question, records, deadline=deadline, token=token)


def rank(question, records, *, top=None, deadline=None, token=None):
    return _engine().rank(question, records, top=top, deadline=deadline, token=token)


def find(question, units, *, deadline=None, token=None):
    return _engine().find(question, units, deadline=deadline, token=token)


def annotate(questions, records, *, on=None, deadline=None, token=None):
    return _engine().annotate(questions, records, on=on, deadline=deadline, token=token)


def recognize(text, ask=None, *, kinds=None, relations=None, either=None, threshold=None,
              relation_threshold=None, on=None, deadline=None, token=None):
    return _engine().recognize(text, ask, kinds=kinds, relations=relations, either=either,
                               threshold=threshold, relation_threshold=relation_threshold,
                               on=on, deadline=deadline, token=token)


def relate(entities, ask=None, *, relations=None, either=None, threshold=None,
           deadline=None, token=None):
    return _engine().relate(entities, ask, relations=relations, either=either,
                            threshold=threshold, deadline=deadline, token=token)


def usage():
    """The process engine's totals."""
    return _engine().usage()


for _name in ("decide", "decide_many", "choose", "score", "tag", "details", "filter",
              "rank", "find", "annotate", "recognize", "relate"):
    globals()[_name].__doc__ = getattr(Engine, _name).__doc__
del _name
