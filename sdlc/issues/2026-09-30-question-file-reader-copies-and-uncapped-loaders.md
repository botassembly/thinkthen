# The question-file reader has four capped copies and three uncapped ones

Status: open. Filed by the Quick Fix that capped the command's question file. Owner: a Quick Fix after 0304 slice 4 lands, since slice 4 edits `public/relate.rs`.

Kind: debt

Pay when: before 0.1, since the uncapped Rust and Python loaders can exhaust memory.

Keeping it lets one copy's cap or sentence drift from the others, and lets a Rust or Python caller read `/dev/zero` until memory runs out.

## What happens

Four places read a question file under the same 1 MiB cap with `the question file is too large`: the command's `crates/thinkthen/src/cli/question_text.rs`, `question_file` in `libraries/c/src/door.rs`, `bounded_file` in `libraries/typescript/src/door.rs` and `bounded_source` in `libraries/ruby/src/ffi/question_file.rs`. Each maps its failure to its own error type. The Quick Fix left the libraries alone because lane 1 was changing the C door for slice 3b, which has since landed.

Three loaders read a whole file with `fs::read_to_string` and no cap: `Question::load` in `crates/thinkthen/src/public/question.rs`, `Relate::load` in `crates/thinkthen/src/public/relate.rs` and `_load` in `libraries/python/src/asked.rs`.

## What should happen

The `thinkthen` crate exposes one reader that returns the text or a typed reason (unreadable, too large, not UTF-8). The command, the public Rust loaders, Python and the three capped libraries map that reason to their own error. The capped sentences stay as they are, and the Rust and Python loaders gain `the question file is too large`, stated in `specification/question-file.md`. The binding checks for C, TypeScript and Ruby pass unchanged.
