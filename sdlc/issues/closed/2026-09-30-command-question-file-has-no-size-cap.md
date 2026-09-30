# The command reads a question file of any size

Status: Closed by a Quick Fix, "Land quick fix: cap the command's question file at 1 MiB". The libraries keep their own copies; `sdlc/issues/2026-09-30-question-file-reader-copies-and-uncapped-loaders.md` records that debt. The command reads a question file or question set through one capped reader, `crates/thinkthen/src/cli/question_text.rs`, and `tests/question_file/size.rs` pins the boundary, the sentence, exit 5 and zero loopback requests on all nine verbs. Reported by the Beatles Bench team on 2026-09-30. Verified by reading main `7230f1215`.

## The problem

The libraries read a named question file of at most 1 MiB and refuse a larger one with `the question file is too large`. That cap sits in `libraries/c/src/door.rs` (`question_file`, `LIMIT` of 1,048,576 bytes), `libraries/typescript/src/door.rs` and `libraries/ruby/src/ffi/question_file.rs`. The DuckDB and PostgreSQL extensions cap the same file at 1 MiB with their own sentences.

The command has no cap. It reads the whole file with `fs::read_to_string` in four places:

- `crates/thinkthen/src/cli/asked.rs`, `read_top`, for `decide`, `filter`, `rank`, `choose`, `tag` and `score`
- `crates/thinkthen/src/cli/annotate.rs`, the question set named by `@FILE` or a path
- `crates/thinkthen/src/cli/recognize/config.rs` and `crates/thinkthen/src/cli/relate/config.rs`

So `thinkthen decide @/dev/zero` reads until memory runs out.

## What should happen

Coordinator default, which Ian can overturn: the command applies the same 1 MiB cap to every question file and question set, reads at most 1 MiB plus one byte, and refuses a larger file with the libraries' sentence, `the question file is too large`, before it sends anything. The exit code follows the command's other question-file failures (exit 5 today for a file that cannot be opened). `specification/question-file.md` states the cap. A command-line test pins the sentence, the exit code, empty standard output and zero loopback requests for a file of 1 MiB plus one byte and for `/dev/zero`.
