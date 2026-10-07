"""Select actual dataframe calls inside the existing complete consumer."""
import os
from types import SimpleNamespace
from thinkthen import complete as c, frames
import native_case

LIBRARY = os.environ['THINKTHEN_FRAME_LIBRARY']


def column(source):
    if isinstance(source, c.Files): return source
    if LIBRARY == 'pandas':
        import pandas as pd
        return pd.Series(source.items, index=[7] * len(source.items), name='original', dtype='object')
    import polars as pl
    return pl.Series('original', list(source.items), dtype=pl.Object)


class FrameConsumer:
    def __init__(self, **settings):
        self._engine = frames.Engine(engine=c.Engine(**settings), library=LIBRARY)
    def __getattr__(self, verb):
        def call(question, source, **controls):
            source = column(source)
            done = getattr(self._engine, verb)(question, source, **controls)
            if verb.endswith('_batch'):
                assert isinstance(done, frames.FrameBatch)
                assert done.source is source or isinstance(source, c.Files)
                return done
            assert isinstance(done, frames.FrameCompleted)
            assert done.facts is done.native.facts
            assert done.results is done.native.results
            if not isinstance(source, c.Files):
                assert done.source is source
                assert done.name == 'original'
                if LIBRARY == 'pandas': assert list(done.index) == [7] * len(source)
                assert done.positions == done.native.ordinals
            assert len(done.frame) == (len(done.index) if verb in ('decide','choose','tag','score','annotate','recognize') else len(done.results))
            return done.native
        return call

# The retained ordinary consumer still owns framing, typed access and serialization.
native_case.c = SimpleNamespace(**{**vars(c), 'Engine':FrameConsumer})
native_case.main()
