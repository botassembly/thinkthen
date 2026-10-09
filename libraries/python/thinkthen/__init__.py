"""Ten named calls over the canonical native Request session.

Ordinary direct calls return Result with an ordinary value, generated owned
complete results, and native final facts. Engine.asyncio mirrors those calls.
Engine context exit closes active sessions. Curried Judge and frame calls
retain their compatibility implementation until their migration.
"""

import json
import os
from typing import Annotated as _Annotated, get_origin as _get_origin

from . import _thinkthen
from ._labels import normalize as _labels
from ._thinkthen import (
    BackendError,
    Cancelled,
    CancelToken,
    Call,
    Completion,
    CompletionReceipt,
    DeadlineError,
    DefectError,
    Edge,
    Entity,
    LocalError,
    Question,
    Recognized,
    RecognizedEntity,
    Relation,
    ThinkThenError,
    UsageError,
    Tally,
)
from .judge import Judge, make as _make_judge
from .stream import Stream
from . import _frames
from ._calls import Result
from .files import FileSelection, SourceRecord, Located, read_files
from .files import call as _source_call, spec_source as _source_spec

__all__ = [
    "BackendError", "Cancelled", "CancelToken", "DeadlineError", "DefectError",
    "Call", "Result", "Completion", "CompletionReceipt", "Edge", "Engine", "Entity", "Judge", "Stream", "Tally", "LocalError", "Question", "Recognized",
    "RecognizedEntity", "Relation", "ThinkThenError", "UsageError",
    "annotate", "choose", "decide", "details", "filter",
    "find", "plan", "question", "rank", "recognize", "relate", "score", "tag",
    "usage", "FileSelection", "SourceRecord", "Located", "read_files",
]

_VERBS = ("decide", "choose", "score", "tag")
_PARTS = {"threshold": "threshold", "options": "options", "labels": "labels",
          "levels": "levels", "true": "true", "false": "false", "model": "model"}
_MISSING = object()


def question(*, decide=None, choose=None, score=None, tag=None, threshold=None,
             options=None, labels=None, levels=None, true=None, false=None,
             true_=_MISSING, false_=_MISSING,
             model=None, file=None, descriptions=None):
    """Build one question from its parts, or read it from ``file``.

    Give exactly one of ``decide``, ``choose``, ``score``, or ``tag``. A cut
    is a number, and a band for ``decide`` is a ``(low, high)`` pair or the
    file's ``"low:high"`` text. Parts
    and files make the same question, since both go through the file form.
    """
    if true_ is not _MISSING or false_ is not _MISSING:
        raise UsageError("use true= and false= instead of true_= and false_=")
    if descriptions is not None:
        raise UsageError("put descriptions inside options, levels, or labels")
    verbs = [(verb, text) for verb, text in zip(_VERBS, (decide, choose, score, tag))
             if text is not None]
    if len(verbs) > 1:
        raise TypeError(f"question() takes one verb, and both {verbs[0][0]!r} and "
                        f"{verbs[1][0]!r} are given")
    given = {key: value for key, value in dict(
        threshold=threshold, options=options, labels=labels, levels=levels,
        true=true, false=false, model=model).items() if value is not None}
    if file is not None:
        if verbs or given or descriptions is not None:
            raise TypeError("question(file=...) takes nothing beside the file")
        return Question._load(os.fspath(file))
    if not verbs:
        raise TypeError("question() takes one of decide, choose, score, or tag")
    body = {verbs[0][0]: verbs[0][1]}
    for key, value in given.items():
        body[_PARTS[key]] = (_threshold(value) if key == "threshold" else
                             _labels(value, bare_level_names=key == "levels")
                             if key in ("options", "labels", "levels") else
                             value)
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


_QUESTION_KEYS = frozenset(("threshold", "true", "false", "options", "levels", "labels", "model"))


