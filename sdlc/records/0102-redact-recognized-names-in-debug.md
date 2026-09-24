# 0102: Redact recognized names in Debug output

Status: landed. A fresh review accepted it (`sdlc/records/0102-review.md`). Quick Fix under `sdlc/tickets/0102-redact-recognized-names-in-debug.md`.

## Result

- `RecognizedName` withholds its name and shows its kind, span, and strength. The kind comes from the recognize configuration, so it stays visible.
- `Token` withholds its text and shows its token span.
- `TokenInput` in `cli/recognize.rs` withholds its token and shows its probabilities. It became `pub(crate)` with `pub(crate)` fields so the shared suite can build one.
- `Record` shows `Record(<withheld>)`. It derived `Debug` over the raw text or JSON record, and `recognize --details` holds one.
- `RelationEdge<RecognizedName>`, `Recognized`, and `Detailed` keep their derived `Debug`. They now reach only redacted fields.
- `Json` still derives `Debug`. Question text prints on purpose (`core/question/tests.rs`), so `Json` stays as it is.

## Red and green

The new test `cli::failure::tests::no_recognize_debug_line_shows_the_evidence` formats a `RecognizedName`, a `RelationEdge` of names, `tokenize` output, a `TokenInput`, a text `Record`, and an object `Record` with `{:?}` and `{:#?}`. It checks that the planted marker is absent, that "withheld" appears exactly 14 times, and that the name's kind, span, and strength still print.

- Red, before the fix: it failed at `tests.rs:129`, the absence check. The output held `Token { text: "marker-evidence-7b3ac5", ... }` and `Record(Text("marker-evidence-7b3ac5"))`, among others.
- Green, after the fix: `cargo test --lib no_recognize_debug_line` passed.

## Size

The ratchet rose by 89 non-blank lines. It went from 44335 to 44424 on the first build and from 44778 to 44867 after the rebase onto 0091. The Debug impls add 50 lines and the test adds 39. The commit message names where I looked for duplication.

## Ladder

Run in the worktree with `THINKTHEN_API_KEY`, `THINKTHEN_BASE_URL`, and `THINKTHEN_URL` unset, after the one-minute load fell below 10.

- `install` exited 0.
- `lint` failed once at the package rung on a stale crate with trailing bytes (`sdlc/issues/2026-09-24-a-rerun-package-rung-can-read-a-crate-with-trailing-bytes.md`). After deleting `target/package/thinkthen-0.0.1.crate`, `lint` exited 0 and printed "ratchet: crates 44424/44424".
- `test` exited 0 with 737 passed and 0 failed. An earlier run under a load of 14.7 failed once on `annotate::scheduling::one_global_queue_bounds_document_and_stream_requests_at_jobs_1_4_and_32` (peak concurrency at 32 jobs). The rerun passed it.
- `spec` exited 0 with "demos: 21 green, 0 red".
- `sdlc/scripts/live` never ran.
