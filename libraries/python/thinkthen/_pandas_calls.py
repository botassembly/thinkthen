"""Pandas presentation over owned canonical Python results."""
from dataclasses import dataclass
from ._calls import Result


def records(source, verb):
    import pandas as pd
    from .complete import Item
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

    def to_dict(self):
        return {**super().to_dict(), 'present': list(self.present),
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
            if self.detailed:
                return self._column(self.results, self.positions)
            return self._selected()
        value = super().value
        if self.function == 'rank' and not self.detailed:
            return [dict(row, index=self.present[row['index']]) for row in value]
        if self.function == 'find' and not self.detailed:
            return None if value is None else dict(value, index=self.present[value['index']])
        if self.function == 'relate' and not self.detailed:
            return value[0] if value else []
        if self.function in ('rank', 'find', 'relate'):
            return value
        rows = [None] * len(self.source)
        for at, item in zip(self.positions, value, strict=True):
            rows[at] = item
        return self._column(rows)

    @property
    def probability(self):
        values = super().probability
        if self.function in ('find', 'rank', 'relate'):
            return values
        rows = [None] * len(self.source)
        for at, value in zip(self.positions, values, strict=True):
            rows[at] = value
        return self._column(rows)
