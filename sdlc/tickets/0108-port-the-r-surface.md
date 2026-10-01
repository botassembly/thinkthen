---
flow: build
priority: 108
opens: libraries/r sdlc/scripts sdlc/planning/libraries/r.md sdlc/planning/adr/0042-r-interrupt-noticed-at-the-next-tick.md sdlc/planning/adr/0047-bindings-are-unpublished-crates-over-the-public-api.md
---

# 0108: Port the R surface

Status: landed 2026-09-25 in the surface batch, at batch head `03580733` on `ticket/surface-batch`. Integration record: `sdlc/records/surface-batch-integration.md`. Owner: Claude.

## Outcome and authority

Port the R package from tag `surfaces-wave7-frozen-2026-09-24b` (`9df8bae9`) onto the public Rust API. The Rust half becomes the unpublished crate `thinkthen-r` at `libraries/r/thinkthen/src/rust`, in its own Cargo workspace. `library(thinkthen)` keeps its thirteen exported names: `tt_question`, the ten verbs with the `tt_` prefix, `tt_details`, and `tt_usage`. It gains a fourteenth, `tt_engine`, for the engine settings (decision 4). It also keeps `NA` for unsure, a column in and a column out, and the six kinds as R conditions with the retry signal. Every call reaches the real engine through `thinkthen`. Queue item 3 of `sdlc/planning/one-line-plan-2026-09-24.md` lists R among the remaining surfaces.

Draft ADR 0047 (on the 0093 branch) fixes the crate's place and its checklist. Ticket 0093 sets the workspace, lint, ratchet, and surface-rung pattern. Ticket 0095 fixes the members this binding calls, and 0098 builds them. ADR 0042 (on main) holds the R interrupt ruling, and its 2026-09-24 amendment names this ticket as its owner. ADR 0041 (on main) holds the deadline rule. Section 3.2 of `sdlc/planning/surfaces-port-guide.md` gives the port map. Ticket 0105 is the template this ticket follows. The 2026-09-24 design review (`sdlc/records/2026-09-24-design-review-0108.md`) found eight findings and a list of small fixes. This version answers all of them. Ian can overturn every decision below.

## Design and decisions

1. **Bind the Rust API directly.** The R package keeps extendr and calls `thinkthen` in Rust. It does not link `thinkthen-c` from ticket 0094. Reasons:
   - The C door's `thinkthen_decide_many` returns no rows on a cancel, and its JSON door drops typed results that R converts directly.
   - The C door has no interrupt symbol. R would still need its own worker thread, written in C.
   - The offline tarball would have to vendor and build two crates plus hand-written C glue.
   From 0094 this ticket takes only the reference shape for the FFI edge (ADR 0047 item 7): one panic guard, one error-kind table, and one module that allows `unsafe`. The lever is a later ticket that moves R onto the C door, if a second consumer of that door needs it.
2. **Crate name.** At the tag the R crate is named `thinkthen`. It would collide with its dependency's library file. The crate becomes `thinkthen-r` with library name `thinkthen_r`. The rename touches three build files:
   - `src/Makevars.in` sets `STATLIB = $(LIBDIR)/libthinkthen_r.a` and `PKG_LIBS = -L$(LIBDIR) -lthinkthen_r`.
   - `src/rust/document.rs` calls `thinkthen_r::get_thinkthen_metadata()`.
   - `src/rust/src/lib.rs` re-exports that function with `pub use ffi::get_thinkthen_metadata;`. `extendr_module!` generates it inside the FFI module (decision 7), and `document.rs` reaches it only through the crate root.
   The extendr module stays `mod thinkthen`, so `R_init_thinkthen_extendr`, `useDynLib(thinkthen)`, and `entrypoint.c` do not change. The crate keeps edition 2021. extendr 0.8.2's macros emit a bare `#[no_mangle]`, and edition 2024 refuses it without `unsafe(...)`.
3. **Manifest and ratchet place.** R's package layout puts the manifest at `libraries/r/thinkthen/src/rust/Cargo.toml`. 0093's checks scan `libraries/*/Cargo.toml` and would miss it. The surface registry entry for R names its manifest path, its ratchet files, and its deny config. The 0093 policy checks read those paths from the registry. `ratchet.json` and `ratchet.R.json` sit in `libraries/r`, three levels above the manifest. ADR 0047 item 4 puts the ratchet beside the manifest. R departs from it because everything under `libraries/r/thinkthen` ships in the tarball, and a ratchet file there would ship too. The ADR 0047 R section records the departure. Registry and policy changes stay within 40 lines under `sdlc/scripts`.
4. **The engine value.** ADR 0017 section 5 lists the library settings on the engine value, and shared rule 6 makes every surface expose them. R has no engine object, and it spells a setting as a snake-case argument, as the verbs spell `deadline`. So the R engine value is one exported function, `tt_engine(base_url = NULL, model = NULL, throttle = NULL, max_requests = NULL, cache = NULL, cache_bytes = NULL)`.
   - The Rust half starts from `EngineBuilder::from_env()`, the 0084 amendment at `f19cf437`. A `NULL` argument keeps the value the environment gives, exactly as the default engine reads it. A set argument calls one setter over it: `base_url`, `model`, `throttle` (an integer from 1 to 32), `max_requests`, `cache_at` for a folder, `no_cache` for `cache = FALSE`, and `cache_bytes`. The key stays on `THINKTHEN_API_KEY` alone, as section 5 rules. The shim reads no environment variable itself (ADR 0047 item 7).
   - Each argument keeps R's type rules: a string of length one, a whole number of length one, or `FALSE` for `cache`. Anything else, and every refusal a setter or `build` returns, raises `thinkthen_usage` before any request.
   - The built engine goes into one process slot. Every verb, `tt_details`, and `tt_usage` then use it. With the slot empty, they use `thinkthen::default_engine()`, which reads the same environment at throttle 4. `tt_usage` reads only the engine in use, so counts made on the default engine before `tt_engine` stay with it and are not added.
   - Throttle is process-wide under 0077. A second `tt_engine` with equal settings does nothing, and one with any other setting raises `thinkthen_usage` naming the settings in force. `tt_engine` after a verb has run on the default engine keeps 0077's rule for a later explicit throttle. The builder records that rule, and a test pins it.
   - `tt_engine` returns `NULL` invisibly. The ADR 0047 R section records the fourteen-name export list.
   The tag's `LazyLock` over `StandinConnector` leaves.
