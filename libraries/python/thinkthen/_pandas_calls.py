"""Pandas presentation over owned canonical Python results."""
from dataclasses import dataclass
from ._calls import Result


def records(source, verb):
    import pandas as pd
    from ._inputs import Item
    positions, values = [], []
    for at, value in enumerate(source):
        marker = False if isinstance(value, Item) else pd.isna(value)
        if isinstance(marker, bool) and marker:
            continue
        positions.append(at)
        values.append(value if verb == 'relate' or isinstance(value, Item) else Item(value=value, text=True))
    return values, tuple(positions)


@dataclass(frozen=True, repr=False, eq=False)
class PandasResult(Result):
    source: object = None
    present: tuple = ()

    def _to_dict(self):
        return {**super()._to_dict(), 'present': list(self.present),
                'index': self.source.index.tolist(), 'name': self.source.name}

    def _column(self, rows, positions=None):
        import pandas as pd
        index = self.source.index if positions is None else self.source.index.take(positions)
        return pd.Series(rows, index=index, name=self.source.name, dtype=object)

    def _selected(self):
        return self.source.iloc[list(self.positions)].copy()

    @property
    def positions(self):
        return tuple(None if getattr(row, 'index', None) is None else self.present[row.index]
                     for row in self.results)

    @property
    def value(self):
        if self.function == 'filter':
            return self._selected()
        value = super().value
        if self.function == 'rank':
            return [self.present[index] for index in value]
        if self.function in ('rank', 'find', 'relate'):
            return value
        rows = [None] * len(self.source)
        for at, item in zip(self.positions, value, strict=True):
            rows[at] = item
        return self._column(rows)

    def _probability_rows(self):
        values = Result.probability.fget(self)
        rows = [None] * len(self.source)
        if self.function == 'relate':
            # Every admitted row belongs to the same whole-set ordered edge answer.
            for at in self.present:
                rows[at] = list(values or ())
            return rows
        if self.function == 'find':
            for row in self.results:
                if row.index is not None: rows[self.present[row.index]] = values
            return rows
        if values is None: values = [None] * len(self.results)
        for at, value in zip(self.positions, values, strict=True):
            rows[at] = value
        return rows

    @property
    def probability(self):
        import pandas as pd
        rows = self._probability_rows()
        name = getattr(self.source, 'name', getattr(self, 'on', None))
        if self.function in ('recognize', 'relate'):
            import pyarrow as pa
            return pd.Series(rows, index=self.source.index, name=name,
                             dtype=pd.ArrowDtype(pa.list_(pa.float64())))
        return pd.Series(rows, index=self.source.index, name=name, dtype='Float64')
