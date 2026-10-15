"""Installed dataframe calls retain absent probabilities and nullable columns."""
import pytest
from conftest import child_env, run


@pytest.mark.parametrize('shape', ['pandas', 'polars'])
@pytest.mark.parametrize('frame', [False, True])
@pytest.mark.parametrize('verb', ['score', 'tag', 'annotate'])
def test_functions_without_probability_return_none(backend, tmp_path, shape, frame, verb):
    output = run(f'''
        import pickle, pandas as pd, polars as pl, thinkthen as tt
        import thinkthen.pandas, thinkthen.polars
        shape, frame, verb = {shape!r}, {frame!r}, {verb!r}
        with tt.Engine(cache=False, batch=1) as engine:
            for values in (['first', None, 'second'], [None, None], []):
                source = (pd.Series(values, name='body', dtype=object) if shape == 'pandas'
                          else pl.Series('body', values, dtype=pl.String))
                controls = {{}}
                if frame:
                    source = source.to_frame() if shape == 'pandas' else pl.DataFrame(source)
                    controls['on'] = 'body'
                question = 'Late?'
                if verb == 'score': controls['levels'] = ['Routine.', 'Soon.', 'Now.']
                if verb == 'tag': controls['labels'] = ['bill', 'ship']
                if verb == 'annotate':
                    question = {{'version': 1, 'questions': {{'late': {{'decide': 'Late?'}}}}}}
                result = (getattr(engine, verb)(question, source, **controls) if frame
                          else getattr(source.tt, verb)(question, engine=engine, **controls))
                assert result.probability is None
                assert pickle.loads(pickle.dumps(result)).probability is None
                assert result.facts.records == len(result.results) == sum(value is not None for value in values)
                assert result.facts.requests_sent == sum(value is not None for value in values)
        print('absent probability')
    ''', child_env(backend, tmp_path))
    assert output.strip() == 'absent probability'
    assert backend.count() == 2


@pytest.mark.parametrize('shape', ['pandas', 'polars'])
@pytest.mark.parametrize('frame', [False, True])
def test_unresolved_and_empty_answers_keep_probability_columns(backend, tmp_path, shape, frame):
    output = run(f'''
        import pickle, pandas as pd, polars as pl, thinkthen as tt
        import thinkthen.pandas, thinkthen.polars
        shape, frame = {shape!r}, {frame!r}
        with tt.Engine(cache=False, batch=1) as engine:
            for values in (['first', None, 'second'], [None, None], []):
                source = (pd.Series(values, name='body', dtype=object) if shape == 'pandas'
                          else pl.Series('body', values, dtype=pl.String))
                controls = {{}}
                if frame:
                    source = source.to_frame() if shape == 'pandas' else pl.DataFrame(source)
                    controls['on'] = 'body'
                for verb in ('decide', 'choose', 'filter', 'find', 'rank'):
                    options = {{'options': ['billing', 'shipping']}} if verb == 'choose' else {{}}
                    if verb == 'decide': options['threshold'] = '0.2:0.95'
                    if verb == 'filter': options['threshold'] = .95
                    if verb == 'choose': options['threshold'] = .99
                    if verb == 'find': options['none'] = True
                    try:
                        result = (getattr(engine, verb)('Late?', source, on='body', **options) if frame
                                  else getattr(source.tt, verb)('Late?', engine=engine, **options))
                    except tt.UsageError:
                        assert verb == 'find' and not any(value is not None for value in values)
                        continue
                    probability = result.probability
                    assert isinstance(probability, pd.Series if shape == 'pandas' else pl.Series)
                    assert probability.dtype == ('Float64' if shape == 'pandas' else pl.Float64)
                    assert len(probability) == len(values)
                    assert probability.name == 'body'
                    restored = pickle.loads(pickle.dumps(result)).probability
                    assert restored.dtype == probability.dtype
                    if verb in ('decide', 'choose') or not any(value is not None for value in values):
                        assert (probability.isna().all() if shape == 'pandas' else probability.null_count() == len(values)), (verb, probability.to_list())
                    if shape == 'pandas': assert probability.index.equals(source.index)
                    else: assert restored.to_list() == probability.to_list()
        print('nullable probability columns')
    ''', child_env(backend, tmp_path))
    assert output.strip() == 'nullable probability columns'
    assert backend.count() == 9
