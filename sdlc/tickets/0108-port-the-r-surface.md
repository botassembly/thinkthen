---
flow: build
priority: 108
opens: libraries/r sdlc/scripts deny.toml sdlc/planning/libraries/r.md sdlc/planning/adr/0042-r-interrupt-noticed-at-the-next-tick.md sdlc/planning/adr/0047-bindings-are-unpublished-crates-over-the-public-api.md
---

# 0108: Port the R surface

Status: design draft; review pending. Owner: Claude.

## Outcome and authority

Port the R package from tag `surfaces-wave7-frozen-2026-09-24b` (`9df8bae9`) onto the public Rust API. The Rust half becomes the unpublished crate `thinkthen-r` at `libraries/r/thinkthen/src/rust`, in its own Cargo workspace. `library(thinkthen)` keeps its thirteen exported names: `tt_question`, the ten verbs with the `tt_` prefix, `tt_details`, and `tt_usage`. It also keeps `NA` for unsure, a column in and a column out, and the six kinds as R conditions with the retry signal. Every call reaches the real engine through `thinkthen`. Queue item 3 of `sdlc/planning/one-line-plan-2026-09-24.md` lists R among the remaining surfaces.

Draft ADR 0047 (on the 0093 branch) fixes the crate's place and its checklist. Ticket 0093 sets the workspace, lint, ratchet, and surface-rung pattern. Ticket 0095 fixes the members this binding calls, and 0098 builds them. ADR 0042 (on main) holds the R interrupt ruling, and its 2026-09-24 amendment names this ticket as its owner. ADR 0041 (on main) holds the deadline rule. Section 3.2 of `sdlc/planning/surfaces-port-guide.md` gives the port map. Ticket 0105 is the template this ticket follows. Ian can overturn every decision below.

## Design and decisions

1. **Bind the Rust API directly.** The R package keeps extendr and calls `thinkthen` in Rust. It does not link `thinkthen-c` from ticket 0094. Reasons:
   - The C door's `thinkthen_decide_many` returns no rows on a cancel, and its JSON door drops typed results that R converts directly.
   - The C door has no interrupt symbol. R would still need its own worker thread, written in C.
   - The offline tarball would have to vendor and build two crates plus hand-written C glue.
   From 0094 this ticket takes only the reference shape for the FFI edge (ADR 0047 item 7): one panic guard, one error-kind table, and one module that allows `unsafe`. The lever is a later ticket that moves R onto the C door, if a second consumer of that door needs it.
2. **Crate name.** At the tag the R crate is named `thinkthen`. It would collide with its dependency's library file. The crate becomes `thinkthen-r` with library name `thinkthen_r`. `Makevars.in` links `-lthinkthen_r`. The extendr module stays `mod thinkthen`, so `R_init_thinkthen_extendr` and `entrypoint.c` do not change. The crate keeps edition 2021. extendr 0.8.2's macros emit a bare `#[no_mangle]`, and edition 2024 refuses it without `unsafe(...)`.
3. **Manifest place.** R's package layout puts the manifest at `libraries/r/thinkthen/src/rust/Cargo.toml`. 0093's checks scan `libraries/*/Cargo.toml` and would miss it. The surface registry entry for R names its manifest path. The 0093 policy checks read manifest paths from the registry. This change is at most 15 lines under `sdlc/scripts`.
4. **Engine.** Every call uses `thinkthen::default_engine()`. The shim adds no engine setting. The tag's `LazyLock` over `StandinConnector` leaves.
5. **Interrupts: the worker for every call.** The tag's `call()` shape stays. Each call runs on a fresh plain worker thread. The worker owns the question clone, the texts as `String`s, and a clone of an internal `CancelToken`, and builds its own `CallOptions`. The main thread waits on a channel in 100 ms ticks. On each tick it runs `R_CheckUserInterrupt` inside `R_ToplevelExec` (ADR 0042). On a pending interrupt it cancels the token, drops the receiver, and returns the interrupt marker at once. The R half then raises R's own interrupt condition. The engine's `CallOptions::interrupt` stays unused, for three reasons:
   - R's check must run on R's main thread.
   - The engine does not run the check during one blocking send.
   - ADR 0042 promises that a call returns without waiting for a request already on the wire.

   0095 and ADR 0047 both allow the worker pattern. Cost: one thread spawn per call, as at the tag. Until its send ends, a detached send holds one width permit. Nothing on the path sends again. The token is cancelled, and 0073 and 0076 stop new attempts and retry waits.
