"""Opt-in pandas Series accessors: ``import thinkthen.pandas``.

Every route delegates to the selected public Engine (or the module engine)
and returns its owned Result with native facts and rows intact. Importing thinkthen alone
does not import pandas or register this accessor.
"""
import pandas as pd

from . import _engine


@pd.api.extensions.register_series_accessor("tt")
class ThinkThenAccessor:
    """Ask all ten functions of a Series without losing its row identity."""

    def __init__(self, series):
        self._series = series

    def _call(self, verb, *args, engine=None, **keywords):
        selected = _engine() if engine is None else engine
        if verb in ("recognize", "relate"):
            return getattr(selected, verb)(self._series, *args, **keywords)
        return getattr(selected, verb)(*args, self._series, **keywords)

    def decide(self, question, *, engine=None, **keywords):
        return self._call("decide", question, engine=engine, **keywords)

    def choose(self, question, *, engine=None, **keywords):
        return self._call("choose", question, engine=engine, **keywords)

    def score(self, question, *, engine=None, **keywords):
        return self._call("score", question, engine=engine, **keywords)

    def tag(self, question, *, engine=None, **keywords):
        return self._call("tag", question, engine=engine, **keywords)

    def filter(self, question, *, engine=None, **keywords):
        return self._call("filter", question, engine=engine, **keywords)

    def rank(self, question, *, engine=None, **keywords):
        return self._call("rank", question, engine=engine, **keywords)

    def find(self, question, *, engine=None, **keywords):
        return self._call("find", question, engine=engine, **keywords)

    def annotate(self, questions, *, engine=None, **keywords):
        return self._call("annotate", questions, engine=engine, **keywords)

    def recognize(self, ask=None, *, engine=None, **keywords):
        return self._call("recognize", ask, engine=engine, **keywords)

    def relate(self, ask=None, *, engine=None, **keywords):
        return self._call("relate", ask, engine=engine, **keywords)

    def complete(self, *, engine=None):
        """Select complete typed calls while retaining this Series presentation."""
        from .frames import Engine
        return CompleteAccessor(self._series, Engine(engine=engine))


class CompleteAccessor:
    """A complete engine bound to the accessor's original Series."""
    def __init__(self, series, engine):
        self._series, self._engine = series, engine
    def _call(self, verb, question, **controls):
        return getattr(self._engine, verb)(question, self._series, **controls)
    def decide(self, question, **controls): return self._call("decide", question, **controls)
    def choose(self, question, **controls): return self._call("choose", question, **controls)
    def tag(self, question, **controls): return self._call("tag", question, **controls)
    def score(self, question, **controls): return self._call("score", question, **controls)
    def filter(self, question, **controls): return self._call("filter", question, **controls)
    def rank(self, question, **controls): return self._call("rank", question, **controls)
    def find(self, question, **controls): return self._call("find", question, **controls)
    def annotate(self, question, **controls): return self._call("annotate", question, **controls)
    def recognize(self, question, **controls): return self._call("recognize", question, **controls)
    def relate(self, question, **controls): return self._call("relate", question, **controls)
