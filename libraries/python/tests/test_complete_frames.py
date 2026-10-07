"""Native complete judgments preserve frame presentation and whole logical sets."""
import pytest
from conftest import child_env, run


@pytest.mark.parametrize('library', ['pandas', 'polars'])
def test_complete_frame_keeps_nulls_duplicate_positions_names_and_native_identity(backend, tmp_path, library):
    output = run(f'''
        from thinkthen import complete as c, frames
        import pandas as pd, polars as pl, thinkthen.pandas
        q = c.QuestionSource(role='atomic', body={{'decide':'Need attention?'}})
        source = (pd.Series(['one', None, 'one'], index=[9,9,2], name='body', dtype=object)
                  if {library!r} == 'pandas' else pl.Series('body', ['one', None, 'one']))
        engine = c.Engine(cache=False, batch=1)
        facade = frames.Engine(engine=engine)
        done = (source.tt.complete(engine=engine).decide(q) if {library!r} == 'pandas'
                else facade.decide(q, source))
        assert done.source is source and done.name == 'body'
        assert done.positions == (0,2) and done.native.ordinals == (0,1)
        assert done.results is done.native.results and done.facts is done.native.facts
        assert all(isinstance(r.answer_id, c.AnswerId) for r in done.results)
        values = list(done.frame) if {library!r} == 'pandas' else done.frame.to_list()
        assert values[1] is None and values[0] is done.results[0] and values[2] is done.results[1]
        if {library!r} == 'pandas': assert list(done.frame.index) == [9,9,2]
        print(done.facts.requests_sent, done.facts.records)
        print(repr(done))
    ''', child_env(backend, tmp_path))
    assert output.splitlines() == ['2 2', '<FrameCompleted: content withheld>']
    assert backend.count() == 2


def test_complete_lazy_find_materializes_one_whole_candidate_set(backend, tmp_path):
    output = run('''
        import polars as pl
        from thinkthen import complete as c, frames
        source = pl.DataFrame({'body':['First passage.',None,'Second passage.','Third passage.']}).lazy()
        done = frames.Engine(engine=c.Engine(cache=False), library='polars').find(
            c.QuestionSource(role='find',body={'find':'Which passage answers the question?'},none=True),source,on='body')
        assert done.positions == (2,) and done.native.ordinals == (1,)
        assert done.results[0].value == 'Second passage.'
        assert [r.index for r in done.results[0].candidates] == [0,1,2,None]
        assert done.source.to_list() == ['First passage.',None,'Second passage.','Third passage.']
        print(done.facts.requests_sent)
    ''', child_env(backend, tmp_path, 'case/18-find-second'))
    assert output.strip() == '1'
    assert backend.count() == 1


@pytest.mark.parametrize('library', ['pandas','polars'])
def test_complete_frame_refuses_bad_rows_and_preserves_cancel_deadline_secrecy(backend, tmp_path, library):
    output = run(f'''
        import pandas as pd, polars as pl, thinkthen as tt
        from thinkthen import complete as c, frames
        def series(items):
            return (pd.Series(items,dtype=object) if {library!r} == 'pandas' else pl.Series('body',items,dtype=pl.Object))
        facade = frames.Engine(engine=c.Engine(cache=False))
        q = c.QuestionSource(role='atomic',body={{'decide':'secret question'}})
        try: facade.decide(q,series(['secret text', 7]))
        except tt.UsageError as e: assert 'secret' not in str(e); print(e.kind)
        else: raise AssertionError('bad row admitted')
        token = tt.CancelToken(); token.cancel()
        for controls in ({{'token':token}},{{'deadline_ms':0}}):
            try: facade.decide(q,series(['secret text']),**controls)
            except (tt.Cancelled,tt.DeadlineError) as e: assert 'secret' not in str(e); print(e.kind)
            else: raise AssertionError('controls ignored')
    ''', child_env(backend, tmp_path))
    assert output.splitlines() == ['usage','cancelled','deadline']
    assert backend.count() == 0