5. **Interrupts: the worker for every call.** The tag's `call()` shape stays. Each call runs on a fresh plain worker thread. The worker owns the question clone, the texts as `String`s, and a clone of an internal `CancelToken`, and builds its own `CallOptions`. The main thread waits on a channel in 100 ms ticks. The engine's `CallOptions::interrupt` stays unused, for three reasons:
   - R's check must run on R's main thread.
   - The engine does not run the check during one blocking send.
   - ADR 0042 promises that a call returns without waiting for a request already on the wire.

   0095 and ADR 0047 both allow the worker pattern. Cost: one thread spawn per call, as at the tag. Until its send ends, a detached send holds one throttle permit. Nothing on the path sends again. The token is cancelled, and 0073 and 0076 stop new attempts and retry waits. When the caller has left, the worker ignores the failed channel send and does not panic.
6. **ADR 0042's three check points stay.** Each runs `R_CheckUserInterrupt` inside `R_ToplevelExec` on the main thread, through one function in the FFI module.
   - Before a call: the Rust call checks once before it spawns the worker. A pending interrupt returns the marker, and nothing is sent.
   - At each tick: the wait checks every 100 ms. A pending interrupt cancels the token, drops the receiver, and returns the marker at once.
   - Before and after every `.tt_call`: the R half checks before it forces the crossing and after the crossing returns. A pending interrupt raises R's own interrupt condition in place of the value.

   The four interrupt pieces at the tag:
   - `tt_interrupt_pending` stays. It is the `.tt_call` check.
   - `ACTIVE`, `tt_cancel_active`, and `.tt_cleanup` retire. A drop guard in the wait now cancels the call's token on every exit that leaves the worker running: the tick's marker, an error, and a panic unwind. No R code runs while the Rust call holds the token, so no R jump can leave a call in flight, and an R-side cleanup has nothing to reach.
7. **`unsafe` lives in one module.** The FFI module `ffi` carries the crate's only `#[allow(unsafe_code, reason = "…")]` (ADR 0047 item 3). Every R API call sits in it or in its child module `ffi::text`: the `extern` block, `R_ToplevelExec` and `R_CheckUserInterrupt`, the text crossing (`STRING_ELT`, `Rf_getCharCE`, `R_CHAR`, `Rf_translateCharUTF8`, `R_NaString`), and the deadline reads (`REAL_ELT`, `INTEGER_ELT`, `R_NaInt`, `R_IsNA`). The `#[extendr]` functions and `extendr_module!` live there too. They hand safe Rust values to `calls` and `relate`, which hold no `unsafe`. The binding's `unsafe_code = "deny"` refuses `unsafe` anywhere else in `src`, and `policy.py`'s planted failure (an `unsafe` block in `calls.rs`) proves it. The tag's crate-wide `#![allow(missing_docs)]` becomes an `allow` on `ffi` alone, with the reason that extendr's generated wrappers carry no documentation.
8. **Deadlines.** `deadline_of` keeps R's type rules. `NULL` means none. A plain double or integer of length one is seconds. A number whose only class is `AsIs` is seconds. `NA`, a logical, a factor, a difftime, and `I()` over a classed value raise `thinkthen_usage`. The number then goes through `CallOptions::deadline_seconds` on the main thread before any thread starts. A refusal therefore sends nothing. The worker repeats the call to build its own options. The tag's `deadline_from_seconds` leaves.
9. **Questions.** The R half keeps composing the question-file object with `jsonlite` and passes it to `Question::from_json`. Parts and files then give one digest, as in 0105. `annotate`, `recognize`, and `relate` given a path call `QuestionSet::load`, `Recognize::load`, or `Relate::load`. The R-side `.tt_section` file reader leaves. `sdlc/planning/libraries/r.md` sets the goal "`Imports:` names nothing beyond base R". The tag already imports `jsonlite`. This ticket keeps `jsonlite` and amends that goal line with the reason. The lever is moving question building into the shim with the 0084 and 0095 builders. That would drop `.tt_json` and move the R5-10 fix into Rust.
10. **jsonlite version.** On this machine jsonlite 2.0.0 is in the user library `~/R/x86_64-pc-linux-gnu-library/4.3`, and the system site library holds 1.8.8. `DESCRIPTION` keeps `Imports: jsonlite (>= 2.0.0)`, the tested version. The check uses 2.0.0 from the user library. `check.sh` keeps `R_LIBS_USER` in every install's environment. Before any install it runs `packageVersion("jsonlite") >= "2.0.0"` on the library path the install uses and prints the version and its path. A shorter library reports "not run", and never "pass" or "fail". The line names the setup step `libraries/r/tools/setup.sh`. That step downloads the CRAN source archive `jsonlite_2.0.0.tar.gz` once, refuses it unless its sha256 equals the one pinned in `libraries/r/tools/pins.sha256` (`75eb910c82b350ec33f094779da0f87bff154c232e4ae39c9896a9b89f3ac82d`, amended 2026-09-24, change 1), and installs it into `~/.cache/thinkthen-toolchains/r-library` (shared rule 2). This machine needs no download, since the user library already holds 2.0.0.
11. **Bulk choose, score, and tag.** `tt_choose`, `tt_score`, and `tt_tag` over a column build a one-question set and call `annotate_with` once, the ruled bulk form in 0095. This closes R2-23. It supersedes the tag's NOTES decision to keep one request per row. A `Failed` cell in these three verbs raises `thinkthen_backend`, naming the row and the cause. It never reads as `NA`. Choose and tag labels known at run time go through `Question::choose_labels` and `tag_labels`, or through `from_json` for the composed object.
12. **Results.**
    - `tt_decide` calls `decide_many_with` for every length. The one-evidence branch (`tt_decide_one`) leaves.
    - `tt_score` returns the position. The nearest level lives in `tt_details`.
    - `tt_filter`, `tt_rank`, and `tt_find` wrap each text in one binding `Evidence` type that holds its one-based place. They keep the tag's shapes: records for `filter`, `data.frame(place, record, probability)` for `rank`, and `list(place, unit, probability)` for `find`. When `find` selects nothing, `place` and `unit` are `NA`, and `probability` comes from the `none` candidate.
    - `tt_annotate` reads column kinds from `QuestionSet::members`. The shim function `tt_annotate_kinds` leaves. Values come from `AnnotatedRecord::values`, and a failed cell keeps the ruled `list(failed = list(kind, cause))` marker from `Failed::kind` and `cause`.
    - `tt_details` returns `jsonlite::fromJSON(Details::to_json(), simplifyVector = FALSE)`, equal to the command's `--details` document. This changes the tag's list shape (`probability`, `sends`, `digest`) to the shared document.
    - `tt_usage` returns `requests_sent`, `cache_answers`, `input_tokens`, and `output_tokens` as doubles.
