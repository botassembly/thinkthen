ACCEPT

# Review of Quick Fix qf-debug-withholds

Reviewer: a fresh read-only Opus session that did not write the work. This page restates its two replies.

## First pass, commit f5901161

Findings, none blocking:

1. `choose --options` answers still print record labels under `Debug`: `Answer`, its `Shape`, `Distribution`, `TagProbabilities`, `Value::Choice`, and `DecisionResult`. File a follow-up.
2. `Questions`, `RequestQuestion`, `NoulCriteria`, and `Criteria` still derive `Debug`. Only `Request` reaches them today.
3. The text `Record` fixture was plain ASCII, so a character count passed as a byte count.

Plants, each in a scratch copy with `cargo test -p thinkthen --lib cli::failure::tests`:

| Plant | Result |
| --- | --- |
| Derive `Debug` on `Request` again | red: `no_record_label_or_request_debug_line_shows_the_evidence` |
| `Labels` `Debug` prints names | red: the same test |
| Drop `byte_start` from `Token` | red: `no_recognize_debug_line_shows_the_evidence` |
| Text `Record` length plus 1 | red: the recognize test |
| JSON `Record` length plus 1 | red: the recognize test |
| Drop the `Record` kind | red: the recognize test |
| Text `Record` length in characters | green, finding 3 |

## Second pass, commit c2cdbff7 and merge 6e6dd9f2

ACCEPT. Finding 1 is filed as `sdlc/issues/2026-09-24-answers-still-print-record-labels-under-debug.md`. Finding 2: the four wire types derive `Debug` only in tests, and clippy on all targets is clean. Finding 3: the text record starts with `é`, and the character-count plant now fails the recognize test. The lib suite passed 315 with 6 ignored. The ratchet reads 48212/48212.
