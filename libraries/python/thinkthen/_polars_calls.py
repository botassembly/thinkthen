"""Polars column presentation over owned canonical results."""
from dataclasses import dataclass
import json
from ._calls import Result, _json
from ._pandas_calls import PandasResult


def _plain(value):
    if hasattr(value, 'failed'): raise TypeError('embedded failures retain native behavior')
    if isinstance(value, dict): return {key: _plain(item) for key, item in value.items()}
    if isinstance(value, (list, tuple)): return [_plain(item) for item in value]
    return _json(value)


def _dtype(rows, schema=None):
    import polars as pl
    values = [value for value in rows if value is not None]
    schema = getattr(values[0], '_column_schema', schema) if values else schema
    from ._native_results import _COLUMN_SCHEMAS
    while isinstance(schema, tuple): schema = _COLUMN_SCHEMAS[schema[1]]
    values = [_plain(value) for value in values]
    kinds = dict(boolean=pl.Boolean, string=pl.String, integer=pl.Int64, number=pl.Float64)
    if values and all(type(value) is int for value in values) and min(values) < 0 and max(values) >= 2**63: return pl.Int128
    if isinstance(schema, str):
        return pl.Series(values, strict=True).dtype if schema == 'integer' and values else kinds[schema]
    if not values:
        if isinstance(schema, dict): return pl.Struct({key: _dtype([], hint) for key, hint in schema.items()})
        return pl.List(_dtype([], schema[0])) if isinstance(schema, list) else pl.Null
    if all(isinstance(value, dict) for value in values):
        keys = values[0].keys()
        if any(value.keys() != keys for value in values): raise TypeError('different member presence')
        return pl.Struct({key: _dtype([value[key] for value in values], schema.get(key) if isinstance(schema, dict) else None) for key in keys})
    if all(isinstance(value, list) for value in values):
        return pl.List(_dtype([item for value in values for item in value], schema[0] if isinstance(schema, list) else None))
    return pl.Series(values, strict=True).dtype


def answer_column(name, rows, function, detailed):
    import polars as pl
    rows = list(rows)
    if not detailed:
        try:
            values = [_plain(value) for value in rows]
            dtype = _dtype(rows)
            if dtype == pl.Null:
                dtype = dict(decide=pl.Boolean, choose=pl.String, score=pl.Float64, tag=pl.List(pl.String)).get(function, dtype)
            column = pl.Series(name, values, dtype=dtype, strict=True)
            if json.dumps(column.to_list(), sort_keys=True) == json.dumps(values, sort_keys=True): return column
        except (TypeError, OverflowError, pl.exceptions.PolarsError):
            pass
    return pl.Series(name, rows, dtype=pl.Object)


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
    def _to_dict(self):
        return {**Result._to_dict(self), 'present': list(self.present), 'name': self.source.name}

    def _column(self, rows, positions=None):
        return answer_column(self.source.name, rows, self.function, self.detailed)

    @property
    def value(self):
        value = super().value
        if self.detailed or self.function not in ('rank', 'find', 'relate'): return value
        return self._column([value] if self.function == 'find' else value)

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
