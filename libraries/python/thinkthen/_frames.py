"""Identify optional columns without importing their libraries."""
def is_series(value):
    from . import _pandas
    return _pandas(value) == 'Series' or (type(value).__module__.partition('.')[0] == 'polars' and type(value).__name__ == 'Series')