6. **Deadlines.** `deadline_of` keeps R's type rules. `NULL` means none. A plain double or integer of length one is seconds. A number whose only class is `AsIs` is seconds. `NA`, a logical, a factor, a difftime, and `I()` over a classed value raise `thinkthen_usage`. The number then goes through `CallOptions::deadline_seconds` on the main thread before any thread starts. A refusal therefore sends nothing. The worker repeats the call to build its own options. The tag's `deadline_from_seconds` leaves.
7. **Questions.** The R half keeps composing the question-file object with `jsonlite` and passes it to `Question::from_json`. Parts and files then give one digest, as in 0105. `annotate`, `recognize`, and `relate` given a path call `QuestionSet::load`, `Recognize::load`, or `Relate::load`. The R-side `.tt_section` file reader leaves. `sdlc/planning/libraries/r.md` sets the goal "`Imports:` names nothing beyond base R". The tag already imports `jsonlite`. This ticket keeps `jsonlite` and amends that goal line with the reason. The lever is moving question building into the shim with the 0084 and 0095 builders. That would drop `.tt_json` and move the R5-10 fix into Rust.
8. **Bulk choose, score, and tag.** `tt_choose`, `tt_score`, and `tt_tag` over a column build a one-question set and call `annotate_with` once, the ruled bulk form in 0095. This closes R2-23. It supersedes the tag's NOTES decision to keep one request per row. A `Failed` cell in these three verbs raises `thinkthen_backend`, naming the row and the cause. It never reads as `NA`. Choose and tag labels known at run time go through `Question::choose_labels` and `tag_labels`, or through `from_json` for the composed object.
9. **Results.**
   - `tt_decide` calls `decide_many_with` for every length. The one-evidence branch (`tt_decide_one`) leaves.
   - `tt_score` returns the position. The nearest level lives in `tt_details`.
   - `tt_filter`, `tt_rank`, and `tt_find` wrap each text in one binding `Evidence` type that holds its one-based place. They keep the tag's shapes: records for `filter`, `data.frame(place, record, probability)` for `rank`, and `list(place, unit, probability)` for `find`. When `find` selects nothing, `place` and `unit` are `NA`, and `probability` comes from the `none` candidate.
   - `tt_annotate` reads column kinds from `QuestionSet::members`. The shim function `tt_annotate_kinds` leaves. Values come from `AnnotatedRecord::values`, and a failed cell keeps the ruled `list(failed = list(kind, cause))` marker from `Failed::kind` and `cause`.
   - `tt_details` returns `jsonlite::fromJSON(Details::to_json(), simplifyVector = FALSE)`, equal to the command's `--details` document. This changes the tag's list shape (`probability`, `sends`, `digest`) to the shared document.
   - `tt_usage` returns `requests_sent`, `cache_answers`, `input_tokens`, and `output_tokens` as doubles.
