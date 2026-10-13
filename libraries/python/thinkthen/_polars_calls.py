"""Polars column presentation over owned canonical results."""
from dataclasses import dataclass
from ._calls import Result
from ._pandas_calls import PandasResult


def answer_column(name, rows, function, detailed):
    import polars as pl
    kinds = dict(decide=pl.Boolean, choose=pl.String, score=pl.Float64, tag=pl.List(pl.String))
    dtype = pl.Object if detailed else kinds.get(function, pl.Object)
    return pl.Series(name, list(rows), dtype=dtype)


def probability_column(result, name):
    import polars as pl
    values = Result.probability.fget(result)
    if result.function in ('find', 'rank', 'relate'):
        return values
    rows = [None] * len(result.source)
    for at, value in zip(result.positions, values, strict=True):
        rows[at] = value
    return pl.Series(name, rows, dtype=pl.Float64)


def records(source, verb):
    from ._inputs import Item
    positions, values = [], []
    for at, value in enumerate(source):
        if value is None:
            continue
        positions.append(at)
        values.append(value if verb == 'relate' or isinstance(value, Item) else Item(value=value, text=True))
    return values, tuple(positions)


@dataclass(frozen=True, repr=False, eq=False)
class PolarsResult(PandasResult):
    def to_dict(self):
        return {**Result.to_dict(self), 'present': list(self.present), 'name': self.source.name}

    def _column(self, rows, positions=None):
        return answer_column(self.source.name, rows, self.function, self.detailed)

    @property
    def probability(self):
        return probability_column(self, self.source.name)

    def _selected(self):
        return self.source.gather(list(self.positions))

    def __reduce__(self):
        return (_restore, (self.results, self.terminal, self.function, self.scalar, self.detailed,
                           self.source.name, self.source.to_list(), self.source.dtype, self.present))


def _restore(results, terminal, function, scalar, detailed, name, values, dtype, present):
    import polars as pl
    return PolarsResult(results, terminal, function, scalar, detailed,
                        pl.Series(name, values, dtype=dtype), present)
