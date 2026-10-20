import { NATIVE_PLATFORMS } from './platforms.mjs';

export default {
    slug: 'python', name: 'Python', deckHeading: 'Python',
    versions: { runtime: 'Python 3.10 or later', platforms: NATIVE_PLATFORMS },
    backends: 'code',
    lang: 'python', tab: 'Python',
    blurb: "Use one `Engine` and read each answer from `Result.value`.",
    unsureWord: '`None`',
    facts: "`Result.facts` retains actual final native settlement.",
    errors: 'Every failure raises a `ThinkThenError`. Its subclass names the kind: `UsageError`, `BackendError`, `LocalError`, `DeadlineError`, `DefectError` or `Cancelled`.',
    settings: '`tt.Engine` takes each setting as a keyword and reads the environment for the rest. `record=` writes a recording to a folder. `replay=` answers from that recording with no connection.',
    install: [['pip install thinkthen', null], ['uv add thinkthen', null]],
    particular: ["An authored question dictionary carries thresholds and options. Explicit `QuestionSource` selectors choose saved questions.", "Use `engine.iterate` for lazy typed rows and `engine.plan` for a no-send plan."],
  };