10. **Recognize and relate.**
    - `tt_recognize` keeps one data frame per record. It has the columns `name`, `kind`, `start`, `end`, and `strength` (the tag's `text` column becomes `name`). Offsets count Unicode scalar values, so `start + 1` and `end` keep `substr(text, start, end)` equal to the name. Relations ride in the `relations` attribute as `source`, `source_kind`, `target`, `target_kind`, `relation`, and `probability`.
    - `tt_relate(entities, relations, either, threshold, deadline)` takes a data frame with `name` and `kind` columns, such as `tidyr::unnest` of `tt_recognize`. The tag's text records and `kind_field` leave. A non-default fields pointer is `usage` under 0095.
    - The binding dedupes by name and kind in first-seen order, as ADR 0047 item 9 rules for databases. The engine's 255 cap then counts unique pairs.
    - `tt_relate` returns `data.frame(source, target, relation, probability, source_kind, target_kind)`. The first two columns are the endpoints, so `igraph::graph_from_data_frame` reads it unchanged.
11. **Errors and panics.** Errors cross as `kind␟retryable␟message`, with `%` doubled and NUL and `␟` escaped, as at the tag. The kind word comes from `ErrorKind::name`. A Rust unit test maps each of the six kinds to its word. The worker body runs inside one `catch_unwind`, which returns `defect` carrying the panic's words. The engine already stops its own panics at public methods (0086). A closed channel with no answer returns `defect`. Every `#[extendr]` function and `extendr_module!` live in the one FFI module that allows `unsafe_code`.
12. **Build and install.**
    - `tools/config.R` passes `--locked --offline` in both shapes. The `synthetic-partial` flag and `THINKTHEN_R_SYNTHETIC_PARTIAL` leave.
    - `check.sh` installs one production build. The fixture install and the restore trap leave with the stand-in.
    - `[profile.release]` equals the root one under ADR 0047 item 1. The tag's `lto` and `codegen-units` lines leave.
    - `tools/make-tarball.sh` vendors the unpacked output of `cargo package --locked --offline -p thinkthen`. That manifest carries no workspace inheritance. It also vendors the registry tree with `cargo vendor --locked`. The contract, stand-in, and `thinkthen-core` copies leave. This is the standalone crate that ADR 0047 item 1 names.

## What moves from the tag

- The package: `thinkthen/R/thinkthen.R`, `R/extendr-wrappers.R` (regenerated), `NAMESPACE` with its explicit export list, `DESCRIPTION`, `LICENSE`, `configure`, `src/Makevars.in`, `src/entrypoint.c`, `src/rust/document.rs`, `tools/config.R`, and `tools/msrv.R`.
- `src/rust/src/lib.rs`, split by topic into at most four files: the FFI module, text and deadline crossing, calls and results, and recognize and relate.
- `check.sh` keeps its shape, with the changes in the gate section. `tools/make-tarball.sh` is retargeted.
- `examples.R`, `examples.json`, `slide.R`, and `slide_check.R`. The drawn slide block stays untouched, and each expected value is re-derived against 0092's generic arm.
- Tests that call the package keep their assertions: `tests_null.R` (renamed, since the null backend retires), `text_check.R`, `hook_check.R`, `ownership_check.R`, `recognize_check.R`, `fork_check.R`, and the interrupt children with their bash parents. The builder regroups them by topic into at most ten R files. Cases that replayed stand-in recordings move to the 0092 case arm, or to the generic arm for shape checks.
- `conformance.R`, rewritten onto main's `conformance/cases.json`.

These retire:
- `wire_width.R`, because 0086 proves width.
- `interrupt_fast.sh`'s null-backend column. The held-arm tests below replace it.
- The stand-in fixture build and every `ENGINE_NULL` line.

`NOTES.md` stays at the tag as history. The port writes a new `NOTES.md` of at most 120 lines. `sdlc/planning/libraries/r.md` is the design page, and this ticket updates it.

## Error-index rows

Source: `sdlc/issues/2026-09-23-surfaces-branch-error-index.md`. The index lists 13 R rows, and the port guide classes all 13 as binding. This ticket carries all 13. It also carries the R halves of eleven cross-surface rows and of two engine rows. It retires the R halves of five more rows with their reasons. Each re-proof runs against the real engine through the 0092 loopback backend unless marked as a unit test or a script step.

"Counted" means the 0092 backend's `count` line. A bash parent starts the backend with `coproc`, exports `THINKTHEN_BASE_URL` with the arm's path, and reads `count` on the backend's standard input. `tests/with-backend.sh` runs one R file per backend process and compares the final count with the file's printed `expect count N` line. The record plants each bug below and shows its test turning red, then green once the bug is removed.

### R rows

| Row | Status at tag | Re-proof here | Planted bug |
|---|---|---|---|
| R1-5 | closed | A set holds a tag member with threshold 0.95. The generic arm gives no label that reaches it, so every cell is `character(0)`. The column stays a list of character vectors. A decide member under a band stays logical. | Type each column from its first cell. The tag column becomes character with `NA`. |
| R1-13 | closed | A `check.sh` step finds exactly one `R_CheckUserInterrupt` call site, inside the function passed to `R_ToplevelExec`. The interrupt tests below run beside it. | Add a direct `R_CheckUserInterrupt()` in the tick loop. The step fails. |
| R2-5 | closed | In a child `Rscript`, a duplicate option spelled `100% sure %s` raises `thinkthen_usage`. The message holds the option verbatim, and the child exits 0. A Rust unit test pins `carry`'s doubled `%`. | Drop the `%` doubling. The unit test and the pinned message turn red. |
| R3-3 | closed | In a child, a set file with a member named `a\u0000b` raises `thinkthen_usage`, and the child exits 0. A Rust unit test pins `carry`'s `\\u0000` escape. If main's parser refuses the name first, the record says so, and the unit test carries the plant. | Drop the NUL escape. The unit test turns red. |
| R3-15 | closed | `tt_annotate` with a question named like an input column raises `thinkthen_usage` naming it. The count is 0. | Drop the clash check. The count is nonzero and the `on` column is overwritten. |
| R3-17 | closed | Under `LC_ALL=C`, a native-marked question and evidence that are valid UTF-8 give the same `question_sha256` and request digests in `tt_details` as under `C.UTF-8`. Native invalid bytes and a bytes-marked string raise `thinkthen_usage` with a count of 0. | Translate native text through `Rf_translateCharUTF8`. The digests differ. |
| R4-7 | closed | The tarball step below installs from a fresh `git archive` with an empty `CARGO_HOME` and `CARGO_NET_OFFLINE=true`, and one `tt_decide` answers. | Skip `cargo vendor`. The install fails. |
| R5-10 | closed | Under `LC_ALL=C`, native-marked choose options and relate rules give the same digest as under `C.UTF-8`. | Drop the `Encoding` fix in `.tt_json`. The digests differ. |
| R5-11 | closed | An option holding `\x1f` raises `thinkthen_usage` with its kind intact. A Rust unit test pins the `\\u001f` escape. | Drop the separator escape. The kind is lost and the class check fails. |
| R5-12 | closed | `hook_check.R` stubs `tools::pskill` to return `TRUE` without sending a signal. The call still stops and prints `OUTCOME stopped`. | Return `NULL` after the sleep. The script goes on. |
| R6-10 | closed | `tt_tag(q, x, labels = "refund")` answers `"refund"` from the generic arm. | Send labels without `I()`. jsonlite unboxes the label and the call raises `usage`. |
| R7-11 | open at index, fixed at tag | `deadline = 5L`, `I(5)`, and `-1L` answer. `0L` and `I(0L)` raise `thinkthen_deadline` with a count of 0. `NA_integer_`, `factor("5")`, `I(factor("5"))`, `as.difftime(5, units = "secs")`, and `TRUE` raise `thinkthen_usage` with a count of 0. | Read only doubles. `5L` raises `usage`. |
| R7-13 | open at index, fixed at tag | `interrupt_hook.sh` runs one child as plain R and one as a held `tt_decide`, each with an `options(error=)` hook. Uncaught, both print `HOOK RAN` and `AFTER`. Caught by `tryCatch(interrupt=)`, both print `CAUGHT` and `AFTER` and no hook line. | Deliver the interrupt with the hook still set aside. The uncaught case loses `HOOK RAN`. |

### R halves of cross-surface and engine rows

| Row | Status at tag | Re-proof here | Planted bug |
|---|---|---|---|
| R1-10 host half | engine | Rust unit test, no libR needed: the worker helper runs a closure that panics with `boom`. The result is `defect` with the panic's words. A second call on a new worker then answers. | Remove the `catch_unwind`. The disconnect answer lacks `boom`, and the pinned message turns red. |
| R1-11 host half | engine | `deadline = 1e300`, `Inf`, `-Inf`, and `NaN` raise `thinkthen_usage` with a count of 0. | Convert with `Duration::from_secs_f64` in the shim. extendr's wrapper turns the panic into a plain error, and the class check fails. |
| R2-10 R half | partial | `-1` and `NULL` mean none. `0` raises `thinkthen_deadline` with a count of 0. `-2` and `4294967296` raise `thinkthen_usage` with a count of 0. | Treat any negative as none. `-2` answers. |
| R2-23 R half | open/waive | `tt_choose` over 50 texts on the held arm shows a count above 1 before any release. `tt_score` and `tt_tag` do the same. | Loop one call per row. The count stays at 1. |
| R2-29 R half | open | The binding re-proves ADR 0042's windows: the interrupt tests below measure the tick, and a held single call returns before its reply. The NOTES-only R rulings land in a short R section of ADR 0047: `I()` over a number, a refused classed deadline, the annotate route for R2-23, `jsonlite` kept, and relate's frame input. ADR 0042 gains an amendment with the tick measured on main. | Not a code row. The review checks that each ruling has an ADR line. |
| R2-32 R half | not re-probed | `extendr-api` 0.8.2 pulls `paste` 1.0.15. RUSTSEC-2024-0436 marks it unmaintained. Root `deny.toml` gains one `ignore` entry with a reason naming this ticket. The binding's deny call passes with it. | Remove the entry. Deny fails on the advisory. |
| R3-30 and R5-32 R half | not re-probed | The runner has no local skip list. It reports every case in `cases.json` as pass, fail, or not run with the reason, and the three counts sum to the file's count. Case 18 reports not run and names the batch interrupt test that proves it. | Skip one case silently. The sum check fails. |
| R4-19 R half | closed | A `check.sh` step reads `tools/config.R`, `check.sh`, and `make-tarball.sh`, and fails on any `cargo` call without `--locked` and `--offline`. | Drop `--locked` from the repository shape. The step fails. |
| R1-34 R half | closed | The built tarball carries `LICENSE`, and `DESCRIPTION` reads `License: MIT + file LICENSE`. The stand-in's recordings no longer exist to ship. | Exclude `LICENSE` from the tarball. The content check fails. |
| R6-12 R half | not re-probed | Every interrupt test signals once the backend's `count` line reads at least 1, and asserts that reading. It never signals at a fixed time after `ready`. | A harness plant: the child prints `ready`, sleeps 1 s, then calls, and the parent signals on `ready`. The count reads 0 at the signal, and the assertion fails. |
| R1-31 and R2-31 R half | waive and partial | One kind table and one panic guard. A `check.sh` step counts one `catch_unwind` site in `src`. | Add a second guard. The step fails. |

Retired halves:
- R4-9 and R5-13: the fixture build and its restore trap leave with the stand-in. `check.sh` installs one production build, and the final proof reads it.
- R5-42: the branch's hermeticity document retires. The offline rule in the gate section covers its substance.
- R3-32 and R4-10 R half: `make-tarball.sh` already uses `perl -pi` at the tag, and the port keeps that line. A Mac run belongs to the release ticket (queue item 11).

Not closed here:
- G9 waits on ADR 0047 item 5. The R page states whichever answer Ian gives.
- R2-27 names no R half.

## Other acceptance

- Red first: the ported tests fail against an empty `libraries/r` workspace for the stated reason, then pass.
- `cargo test --lib --locked --offline` in `libraries/r/thinkthen/src/rust` passes. The unit tests need no running R.
- The conformance runner runs every applicable case through the 0092 case arm with recomputed digests. Case 17 asserts counter differences. No backend arm is added.
- Case 68 (an accent and an emoji) gives the same `start` and `end` in R, after the one-based shift, as in Rust.
- `fork_check.R` warms the default engine and forks with `parallel::mcparallel`. The child answers, and the parent's counters do not move (0096, Q15).
- A relate frame with a repeated name and kind sends one entity for it, and each edge carries names and kinds. `igraph::graph_from_data_frame` accepts the result.
- `tt_details` on a score question carries `answer.level`, and on a decide question it carries none.
- Nothing reaches a non-loopback address. `THINKTHEN_API_KEY` stays unset. A secrecy test reads every condition message and `print` output for the key and for credentials in the base URL.
- Interrupt tests (ADR 0042), each under `timeout`:
  - Single call. Hold one `tt_decide` on the held arm. Signal once `count` reads 1. The interrupt condition arrives within 0.5 s of the signal. Release, and the count stays at exactly 1. Plant: block on `recv` with no tick. No answer arrives within 5 s, and `timeout` ends the child.
  - Batch. Run a 200-text `tt_decide` on the held arm. Signal once two `count` reads 100 ms apart agree above 0. The interrupt arrives within 0.5 s. After release, the count read at +1 s and +3 s equals the count at the signal and stays below 200. Plant: skip `token.cancel()`. After release the detached batch sends all 200.
- Every test process runs under `timeout`. A held reply is always released or its child killed before the test returns.

## The check it adds to the gate ladder

- `libraries/r/check.sh` joins the surface registry as landed. The `surfaces` rung (ADR 0047, the fifth rung after `spec`) runs it with the 0092 loopback port.
- It runs on this Linux host with no Docker and no network. Observed by command on 2026-09-24:
  - R 4.3.3 at `/usr/bin/R`, rustc and cargo 1.93.1, and cargo-deny 0.19.4 are installed.
  - jsonlite 2.0.0, dplyr 1.1.4, tidyr 1.3.1, and igraph 1.6.0 are in the system R library.
  - Every registry crate in the tag's R lock, extendr 0.8.2 and paste 1.0.15 among them, is in the cargo cache.
  - The freeze record shows R's check passed at the tag with the wire suite (94 ok lines).
- A missing R 4.2 or later, a missing required R package, or a cargo cache miss reports "not run" and never "pass" (R6-2). The line names the one fetch to run on a networked machine: `cargo fetch --locked --manifest-path libraries/r/thinkthen/src/rust/Cargo.toml`, or `install.packages()` for the named R package.
- The tarball step builds the tarball from a `git archive` of the tree. It installs the tarball into a scratch library with an empty `CARGO_HOME` and `CARGO_NET_OFFLINE=true`, and one call answers there. It replaces the tag's `--stage-only` step.
- `lint` runs on the binding:
  - the ADR 0047 manifest, lock, lint-table, and profile checks, reading R's manifest path from the registry;
  - deny as `cargo deny --offline --manifest-path libraries/r/thinkthen/src/rust/Cargo.toml check --config deny.toml advisories bans licenses sources` against the root `deny.toml`, planted with a git-sourced dependency that `[sources] unknown-git = "deny"` refuses;
  - `ratchet.mjs` on `libraries/r/ratchet.json` for Rust and `ratchet.R.json` for R;
  - the registry check.
- Lints. The binding's table equals the root table except `unsafe_code = "deny"`. The root forbids `missing_debug_implementations`, `unreachable_pub`, and `unsafe_code`, and denies `expect_used`, `unwrap_used`, `indexing_slicing`, and `panic`. A local `allow` cannot lift a forbid. The builder's first step compiles one empty `#[extendr]` function and `extendr_module!` under this table and records the result. If extendr's generated code trips a forbid-level lint, the builder stops and records the case for an ADR 0047 amendment.

## Dependencies and second review

- Rust: `thinkthen` by path with default features off, and `extendr-api` 0.8.2 with `extendr-ffi` 0.8.2 and `extendr-macros` 0.8.2, from the tag's lock and this machine's cache. `paste` 1.0.15 comes in through `extendr-api`. The lock's versions for `thinkthen`'s own tree equal the root lock's (ADR 0047 item 1).
- R: `jsonlite (>= 2.0.0)` in `Imports:`, the tested version. `Suggests:` lists dplyr, tidyr, and igraph for the tests. The package depends on no other package.
- These enter main for the first time. The code reviewer checks each entry, the binding lock, deny's result, and the `paste` ignore entry, and the review record says so (repo `CLAUDE.md`).

## Budgets

- Production Rust: at most four files and 950 nonblank lines, each file under 500. The tag's `lib.rs` measures 899 nonblank lines, about 860 before its test module. The shim loses the connector, `tt_annotate_kinds`, the three one-row functions, and `tt_decide_one`. It gains the `Evidence` type, the one-question set, relate entity mapping, and the kind table.
- R package code: `thinkthen.R` and `extendr-wrappers.R` together at most 650 nonblank lines. The tag measures 639. The removed `.tt_section`, `kind_field`, and the one-row branches pay for the relate frame and the failed-cell raise.
- Tests: at most ten R test files and 1,300 nonblank lines, bash parents at most 250, Rust unit tests at most 150, and the conformance runner at most 300. The tag's moving tests measure about 880 R lines, and `conformance.R` measures 306.
- Scripts: `check.sh`, `make-tarball.sh`, `tools/config.R`, and `tools/msrv.R` together at most 420 nonblank lines. The tag measures 373. Gate changes under `sdlc/scripts` at most 40 nonblank lines, the registry manifest path included.
- Documentation: `README.md`, the new `NOTES.md`, `sdlc/planning/libraries/r.md`, the ADR 0047 R section, and the ADR 0042 amendment, at most 240 net nonblank lines.
- Ratchet: `libraries/r/ratchet.json` and `ratchet.R.json` each set `max` to the measured total. The root `sdlc/ratchet.json` does not change. The record names what each block earns and where the tag's duplicate code went first.

Stop and re-score before crossing a budget, adding a dependency beyond this list, touching `crates/thinkthen`, or widening the export list.

## Exclusions

CRAN, R-universe, prebuilt binaries, and Windows or macOS installs (the release ticket, queue item 11). The C door as R's backend. Any change to `thinkthen`. A user-facing cancel token in R. Async forms. Any live or paid call.

## Dependencies

After 0086. Also after 0098 (labels, spec readers, JSON methods, `ErrorKind::name`, `members`), 0093 (the registry, the `surfaces` rung, the ratchet argument, and the binding policy checks), and 0094, since the plan puts C before every other surface. 0092 and 0099 have landed.

## Routing

Builder: Claude (Opus subagent). Reviewer: a fresh Claude session for design and for code.

## Complexity

Contract 2; state and timing 3; reach 2; proof 3; cost of error 3; total 13. Final level: 3. R's interrupt is a longjmp that must never cross a Rust frame. A wrong tick loses a user's Ctrl-C or spends a column.

## Review

- Design review: pending.
- Code review: pending.
