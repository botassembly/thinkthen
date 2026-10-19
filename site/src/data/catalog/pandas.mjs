import { NATIVE_PLATFORMS } from './platforms.mjs';

export default {
    slug: 'pandas', name: 'pandas', deckHeading: null,
    versions: { runtime: 'Python 3.10 or later', platforms: NATIVE_PLATFORMS },
    backends: 'python',
    lang: 'python', tab: 'Python',
    blurb: "Use the opt-in `series.tt` namespace or named engine methods.",
    unsureWord: '`pd.NA`',
    facts: "`Result.facts` retains settled call observations.",
    errors: 'Every failure raises a `ThinkThenError`, as in Python. Its subclass names the kind.',
    settings: '`tt.Engine` takes each setting as a keyword, as in Python. `record=` writes a recording to a folder. `replay=` answers from that recording with no connection.',
    install: [['pip install thinkthen pandas', null]],
    particular: ["Import `thinkthen.pandas` to register the Series namespace.", "DataFrame `on=` selects evidence; annotation adds named typed columns and failures."],
  };
