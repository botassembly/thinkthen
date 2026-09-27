# Ticket 0153 build record

## Scope correction, 2026-09-27

The build preflight at `46c87f8f` confirmed that ticket 0146 is on main and the stopped-record failure keeps its record number and boxed cause. It also found that the accepted typed match needs `JsonError`, which the private core JSON module did not re-export. No runtime change was retained from that preflight.

Ticket amendment `0d488e15` adds `core/mod.rs` and a two-line budget for a command-feature-gated crate-private re-export. A fresh read-only Codex Sol Medium reviewer returned ACCEPT. It checked that the exact record-1, typed-JSONL, no-field conditions remain, duplicate-name and nonfinite errors retain their paths, the core remains pure, and no public library surface is added. The coordinator accepts this bounded correction. The ticket checker and diff whitespace check passed. Runtime implementation and final code review remain open.

## Code candidate, 2026-09-27

The builder added only the amended ticket's claimed source, test, specification and ratchet files. `core/mod.rs` re-exports `JsonError` within the crate only with the `cli` feature. `told` matches that typed syntax error under a stopped record 1, typed `--jsonl`, and no typed `--field`. The command also gives exact guessed-verb hints, refuses a loose second word on four one-question verbs, and points `choose --help` to `tag`.

The four new compiled-binary tests first failed against the original behavior for their named reasons, then passed. The existing `decide_edge` suite passed 20 tests, and `json_syntax::jsonl_syntax_keeps_the_record_sentence_and_sends_nothing` passed. Plants (a) through (i) each failed the intended assertion; the first selector for (h) ran zero tests, so the builder corrected it to the full module-qualified test name and reran it red. All planted changes were restored and the three targeted suites passed again. Captured help and unknown-option usage bytes for `decide`, `filter`, `rank` and `find` matched before and after. Formatting, diff whitespace and the ratchet check passed at 74,136 measured nonblank lines.

The ceiling grew by 246 measured lines for the command guidance and its outside-in edge tables. The builder checked `cli/mod.rs` and the `Command` accessors for a reusable path before adding one localized hint module and two typed accessors. No dependency, clap feature or public library surface changed. Fresh independent code review and full gates remain for the coordinator's next step. The docs issue item and three stumble-register rows close at landing with that commit, as the ticket directs.
