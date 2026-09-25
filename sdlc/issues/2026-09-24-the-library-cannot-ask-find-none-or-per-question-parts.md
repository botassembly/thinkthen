# The library cannot ask find with a none option or annotate over record parts

Status: Open. Found by ticket 0086's build and code review, 2026-09-24. Owner: an 0084 amendment, or ticket 0098 with the binding members.

## What happens

The command can do two things the 0084 Rust contract cannot express.

- `find --none` adds a "none of these" candidate. `Question::find` has no switch for it, so shared cases `18-find-second` and `19-find-none` cannot run through the public API.
- A question set can give each question an `on` pointer that reads one part of a record. `QuestionSet::from_json` refuses a non-root `on`, because a library call's evidence is one whole text. So shared case `18-annotate-two-groups` cannot run through the public API.

`conformance/consumer/consumer/tests/public/cases.rs` skips these three cases and says why. Case `25-defect-fault` needs no change, because R1-10 covers the defect kind.

## Fix

Amend 0084 with a find option for the none candidate, such as a `FindBuilder` step, and decide whether structured records get per-question parts through `Evidence`. Then remove the three skips so every shared case runs through the library.
