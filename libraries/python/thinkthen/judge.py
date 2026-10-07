"""One captured question and its public eager or lazy input shape."""

from . import _thinkthen


class Judge:
    """An immutable question and its construction-bound call settings."""

    __slots__ = ("_verb", "_asked", "_engine", "_batch", "_context", "_probability", "_tally")

    def __init__(self, verb, asked, engine, batch, context, probability, tally):
        for name, value in (("_verb", verb), ("_asked", asked), ("_engine", engine),
                            ("_batch", batch), ("_context", context),
                            ("_probability", probability), ("_tally", tally)):
            object.__setattr__(self, name, value)

    def __setattr__(self, name, value):
        raise AttributeError("a ThinkThen Judge is immutable")

    def __repr__(self):
        return f"Judge(kind={self._verb!r})"

    def __reduce__(self):
        if self._tally is not None:
            raise TypeError("a judge bound to a tally cannot be pickled")
        settings = None if self._engine is None else self._engine._settings_json
        return (_restore, (self._verb, self._asked._json(), self._batch,
                           self._context, self._probability, settings))

    def __call__(self, value, *, deadline_ms=-1, token=None, **keywords):
        from . import UsageError, _column, _deadline, _paired, _pandas, _engine

        if keywords:
            key = sorted(keywords)[0]
            raise UsageError(f"the settings key `{key}` belongs when the judge is built")
        deadline_ms = _deadline(deadline_ms)
        engine = self._engine if self._engine is not None else _engine()
        from .files import FileSelection, call as source_call
        if isinstance(value, FileSelection):
            if self._tally is not None or self._probability:
                raise UsageError("source judgments keep values and call facts; tally and probability are not supported")
            return source_call(engine, self._verb, self._asked._json(), value,
                               self._batch, self._context, deadline_ms, token)
        native = engine._engine
        verb = self._verb
        asked = self._asked
        tally = self._tally
        if isinstance(value, str):
            if verb == "filter":
                raise UsageError("filter reads records; pass a list or iterator")
            call = native.ask(verb, asked, value, deadline_ms, token,
                              self._batch, self._context, tally)
            return _paired(call, verb) if verb in ("decide", "choose") else call
        if isinstance(value, (bytes, bytearray)):
            raise UsageError("the input is text, an ordered collection, a column, or an iterator")
        if isinstance(value, (set, frozenset)):
            raise UsageError("an unordered set cannot align records with answers")
        kind = _pandas(value)
        if kind == "DataFrame":
            raise UsageError('a data frame is not a column; pass df["name"], or annotate with on=')
        if kind == "Series" or (kind is None and
                                (hasattr(value, "__arrow_c_stream__") or
                                 hasattr(value, "__arrow_c_array__"))):
            if verb == "filter":
                from ._frames import filter_series
                return filter_series(engine, asked, value, self._batch, self._context,
                                     deadline_ms, token, tally)
            call = _column(lambda selected, question, column, due, held:
                           native.ask(selected, question, column, due, held,
                                      self._batch, self._context, tally),
                           verb, asked, value, deadline_ms, token)
            return _paired(call, verb, True) if verb in ("decide", "choose") else call
        try:
            iterator = iter(value)
        except TypeError as source:
            raise UsageError("the input is text, an ordered collection, a column, or an iterator") from source
        if iterator is value:
            from .stream import Stream
            return Stream(lambda: native._stream(verb, asked, iterator, self._batch,
                                                 self._context, deadline_ms, token, tally),
                          probability=self._probability)
        value = list(iterator)  # Every reiterable ordered input is eager, including Index.
        if verb in ("decide", "filter"):
            call = native.many("decide_many" if verb == "decide" else verb,
                               asked, value, self._batch, self._context,
                               deadline_ms, token, tally)
        else:
            call = native.label_many(verb, asked, value, self._batch,
                                     self._context, deadline_ms, token, tally)
        return _paired(call, verb, True) if verb in ("decide", "choose") else call


def _restore(verb, source, batch, context, probability, engine_settings):
    from . import Engine, Question
    import json
    engine = None if engine_settings is None else Engine(**json.loads(engine_settings))
    return Judge(verb, Question._from_json(source), engine, batch, context, probability, None)


def make(verb, question, engine, keywords):
    from . import UsageError, _configured
    fields = dict(keywords)
    probability = fields.pop("probability", False)
    tally = fields.pop("tally", None)
    if type(probability) is not bool:
        raise UsageError("probability is True or False")
    if probability and verb not in ("decide", "choose"):
        raise UsageError("probability=True belongs to decide or choose")
    if tally is not None and not isinstance(tally, _thinkthen.Tally):
        raise UsageError("tally is a tt.Tally")
    asked, batch, context, deadline = _configured(
        "decide" if verb == "filter" else verb, question, fields)
    if deadline is not None:
        raise UsageError("deadline_ms belongs when the judge is applied")
    if asked.kind != ("decide" if verb == "filter" else verb):
        raise UsageError(f"{verb} does not take a {asked.kind} question")
    return Judge(verb, asked, engine, batch, context, probability, tally)
