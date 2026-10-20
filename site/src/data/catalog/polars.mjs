import { NATIVE_PLATFORMS } from './platforms.mjs';

export default {
    slug: 'polars', name: 'Polars', deckHeading: 'Polars',
    versions: { runtime: 'Python 3.10 or later; Polars 1.44.2 or later, below 2', platforms: NATIVE_PLATFORMS },
    backends: 'python',
    lang: 'python', tab: 'Python',
    blurb: "Use named Python engine methods or the opt-in Polars Series namespace.",
    unsureWord: '`None`',
    facts: "`Result.results` retains generated native rows; `Result.facts` counts the whole call.",
    errors: 'Every failure raises a `ThinkThenError`, as in Python. A failed row in a column call ends the call with `BackendError`.',
    settings: '`tt.Engine` takes each setting as a keyword, as in Python. `record=` writes a recording to a folder. `replay=` answers from that recording with no connection.',
    install: [['pip install thinkthen[polars]', null]],
    particular: [
      '`decide`, `choose`, `score`, and `tag` send a whole column to the engine in one call.',
      '`on=` names the column the questions read.',
    ],
  };
