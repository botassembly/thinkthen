# 0066: Help introduction and example layout

Status: Landed on main as `42039dd266c54472a37e4338dc7fe9edb595a020` after hosted gate run `35742562545` passed on that exact revision. The coordinator fast-forwarded main, pushed it, and verified ancestry. This follow-up records the completed landing; it changes no product source.

## What changed

Root help uses the approved semantic-commands introduction. Tag and annotate descriptions now precede usage and options, with the same two examples under `Examples:` afterward. Package metadata, option parsing, outcome vocabulary, runtime behavior, and record/partial-failure advice are unchanged. Only wording-list items 2 and 5 are addressed. Their issue entries notify marketing that captures of these help screens need refreshing.

## Review and verification

- Independent Sol design review accepted the bounded level-2 ticket. It accepted a narrow correction after implementation discovered that bare Clap `about` overrides the root doc-string with package metadata. Removing that override and updating two executable identity-page expectations remain within the approved outcome.
- SWE-2 added compiled-help tests first. The layout test failed on examples before the description; the root test failed on the old first line. After the fix, the focused tests passed: nineteen `decide_edge`, two `tag_edge`, two `version`. The identity page failed its two stale first-line expectations before their update, then all four checks passed.
- A fresh independent Sol code reviewer accepted the diff, exact examples and ordering, both help flags, retained safety advice, and ratchet adjustment. No implementation reviewed itself.
- The coordinator ran `sdlc/scripts/install && sdlc/scripts/lint && sdlc/scripts/test && sdlc/scripts/spec && git diff --check` with the provider key empty and Cargo offline mode. Exit 0: policy, library packaging, dependency audit, formatting, clippy, docs, 547 Rust tests, doctests, replay checks, 27 specification checks, seven transform-page checks, and nineteen green how-tos.
- The measured Rust ceiling is 32,787, up 74. Two tests add 75 nonblank lines; the product change removes one. A single table covers both verbs and help flags. Existing tag option checks own different assertions and remain useful.

## Limits and next step

No provider call, library-runtime run, package publication, or website deployment was performed. The library team's worktree and ticket 0065 were untouched. Package metadata still carries the old pitch by explicit scope exclusion. Continue verifying surviving command wording against the binary before creating another bounded ticket. Engine cache/settings work still waits for 0065's owner to land it; the relation request method still needs its separate ruling.