def _configured(verb, asked, keywords):
    """One shared Rust grammar for question fields and call controls."""
    for old, new in (("deadline", "deadline_ms"), ("true_", "true"),
                     ("false_", "false")):
        if old in keywords:
            raise UsageError(f"use {new}= instead of {old}=")
    if "descriptions" in keywords:
        raise UsageError("put descriptions inside options, levels, or labels")
    fields = dict(keywords)
    if not isinstance(asked, str):
        if not isinstance(asked, Question):
            raise UsageError(f"{verb} takes a question text or tt.question(), not a {type(asked).__name__}")
        if "none" in fields:
            raise UsageError("the settings key `none` does not belong to this verb")
        repeated = _QUESTION_KEYS.intersection(fields)
        if repeated:
            raise UsageError(f"settings repeats `{sorted(repeated)[0]}` from the question or named arguments")
    if "threshold" in fields:
        fields["threshold"] = _threshold(fields["threshold"])
    for key in ("options", "levels", "labels"):
        if key in fields:
            fields[key] = _labels(fields[key], bare_level_names=key == "levels")
    try:
        encoded = json.dumps(fields, allow_nan=False)
    except (TypeError, ValueError) as source:
        raise UsageError("the settings hold a value JSON cannot represent") from source
    built, count, maximum, context, deadline_ms = Question._settings(
        verb, asked if isinstance(asked, str) else None, encoded)
    return built or asked, ("max" if maximum else count), context, deadline_ms


def _due_keyword(deadline_ms, legacy):
    if "deadline" in legacy:
        raise UsageError("use deadline_ms= instead of deadline=")
    if legacy:
        raise UsageError(f"the settings key `{sorted(legacy)[0]}` does not exist")
    return _deadline(deadline_ms)


def _deadline(deadline_ms):
    """ADR 0041: omission and -1 mean no deadline; an explicit None is refused before a send."""
    if deadline_ms is None:
        raise UsageError("`deadline_ms` is a whole number of milliseconds")
    return -1 if deadline_ms is _MISSING else deadline_ms


def _rebuilt(value, answer):
    """A Polars input gets its answers back through its own class."""
    if isinstance(answer, _thinkthen._Arrow):
        rebuilt = type(value)(answer)
        return rebuilt.rename(value.name) if _frames.is_series(value) else rebuilt
    return answer


def _mapped(call, convert):
    """Rebuild an old value inside its completed account."""
    try:
        return call._with_value(convert(call.value))
    except Exception as source:
        error = LocalError("the completed result could not be rebuilt")
        error.kind, error.retryable = "local", False
        error.facts, error.details = call.facts, call.details
        raise error from source


def _paired(call, verb, multiple=False):
    """Use the owned observations from this call; never ask the engine again."""
    try:
        probabilities = [None] * len(call.value) if multiple else []
        seen = set()
        for detail in call.details:
            index = detail["index"]
            if type(index) is not int or index in seen or index < 0 or \
                    (multiple and index >= len(probabilities)) or (not multiple and index != 0):
                raise ValueError("the answer has a duplicate or missing row place")
            seen.add(index)
            reported = detail["probabilities"]
            if verb == "decide":
                probability = reported
            else:
                selected = detail["answer"]
                probability = next((chance for name, chance in reported if name == selected), None)
            if multiple:
                probabilities[index] = probability
            else:
                probabilities.append(probability)
        if multiple:
            value = call.value
            if _pandas(value) == "Series":
                probability = type(value)(probabilities, index=value.index, name=value.name,
                                          dtype="Float64")
            elif type(value).__module__.partition(".")[0] == "polars":
                probability = type(value)(probabilities)
            else:
                probability = probabilities
        else:
            if len(probabilities) != 1:
                raise ValueError("the scalar answer has no matching probability")
            probability = probabilities[0]
        return call._with_probability(probability)
    except Exception as source:
        error = DefectError("the completed result has no matching probability")
        error.kind, error.retryable = "defect", False
        error.facts, error.details = call.facts, call.details
        raise error from source


def _pandas(value):
    """The pandas class a value is, by name, found through its type's method
    resolution order so a subclass counts, or ``None``. It imports nothing."""
    for kind in type(value).__mro__:
        if kind.__module__.partition(".")[0] == "pandas":
            return kind.__name__
    return None


class _Once:
    """pandas' exported stream, handed over once and as it is."""

    def __init__(self, capsule):
        self._capsule = capsule

    def __arrow_c_stream__(self, requested_schema=None):
        capsule, self._capsule = self._capsule, None
        if capsule is None or requested_schema is not None:
            raise UsageError("a pandas column's stream is read once, as it is")
        return capsule


def _marked(series, kind):
    """A pandas Series, marked with its reader before any export (decision 2):
    the list reader when it is empty or categorical, lacks the stream, or its
    export raises, and the Arrow door otherwise."""
    if kind == "DataFrame":
        raise UsageError('a data frame is not a column; pass df["name"], or annotate with on=')
    if kind != "Series":
        raise UsageError(f"thinkthen reads a pandas Series, not a pandas {kind}; "
                         "pass a pandas Series")
    if not len(series) or series.dtype.name == "category" or \
            (series.dtype.name == "object" and series.isna().all()) or \
            not hasattr(series, "__arrow_c_stream__"):
        return _thinkthen._Pandas([None if missing else value
                                  for value, missing in zip(series, series.isna())], True)
    try:
        capsule = series.__arrow_c_stream__()
    except Exception:
        return _thinkthen._Pandas([None if missing else value
                                  for value, missing in zip(series, series.isna())], True)
    return _thinkthen._Pandas(_Once(capsule), False)


