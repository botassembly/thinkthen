import { UNIX_PLATFORMS } from './platforms.mjs';

export default {
    slug: 'ruby', name: 'Ruby', deckHeading: 'Ruby',
    versions: { runtime: 'Ruby 3.4 or later, below 4.1', platforms: UNIX_PLATFORMS },
    backends: 'code',
    lang: 'ruby', tab: 'Ruby',
    blurb: "Use the ten named methods on `ThinkThen::Client` and read each answer from the result.",
    unsureWord: '`nil`',
    facts: "`result.facts` retains final native observations.",
    errors: 'Every failure raises a `ThinkThen::Error`. Its subclass names the kind: `UsageError`, `BackendError`, `LocalError`, `DeadlineError`, `DefectError` or `CancelledError`.',
    settings: "`Client.open` scopes cleanup. Constructor keywords override native settings; `record:` writes and `replay:` sends nothing.",
    install: [['gem install thinkthen', null]],
    particular: ["Use ordinary authored hashes and explicit `Client.question_file` selectors.", "Enumerable inputs use bounded native sessions; generated results survive client cleanup."],
  };
