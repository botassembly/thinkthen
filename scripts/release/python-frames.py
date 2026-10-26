"""Check optional column namespaces in the installed CPython 3.10 environment."""
import sys
import pandas as pd
import polars as pl
import thinkthen as tt
import thinkthen.pandas
import thinkthen.polars

assert sys.version_info[:2] == (3, 10), sys.version
client = tt.Client(backend='stub')
source = pd.Series(['alpha', None, 'evidence 5'], index=['x', 'x', 'z'], dtype=object)
result = source.thinkthen.decide('Ready?', client=client)
assert result.index.equals(source.index)
assert result['position'].tolist() == [1, 2, 3]
assert result['answer'].iloc[[0, 2]].tolist() == [True, True]
assert pd.isna(result['answer'].iloc[1]) and result['status'].iloc[1] == 'failed'
assert result['error_code'].iloc[1] == 'record.missing'
assert 'probability' in result.columns and 'p' not in result.columns
result = pl.Series('body', ['alpha', None, 'evidence 5']).thinkthen.decide('Ready?', client=client).struct.unnest()
assert result['position'].to_list() == [1, 2, 3]
assert result['answer'].to_list() == [True, None, True]
assert result['status'].to_list() == ['answered', 'failed', 'answered']
assert result['error'].to_list()[1]['code'] == 'record.missing'
assert 'probability' in result.columns and 'p' not in result.columns
print('candidate pandas and Polars: installed CPython 3.10 namespaces pass')