def _column(call, verb, asked, value, deadline, token):
    """A column verb. A pandas Series gets the caller's Series back, with its
    index and name, through its own class (decision 3)."""
    kind = _pandas(value)
    if kind is None:
        return _mapped(call(verb, asked, value, deadline, token),
                       lambda answer: _rebuilt(value, answer))
    result = call(verb, asked, _marked(value, kind), deadline, token)
    return _mapped(result, lambda answer: type(value)(answer[0], index=value.index,
                                                       name=value.name, dtype=answer[1]))


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
    if maker is _thinkthen._QuestionSet and _get_origin(ask) is _Annotated:
        raise UsageError("a question-set model takes no root Field description")
    if not isinstance(ask, type) and any(base.__module__.startswith("pydantic.")
                                         for base in type(ask).__mro__):
        raise UsageError("a question set takes a Pydantic model class, not an instance")
    if maker is _thinkthen._QuestionSet and isinstance(ask, type) and any(
            base.__module__.startswith("pydantic.") for base in ask.__mro__):
        from . import pydantic as adapter
        ask = adapter.question_set(ask)
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

    ``backend``, ``base_url``, ``model``, ``throttle`` (1 to 32 requests in flight), ``batch``,
    ``max_requests``, ``max_request_bytes``, ``cache`` (a folder, ``False`` for none, or ``True``
    for the default folder), ``timeout``, ``max_retries``, ``record``, ``replay``, and ``profile``. An omitted setting comes
    from the environment. ``backend`` selects its captured named key; an explicit
    ``base_url`` receives that key and retains its setup path and limits. The throttle is one per loaded copy of this
    package: a second, different throttle raises ``UsageError``.

    Each verb takes a ``tt.question()`` or its text. Beside a text,
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
                 record=None, replay=None, profile=None):
        self._sessions = set()
        self._engine = _thinkthen._Engine(
            backend=backend, base_url=base_url, model=model, throttle=throttle,
            batch=batch,
            max_requests=max_requests, max_requests_total=max_requests_total,
            max_request_bytes=max_request_bytes,
            cache=cache, timeout=timeout,
            max_retries=max_retries, record=record, replay=replay, profile=profile)
        given = dict(backend=backend, base_url=base_url, model=model, throttle=throttle, batch=batch,
                     max_requests=max_requests, max_requests_total=max_requests_total,
                     max_request_bytes=max_request_bytes, cache=cache, timeout=timeout,
                     max_retries=max_retries, record=record, replay=replay, profile=profile)
        self._settings_json = json.dumps({key: value for key, value in given.items()
                                          if value is not None}, default=os.fspath)

    def __repr__(self):
        return "Engine()"

    @property
    def asyncio(self):
        from ._calls import AsyncCalls
        return AsyncCalls(self)

    def close(self):
        for operation in tuple(self._sessions):
            operation.cancel()

    def __enter__(self): return self
    def __exit__(self, *args): self.close()
    async def __aenter__(self): return self
    async def __aexit__(self, *args): self.close()

    def _named(self, verb, question, value, **controls):
        from ._calls import call
        return call(self, verb, question, value, controls)

    @property
    def complete(self):
        from .complete import Engine as CompleteEngine
        return CompleteEngine(_engine=self._engine)

    def decide(self, question, text=_MISSING, *, token=None, **keywords):
        """Build a judge, or answer one text, ordered input, or column."""
        return self._judged("decide", question, text, token, keywords)

    def choose(self, question, text=_MISSING, *, token=None, **keywords):
        """Build a judge, or pick one option from an input."""
        return self._judged("choose", question, text, token, keywords)

    def score(self, question, text=_MISSING, *, token=None, **keywords):
        """Build a judge, or score an input."""
        return self._judged("score", question, text, token, keywords)

    def tag(self, question, text=_MISSING, *, token=None, **keywords):
        """Build a judge, or tag an input."""
        return self._judged("tag", question, text, token, keywords)

    def details(self, question, text, *, deadline_ms=_MISSING, token=None, **legacy):
        """The command's ``--details`` document, as a ``dict``."""
        deadline_ms = _due_keyword(deadline_ms, legacy)
        return _mapped(self._engine.ask("details", _asked(question, "details"), text,
                                        deadline_ms, token), json.loads)

    def filter(self, question, records=_MISSING, *, token=None, **keywords):
        """Build a judge, or keep matching records in input order."""
        return self._judged("filter", question, records, token, keywords)

    def _judged(self, verb, question, value, token, keywords):
        if value is not _MISSING and not _frames.is_series(value):
            return self._named(verb, question, value, token=token, **keywords)
        fields = dict(keywords)
        deadline_ms = fields.pop("deadline_ms", _MISSING)
        if value is _MISSING and (deadline_ms is not _MISSING or token is not None):
            raise UsageError("deadline_ms and token belong when the judge is applied")
        judge = _make_judge(verb, question, self, fields)
        if value is _MISSING:
            return judge
        return judge(value, deadline_ms=_deadline(deadline_ms), token=token)

    def plan(self, judge, records):
        """Preview a judge on a complete input without a key or send."""
        if not isinstance(judge, Judge):
            raise UsageError("plan takes a ThinkThen Judge")
        if judge._engine is not None and judge._engine is not self:
            raise UsageError("the judge belongs to another engine")
        if isinstance(records, (set, frozenset)):
            raise UsageError("an unordered set cannot align records with answers")
        if isinstance(records, FileSelection):
            return self._engine.plan(judge._asked, _thinkthen._read_files(records._json()),
                                     judge._batch, judge._context)
        kind = _pandas(records)
        if kind == "Series":
            records = _marked(records, kind)
        elif kind == "DataFrame":
            raise UsageError("plan reads a complete list or text column, not a data frame")
        elif kind is not None:
            records = list(records)
        elif hasattr(records, "__arrow_c_stream__") or hasattr(records, "__arrow_c_array__"):
            pass
        elif iter(records) is records:
            raise UsageError("plan reads a complete list or column, not an iterator")
        return self._engine.plan(judge._asked, records, judge._batch, judge._context)

    def rank(self, question, records, *, top=None, batch=None, context=None,
             deadline_ms=_MISSING, token=None, **legacy):
        """Records most likely first, as ``{"index", "record", "probability"}``.

        ``question`` is the question text. ``top`` keeps the first entries.
        """
        if not _frames.is_series(records):
            controls = dict(legacy)
            for key, item in dict(top=top, batch=batch, context=context).items():
                if item is not None: controls[key] = item
            if deadline_ms is not _MISSING: controls["deadline_ms"] = deadline_ms
            return self._named('rank', question, records, token=token, **controls)
        deadline_ms = _due_keyword(deadline_ms, legacy)
        if isinstance(records, FileSelection):
            asked = _ordering(question, "rank")
            if asked.kind != "rank":
                raise UsageError("rank takes a rank question")
            call = _source_call(self, "rank", asked._json(), records, batch, context, deadline_ms, token)
            return _mapped(call, lambda rows: rows[:top])
        if _frames.is_series(records):
            ranked = _frames.collection(self, "rank", _ordering(question, "rank"), records,
                                        batch, context, deadline_ms, token)
        else:
            ranked = self._engine.order("rank", _ordering(question, "rank"), records,
                                         batch, context, deadline_ms, token)
        return _mapped(ranked, lambda rows: [{"index": index, "record": record,
                                               "probability": probability}
                                              for index, record, probability in rows][:top])

    def find(self, question, units, *, none=False, deadline_ms=_MISSING, token=None, **legacy):
        """The unit that answers the question best, as ``{"index", "unit",
        "probability"}``, or ``None`` when nothing fits. ``none=True`` offers
        a none candidate, as ``find --none`` does."""
        if not _frames.is_series(units):
            controls = dict(legacy, none=none, token=token)
            if deadline_ms is not _MISSING: controls['deadline_ms'] = deadline_ms
            return self._named('find', question, units, **controls)
        if not isinstance(none, bool):
            raise UsageError("none is True or False")
        asked = _ordering(question, "find")
        deadline_ms = _due_keyword(deadline_ms, legacy)
        if isinstance(units, FileSelection):
            if asked.kind != "find":
                raise UsageError("find takes a find question")
            body = json.loads(asked._json())
            body["none"] = none
            return _source_call(self, "find", json.dumps(body), units, None, None, deadline_ms, token)
        if none:
            asked = asked._offering_none()
        found = (_frames.collection(self, "find", asked, units, None, None, deadline_ms, token)
                 if _frames.is_series(units) else
                 self._engine.order("find", asked, units, None, None, deadline_ms, token))
        def picked(rows):
            if not rows:
                return None
            [(index, unit, probability)] = rows
            return {"index": index, "unit": unit, "probability": probability}
        return _mapped(found, picked)

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
        if on is None and not _frames.is_series(records):
            controls = dict(legacy, token=token)
            for key, item in dict(batch=batch, context=context).items():
                if item is not None: controls[key] = item
            if deadline_ms is not _MISSING: controls['deadline_ms'] = deadline_ms
            return self._named('annotate', questions, records, **controls)
        deadline_ms = _due_keyword(deadline_ms, legacy)
        if context is not None:
            raise UsageError("annotate does not take a shared context")
        if isinstance(records, FileSelection):
            if on is not None:
                raise UsageError("source annotate takes question-member on, not frame on")
            return _source_call(self, "annotate", _source_spec(questions), records, batch, None, deadline_ms, token)
        if on is None and _frames.is_series(records):
            return _frames.annotate(self, questions, records, batch, deadline_ms, token)
        asked = _spec(_thinkthen._QuestionSet, questions)
        if on is None:
            return self._engine.annotate(asked, records, batch, deadline_ms, token)
        if "failed" in asked._names():
            raise UsageError("the question name failed is reserved for frame failures")
        if _pandas(records) == "DataFrame":
            column = _on(records, on, (*asked._names(), "failed"))
            answers = _thinkthen._annotate_column(self._engine, asked, _marked(column, "Series"),
                                                  batch, deadline_ms, token)
            def rebuild(columns):
                out = records.assign()
                for name, (values, dtype) in columns.items():
                    out[name] = type(column)(values, index=records.index, dtype=dtype)
                return out
            return _mapped(answers, rebuild)
        result = _thinkthen._annotate_frame(self._engine, asked, records, on,
                                            batch, deadline_ms, token)
        return _mapped(result, lambda value: type(records)(_Stream(value)))

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
        if on is None and not _frames.is_series(text):
            controls = dict(legacy, token=token)
            if ask is None:
                controls.update(kinds=kinds or [], relations=relations, either=either, descriptions=descriptions)
                if instructions is not None: controls['instructions'] = instructions
                if entity_definition is not None: controls['entity_definition'] = entity_definition
            for key, item in dict(threshold=threshold, relation_threshold=relation_threshold).items():
                if item is not None: controls[key] = item
            if deadline_ms is not _MISSING: controls['deadline_ms'] = deadline_ms
            return self._named('recognize', ask, text, **controls)
        deadline_ms = _due_keyword(deadline_ms, legacy)
        if on is not None and relations is not None:
            raise UsageError("recognize with on= takes no relations; ask them of one text")
        if ask is not None:
            if instructions is not None or entity_definition is not None:
                raise UsageError("recognize task keywords take an inline declaration")
            if descriptions is not None:
                raise UsageError("descriptions= takes kinds=, not a recognize ask")
            spec = _spec(_thinkthen._Recognize, ask)
        else:
            named = _labels(kinds or [], descriptions)
            if not isinstance(named, dict):
                named = {name: None for name in named}
            body = {"version": 1, "recognize": {"kinds": named}}
            if instructions is not None:
                body["recognize"]["instructions"] = instructions
            if entity_definition is not None:
                body["recognize"]["entity_definition"] = entity_definition
            rules = _rules(relations, either)
            if rules:
                body["recognize"]["relations"] = [
                    {"name": name, "source": source, "target": target, "either": both}
                    for name, source, target, both in rules]
            if threshold is not None:
                body["threshold"] = threshold
            if relation_threshold is not None:
                body["relation_threshold"] = relation_threshold
            spec = _thinkthen._Recognize._from_json(json.dumps(body))
        if isinstance(text, FileSelection):
            if on is not None:
                raise UsageError("source recognize reads text units; on requires a parser source map")
            return _source_call(self, "recognize", _source_spec(ask) if ask is not None else json.dumps(body), text, None, None, deadline_ms, token)
        if on is not None and _pandas(text) == "DataFrame":
            column = _on(text, on, ["names"])
            found = _thinkthen._recognize_column(self._engine, spec, _marked(column, "Series"),
                                                 deadline_ms, token)
            return _mapped(found, lambda value: text.assign(
                names=type(column)(value, index=text.index, dtype="object")))
        if on is not None:
            result = _thinkthen._recognize_frame(self._engine, spec, text, on, deadline_ms, token)
            return _mapped(result, lambda value: type(text)(_Stream(value)))
        if on is None and _frames.is_series(text):
            return _frames.recognize(self, spec, text, deadline_ms, token)
        return self._engine.recognize(spec, text, deadline_ms, token)

    def relate(self, entities, ask=None, *, relations=None, either=None, threshold=None,
               deadline_ms=_MISSING, token=None, **legacy):
        """Say how the entities relate: a list of ``Edge``.

        ``entities`` holds ``(name, kind)`` pairs, dicts with ``name`` and
        ``kind``, ``Entity`` values, or what ``recognize`` found. A
        ``RecognizedEntity``, or a dict with ``text`` and no ``name``, is
        named by its ``text``. ``relations`` and ``either`` read as for
        ``recognize``.
        """
        if not _frames.is_series(entities):
            controls = dict(legacy, token=token)
            if ask is None: controls.update(relations=relations, either=either)
            if threshold is not None: controls['threshold'] = threshold
            if deadline_ms is not _MISSING: controls['deadline_ms'] = deadline_ms
            return self._named('relate', ask, entities, **controls)
        deadline_ms = _due_keyword(deadline_ms, legacy)
        if ask is not None:
            spec = _spec(_thinkthen._Relate, ask)
        else:
            spec = _thinkthen._Relate._build(_rules(relations, either), threshold)
        if isinstance(entities, FileSelection):
            body = {"version": 1, "relate": {"relations": [
                {"name": name, "source": source, "target": target, "either": both}
                for name, source, target, both in _rules(relations, either)]}}
            if threshold is not None:
                body["threshold"] = threshold
            return _source_call(self, "relate", _source_spec(ask) if ask is not None else json.dumps(body), entities, None, None, deadline_ms, token)
        if _frames.is_series(entities):
            entities = _frames.entities(entities)
        return self._engine.relate(spec, entities, deadline_ms, token)

    def usage(self):
        """This engine's totals: requests sent, cache answers, and tokens."""
        return self._engine.usage()

    def __getattr__(self, name):
        if name in ("decide_many", "choose_many", "score_many", "tag_many"):
            raise AttributeError(f"thinkthen: {name} was removed; apply the judge to a list: tt.decide(q)(rows)")
        raise AttributeError(name)


