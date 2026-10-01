# cache convert --quote skips questions that start "The text is "

Status: closed 2026-09-30 by ticket 0364. Reported by the site team on 2026-09-30 while converting an old deck recording, and confirmed on main `d75a4bdf1`. Owner: the queue owner, as batch C3 in `../planning/issue-priorities-2026-09-30.md`. Resolution: An instruction counts as already quoted only when it is `The text is `, one whole JSON value and `. `, and that value is the record or a string, array or object. The summary counts answers left unquoted. `specification/recording.md` names the two leftover cases.
Kind: bug

1. **`--quote` quotes nothing when the user's question starts "The text is ".** `requote` in `crates/thinkthen/src/core/recording/convert.rs:200-202` returns no rewrite for the whole exchange when any instruction starts with "The text is ". It takes that as a question that already quotes its record. The site's own writing style starts questions this way, as in "The text is the title of a song by the Beatles." `specification/recording.md:113` documents the bare-prefix test.
2. **The summary gives no warning.** `cli/cache.rs:68-75` prints the answers written, converted and skipped. A user who sees `skipped 0` has no sign that the answers stay in the old form, which main cannot replay.

Not a bug: plain `cache convert` without `--quote` writes old-form answers that miss on main. ADR 0111 quotes each record in its question.

Fix: treat an instruction as already quoted only when it starts with `The text is `, the record's JSON and `. `. Add a count of answers left unquoted to the summary. Touches `crates/thinkthen/src/core/recording/convert.rs`, `crates/thinkthen/src/engine/store/convert.rs`, `crates/thinkthen/src/engine/store.rs`, `crates/thinkthen/src/cli/cache.rs` and `specification/recording.md`.
