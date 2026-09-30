# The question-file reader has four capped copies and three uncapped ones

Status: Closed by ticket 0345, landed as `Land 0345: every surface reads a question file through one capped public reader`. Resolution: `thinkthen::read_question_file` reads at most 1 MiB and one byte and names why it gave no text. The command, the four Rust loaders, and the C, Python, TypeScript, Ruby and R libraries call it and keep their own sentences. The SQL extensions keep their own 1 MiB readers.

Kind: debt

Pay when: before 0.1, since the uncapped Rust and Python loaders can exhaust memory.

Debt: 011

Severity: high

Paid: 2026-09-30

Keeping it lets one copy's cap or sentence drift from the others, and lets a Rust or Python caller read `/dev/zero` until memory runs out.

## What happens

Four places read a question file under the same 1 MiB cap with `the question file is too large`: the command's `crates/thinkthen/src/cli/question_text.rs`, `question_file` in `libraries/c/src/door.rs`, `bounded_file` in `libraries/typescript/src/door.rs` and `bounded_source` in `libraries/ruby/src/ffi/question_file.rs`. Each maps its failure to its own error type. The Quick Fix left the libraries alone because lane 1 was changing the C door for slice 3b, which has since landed.

Three loaders read a whole file with `fs::read_to_string` and no cap: `Question::load` in `crates/thinkthen/src/public/question.rs`, `Relate::load` in `crates/thinkthen/src/public/relate.rs` and `_load` in `libraries/python/src/asked.rs`.

## What should happen

The `thinkthen` crate exposes one reader that returns the text or a typed reason (unreadable, too large, not UTF-8). The command, the public Rust loaders, Python and the three capped libraries map that reason to their own error. The capped sentences stay as they are, and the Rust and Python loaders gain `the question file is too large`, stated in `specification/question-file.md`. The binding checks for C, TypeScript and Ruby pass unchanged.
