"""Frame conversion and presentation over the canonical named session."""
from dataclasses import dataclass
from ._pandas_calls import PandasResult
from ._calls import Result


def library(value):
    from . import _pandas
    if _pandas(value) == 'DataFrame':
        return 'pandas'
    if type(value).__module__.partition('.')[0] == 'polars' and type(value).__name__ == 'DataFrame':
        return 'python-polars'
    return None


def members(question):
    from . import _thinkthen as native
    from ._calls import _dump
    request = {'schema': 'thinkthen.request/1', 'call': {'function': 'annotate',
               'question': question, 'input': {'kind': 'records', 'items': []}, 'options': {}}}
    spec = native._QuestionSet._resolve(_dump(request))
    return tuple(spec._members())


def records(source, verb, controls, surface, members=()):
    from . import _on, UsageError
    from .complete import Item
    on = controls.pop('on', None)
    if on is None:
        raise UsageError('a data frame is not a column; pass df["name"], or annotate with on=')
    names = [on] if verb in ('annotate', 'recognize') or not isinstance(on, list) else on
    new = [name for name, _ in members] + ['failed'] if verb == 'annotate' else ['names'] if verb == 'recognize' and surface == 'pandas' else []
    if verb == 'annotate' and 'failed' in dict(members):
        raise UsageError('the question name failed is reserved for frame failures')
    if surface == 'pandas':
        columns = [_on(source, name, new) for name in names]
        def missing(value):
            import pandas as pd
            mark = False if isinstance(value, Item) else pd.isna(value)
            return isinstance(mark, bool) and mark
    else:
        for name in names:
            if name not in source.columns:
                raise UsageError(f'the frame has no column named {name!r}')
        for name in new:
            if name in source.columns:
                raise UsageError(f'the frame already has a column named {name!r}; rename it first')
        columns = [source.get_column(name) for name in names]
        def missing(value): return value is None
    # Field selectors must retain complete records for native context and shortlist projection.
    projected = len(names) != 1 or any(key.endswith('_field') for key in controls) or 'field' in controls
    rows = (source.to_dict(orient='records') if surface == 'pandas' else source.to_dicts()) if projected else None
    values, positions = [], []
    for at, cells in enumerate(zip(*columns)):
        if all(missing(cell) for cell in cells):
            continue
        positions.append(at)
        if projected:
            row = {name: None if missing(value) else value for name, value in rows[at].items()}
            values.append(Item(value=row))
        else:
            cell = cells[0]
            values.append(cell if isinstance(cell, Item) or verb == 'relate' else Item(value=cell, text=True))
    if projected and 'field' not in controls:
        controls['field'] = ['/' + str(name).replace('~', '~0').replace('/', '~1') for name in names]
    return values, tuple(positions), on


@dataclass(frozen=True, repr=False, eq=False)
class FrameResult(PandasResult):
    on: object = None
    members: tuple = ()

    def to_dict(self):
        return {**Result.to_dict(self), 'present': list(self.present), 'columns': list(self.source.columns)}

    def _column(self, rows, positions=None):
        import pandas as pd
        index = self.source.index if positions is None else self.source.index.take(positions)
        return pd.Series(rows, index=index, name=self.on, dtype=object)

    def _annotation_columns(self):
        columns = {name: [None] * len(self.source) for name, _ in self.members}
        failures = [None] * len(self.source)
        for at, result in zip(self.positions, self.results, strict=True):
            for name, value in result.value.items():
                if hasattr(value, 'failed'):
                    if failures[at] is None: failures[at] = {}
                    failures[at][name] = value.to_dict()
                else:
                    columns[name][at] = value
        return columns, failures

    def _annotated(self):
        import pandas as pd
        columns, failures = self._annotation_columns()
        out = self.source.copy()
        kinds = dict(decide='boolean', choose='string', score='Float64', tag='object')
        for name, kind in self.members:
            out[name] = pd.Series(columns[name], index=out.index, dtype=kinds[kind])
        out['failed'] = pd.Series(failures, index=out.index, dtype=object)
        return out

    @property
    def value(self):
        if self.function == 'annotate' and not self.detailed:
            return self._annotated()
        if self.function != 'recognize' or self.detailed:
            return super().value
        rows = [None] * len(self.source)
        for at, result in zip(self.positions, self.results, strict=True):
            rows[at] = [entity.to_dict() for entity in result.value.entities]
        out = self.source.copy()
        out['names'] = self._column(rows)
        return out


@dataclass(frozen=True, repr=False, eq=False)
class PolarsFrameResult(FrameResult):
    def _column(self, rows, positions=None):
        import polars as pl
        return pl.Series(str(self.on), list(rows), dtype=pl.Object)

    def _selected(self):
        return self.source[list(self.positions), :]

    def __reduce__(self):
        columns = [(name, self.source[name].to_list(), self.source[name].dtype) for name in self.source.columns]
        return (_restore, (self.results, self.terminal, self.function, self.scalar, self.detailed,
                           columns, self.present, self.on, self.members))

    def _annotated(self):
        import polars as pl
        columns, failures = self._annotation_columns()
        kinds = dict(decide=pl.Boolean, choose=pl.String, score=pl.Float64, tag=pl.List(pl.String))
        marker = pl.Struct({'failed': pl.Struct({'kind': pl.String, 'cause': pl.String})})
        failure_type = pl.Struct({name: marker for name, _ in self.members})
        added = [pl.Series(name, columns[name], dtype=kinds[kind]) for name, kind in self.members]
        added.append(pl.Series('failed', failures, dtype=failure_type))
        return self.source.with_columns(added)

    @property
    def value(self):
        if self.function == 'annotate' and not self.detailed:
            return self._annotated()
        if self.function != 'recognize' or self.detailed:
            return PandasResult.value.fget(self)
        import polars as pl
        rows = [dict(row=at + 1, **entity.to_dict())
                for at, result in zip(self.positions, self.results, strict=True)
                for entity in result.value.entities]
        schema = dict(row=pl.Int64, text=pl.String, start=pl.Int64, end=pl.Int64,
                      length=pl.Int64, kind=pl.String, strength=pl.Float64)
        return pl.DataFrame(rows, schema=schema)


def _restore(results, terminal, function, scalar, detailed, columns, present, on, members):
    import polars as pl
    source = pl.DataFrame([pl.Series(name, values, dtype=dtype) for name, values, dtype in columns])
    return PolarsFrameResult(results, terminal, function, scalar, detailed, source, present, on, members)
