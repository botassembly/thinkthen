# R surface notes

Ticket 0108 ported the R surface from tag `surfaces-wave7-frozen-2026-09-24b` onto the public API. The tag's `NOTES.md` stays there as history. This file holds the rulings and measurements that still bind. The build record is `sdlc/records/2026-09-25-0108-port-r-surface.md`.

## Shape

- The crate `thinkthen-r` lives at `thinkthen/src/rust`, its own workspace with its own lock. It depends on `thinkthen` by path with default features off, and on `extendr-api` 0.8.2.
- `lib.rs` holds the kind table, `carry`, and the engine slot. `calls.rs` holds the verbs; its private `worker`, `account`, `receipt`, and `render` children keep the prompt wait, checked facts, Rust-owned completion, and main-thread R conversion separate. `relate.rs` holds recognize and relate. `ffi.rs` holds every R API call and every `unsafe`.
- The ticket planned a child file `ffi/text.rs`. It merged into `ffi.rs`, because `policy.py` allows `unsafe` only in a file named `ffi.rs`.
- The crate uses edition 2024, the root edition, as ADR 0047 item 2 says. The ticket's "2021" came from the tag.
- Questions cross as question-file JSON. The R half builds it with jsonlite, and Rust parses it with `Question::from_json` on each call.

## Calls and interrupts (ADR 0042)

- Every call runs on a worker thread. The main thread waits in 100 ms ticks and checks R's interrupt flag only inside `R_ToplevelExec`. A drop guard cancels the call's token, so a detached batch stops sending.
- `.tt_call` checks before it forces the call and after the call returns. The Rust half checks before it spawns the worker and at each tick.
- One `catch_unwind` turns a worker panic into `defect` and settles a supplied completion with unavailable facts. A worker whose caller left ignores the failed send.
- The optional `tt_completion()` handle claims before R question evaluation, then becomes running after spawn. The original worker publishes one owned terminal account after core work stops. `tt_completion_read()` converts copies on R's main thread. A prompt Ctrl-C keeps R's own interrupt while the worker retains only an `Arc`, so collecting the R handle cannot run a finalizer on that worker.
- `tests/interrupt.R` measures signal to `CAUGHT`. The single call and the batch each answer within one tick. The record holds ten runs.
- The retired pieces: `ACTIVE`, `tt_cancel_active`, and `.tt_cleanup`. The drop guard does their work.

## Rulings

- **`I()` over a number.** `deadline = I(5)` is 5 seconds. A classed deadline such as `factor` or `difftime` is `usage`, because R's own coercion reads a factor's code.
- **Deadline numbers.** `NULL`, `-1`, and `-1L` mean no deadline. `0` is spent and raises `deadline` with no request. `NA`, `-2`, `Inf`, `NaN`, `1e300`, and `4294967296` are `usage`.
- **Bulk choose, score, and tag.** A column crosses as one `details_many_with` call on the loaded question. This preserves runtime labels, structured descriptions and saved profiles. The batch selector reaches the core; `batch = 2L` with enough records fills eight held request slots under throttle 8. A structured question text intentionally remains one record per request in the core planner.
- **A failed cell in those three verbs** cannot become `NA`. The engine refuses a broken one-question answer as a whole call, and R raises that call's kind. `tt_annotate` still carries a per-member failed marker beside valid cells.
- **jsonlite stays.** It is the one import. Version 2.0.0 is the tested one. Its archive is pinned in `tools/pins.sha256`.
- **relate takes a frame.** `tt_relate` takes `name` and `kind` columns and dedupes them in first-seen order (ADR 0047 item 9). The engine's 255 cap counts unique pairs.
- **The ratchets.** `ratchet.json` counts the crate's Rust, and `ratchet.R.json` counts every `.R` file in this folder.

## Behavior that differs from the ticket's text

- **R3-3.** Main's question-set parser refuses a NUL in a member name before `carry` sees it. The refusal names the member with the NUL escaped, and the process lives. The Rust unit test on `carry` carries the plant.
- **R2-5.** Main's duplicate-option message names no option: "the question file's `options`: a list holds each option once". The class check and the doubled-percent unit test stand.
- **max_requests** counts the records of one engine call. Rank and find hold their input and refuse before any request. A streaming `tt_decide`, `tt_filter`, or bulk choose, score, or tag sends the records under the limit and refuses at the record past it, so `max_requests = 1L` over two texts sends one request. The ticket said the refusal comes before the first request, and its acceptance line now says this. `tt_recognize` runs one engine call a text, so the limit counts each text alone and never refuses a column. `tt_relate` is one call over its unique entities.
- **A throttle after a default-engine verb** is accepted. The default engine selects no throttle, so 0077's rule allows a first explicit one.
- **find.** `tt_find(question, units, none = FALSE)` adds a none candidate when `none` is `TRUE`, through `Question::offering_none` (ticket 0150). A `none` other than one `TRUE` or `FALSE` raises `thinkthen_usage` before any request. When nothing is selected, `place` and `unit` are `NA` and `probability` is `NA`.
- **Record parts.** A question set member with an `on` pointer reads that part of each `tt_annotate` cell, which holds the record as JSON text (ticket 0150).
- **A question that names a model** cannot join a question set, but the dynamic-label many path keeps its model and call controls. `tests/profile.R` uses explicit batch one to retain its older two-send assertion; default Max can pack compatible plain text.
- **One deadline a call.** The Rust half fixes the deadline as one instant before the call starts, and every engine call a verb makes shares it. `tt_recognize` over three texts at 600 ms each under `deadline = 1` raises `thinkthen_deadline` after two sends.
- **Interrupt parents** are R files run by `tests/with-backend.sh`, where the ticket planned bash `coproc` parents. R reads the backend's count through the same fifo, and one harness serves every test.
- **Forked children** count from zero. A child forked after the parent's first call answers, reports one send, and the parent's counters stay where they were.

## Conformance

`tests/conformance.R` selects from all 54 cases of `conformance/cases.json` and runs each selected case in its own child on the 0092 case arm. It recomputes each request digest for the URL the backend served. Four cases retain explicit not-run reasons: three engine injection points (20, 22, and 25) and R's own interrupt (23), whose held batch stop `tests/interrupt.R` proves. On 2026-09-28, selected case `30-local-question-file` passed through public `tt_question(file=)` with a non-retryable Local error and zero loopback sends. The [Quick Fix record](../../sdlc/records/qf-r-question-file-conformance.md) names the installed artifact and focused check. Its four full-corpus not-run cases are a source-derived count; the historical full run was not repeated.

Cases 13–16, 27–28, and 34–35 select batch one in their own children to keep the frozen per-record bodies and digests. `tests/facts.R` separately captures a default-Max packed body and derives its digest from the expected bytes and actual loopback URL. The same case checks original R indexes, nested description values, explicit JSON null, and host no-work facts. `tests/recognize.R` proves that a later oversized text preserves the earlier subcall's counted prefix. `tests/interrupt.R` holds eight two-record requests and reads final cancellation facts after R's prompt interrupt; it also collects a handle while a send is held.

## Engine findings

- The command prints the engine's internal `WidthActive` sentence, which still says width. R prints the public one, which says throttle. See `sdlc/issues/2026-09-25-command-wording-and-help-fixes-before-0-1.md`.

Experiment 302 later ran the full corpus at `4c0ef210`: 49 passed, case40 failed, and four kept their not-run reasons. The counter case tried to replace an already chosen engine configuration. The [package harness correction](../../sdlc/records/qf-package-counter-and-negative-diagnostics.md) puts that measurement in a fresh child and records a focused pass with two observed requests. That focused correction is separate from a new full-corpus result.
