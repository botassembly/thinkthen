"""Opt-in Polars Series namespace: ``import thinkthen.polars``."""
import polars as pl
from . import _engine


@pl.api.register_series_namespace("tt")
class ThinkThenNamespace:
    """Named calls retain native rows and present values as Object columns."""
    def __init__(self, series):
        self._series = series

    def _call(self, verb, *args, engine=None, **controls):
        selected = _engine() if engine is None else engine
        if verb in ('recognize', 'relate'):
            return getattr(selected, verb)(self._series, *args, **controls)
        return getattr(selected, verb)(*args, self._series, **controls)
    def decide(self, question, *, engine=None, **controls):
        return self._call('decide', question, engine=engine, **controls)
    def choose(self, question, *, engine=None, **controls):
        return self._call('choose', question, engine=engine, **controls)
    def score(self, question, *, engine=None, **controls):
        return self._call('score', question, engine=engine, **controls)
    def tag(self, question, *, engine=None, **controls):
        return self._call('tag', question, engine=engine, **controls)
    def filter(self, question, *, engine=None, **controls):
        return self._call('filter', question, engine=engine, **controls)
    def rank(self, question, *, engine=None, **controls):
        return self._call('rank', question, engine=engine, **controls)
    def find(self, question, *, engine=None, **controls):
        return self._call('find', question, engine=engine, **controls)
    def annotate(self, question, *, engine=None, **controls):
        return self._call('annotate', question, engine=engine, **controls)
    def recognize(self, ask=None, *, engine=None, **controls):
        return self._call('recognize', ask, engine=engine, **controls)
    def relate(self, ask=None, *, engine=None, **controls):
        return self._call('relate', ask, engine=engine, **controls)
