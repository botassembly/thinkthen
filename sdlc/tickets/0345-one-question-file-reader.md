# 0345: One capped question-file reader serves every surface

Status: in progress. Plan: `sdlc/planning/issue-priorities-2026-09-30.md`, batch B3. Pays Debt 011, `sdlc/issues/2026-09-30-question-file-reader-copies-and-uncapped-loaders.md`. Starts after ticket 0344 lands, because 0344 edits `libraries/python/src/asked.rs`.

## Outcome

The `thinkthen` crate holds the one question-file reader. It reads at most 1 MiB plus one byte and returns the text or a typed reason: unreadable, too large, or not UTF-8. The command, `Question::load`, `Relate::load`, Python's `_load`, and the C, TypeScript and Ruby libraries all call it and map the reason to their own error. `Question::load` and Python's `_load` now refuse a file over 1 MiB, such as `/dev/zero`, with `the question file is too large`, and `Relate::load` with `the relate file is too large`. Each sends nothing.

## Evidence

- Starts from: the debt issue above; `crates/thinkthen/src/cli/question_text.rs` `read` (the command's cap, `LIMIT` 1,048,576); `question_file` in `libraries/c/src/door.rs`; `bounded_file` in `libraries/typescript/src/door.rs`; `bounded_source` in `libraries/ruby/src/ffi/question_file.rs`; the uncapped `std::fs::read_to_string` in `public/question.rs` (`Question::load`), `public/relate.rs` (`Relate::load`) and `libraries/python/src/asked.rs` (`_load`), checked on main `d8018dd96`. `specification/question-file.md` states the cap for the command and three libraries.
- Keeps: every sentence and exit code the four capped copies give today, including the TypeScript and Ruby role word (`the {role} file ...`), the capped copies' `is not UTF-8` sentence, and the command's per-verb unopened failures. The command's pinned test `crates/thinkthen/tests/backend/question_file/size.rs`. The C, TypeScript and Ruby checks pass with no expected text changed. A file of exactly 1 MiB still loads.
- Changes: one public reader in `crates/thinkthen/src/public/` with a small public reason enum, named in the ticket's added public declarations block when the build lands. The four capped copies become calls to it. `Question::load`, `Relate::load`, `QuestionSet::load`, `Recognize::load` and Python's `_load` call it; the build added the set and recognize loaders, R's `tt_question(file=)` through a new `tt_question_file` shim call, and the command's `annotate` look at `--input` for a swapped set, which all had the same uncapped read. Each maps too large to its own role word as a local error, and keeps `could not be read` for an unreadable file and for invalid UTF-8, as it says today. `specification/question-file.md` names every surface under the one cap. CHANGELOG, ratchet.
- Proof: an edge table through the public Rust API for `Question::load` and `Relate::load`: a missing file, 1 MiB exactly, 1 MiB plus one byte, `/dev/zero`, and invalid UTF-8, each with its exact sentence and error kind. A Python test for `_load` over 1 MiB plus one byte and `/dev/zero`, with zero requests counted at a loopback backend. The existing command size test and the C, TypeScript and Ruby checks pass unchanged. `policy.py`, `sdlc/scripts/test`, workspace clippy with `-D warnings`, the C door tests, `tickets`, and `lint` in a clean checkout.
- Defers: the SQLite, DuckDB and PostgreSQL extensions' own 1 MiB caps and sentences, which `closed/2026-09-30-command-question-file-has-no-size-cap.md` left as they are.

## What the build taught us

## Public API delta

### Added public declarations

```text
QuestionFileError::NotUtf8
QuestionFileError::TooLarge
QuestionFileError::Unreadable(Error)
enum QuestionFileError
fn read_question_file(impl AsRef<Path>) -> Result<String, QuestionFileError>
```