_process = None


def _engine():
    global _process
    if _process is None:
        engine = Engine.__new__(Engine)
        engine._engine = _thinkthen._Engine._process()
        engine._settings_json = None
        engine._sessions = set()
        _process = engine
    return _process


def decide(question, text=_MISSING, *, token=None, **keywords):
    return _module_judged("decide", question, text, token, keywords)


def choose(question, text=_MISSING, *, token=None, **keywords):
    return _module_judged("choose", question, text, token, keywords)


def score(question, text=_MISSING, *, token=None, **keywords):
    return _module_judged("score", question, text, token, keywords)


def tag(question, text=_MISSING, *, token=None, **keywords):
    return _module_judged("tag", question, text, token, keywords)


def details(question, text, **keywords):
    return _engine().details(question, text, **keywords)


def filter(question, records=_MISSING, *, token=None, **keywords):
    return _module_judged("filter", question, records, token, keywords)


def _module_judged(verb, question, value, token, keywords):
    if value is not _MISSING:
        return getattr(_engine(), verb)(question, value, token=token, **keywords)
    fields = dict(keywords)
    deadline_ms = fields.pop("deadline_ms", _MISSING)
    if value is _MISSING and (deadline_ms is not _MISSING or token is not None):
        raise UsageError("deadline_ms and token belong when the judge is applied")
    judge = _make_judge(verb, question, None, fields)
    if value is _MISSING:
        return judge
    return judge(value, deadline_ms=_deadline(deadline_ms), token=token)


def plan(judge, records):
    if not isinstance(judge, Judge):
        raise UsageError("plan takes a ThinkThen Judge")
    return (judge._engine if judge._engine is not None else _engine()).plan(judge, records)


def __getattr__(name):
    if name in ("decide_many", "choose_many", "score_many", "tag_many"):
        raise AttributeError(f"thinkthen: {name} was removed; apply the judge to a list: tt.decide(q)(rows)")
    raise AttributeError(name)


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


for _name in ("decide", "choose", "score", "tag", "details", "filter",
              "rank", "find", "annotate", "recognize", "relate"):
    globals()[_name].__doc__ = getattr(Engine, _name).__doc__
del _name
