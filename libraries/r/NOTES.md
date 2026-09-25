# R surface notes

Ticket 0108 ported the R surface from tag `surfaces-wave7-frozen-2026-09-24b` onto the public API. The tag's `NOTES.md` stays there as history. This file holds the rulings and measurements that still bind. The build record is `sdlc/records/2026-09-25-0108-port-r-surface.md`.

## Shape

- The crate `thinkthen-r` lives at `thinkthen/src/rust`, its own workspace with its own lock. It depends on `thinkthen` by path with default features off, and on `extendr-api` 0.8.2.
- `lib.rs` holds the kind table, `carry`, and the engine slot. `calls.rs` holds the worker, the wait, and the verbs. `relate.rs` holds recognize and relate. `ffi.rs` holds every R API call and every `unsafe`.
- The ticket planned a child file `ffi/text.rs`. It merged into `ffi.rs`, because `policy.py` allows `unsafe` only in a file named `ffi.rs`.
- The crate uses edition 2024, the root edition, as ADR 0047 item 2 says. The ticket's "2021" came from the tag.
- Questions cross as question-file JSON. The R half builds it with jsonlite, and Rust parses it with `Question::from_json` on each call.

## Calls and interrupts (ADR 0042)

- Every call runs on a worker thread. The main thread waits in 100 ms ticks and checks R's interrupt flag only inside `R_ToplevelExec`. A drop guard cancels the call's token, so a detached batch stops sending.
- `.tt_call` checks before it forces the call and after the call returns. The Rust half checks before it spawns the worker and at each tick.
- One `catch_unwind` turns a worker panic into `defect`. A worker whose caller left ignores the failed send.
- `tests/interrupt.R` measures signal to `CAUGHT`. The single call and the batch each answer within one tick. The record holds ten runs.
- The retired pieces: `ACTIVE`, `tt_cancel_active`, and `.tt_cleanup`. The drop guard does their work.

## Rulings

- **`I()` over a number.** `deadline = I(5)` is 5 seconds. A classed deadline such as `factor` or `difftime` is `usage`, because R's own coercion reads a factor's code.
- **Deadline numbers.** `NULL`, `-1`, and `-1L` mean no deadline. `0` is spent and raises `deadline` with no request. `NA`, `-2`, `Inf`, `NaN`, `1e300`, and `4294967296` are `usage`.
- **Bulk choose, score, and tag.** A column crosses as one `annotate` of a one-question set (0095). This closes R2-23. At throttle 8 each verb holds 8 requests on the wire at once.
- **A failed cell in those three verbs** cannot happen. The engine refuses a one-question request whose answer is broken as a whole call, and R raises that call's kind. An unexpected failed cell raises `thinkthen_defect`.
- **jsonlite stays.** It is the one import. Version 2.0.0 is the tested one. Its archive is pinned in `tools/pins.sha256`.
- **relate takes a frame.** `tt_relate` takes `name` and `kind` columns and dedupes them in first-seen order (ADR 0047 item 9). The engine's 255 cap counts unique pairs.
- **The ratchets.** `ratchet.json` counts the crate's Rust, and `ratchet.R.json` counts every `.R` file in this folder.

## Behavior that differs from the ticket's text

- **R3-3.** Main's question-set parser refuses a NUL in a member name before `carry` sees it. The refusal names the member with the NUL escaped, and the process lives. The Rust unit test on `carry` carries the plant.
- **R2-5.** Main's duplicate-option message names no option: "the question file's `options`: a list holds each option once". The class check and the doubled-percent unit test stand.
- **max_requests** counts the records of one engine call. Rank and find hold their input and refuse before any request. A streaming `tt_decide`, `tt_filter`, or bulk choose, score, or tag sends the records under the limit and refuses at the record past it, so `max_requests = 1L` over two texts sends one request. The ticket said the refusal comes before the first request, and its acceptance line now says this. `tt_recognize` runs one engine call a text, so the limit counts each text alone and never refuses a column. `tt_relate` is one call over its unique entities.
- **A throttle after a default-engine verb** is accepted. The default engine selects no throttle, so 0077's rule allows a first explicit one.
- **find.** `tt_find` asks with no none candidate, because `Question::find` has no switch for one (`sdlc/issues/2026-09-24-the-library-cannot-ask-find-none-or-per-question-parts.md`). When nothing is selected, `place` and `unit` are `NA` and `probability` is `NA`. Cases 18 and 19 of `cases.json` ask with a none candidate, so they report not run.
- **A question that names a model** cannot join a question set. `tt_choose`, `tt_score`, and `tt_tag` then ask it one row at a time through `details_with`, under the one deadline. `tests/verbs.R` proves a named-model choose answers.
- **One deadline a call.** The Rust half fixes the deadline as one instant before the call starts, and every engine call a verb makes shares it. `tt_recognize` over three texts at 600 ms each under `deadline = 1` raises `thinkthen_deadline` after two sends.
- **Interrupt parents** are R files run by `tests/with-backend.sh`, where the ticket planned bash `coproc` parents. R reads the backend's count through the same fifo, and one harness serves every test.
- **Forked children** count from zero. A child forked after the parent's first call answers, reports one send, and the parent's counters stay where they were.

## Conformance

`tests/conformance.R` runs every case of `conformance/cases.json` in its own child on the 0092 case arm. It recomputes each request digest for the URL the backend served. Eight cases report not run with their reason: three engine injection points (cases 20, 22, and 25), case 23 (R raises its own interrupt, and `tests/interrupt.R` proves the batch stop), case 30 (no decide question file in R), the two find cases, and the two-group annotate case (`tt_annotate` reads one column).

## Engine findings

- The command prints the engine's internal `WidthActive` sentence, which still says width. R prints the public one, which says throttle. See `sdlc/issues/2026-09-25-the-engine-width-active-message-still-says-width.md`.
- `Question::find` has no none candidate: `sdlc/issues/2026-09-24-the-library-cannot-ask-find-none-or-per-question-parts.md`, filed on main by ticket 0086.
