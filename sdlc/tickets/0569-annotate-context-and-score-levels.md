# 0569: Make annotate take shared context once, name score levels, and fix status

Status: OPEN.

Milestone: 0.2

## Outcome

`annotate` accepts `--context` as its help says. It sends a shared document once per request instead of once per question. A score answer reports the level the question names. `thinkthen status` works on a machine with a fresh cache.

## Evidence

- Starts from: a 2026-10-12 report from a team that scored three 2,000-word documents against 21 questions over 25 live requests, with no failures.
  - `annotate --help` lists `--context`, but `annotate` refuses it. The team pasted the document into each record instead.
  - `annotate` repeats the whole record text inside every question. `--plan` estimated 1.65 MB and 21 requests for three records and 21 questions.
  - Score answers came back numbered 0 to 4 when the question named a 1 to 5 scale.
  - `thinkthen status` failed with "could not read the local cache or usage state" on the Beelink.
- Keeps: annotate's output shape and every other command's handling of `--context`.
- Changes:
  - Make `annotate` accept `--context`, or remove it from `annotate --help`. Prefer acceptance when the request shape already carries shared context.
  - Send the record once per request, with the questions referring to it. Show the smaller size in `--plan`.
  - Return the score level the question names. If a scale has no names, document the numbering in the score help and the result schema.
  - Find why `status` cannot read the cache or usage state on a fresh machine, and fix it. A missing cache or usage file reads as empty.
- Proof:
  - An outside-in CLI test shows `annotate --context FILE` runs, and that `--plan` for three records and 21 questions is smaller than before.
  - An outside-in test shows a 1 to 5 score question returns levels 1 to 5.
  - An outside-in test shows `status` succeeds with an empty cache directory and no usage file.
  - Routine `test` and `lint` pass.
- Defers: nothing.

PM amendment, 2026-10-12. The coordinator's recommendation stands. Keep the current score contract from ADR 0007: score returns the probability-weighted position from 0 to the number of levels minus one. State that numbering in the score help, the annotate help and the result schema, and show that a question's own 1 to 5 wording does not change it. Keep the quoted wire form from ADR 0111 and Ian's 2026-09-29 ruling, so annotate still repeats the record inside each question. Drop the single-copy change and its smaller `--plan` proof. Still make `annotate --context` work or remove it from the help, and still fix `status` on a fresh cache. Reproduce each reported CLI defect with an outside-in test before fixing it. 0574's score accessor returns this number.

## Progress

PM amendment, 2026-10-13. Remaining work. Score numbering is done: d9181d5f7 documents zero-based positions in the help and schema, and 2d5b38823 records the five-level proof. The two CLI defects were never reproduced, and d9181d5f7 says they "already pass" without a test, which breaks the 2026-10-12 amendment. Reproduce the `status` failure on the Beelink with real default cache and usage paths. If a clean home does not reproduce it, test against a copy of a real cache folder, and close the item only on a named cause. Write an outside-in test in `crates/thinkthen/tests/` that fails for that cause, then fix `crates/thinkthen/src/cli/status.rs` so an absent cache or usage file reads as empty. Reproduce the `annotate --context` refusal. If the refusal is deliberate, remove `--context` from `annotate --help`; that is the default ruling. Confirm `annotate --help` states the zero-based numbering. Run routine `test` and `lint` and merge into `ticket/0543-final`.

PM amendment, 2026-10-14. The evidence at e363ee504 discharges both reported failures. The annotate `--context` and status reports came from older builds, and the status exit 5 matches a 0.1 binary reading a cache that a newer build had migrated. Keep one outside-in test for annotate shared context with exactly one send and one for status on default paths, then close.
- 2026-10-14 started
- 2026-10-14 next: The f708e070c ruling discharges both reported failures at e363ee504; retained outside-in shared-context one-send and default-path status assertions are undergoing routine checks and fresh review.
