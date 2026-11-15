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
result = source.thinkthen.decide(
    'Ready?', client=client, threshold=(0.2, 0.8), details=True, errors='coerce',
)
assert isinstance(result, pd.DataFrame)
assert len(result) == 3
assert result.index.equals(source.index)
assert result['position'].tolist() == [1, 2, 3]
assert result['status'].tolist() == ['answered', 'failed', 'unsure']
assert result['answer'].iloc[0]
assert result['answer'].iloc[[1, 2]].isna().all()
# The deterministic stub gives 0.85 and 0.60 for this question and band.
assert result['probability'].iloc[[0, 2]].tolist() == [0.85, 0.60]
assert pd.isna(result['probability'].iloc[1])
assert result['error_code'].iloc[1] == 'record.missing'
assert isinstance(result['error_message'].iloc[1], str)
assert result['error_message'].iloc[1].strip()
for column in ('error_code', 'error_message'):
    assert result[column].iloc[[0, 2]].isna().all()
assert result['error_http_status'].isna().all()
assert 'probability' in result.columns and 'p' not in result.columns
assert 'error' not in result.columns
result = pl.Series('body', ['alpha', None, 'evidence 5']).thinkthen.decide(
    'Ready?', client=client, threshold=(0.2, 0.8), details=True, errors='coerce',
)
assert isinstance(result, pl.DataFrame)
assert result.height == 3
assert result['position'].to_list() == [1, 2, 3]
assert result['answer'].to_list() == [True, None, None]
assert result['status'].to_list() == ['answered', 'failed', 'unsure']
assert result['probability'].to_list() == [0.85, None, 0.60]
assert result['probability'].is_null().to_list() == [False, True, False]
assert result['error_code'][1] == 'record.missing'
assert isinstance(result['error_message'][1], str)
assert result['error_message'][1].strip()
for column in ('error_code', 'error_message'):
    assert result[column].is_null().to_list() == [True, False, True]
assert result['error_http_status'].is_null().all()
assert 'probability' in result.columns and 'p' not in result.columns
assert 'error' not in result.columns
print('candidate pandas and Polars: installed CPython 3.10 namespaces pass')
