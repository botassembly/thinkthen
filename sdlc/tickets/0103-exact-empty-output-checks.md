---
flow: quick-fix
priority: 103
opens: spec
---

# 0103: Exact empty-output checks

Status: landed. Owner: Claude. The review accepted it (`sdlc/records/0103-review.md`).

## Outcome and authority

`mustmatch like ""` matches any output, so four spec lines prove nothing (`sdlc/issues/2026-09-24-empty-output-checks-pass-on-any-output.md`). This quick fix follows the form 0083 used in `spec/transform.md`.

## Work

1. Replace `spec/version.md:16` and `:31` and `spec/decide.md:151` and `:230` with `test -z`. Show each new line fail on planted output.
2. Make `sdlc/scripts/demos` refuse `like ""` in every page under `spec/` and the demo root. Add a `demos-self-test` case that goes red without the check.
3. Replace loose `like` matches of short numbers in `spec/` and `demos/` with exact matches.

Touches only `spec/`, `demos/`, `sdlc/scripts/demos`, `sdlc/scripts/demos-self-test`, the issue, this ticket, and its record.
