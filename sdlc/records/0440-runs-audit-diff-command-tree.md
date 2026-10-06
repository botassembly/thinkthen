# 0440: Group offline audit and diff under runs

The visible commands are runs audit and runs diff. Hidden top-level aliases retain the same options, outputs, diagnostics and exits. Both paths use the same handlers before configuration or network setup. Docs and saved examples use the visible names.

Fresh read-only code review: ACCEPT. Namespace, audit, diff and transform regressions, policy, settings, Clippy, 34 documentation blocks and 11 site examples passed. Full tests and lint run on the landing commit.

## What the build taught us

Share argument types and dispatch rather than translating aliases into a second parser. Existing outside-in fixtures can compare both spellings and count zero sends without another harness.