13. **Recognize and relate.**
    - `tt_recognize` keeps one data frame per record. It has the columns `name`, `kind`, `start`, `end`, and `strength` (the tag's `text` column becomes `name`). Offsets count Unicode scalar values, so `start + 1` and `end` keep `substr(text, start, end)` equal to the name. Relations ride in the `relations` attribute as `source`, `source_kind`, `target`, `target_kind`, `relation`, and `probability`.
    - `tt_relate(entities, relations, either, threshold, deadline)` takes a data frame with `name` and `kind` columns, such as `tidyr::unnest` of `tt_recognize`. The tag's text records and `kind_field` leave. A non-default fields pointer is `usage` under 0095.
    - The binding dedupes by name and kind in first-seen order, as ADR 0047 item 9 rules for databases. The engine's 255 cap then counts unique pairs.
    - `tt_relate` returns `data.frame(source, target, relation, probability, source_kind, target_kind)`. The first two columns are the endpoints, so `igraph::graph_from_data_frame` reads it unchanged.
14. **Errors and panics.** Errors cross as `kind␟retryable␟message`, with `%` doubled and NUL and `␟` escaped, as at the tag. The kind word comes from `ErrorKind::name`. A Rust unit test maps each of the six kinds to its word. The worker body runs inside one `catch_unwind`, which returns `defect` carrying the panic's words. The engine already stops its own panics at public methods (0086). A closed channel with no answer returns `defect`.
15. **The `paste` advisory.** `extendr-api` 0.8.2 depends directly on `paste` 1.0.15, and RUSTSEC-2024-0436 reports `paste` unmaintained. The binding's deny call needs one `ignore` entry. Main's root lock carries no `paste`, and cargo-deny 0.19.4 warns "no crate matched advisory criteria" on every root run when a config ignores an advisory that matches nothing. So the entry does not go in root `deny.toml`. The binding gets `libraries/r/deny.toml`, equal to root `deny.toml` with its `ignore = []` line replaced by an `ignore` list holding that one entry (amended 2026-09-24, change 2). A `lint` step puts `ignore = []` back in place of that list, compares the result with root `deny.toml`, and fails on any other difference. The registry names the file, and the binding's deny call reads it. The entry's `reason` reads: "paste is a compile-time proc macro that extendr-api 0.8.2 depends on directly, so no paste code ships in thinkthen_r. RUSTSEC-2024-0436 reports it unmaintained and names no vulnerability. Ticket 0108. Remove this entry when an extendr release drops paste." That last sentence is the lever. No cached extendr release drops it today.
16. **Build and install.**
    - `tools/config.R` passes `--locked --offline` in both shapes. The `synthetic-partial` flag and `THINKTHEN_R_SYNTHETIC_PARTIAL` leave.
    - `check.sh` installs one production build. The fixture install and the restore trap leave with the stand-in.
    - `[profile.release]` equals the root one under ADR 0047 item 1. The tag's `lto` and `codegen-units` lines leave.
    - `check.sh` builds R's library path in this order: `~/.cache/thinkthen-toolchains/r-library`, `R_LIBS_USER`, and the system libraries.
    - `tools/make-tarball.sh` runs only inside a `git archive` tree of the commit under test. It never runs in a Git checkout, so it needs no `--allow-dirty`. It vendors the unpacked output of `cargo package --locked --offline --no-verify -p thinkthen`. That manifest carries no workspace inheritance. It then vendors the registry tree with `cargo vendor --locked --offline` from the builder's own cargo home, as the tag's wave-6 fix does. The package's gitignored `src/rust/.cargo` never serves as a cargo home for that step. The contract, stand-in, and `thinkthen-core` copies leave. This is the standalone crate that ADR 0047 item 1 names. `check.sh` prints the archived commit.

17. **The shared surface rules.** This ticket follows all six rules in the "Shared rules" section at the top of `sdlc/planning/surfaces-port-guide.md` (main, `446a4d6b`). ADR 0042 governs R where the fourth rule differs from it.
    - No Docker. `check.sh` runs on any machine with R 4.2 or later, the pinned Rust toolchain, and the cargo cache.
    - Toolchains live under `~/.cache/thinkthen-toolchains/`. R, rustc, and cargo are host installs, and this ticket downloads none of them. The one download it defines is the jsonlite archive of decision 10, pinned by sha256 and refused on a mismatch. Gates then run offline.
    - Each test gets its own product cache folder and its own loopback backend. `tests/with-backend.sh` starts one 0092 backend per R test file and sets `THINKTHEN_CACHE` to a fresh `mktemp -d` folder for it. Each interrupt child and each conformance case run gets the same. The tests here need only 0092's arms, one held reply released once per backend, so nothing waits on ticket 0117.
    - Ctrl-C is prompt for single calls and batches alike, and every call runs on a detachable worker that finishes the requests already sent (decision 5). ADR 0042 governs two differences. R raises its own `interrupt` condition, caught by `tryCatch(interrupt = ...)`, in place of a `Cancelled` class that R does not have. "At once" means at the next 100 ms tick, the first residual window of ADR 0042, and the tests bound it at 0.5 s.
    - A deny plant proves the rule it names while offline. A git-sourced dependency fails at resolution before deny runs offline, so the binding drops the git plant. The deny plant removes the `paste` entry of decision 15. Deny then reads RUSTSEC-2024-0436 from the local advisory database and fails, which proves the advisories rule offline.
    - Tests that need parallel requests set the engine throttle they need. The batch interrupt test and the R2-23 tests call `tt_engine(throttle = 8L)` in their own child before the first verb, through the public setting of decision 4. No test uses a hidden hook. Each such child is its own process, so two throttles never meet.

18. **Amendment of 2026-09-24: no test or plant can reach a paid backend.** Main's shared rule at `d783ab6b` forbids it. The environment-seed plant in "Other acceptance" started from the empty `Engine::builder()`. That engine ignores `THINKTHEN_BASE_URL` and posts to the built-in vendor address. Its safety rested on an unset key, and nothing proved the refusal. That plant leaves, and its fallback clause leaves with it.
    - *The real key never reaches a test.* `check.sh` runs every step under `env -u THINKTHEN_API_KEY`. That covers the tarball install, the conformance runs, and every child.
    - *Only the R test children get a scratch home.* The cargo, deny, install, and conformance steps keep the builder's real `HOME`. They need the cargo home, the rustup toolchains, the local advisory database, and the R library of decision 10. Before any R test child starts, `check.sh` resolves the R library list of decision 16 from the real `HOME` and exports each entry as an absolute path in `R_LIBS`. Each R test child then sets `HOME`, `XDG_CACHE_HOME`, and `XDG_CONFIG_HOME` to fresh scratch folders. It keeps the exported `R_LIBS`, so `library(thinkthen)` still finds jsonlite 2.0.0. A fresh `XDG_CONFIG_HOME` means no configuration file can supply an address or a key.
    - *The fake key rides only beside loopback.* `tests/with-backend.sh` exports the fake key `tt-test-not-a-key` in the same step that exports the loopback `THINKTHEN_BASE_URL`, and nowhere else. Before any R process starts, it refuses to run unless the address's host is `127.0.0.1`, `localhost`, or `[::1]`. A test for that guard hands it a non-loopback address and expects a refusal with no R process started. The secrecy test searches every condition message and `print` output for the fake key.
    - *The environment-seed test sends only to loopback.* It proves the seed through the cache folder. That setting needs no extra request. The child sets `THINKTHEN_CACHE` to folder A and gets the scratch `HOME`, `XDG_CACHE_HOME`, and `XDG_CONFIG_HOME` and the exported `R_LIBS` that every R test child gets. It calls `tt_engine(throttle = 8L, base_url = <its loopback backend>)` and runs `tt_decide` on the same text twice. The test asserts one counted send and one entry in A. Plant: after `from_env()`, the Rust half calls `cache_at` with the default folder under `XDG_CACHE_HOME`. That call drops the `THINKTHEN_CACHE` value. The entry is missing from A, and the planted run writes only into the scratch folder. The address stays loopback in both runs, and the loopback listener counts one send in each. The two runs differ only in which folder gets the entry.
    - *Every other plant keeps the builder.* Each plant in this ticket keeps `EngineBuilder::from_env()` and a loopback address. The record shows the loopback count for every planted run.

## What moves from the tag

- The package: `thinkthen/R/thinkthen.R`, `R/extendr-wrappers.R` (regenerated), `NAMESPACE` with its explicit export list, `DESCRIPTION`, `LICENSE`, `configure`, `src/Makevars.in` (both lines renamed), `src/entrypoint.c`, `src/rust/document.rs` (renamed call), `tools/config.R`, and `tools/msrv.R`.
- `src/rust/src/lib.rs`, split by topic into at most five files: `lib.rs` (the crate root, the kind table, and `carry`), `ffi.rs` and its child `ffi/text.rs` (every R API call and every `unsafe`), `calls.rs` (the worker, the wait, and results), and `relate.rs` (recognize and relate).
- `check.sh` keeps its shape, with the changes in the gate section. `tools/make-tarball.sh` is retargeted.
- `examples.R`, `examples.json`, `slide.R`, and `slide_check.R`. The drawn slide block stays untouched, and each expected value is re-derived against 0092's generic arm.
- Tests that call the package keep their assertions: `tests_null.R` (renamed, since the null backend retires), `text_check.R`, `hook_check.R`, `ownership_check.R`, `recognize_check.R`, `fork_check.R`, and the interrupt children with their bash parents. The builder regroups them by topic into at most ten R files. Cases that replayed stand-in recordings move to the 0092 case arm, or to the generic arm for shape checks.
- `conformance.R`, rewritten onto main's `conformance/cases.json`.

These retire:
- `wire_width.R`, because 0086 proves throttle.
- `interrupt_fast.sh`'s null-backend column. The held-arm tests below replace it.
- The stand-in fixture build and every `ENGINE_NULL` line.
- `ACTIVE`, `tt_cancel_active`, and `.tt_cleanup` (decision 6).

`NOTES.md` stays at the tag as history. The port writes a new `NOTES.md` of at most 120 lines. `sdlc/planning/libraries/r.md` is the design page, and this ticket updates it.

## Error-index rows

Source: `sdlc/issues/closed/2026-09-23-surfaces-branch-error-index.md`. The index lists 13 R rows, and the port guide classes all 13 as binding. This ticket carries all 13. It also carries the R halves of twelve cross-surface rows and of two engine rows. R5-38 rides with R4-7 in the first table, as the port guide pairs them. It retires the R halves of five more rows with their reasons. Each re-proof runs against the real engine through the 0092 loopback backend unless marked as a unit test or a script step.

"Counted" means the 0092 backend's `count` line. A bash parent starts the backend with `coproc`, exports `THINKTHEN_BASE_URL` with the arm's path, and reads `count` on the backend's standard input. `tests/with-backend.sh` runs one R file per backend process and compares the final count with the file's printed `expect count N` line. The record plants each bug below and shows its test turning red, then green once the bug is removed.

### R rows

| Row | Status at tag | Re-proof here | Planted bug |
|---|---|---|---|
| R1-5 | closed | A set holds a tag member with threshold 0.95. The generic arm gives no label that reaches it, so every cell is `character(0)`. The column stays a list of character vectors. A decide member under a band stays logical. | Type each column from its first cell. The tag column becomes character with `NA`. |
| R1-13 | closed | A `check.sh` step counts `R_CheckUserInterrupt()` call expressions in `src` and finds exactly one, inside the function passed to `R_ToplevelExec`. The `extern` declaration is not a call expression and does not count. The interrupt tests below run beside it. | Add a direct `R_CheckUserInterrupt()` in the tick loop. The step fails. |
| R2-5 | closed | In a child `Rscript`, a duplicate option spelled `100% sure %s` raises `thinkthen_usage`. The message holds the option verbatim, and the child exits 0. A Rust unit test pins `carry`'s doubled `%`. | Drop the `%` doubling. The unit test and the pinned message turn red. |
| R3-3 | closed | In a child, a set file with a member named `a\u0000b` raises `thinkthen_usage`, and the child exits 0. A Rust unit test pins `carry`'s `\\u0000` escape. If main's parser refuses the name first, the record says so, and the unit test carries the plant. | Drop the NUL escape. The unit test turns red. |
| R3-15 | closed | `tt_annotate` with a question named like an input column raises `thinkthen_usage` naming it. The count is 0. | Drop the clash check. The count is nonzero and the `on` column is overwritten. |
| R3-17 | closed | Under `LC_ALL=C`, a native-marked question and evidence that are valid UTF-8 give the same `question_sha256` and request digests in `tt_details` as under `C.UTF-8`. Native invalid bytes and a bytes-marked string raise `thinkthen_usage` with a count of 0. | Translate native text through `Rf_translateCharUTF8`. The digests differ. |
| R4-7 and R5-38 | closed | The tarball step below runs `make-tarball.sh` in a fresh `git archive` tree with `CARGO_NET_OFFLINE=true`, and the vendoring step succeeds from the builder's cargo home (R5-38). It then installs the tarball into a scratch library with an empty `CARGO_HOME`, and one `tt_decide` answers (R4-7). The cargo cache test runs first against the builder's cargo home, so either plant reads "fail" and never "not run". | R4-7: skip `cargo vendor`. The install fails with "no matching package". R5-38: point `CARGO_HOME` at the package's empty, gitignored `src/rust/.cargo` before `cargo vendor`, as the round-5 bug did. The vendoring step fails with "no matching package". |
| R5-10 | closed | Under `LC_ALL=C`, native-marked choose options and relate rules give the same digest as under `C.UTF-8`. | Drop the `Encoding` fix in `.tt_json`. The digests differ. |
| R5-11 | closed | An option holding `\x1f` raises `thinkthen_usage` with its kind intact. A Rust unit test pins the `\\u001f` escape. | Drop the separator escape. The kind is lost and the class check fails. |
| R5-12 | closed | `hook_check.R` stubs `tools::pskill` to return `TRUE` without sending a signal. The call still stops and prints `OUTCOME stopped`. | Return `NULL` after the sleep. The script goes on. |
| R6-10 | closed | `tt_tag(q, x, labels = "refund")` answers `"refund"` from the generic arm. | Send labels without `I()`. jsonlite unboxes the label and the call raises `usage`. |
| R7-11 | open at index, fixed at tag | `deadline = 5L`, `I(5)`, and `-1L` answer. `0L` and `I(0L)` raise `thinkthen_deadline` with a count of 0. `NA_integer_`, `factor("5")`, `I(factor("5"))`, `as.difftime(5, units = "secs")`, and `TRUE` raise `thinkthen_usage` with a count of 0. | Read only doubles. `5L` raises `usage`. |
| R7-13 | open at index, fixed at tag | `interrupt_hook.sh` runs one child as plain R and one as a held `tt_decide`, each with an `options(error=)` hook. Uncaught, both print `HOOK RAN` and `AFTER`. Caught by `tryCatch(interrupt=)`, both print `CAUGHT` and `AFTER` and no hook line. | Deliver the interrupt with the hook still set aside. The uncaught case loses `HOOK RAN`. |

### R halves of cross-surface and engine rows

| Row | Status at tag | Re-proof here | Planted bug |
|---|---|---|---|
| R1-10 host half | engine | Rust unit test: the worker helper runs a closure that panics with `boom`. The result is `defect` with the panic's words. A second call on a new worker then answers. | Remove the `catch_unwind`. The disconnect answer lacks `boom`, and the pinned message turns red. |
| R1-11 host half | engine | `deadline = 1e300`, `Inf`, `-Inf`, and `NaN` raise `thinkthen_usage` with a count of 0. | Convert with `Duration::from_secs_f64` in the shim. extendr's wrapper turns the panic into a plain error, and the class check fails. |
| R2-10 R half | partial | `-1` and `NULL` mean none. `0` raises `thinkthen_deadline` with a count of 0. `-2` and `4294967296` raise `thinkthen_usage` with a count of 0. | Treat any negative as none. `-2` answers. |
| R2-23 R half | open/waive | At throttle 8, `tt_choose` over 50 texts on the held arm shows a count above 1 before any release. `tt_score` and `tt_tag` do the same. | Loop one call per row. The count stays at 1. |
| R2-29 R half | open | The binding re-proves ADR 0042's three check points and three windows with the interrupt tests below. The NOTES-only R rulings land in a short R section of ADR 0047: `I()` over a number, a refused classed deadline, the annotate route for R2-23, `jsonlite` kept, relate's frame input, and the ratchet place. ADR 0042 gains an amendment with the measured signal-to-condition time on main and the retired cleanup pieces. | Not a code row. The review checks that each ruling has an ADR line. |
| R2-32 R half | not re-probed | The binding's deny call reads `libraries/r/deny.toml` and passes with the one `paste` entry of decision 15. The root deny run stays free of the unmatched-advisory warning, and a `lint` test pins that. | Remove the entry. Deny fails on RUSTSEC-2024-0436. A second plant adds any other line to the binding's config, and the equality step fails. |
| R3-30 and R5-32 R half | not re-probed | The runner has no local skip list. It reports every case in `cases.json` as pass, fail, or not run with the reason, and the three counts sum to the file's count. Case 18 reports not run and names the batch interrupt test that proves it. | Skip one case silently. The sum check fails. |
| R4-19 R half | closed | A `check.sh` step reads `tools/config.R`, `check.sh`, and `make-tarball.sh`, and fails on any `cargo` call without `--locked` and `--offline`. `cargo vendor` and `cargo package` are such calls. | Drop `--offline` from `cargo vendor`. The step fails. |
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

## Interrupt tests (ADR 0042)

Each child runs under `timeout`, with its own backend and cache folder (decision 17). Each child that expects an interrupt catches it with `tryCatch(interrupt = ...)`, prints `CAUGHT` with the catch time, and then sleeps 5 s before it exits. The parent therefore reads every count while the child and its detached worker still live. A held reply is always released or its child killed before the test returns.

- **Check point before a call.** A child passes an evidence argument that sends itself `SIGINT` with `tools::pskill` as R forces it. The signal lands after `.tt_call`'s first check and before the Rust call spawns its worker. The interrupt arrives, and the count stays 0. Plant: drop the Rust check before the spawn. The tick catches the interrupt after the send, and the count reads 1.
- **Check points around `.tt_call`.** One child runs `thinkthen:::.tt_call` over an expression that would set an environment variable, right after it signals itself. The interrupt arrives, and the variable stays unset. A second child runs `.tt_call` over an expression that signals itself and returns `TRUE`. The interrupt arrives in place of `TRUE`. Plants: drop the first check, and the variable is set. Drop the second, and `TRUE` returns.
- **The tick, single call.** Hold one `tt_decide` on the held arm. Signal once `count` reads 1. `CAUGHT` arrives within 0.5 s of the signal, before any release. That proves the tick and the window for a request already on the wire. Release, and the count read at +1 s stays exactly 1. Plants: block on `recv` with no tick, and no `CAUGHT` arrives within 5 s. Join the worker before returning the marker, and `CAUGHT` arrives only after release, past 0.5 s.
- **The tick, batch.** Run a 200-text `tt_decide` at throttle 8 on the held arm. Signal once two `count` reads 100 ms apart agree above 0. `CAUGHT` arrives within 0.5 s. After release, the counts read at +1 s and +3 s equal the count at the signal and stay below 200. Plant: the drop guard skips `token.cancel()`. The detached batch keeps sending in the living child, and the +3 s count exceeds the count at the signal.
- **The blank line.** `interrupt_hook.sh` keeps standard error for the plain child and the held child, both uncaught and both caught. Each held case prints exactly one more blank line than its plain twin, and the other lines match. Plant: `.tt_interrupt` prints one extra `message("")`. The held case prints two extra blank lines, and the count assertion fails.
- **The bound.** 0.5 s is the 100 ms tick plus the child's slack under load, and the test keeps it. The record measures the signal-to-`CAUGHT` time on main over ten runs. The ADR 0042 amendment records that figure and names the bound as the tick plus the child's slack.

## Other acceptance

- Red first: the ported tests fail against an empty `libraries/r` workspace for the stated reason, then pass.
- `cargo test --lib --locked --offline` in `libraries/r/thinkthen/src/rust` passes. The unit tests need R installed but not running, since extendr-ffi's build script runs R and the test binary links `libR.so`.
- A Rust unit test covers each worker edge. When the caller has left after an interrupt, the worker ignores the failed channel send and does not panic. When the channel closes with no result, the wait returns `defect`.
- The conformance runner runs every applicable case through the 0092 case arm with recomputed digests. Case 17 asserts counter differences. No backend arm is added.
- Case 68 (an accent and an emoji) gives the same `start` and `end` in R, after the one-based shift, as in Rust.
- `fork_check.R` warms the default engine and forks with `parallel::mcparallel`. The child answers, and the parent's counters do not move (0096, Q15).
- `tt_engine(throttle = 8L)` in a child lets a 200-text `tt_decide` on the held arm reach a count of 8 before any release, and never more. Plant: drop the throttle from the builder. The count stops at 4.
- `tt_engine(throttle = 8L)` keeps the environment, proved through the cache folder as decision 18 states. Both runs count one send on the loopback backend.
- A set `base_url` wins over the variable. With `THINKTHEN_BASE_URL` naming a refused loopback port, `tt_engine(base_url = <backend>)` counts 1 on the backend.
- One refusal test per argument, each with a count of 0 and `thinkthen_usage`: `base_url = 5`, `model = ""`, `throttle = 0L` and `33L`, `max_requests = -1`, `cache = TRUE`, and `cache_bytes = -1`. A second `tt_engine(throttle = 4L)` after `throttle = 8L` raises it too.
- `cache = FALSE` sends a repeated question twice. `max_requests = 1L` over a two-text `tt_decide` sends one request and refuses at the second record, because the engine streams (amended 2026-09-25 after the build found the engine's rule; rank refuses before any request).
- A relate frame with a repeated name and kind sends one entity for it, and each edge carries names and kinds. `igraph::graph_from_data_frame` accepts the result.
- `tt_details` on a score question carries `answer.level`, and on a decide question it carries none.
- The `model = ""` test pins `thinkthen_usage` with a count of 0 whether or not `EngineBuilder::model` refuses an empty string. If the builder accepts it, the R type check refuses it first.
- Nothing reaches a non-loopback address. The real `THINKTHEN_API_KEY` is removed, and the fake key rides only beside a loopback address (decision 18). A secrecy test reads every condition message and `print` output for the fake key and for credentials in the base URL.

## The check it adds to the gate ladder

- The check follows the shared surface rules of decision 17.
- `libraries/r/check.sh` joins the surface registry as landed. The `surfaces` rung (ADR 0047, the fifth rung after `spec`) runs it with the 0092 loopback port.
- It runs on this Linux host with no Docker and no network. Observed by command on 2026-09-24:
  - R 4.3.3 at `/usr/bin/R`, rustc and cargo 1.93.1, and cargo-deny 0.19.4 are installed. `/usr/lib/libR.so` exists.
  - dplyr 1.1.4, tidyr 1.3.1, and igraph 1.6.0 are in the system site library `/usr/lib/R/site-library`. jsonlite 2.0.0 is in the user library, and the system site library holds jsonlite 1.8.8 (decision 10).
  - Every registry crate in the tag's R lock, extendr 0.8.2 and paste 1.0.15 among them, is in the cargo cache.
  - The freeze record shows R's check passed at the tag with the wire suite (94 ok lines).
- A missing R 4.2 or later, a missing or short required R package, or a cargo cache miss reports "not run" and never "pass" (R6-2). The line names the one fetch to run on a networked machine: `cargo fetch --locked --manifest-path libraries/r/thinkthen/src/rust/Cargo.toml`, or `libraries/r/tools/setup.sh` for the named R package. `setup.sh` fetches only archives whose sha256 `pins.sha256` holds: jsonlite 2.0.0 and the tested Suggests versions (dplyr 1.1.4, tidyr 1.3.1, igraph 1.6.0) with the dependency archives the builder records.
- The tarball step runs `make-tarball.sh` in a `git archive` tree of the commit under test. It installs the tarball into a scratch library with an empty `CARGO_HOME` and `CARGO_NET_OFFLINE=true`, and one call answers there. It replaces the tag's `--stage-only` step.
- `lint` runs on the binding:
  - the ADR 0047 manifest, lock, lint-table, and profile checks, reading R's manifest path from the registry;
  - the deny-config equality step of decision 15;
  - deny as `cargo deny --offline --manifest-path libraries/r/thinkthen/src/rust/Cargo.toml check --config libraries/r/deny.toml advisories bans licenses sources`, planted offline by removing the `paste` entry (decision 17);
  - `ratchet.mjs` on `libraries/r/ratchet.json` for Rust and `ratchet.R.json` for R;
  - the registry check.
- Lints. The binding's table equals the root table except `unsafe_code = "deny"`. The root forbids `missing_debug_implementations`, `unreachable_pub`, and `unsafe_code`, and denies `expect_used`, `unwrap_used`, `indexing_slicing`, `panic`, and `allow_attributes_without_reason`. A local `allow` cannot lift a forbid. The builder's first step compiles one empty `#[extendr]` function and `extendr_module!` inside `ffi` under this table and records the result. `R_init_thinkthen_extendr` is a generated `pub` item in a private module, so `unreachable_pub` is the likeliest trip. If extendr's generated code trips a forbid-level lint, the builder stops and records the case for an ADR 0047 amendment.

### Departure: the ladder changes move to the landing agent (2026-09-25)

The builder's brief forbade ladder changes, so this build does not touch `sdlc/scripts/`. The code review accepted the departure on the condition that the landing agent makes these five changes before the `surfaces` rung counts R:

1. The registry reads R's manifest at `libraries/r/thinkthen/src/rust/Cargo.toml`. Today `sdlc/scripts/surfaces --registry` looks for `libraries/r/Cargo.toml`, which does not exist.
2. Deny for R uses `libraries/r/deny.toml`. Today the registry passes the root `deny.toml`, which lacks the `paste` entry and fails on RUSTSEC-2024-0436.
3. Deny for R runs the `sources` check with the other three: `advisories bans licenses sources`.
4. `lint` runs the deny-config equality step of decision 15: R's `deny.toml` with its `ignore` list set back to `[]` equals the root file byte for byte.
5. `policy.py` finds R's manifest at `libraries/r/thinkthen/src/rust/Cargo.toml`. Today it globs `libraries/*/Cargo.toml`, and its path rule expects `../../crates/thinkthen` where R's path is `../../../../../crates/thinkthen`.

## Dependencies and second review

- Rust: `thinkthen` by path with default features off, and `extendr-api` 0.8.2 with `extendr-ffi` 0.8.2 and `extendr-macros` 0.8.2, from the tag's lock and this machine's cache. `paste` 1.0.15 comes in through `extendr-api`. The lock's versions for `thinkthen`'s own tree equal the root lock's (ADR 0047 item 1).
- R: `jsonlite (>= 2.0.0)` in `Imports:`, the tested version (decision 10), with its setup archive pinned by sha256. `Suggests:` lists dplyr, tidyr, and igraph for the tests. The package depends on no other package.
- These enter main for the first time. The code reviewer checks each entry, the binding lock, deny's result, and the `paste` entry with its reason, and the review record says so (repo `CLAUDE.md`).

## Budgets

- Production Rust: at most five files and 950 nonblank lines, each file under 500. The tag's `lib.rs` measures 899 nonblank lines, 862 before its test module. The shim loses the connector, `tt_annotate_kinds`, the three one-row functions, `tt_decide_one`, and the `ACTIVE` slot. It gains the `Evidence` type, the one-question set, relate entity mapping, the kind table, and the drop guard.
- R package code: `thinkthen.R` and `extendr-wrappers.R` together at most 670 nonblank lines. The tag measures 639. `tt_engine` and its type checks take about 20 of the 31 lines of room, so the budget is tight. The removed `.tt_section`, `kind_field`, `.tt_cleanup`, and the one-row branches pay for the relate frame and the failed-cell raise and `tt_engine`.
- Tests: at most ten R test files and 1,300 nonblank lines, bash parents at most 250, Rust unit tests at most 150, and the conformance runner at most 300. The tag's moving tests measure about 890 R lines, and `conformance.R` measures 306.
- Scripts: `check.sh`, `make-tarball.sh`, `tools/config.R`, `tools/msrv.R`, and the new `tools/setup.sh` together at most 460 nonblank lines. The tag measures 373, and `setup.sh` is new. `pins.sha256` is data and falls outside the count. Gate changes under `sdlc/scripts` at most 40 nonblank lines, the registry paths and the deny-config equality step included.
- Documentation: `README.md`, the new `NOTES.md`, `sdlc/planning/libraries/r.md`, the ADR 0047 R section, and the ADR 0042 amendment, at most 240 net nonblank lines.
- Ratchet: `libraries/r/ratchet.json` and `ratchet.R.json` each set `max` to the measured total. The root `sdlc/ratchet.json` does not change. The record names what each block earns and where the tag's duplicate code went first.

Stop and re-score before crossing a budget, adding a dependency beyond this list, touching `crates/thinkthen` or root `deny.toml`, or widening the export list past the fourteen names.

## Exclusions

CRAN, R-universe, prebuilt binaries, and Windows or macOS installs (the release ticket, queue item 11). The C door as R's backend. Any change to `thinkthen`. A user-facing cancel token in R. Async forms. Any live or paid call.

## Dependencies

After 0086. Also after the 0084 amendment at `f19cf437`, which adds `EngineBuilder::from_env`, and after 0098 (labels, spec readers, JSON methods, `ErrorKind::name`, `members`), 0093 (the registry, the `surfaces` rung, the ratchet argument, and the binding policy checks), and 0094, since the plan puts C before every other surface. 0092 and 0099 have landed.

## Routing

Builder: Claude (Opus subagent). Reviewer: a fresh Claude session for design and for code.

## Complexity

Contract 2; state and timing 3; reach 2; proof 3; cost of error 3; total 13. Final level: 3. R's interrupt is a longjmp that must never cross a Rust frame. A wrong tick loses a user's Ctrl-C or spends a column.

## Review

- Design review: `sdlc/records/2026-09-24-design-review-0108.md` found eight findings and a list of small fixes, all answered. R5-38 rides with R4-7 and has its own plant. Every interrupt child catches the interrupt and stays alive past the last count read, and the single call gains a plant that can turn red. Every R API call and every `unsafe` sits in `ffi`. ADR 0042's three check points each have a test, the blank-line window has one, and the four cleanup pieces are kept or retired with reasons. `cargo vendor` and `cargo package` carry their flags. The jsonlite versions are named and the check tests the one it uses. The `paste` entry has its full reason in the binding's own deny config, away from the root run. The rename covers `document.rs` and both `Makevars.in` lines. The coordinator's later request is applied too: decision 17 follows the six shared surface rules and says where ADR 0042 governs R. The small fixes are applied: the ratchet place, R needed for unit tests, the worker edge test, R1-13's call-expression count, the tight R budget, and the `missing_docs` reason.
- Confirmation (same file) rejected one point: the throttle reached the engine through a hidden `thinkthen:::` hook, against shared rule 6, and cited the rules at an old commit. Decision 4 now gives R the public `tt_engine(throttle =)` setting, and decision 17 cites `446a4d6b`. The two stop clauses about a missing throttle setting are gone.
- Final check (same file) rejected one point: `tt_engine` built on the empty builder and dropped the address, key, and cache the environment gives. Decision 4 now starts from `EngineBuilder::from_env` (0084 amendment `f19cf437`), exposes every section 5 setting but the key, and names what happens after a verb has run and what `tt_usage` reads. Tests pin the kept environment, a set `base_url` over the variable, and one refusal per argument.
- Second final check (same file) accepted it and left two notes for the code reviewer, now in the acceptance list: the loopback plant sends nothing without a key, and the `model = ""` refusal holds either way.
- Amendment of 2026-09-24: decision 18 applies the no-paid-backend rule at main `d783ab6b`. The empty-builder plant and its note leave. The seed test proves the environment through `THINKTHEN_CACHE`, and the checks run with the real key removed.
- Amendment review (`sdlc/records/2026-09-24-amendment-review-0108-0109-0112.md`) rejected decision 18 at `e89fe8b5`. A scratch `HOME` for every step of `check.sh` hid the R library holding jsonlite 2.0.0, the toolchain folder, the cargo home, and the deny advisory database. Decision 18 now limits the scratch `HOME` to the R test children, exports the R library list as absolute paths first, and gives each child a fresh `XDG_CONFIG_HOME`.
- Amendment re-check (same file) accepted decision 18 at `87e5d053`.
- Spike amendment check (same file) accepted the experiment 256 amendment at `8805497b`. Its one note is applied: each R test child unsets `R_LIBS_USER`.
- Code review: pending. Reviewer's note for it: the "before a call" test and the first `.tt_call` test signal the child itself, and R handles a pending SIGINT at its evaluator's next periodic check. If that check comes before the check under test, the interrupt jumps in R code, and the test passes with or without its plant. The record shows each of the two plants turning red. If one cannot, the builder moves the signal next to the check, for example into the forced argument's last expression.

## Evidence

Builder note, 2026-09-24. Workspace decision `2026-09-24-experiments-reduce-risk.md` asks every product ticket to name these five parts. This note changes no design.

- Starts from: The tag `surfaces-wave7-frozen-2026-09-24b` holds the package in `libraries/r/thinkthen/` (`R/thinkthen.R`, `src/rust/src/lib.rs`, `NAMESPACE`, `DESCRIPTION`, `configure`, `Makevars.in`, `entrypoint.c`) and `libraries/r/check.sh`, `tools/make-tarball.sh`, `examples.R`, and `slide.R`. Its 16 test and interrupt scripts sit in `libraries/r/`. Local experiment 205's `FINDINGS.md` found that R leaks a receiver per interrupt, leaves 16 threads parked, adds 16 to 21 µs a call, and needs no Arrow door. A private experiment repository: none found.
- Keeps: The 13 exports, `NA` for unsure, a column in and a column out, the six kinds as conditions with a retry signal, and the ported test assertions.
- Changes: `tt_engine` holds settings, the crate becomes `thinkthen-r`, `lib.rs` splits into five files, and every call runs on a worker. Column choose, score, and tag make one `annotate_with` call. The global active-call state retires, and R gets its own `deny.toml`.
- Proof: The interrupt tests under ADR 0042, an offline tarball install into a scratch library, the deny plant, and the extendr lint probe.
- Defers: CRAN and R-universe, binaries, Windows and macOS, the C door, a user cancel token, and async.

Amended 2026-09-24 and applied in place 2026-09-25: the ADR 0017 amendment of 2026-09-24 on main renames the width setting to the throttle. This ticket now spells it the engine argument `throttle` throughout, matching main's `EngineBuilder::throttle`. The command keeps `--jobs`.

## Spike finding (2026-09-24)

Experiment 256 on beelink (local experiment 256, `REPORT.md`) built crate `thinkthen-r` on extendr-api 0.8.2 over `crates/thinkthen` at main `e7696ca8`, installed it into a scratch library, and ran R children with a scratch home. It confirms decisions 10, 15 and 18 and fixes two details.

- **Pin.** `jsonlite_2.0.0.tar.gz` sha256 is `75eb910c82b350ec33f094779da0f87bff154c232e4ae39c9896a9b89f3ac82d`. CRAN's MD5 `3e54e6fbc0c9063936e3d01e91419c14` agrees. `tools/setup.sh` can pin it.
- **Deny config.** Root `deny.toml` already holds `ignore = []`. The binding copy must replace that line. A second `ignore` key is a parse error, and it reads as a deny failure.

Retired by the spike: the build, Clippy, `cargo test --lib` (linking `libR.so` without running R), and `R CMD INSTALL` with `CARGO_NET_OFFLINE=true` all pass offline on R 4.3.3. Root deny fails on RUSTSEC-2024-0436, and the one-entry config passes. `#[extendr]` needs the module-level `missing_docs` allow this ticket names. With the library list resolved under the real `HOME` and exported as absolute `R_LIBS`, children with a scratch `HOME`, `XDG_CACHE_HOME` and `XDG_CONFIG_HOME` load jsonlite 2.0.0 and send to loopback. Without `R_LIBS`, or with `R_LIBS_USER` left as `~/…`, they load the system 1.8.8 and `library()` fails loudly. `cargo` under a scratch `HOME` made rustup start a network toolchain sync, which confirms the amendment's limit to the R test children.

## Amended 2026-09-24 after experiment 256

The spike finding above confirms decisions 10, 15, and 18 and settles three details. Decisions 10 and 15 now point here.

1. **The jsonlite archive is pinned.** `libraries/r/tools/pins.sha256` holds `75eb910c82b350ec33f094779da0f87bff154c232e4ae39c9896a9b89f3ac82d` for `jsonlite_2.0.0.tar.gz`, the value the spike downloaded and checked. CRAN's `PACKAGES` MD5 `3e54e6fbc0c9063936e3d01e91419c14` agrees with the same file. `setup.sh` refuses any other archive.
2. **The binding's deny config replaces the `ignore` line.** Root `deny.toml` already holds `ignore = []`. `libraries/r/deny.toml` replaces that line with an `ignore` list holding the one `paste` entry. It adds no second `ignore` key. TOML refuses a repeated key, and cargo-deny would report the parse error as a deny failure. The equality step puts `ignore = []` back in place of the list before it compares. The R2-32 row's second plant already turns red on any added line, a second `ignore` key among them.
3. **Only the R test children get the scratch home, and they keep the absolute `R_LIBS`.** Decision 18 stands as written. The spike ran it: children with a scratch `HOME`, `XDG_CACHE_HOME`, and `XDG_CONFIG_HOME` and the exported absolute `R_LIBS` loaded jsonlite 2.0.0 and sent to loopback. With no `R_LIBS`, or with `R_LIBS_USER` left as `~/…`, they loaded the system jsonlite 1.8.8, and `library(thinkthen)` failed with "namespace 'jsonlite' 1.8.8 is being loaded, but >= 2.0.0 is required". `cargo` under a scratch `HOME` made rustup start a network toolchain sync, so no cargo, deny, or install step gets one. Each R test child also unsets `R_LIBS_USER`, so an absolute user library on the builder's machine cannot hide the plant. New plant: drop the `R_LIBS` export. The first R test child fails on that sentence.

Ian can overturn each change.
