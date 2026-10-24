"""Check optional column namespaces in the installed CPython 3.10 environment."""
import sys
import pandas as pd
import polars as pl
import thinkthen as tt
import thinkthen.pandas
import thinkthen.polars

assert sys.version_info[:2] == (3, 10), sys.version
client = tt.Client(adapter='stub')
source = pd.Series(['alpha', None, 'evidence 5'], index=['x', 'x', 'z'], dtype=object)
result = source.thinkthen.decide('Ready?', client=client)
assert result.index.equals(source.index)
assert result['position'].tolist() == [0, 1, 2]
assert result['answer'].iloc[[0, 2]].tolist() == [False, True]
assert pd.isna(result['answer'].iloc[1]) and pd.isna(result['status'].iloc[1])
result = pl.Series('body', ['alpha', None, 'evidence 5']).thinkthen.decide('Ready?', client=client)
assert result['position'].to_list() == [0, 1, 2]
assert result['answer'].to_list() == [False, None, True]
assert result['status'].to_list() == ['answered', None, 'answered']
print('candidate pandas and Polars: installed CPython 3.10 namespaces pass')
