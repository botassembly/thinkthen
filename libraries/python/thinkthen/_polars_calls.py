"""Polars column presentation over owned canonical results."""
from dataclasses import dataclass
from ._calls import Result
from ._pandas_calls import PandasResult


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
        import polars as pl
        return pl.Series(self.source.name, list(rows), dtype=pl.Float64 if self.function == 'score' and not self.detailed else pl.Object)

    def _selected(self):
        return self.source.gather(list(self.positions))

    def __reduce__(self):
        return (_restore, (self.results, self.terminal, self.function, self.scalar, self.detailed,
                           self.source.name, self.source.to_list(), self.source.dtype, self.present))


def _restore(results, terminal, function, scalar, detailed, name, values, dtype, present):
    import polars as pl
    return PolarsResult(results, terminal, function, scalar, detailed,
                        pl.Series(name, values, dtype=dtype), present)
