"""Complete typed results over pandas and Polars columns, using one native engine."""
from dataclasses import dataclass
from . import UsageError, _frames
from . import complete as c
from . import _complete as carriers


@dataclass(frozen=True, repr=False)
class FrameCompleted:
    """The native result stays intact; presentation positions remain separate."""
    native: c.Completed
    frame: object
    source: object
    positions: tuple[int | None, ...]
    index: object
    name: object

    @property
    def facts(self): return self.native.facts
    @property
    def results(self): return self.native.results
    @property
    def inputs(self): return self.native.inputs
    def __repr__(self): return '<FrameCompleted: content withheld>'


def _series(value, on):
    kind = type(value).__module__.partition('.')[0]
    if kind == 'polars' and type(value).__name__ == 'LazyFrame':
        value = value.collect()
    if type(value).__name__ == 'DataFrame':
        if on is None: raise UsageError('select a dataframe column with on=')
        value = value[on]
    elif on is not None:
        raise UsageError('on selects a dataframe column')
    if not _frames.is_series(value): raise UsageError('pass a pandas or Polars Series')
    return value


def _records(value):
    pandas = type(value).__module__.partition('.')[0] == 'pandas'
    rows = list(value) if pandas else value.to_list()
    present, items = [], []
    for at, row in enumerate(rows):
        # Item(value=None) is explicit JSON null; missing cells ask no question.
        missing = row is None
        if pandas and not isinstance(row, (c.Item, str)):
            marker = __import__('pandas').isna(row)
            missing = isinstance(marker, bool) and marker
        if missing: continue
        if isinstance(row, str): row = c.Item(value=row, text=True)
        if not isinstance(row, c.Item):
            raise UsageError('complete columns contain text or explicit complete.Item values')
        present.append(at); items.append(row)
    return c.Records(tuple(items)), tuple(present)


def _presentation(source, library):
    return (source.index.copy() if library == 'pandas' else tuple(range(len(source)))), source.name


def _presented(native, source, present, verb, library, presentation):
    positions = tuple(None if at is None else present[at] for at in native.ordinals)
    per_row = verb in ('decide', 'choose', 'tag', 'score', 'annotate', 'recognize')
    if source is None:
        labels, name = tuple(range(len(native.inputs))), 'result'
    else:
        labels, name = presentation
    if per_row:
        rows = [None] * len(labels)
        for result, at in zip(native.results, positions, strict=True): rows[at] = result
        selected = labels
    else:
        rows = list(native.results)
        # Aggregate find/relate can have no selected original row.
        selected = (labels.take(positions) if library == 'pandas' and source is not None
                    and all(at is not None for at in positions)
                    else [labels[at] if at is not None else None for at in positions])
    if library == 'pandas':
        import pandas as pd
        output = pd.Series(rows, index=selected, name=name, dtype='object')
    else:
        import polars as pl
        output = pl.Series(name, rows, dtype=pl.Object)
    return FrameCompleted(native, output, source, positions, labels, name)


class Engine:
    """Share an ordinary or complete Engine; no dataframe cache or scheduler exists."""
    def __init__(self, *, engine=None, library='pandas', **settings):
        if library not in ('pandas', 'polars'): raise UsageError('library is pandas or polars')
        if engine is not None and settings: raise UsageError('settings belong to the selected engine')
        self._engine = c.Engine(**settings) if engine is None else (engine if isinstance(engine, c.Engine) else engine.complete)
        self._library = library
    def __repr__(self): return '<CompleteFrameEngine>'

    def _call(self, verb, question, source, *, on=None, **controls):
        if isinstance(source, (c.Files, carriers.Files)):
            if on is not None: raise UsageError('on selects a dataframe column')
            original = None
            presentation = None
            inputs = source
            present = None
            library = self._library
        else:
            original = _series(source, on)
            library = 'pandas' if type(original).__module__.partition('.')[0] == 'pandas' else 'polars'
            presentation = _presentation(original, library)
            inputs, present = _records(original)
        done = getattr(self._engine, verb)(question, inputs, **controls)
        if present is None: present = tuple(range(len(done.inputs)))
        return _presented(done, original, present, verb, library, presentation)

    def decide(self, question, source, **controls): return self._call('decide', question, source, **controls)
    def choose(self, question, source, **controls): return self._call('choose', question, source, **controls)
    def tag(self, question, source, **controls): return self._call('tag', question, source, **controls)
    def score(self, question, source, **controls): return self._call('score', question, source, **controls)
    def filter(self, question, source, **controls): return self._call('filter', question, source, **controls)
    def rank(self, question, source, **controls): return self._call('rank', question, source, **controls)
    def find(self, question, source, **controls): return self._call('find', question, source, **controls)
    def annotate(self, question, source, **controls): return self._call('annotate', question, source, **controls)
    def recognize(self, question, source, **controls): return self._call('recognize', question, source, **controls)
    def relate(self, question, source, **controls): return self._call('relate', question, source, **controls)

    def _batch(self, verb, question, source, *, on=None, **controls):
        if isinstance(source, (c.Files, carriers.Files)):
            if on is not None: raise UsageError('on selects a dataframe column')
            original, inputs, present = None, source, None
        else:
            original = _series(source, on)
            inputs, present = _records(original)
        native = getattr(self._engine, verb + '_batch')(question, inputs, **controls)
        return FrameBatch(native, original, present)

    def decide_batch(self, question, source, **controls): return self._batch('decide', question, source, **controls)
    def choose_batch(self, question, source, **controls): return self._batch('choose', question, source, **controls)
    def tag_batch(self, question, source, **controls): return self._batch('tag', question, source, **controls)
    def score_batch(self, question, source, **controls): return self._batch('score', question, source, **controls)
    def filter_batch(self, question, source, **controls): return self._batch('filter', question, source, **controls)
    def annotate_batch(self, question, source, **controls): return self._batch('annotate', question, source, **controls)


class FrameBatch:
    """The native pull iterator with original nullable frame positions."""
    def __init__(self, native, source, present):
        self.native, self.source, self.positions = native, source, present
        library = 'pandas' if source is not None and type(source).__module__.partition('.')[0] == 'pandas' else 'polars'
        self.index, self.name = (None, None) if source is None else _presentation(source, library)
    def __repr__(self): return '<CompleteFrameBatch>'
    def __iter__(self): return self
    def __next__(self): return next(self.native)
    @property
    def facts(self): return self.native.facts
    def position(self, row): return row.ordinal if self.positions is None else self.positions[row.ordinal]
    def close(self): self.native.close()
    def cancel(self): self.native.cancel()
    def __enter__(self): return self
    def __exit__(self, *args): self.close()
