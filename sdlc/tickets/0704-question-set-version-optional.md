# 0704 — A question set's version is optional

Status: OPEN.

Milestone: 0.2.1

## Outcome

- Changes: `crates/thinkthen/src/core/question_set/parsing.rs`, `crates/thinkthen/tests/`, `specification/`, `site/src/`, `libraries/*/README.md`, `databases/*/README.md`.
- A question set without `version`, inline or in a saved file, parses as version 1. A present `version` other than the number 1 is still refused with the same message.
- Documentation examples drop `version`. The specification states that it is optional and defaults to 1.

### Proof

- A Rust test parses the same set with and without `"version": 1` and gets equal question sets; `"version": 2` and `"version": "1"` are still refused. It fails before the change on the missing-version case.
- One SQLite or PostgreSQL `thinkthen_annotate` example without `version` replays.

## Evidence

`core/question_set/parsing.rs` refuses a set without `version`: `thinkthen usage: version is the number 1`. Version 1 is the only version, so readers of a trigger or query see a field that does nothing for them. Marketing's PostgreSQL example shows the set without it. Ian approved making it optional on 2026-10-19.

This changes the question-set contract before the core freeze (0688), so every language picks it up with no extra work.
