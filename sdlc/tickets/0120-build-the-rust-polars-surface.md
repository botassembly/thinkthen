---
flow: build
priority: 120
opens: libraries/polars sdlc/planning/adr/0047-bindings-are-unpublished-crates-over-the-public-api.md sdlc/planning/libraries/rust.md sdlc/planning/polars-plan.md
---

# 0120: Build the Rust Polars surface

Status: draft 2026-09-24, owner Claude.

## Outcome and authority

A Rust program that holds Polars data can ask ThinkThen questions of it. `decide`, `choose`, `score`, and `tag` run over a text `Series`, and `annotate` runs over a `DataFrame`. Each is one engine call over the whole column. Each goes through the caller's own `thinkthen::Engine`, its throttle, and the same batch path a slice of strings takes.

Ian ruled on 2026-09-24 that Rust Polars and Python Polars are both in 0.1 and that a Rust Polars surface ticket joins the surfaces (`sdlc/planning/one-line-plan-2026-09-24.md`, "Ian's rulings, afternoon of 2026-09-24"). That ruling overturns draft ADR 0047 item 8, which deferred the Rust `Series` door past 0.1. The same afternoon he named the width setting the throttle. ADR 0017's amendment "the width is called the throttle" makes the Rust setting `EngineBuilder::throttle`. ADR 0017's ruling of 2026-09-21, "the data frame is Polars", governs the Python side.

Ian can overturn every decision below except his two rulings.

## Does Python Polars need Rust Polars?

No. Ticket 0106 (design accepted on `origin/ticket/0106-port-python-polars`) reads a Polars column through the Arrow C stream capsule, `__arrow_c_stream__`. Its Arrow layer uses only the standard library and pyo3. It adds no Rust dependency. It builds on neither `pyo3-polars` nor the `polars` crate, and the wheel never imports Polars. Experiment 213 measured that door at 12 to 14 ns a record, reading the producer's own buffers (`~/workspace/experiments/228-polars-experiments/NOTES.md`).

Rust Polars needs nothing from Python Polars either. It calls the `polars` crate's own Rust types and holds no pointer code.

So the two doors share no Polars crate. Building the wheel on the `polars` crate was considered and dropped. It would add the Polars dependency tree and a second Rust toolchain to the wheel. It would tie the wheel to one Polars Rust minor version. 0106's zero-copy and width proofs already hold without it.

## Design and decisions

1. **One crate, `thinkthen-polars`, at `libraries/polars`.** It is its own Cargo workspace with its own `Cargo.lock`, under ADR 0047 item 1. It depends on `crates/thinkthen` by path with default features off, and on `polars` with default features off. It uses only the public API. It sets `publish = false` until Ian claims a registry name. Package names are his (plan, afternoon rulings). The release ticket (queue item 11) brings him the name, and the recommendation is `thinkthen-polars`.
   - A `polars` feature on `thinkthen` was the other landing zone, and this ticket does not take it. 0084 allows no feature and no public module in `thinkthen`. The Polars tree would enter the root lock, the root deny run, and `policy.py`'s dependency tables. Polars also needs a newer Rust than the root pin (decision 7), and the spine would inherit that toolchain.
2. **The engine value is `thinkthen::Engine`.** The caller builds it with `EngineBuilder::from_env()` and its setters. Those setters are the Rust spelling of every ADR 0017 section 5 setting: `base_url`, `api_key`, `model`, `throttle`, `max_requests`, `default_cache`, `cache_at`, `no_cache`, and `cache_bytes`. The address, key, and cache come from the environment unless a setter overrides them. The crate adds no setting, no engine type, no static engine, and no call to `default_engine()`. The README states that the throttle is per loaded copy of the library (ADR 0047 item 5). It describes the throttle in plain words as the most requests in flight at once.
3. **The public shape.** The crate root re-exports `thinkthen` and `polars`. A user can then name the exact versions the door was built with. It adds one extension trait, implemented for `thinkthen::Engine`, and one error type:

   ```rust
   pub trait PolarsEngine {
       fn decide_series<Q: DecisionQuestion + ?Sized>(&self, question: &Q, texts: &Series, options: CallOptions<'_>) -> Result<Series, Error>;
       fn choose_series(&self, question: &Question, texts: &Series, options: CallOptions<'_>) -> Result<Series, Error>;
       fn score_series(&self, question: &Question, texts: &Series, options: CallOptions<'_>) -> Result<Series, Error>;
       fn tag_series(&self, question: &Question, texts: &Series, options: CallOptions<'_>) -> Result<Series, Error>;
       fn annotate_frame(&self, questions: &QuestionSet, frame: &DataFrame, on: &str, options: CallOptions<'_>) -> Result<DataFrame, Error>;
   }
   pub enum Error { Engine(thinkthen::Error), Usage(String), Defect(String) }
   impl Error { pub fn kind(&self) -> ErrorKind; }
   ```

   Each method takes `CallOptions`. A plain call passes `CallOptions::new()`. One method per verb keeps the trait at five methods. Ian can overturn this and ask for the `_with` pairs that `thinkthen` uses.
   - `Error` exists because 0084 gives no outside crate a way to build a `thinkthen::Error`. `kind()` returns `Usage` or `Defect` for the door's own refusals and the engine's kind otherwise. `Display` gives the message. `source()` gives the engine error. `Debug` is log-safe: it names a column and a row number and never a text.
4. **What each method does.**
   - `decide_series` calls `decide_many_with` once over the column's borrowed strings. It returns a `Boolean` series under the input's name. A null means "not sure".
   - `choose_series`, `score_series`, and `tag_series` put the question in a one-question set and call `annotate_with` once. 0095 rules this bulk form for these three verbs. `choose_series` returns a `String` series with a null where nothing fits. `score_series` returns `Float64`, the position from 0 to K−1. `tag_series` returns `List(String)`.
   - A failed row in `choose_series`, `score_series`, or `tag_series` widens the whole series to `String` by the rule of decision 9. The good answers stay, as ADR 0017's amendment "answered or failed logical results" asks of bulk host forms. A failed row never becomes a null. `decide_series` returns the engine's error, as `decide_many` does.
   - The question given to `choose_series`, `score_series`, or `tag_series` must be of that method's kind, read through `Question::kind`. Labels known at run time come from `Question::choose_labels` or `tag_labels`, or from `Question::from_json`. A wrong kind refuses with `Usage` before any request, with the sentence in decision 5. `decide_series` takes a sealed `DecisionQuestion` with no kind getter, so it relies on the engine's own refusal of a non-decision question (0084).
   - `annotate_frame` reads the `on` column, calls `annotate_with` once, and returns the caller's frame with one new column per question, in `QuestionSet::members` order. The caller's columns come back unchanged. A Polars column clone shares its buffers.
   - One `CallOptions` covers the whole column, so one deadline and one cancel token cover it. An interrupt check set with `CallOptions::interrupt` passes through unchanged.
   - The door reads a column in place. `Series::str()` yields borrowed `&str` values across every chunk and slice offset, and the door hands them to the engine with no copy and no per-row allocation.
5. **Refusals.** Each one happens before any request, and each sentence is pinned by a test.
   - A column that is not text: `the column {name} is {dtype}, not text`. A `Categorical` column refuses too, and the caller casts it.
   - A null row: `the column holds nulls; the engine needs text, and NA rows are the caller's to drop`. That is the Python door's sentence at the tag.
   - A frame with no column `on`: `the frame holds no column {on}`.
   - A question name that already names a frame column: `the frame already holds a column named {name}`.
   - A question of the wrong kind: `{method} needs a {verb} question, and this one is a {kind} question`. The kind words are the lowercase names of `QuestionKind`.
   - An empty column returns an empty column of the verb's type with zero requests.
6. **Cancel and deadline follow `thinkthen`.** A Rust caller owns its threads and signals. The door runs on the calling thread and adds no worker. A cancel stops new requests, and sent requests finish, as every `thinkthen` call does under 0073 and 0085. The shared rule of a detachable worker serves hosts with a Ctrl-C key, such as a Python prompt. It does not apply here. A detached worker would outlive the call, and ADR 0017 section 2 says nothing outlives a call. Ian can overturn this.
7. **The toolchain.** Polars 0.55 needs Rust 1.95, because `sysinfo 0.39` refuses 1.93 (experiments 215 and 216). The root pin stays at 1.93.1. `libraries/polars/rust-toolchain.toml` is absent. The crate's `rust-version` names the version Polars needs.
   - `libraries/polars/setup-toolchain.sh` is setup and never a gate. It downloads the standalone `rust-1.95.0` archive from `static.rust-lang.org` for `x86_64-unknown-linux-gnu` or `aarch64-apple-darwin`. It checks the archive against a sha256 pinned in `libraries/polars/toolchain.sha256` and refuses a mismatch. It installs under `~/.cache/thinkthen-toolchains/rust-1.95.0/` with the archive's own `install.sh --prefix`. It needs no sudo and touches no rustup state.
   - `check.sh` puts that folder's `bin` first on `PATH`. A missing folder reports "not run" and names the setup script. It never reports "pass".
   - The builder takes each pinned sha256 from the archive's published `.sha256` file and records both in the build record.
8. **Polars version and features.** The manifest asks for `polars = { version = "0.55", default-features = false }`, plus only the features the five methods need. The builder finds that set, and the review checks it. The lock pins one exact version. Polars' Rust API changes between minor versions, so one door release supports one Polars minor version. The README says which one, and it shows the re-exported `polars` as the way to match it.
9. **The column contract, shared with 0106.** A frame's new columns follow the Python door. A decide column is `Boolean`, a choose column `String`, a score column `Float64`, and a tag column `String` holding the JSON array text. When any row's answer to a question failed, that question's whole column widens to `String`. A widened cell holds its member's JSON text from `AnnotatedRecord::value_json`, lifted unchanged as a `serde_json` `RawValue`. `thinkthen` already turns on serde_json's `raw_value` feature. So a decision reads `true` or `false`, a number and a tag array read as the engine wrote them, and a failed cell reads as the marker, such as `{"failed":{"kind":"backend","cause":"missing_probability"}}`. Two cells differ from the raw text: a choice holds its plain label with no quotes, and a not-sure or nothing-fits cell stays null. Nothing re-serializes a parsed value, so key order and number text stay the engine's. A frame built in Rust then reads the same as one built in Python. This ticket writes the table once, as the Polars item of ADR 0047, and both doors cite it. 0120 lands after 0106. Where 0106's landed behavior differs from this table or from a sentence in decision 5, this ticket follows 0106, and the record names each difference.

## How it shares code with 0106

Neither door depends on the other, and no helper crate joins them. ADR 0047 item 1 bars one binding depending on another. Item 7 puts host-neutral code in the public API. Every piece that does not touch a host type is shared through `thinkthen`:

- The same engine calls: `decide_many_with` for decide columns, one-question `annotate_with` for choose, score, and tag, and `annotate_with` for frames.
- Column order and column kinds from `QuestionSet::members`, and the question's kind from `Question::kind`.
- The failed marker's bytes and every widened cell's text from `AnnotatedRecord::value_json`, the engine's one serializer (0095). The door lifts each member's raw text out of that JSON. No binding keeps its own table of failure causes. The tag's Python door serialized the stand-in's `Failed`, and the real `Failed` has no serializer. So 0106 needs the same source, and the ADR 0047 Polars item names it for both doors.
- Test tools: the 0092 backend with 0117's delay arm, `round`, and `wait`, through its Rust library as a dev-dependency. The shared cases in `conformance/cases.json`.

What each door keeps is host conversion. Python reads Arrow C buffers through `unsafe` code. Rust reads `polars` types through safe calls. No type exists that both could share. The written column contract (decision 9) and one refusal sentence keep the two alike.

## Evidence

- Starts from: tag `surfaces-wave7-frozen-2026-09-24b` (`9df8bae9`), `libraries/rust/src/polars.rs` (the feature-gated Series door over the stand-in, with its `decide_column`, `choose_column`, `score_column`, `tag_column`, and `annotate_frame`) and `libraries/rust/tests/polars_door.rs` (eight tests). They passed under the stand-in's null backend with Rust 1.95 in the freeze run (`sdlc/records/surfaces-freeze-2026-09-24.md` at that tag: Rust, 39 tests). Experiments 213 to 216 (`~/workspace/experiments/228-polars-experiments/NOTES.md`) measured the width gate through a column, the fork with a warm Polars pool, and the Polars 0.55 toolchain need.
- Keeps: one crossing per column through the batch path, answers in input order, null as "not sure" in a decide column, the null-row and non-text refusals, the caller's frame columns unchanged, and the widening of a failed question's column to text.
- Changes: the door leaves the stand-in for the public `thinkthen` API in its own crate. `choose`, `score`, and `tag` over a column become one `annotate_with` call. At the tag they made one engine call per row, the same defect as index rows R1-24 and R4-23 in Python. Every method takes `CallOptions`. The tag's methods took none. The caller's engine value replaces the tag's inherent methods on a stand-in engine. Frame tag columns become JSON text to match the Python door. At the tag the Rust frame wrote `List(String)`. Tests leave `ENGINE_NULL` for the loopback backend.
- Proof: the acceptance tests below, each with its planted bug, run by `libraries/polars/check.sh` in the `surfaces` rung. The equality of a Series call with a slice call at throttle 8, in wall time and in requests in flight, is the proof Ian named for Python Polars on 2026-09-21. This ticket applies it to Rust.
- Defers: a lazy `Expr` door. `filter`, `rank`, `find`, `relate`, and `recognize` over a column. The nearest score level in bulk. The registry name and publication. More than one Polars minor version per release. A `List(String)` tag column in frames for both doors. Windows.

## Error-index rows

The index (`sdlc/issues/2026-09-23-surfaces-branch-error-index.md`) has no Rust Polars row. The tag's Rust door had the defect shapes of these Python Polars rows. 0106 owns the Python rows. This ticket proves the Rust form of each against the real engine through the loopback backend. The record plants each bug, shows its test red, removes the plant, and shows it green. "Counted" means the backend's count.

| Mirrors | Rust proof | Planted bug |
|---|---|---|
| R1-24 Polars half | `score_series` over 200 rows at throttle 8 on the delay arm at 100 ms, with a 1 s deadline, returns `Deadline` near 1 s with at most 96 counted. | Pass `CallOptions::new()` to the engine in place of the caller's options. The call runs past 2 s and all 200 are counted. |
| R4-23 and R2-24 Polars halves | `score_series` over 200 rows at throttle 8 on the held arm: a second thread cancels the token once the count reads 8. After 300 ms the count still reads 8. After `release`, the call returns `Cancelled` with 8 counted. | Loop one `annotate_with` call per row, as the tag did. The count reads 1, and the wait for 8 returns 1. |
| R1-3, with R2-12 folded in | A three-chunk `Series` built with `concat` and no rechunk, then sliced at offset 5, gives each row its own answer on the case arm. In safe Rust a slice is already applied to each chunk, so R2-12's own defect cannot occur here, and this test covers the slice beside the chunks. | Read `chunks()[0]` for every chunk. |
| R1-4 | `annotate_frame` returns the caller's `Categorical`, `Enum`, struct, and list columns with their dtypes and values equal to the input. | Rebuild each caller column through a cast to `String` and back to its declared dtype. The `Categorical` column loses its category order, or the `Enum` cast fails, and the test turns red. |
| R2-28 | No test returns early. A missing toolchain or missing crate makes `check.sh` report "not run" and never "pass". Any failure after the probe step is a plain failure. | Two plants. Remove the toolchain folder: `check.sh` prints "not run", and the rung counts no pass. Add a failing test: `check.sh` fails and does not print "not run". |

## Other acceptance

- **One process for each throttle test.** The throttle is process-wide under 0077, and the shared rules put each throttle test in its own process. Cargo runs each file under `tests/` as its own process. So the R1-24 deadline test, the R4-23 cancel test, and the Series-equals-slice test each sit alone in their own file, one test per file. The other files build engines with no throttle and never hold a request.
- **Series equals slice, the proof Ian named.** In `tests/throttle_equality.rs`, with the engine at throttle 8 on the delay arm at 100 ms: `decide_series` over 200 texts and `decide_many_with` over the same 200 `&str` each finish in about 200 / 8 × 0.1 s = 2.5 s. Each counts 200, their answers are equal, and the two wall times fall within 5 percent of each other. On the held arm each reaches exactly 8 in flight through `wait(8)`, and the count still reads 8 after 300 ms. Plant: loop `decide` per row. One is in flight, and the wait returns 1.
- **The caller's engine carries the call.** The throttle test above counts on the backend named by the caller's `base_url`. Plant: call `default_engine()` in the door. The environment's address is a closed loopback port, and the test's backend counts 0.
- **Shared cases over a Series.** Each case in `conformance/cases.json` whose verb is decide, choose, score, tag, or annotate runs twice on the case arm: once through the `thinkthen` slice form, and once through the door. The values are equal. Plant: map a not-sure answer to `false` in `decide_series`. The band case turns red.
- **Failed marker.** On the case arm, shared case `17-annotate-partial` fails one question with cause `missing_probability` beside good answers. `annotate_frame` widens that question's column to `String`, and the failed cell equals the literal `{"failed":{"kind":"backend","cause":"missing_probability"}}`, pinned in the test. The good cells equal their pinned literal texts. `score_series` over every text of the malformed arm `/arm/malformed/missing_probability` widens to `String` with that marker in each cell. Plants: write `null` for a failed cell, and parse the member and write it back through a sorted map. The first gives a null, and the second gives `cause` before `kind`.
- **Refusals.** Each sentence in decision 5 is pinned whole, and each counts zero requests. Plants: drop the null check, and drop the kind check in `score_series` and pass it a decide question. Each call reaches the engine, and the count is nonzero.
- **No paid backend.** `check.sh` reads the real `HOME` first, to find the toolchain folder and cargo's crate cache, and it never changes `HOME`. It then runs `unset THINKTHEN_API_KEY`, sets `THINKTHEN_API_KEY` to a fake value, and sets `THINKTHEN_BASE_URL` to a closed loopback port. `XDG_CACHE_HOME` and `THINKTHEN_CACHE` point at scratch folders. The test helper `tests/common/mod.rs` refuses to build an engine unless the environment holds exactly that fake key and a loopback address. Every engine it builds names its own test backend with `base_url` and its own cache folder with `cache_at`. Plant: delete both the `unset` line and the fake-key line, and run `check.sh` with a sentinel key set. The helper sees the sentinel, and every engine test turns red before any engine is built.
- **No `unsafe`.** The crate's lint table equals the binding table of ADR 0047 item 3 and carries no `allow(unsafe_code)` anywhere. A `check.sh` step fails on that text in `src/` or `tests/`. Plant: add one `unsafe` block with an allow. The step turns red.
- **The README example builds.** The README's example builds an engine on `EngineBuilder::from_env()` with `base_url`, `model`, `throttle`, `max_requests`, `cache_at`, and `cache_bytes`, then calls `decide_series`. It is a `no_run` doctest, so it compiles and sends nothing. The text names `default_cache` and `no_cache` as the other two cache setters.

## The check it adds to the gate ladder

No new rung. `libraries/polars/check.sh` joins the surface registry (0093) as the tenth surface. The `surfaces` rung runs it. Its steps, in order: the toolchain check against the real `HOME`, the probe `cargo fetch --locked --offline`, the environment of the paid-backend bullet, `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, the `unsafe` text step, and `cargo test`. Every `cargo` call passes `--locked` and `--offline`. Only two failures report "not run": a missing toolchain folder, and a failed probe, which names the one `cargo fetch` to run on a networked machine. Every later failure is a plain failure. `lint` runs the ADR 0047 checks for this binding: the manifest, the lock, the lint table, the profile copy, deny, and `libraries/polars/ratchet.json`.

## Dependencies and second review

- New to main: `polars` (0.55 line) and its tree, in the binding's lock only. `serde_json` with `raw_value` for lifting cells from `value_json`, at the root lock's version. `conformance-backend` as a path dev-dependency.
- The binding lock must resolve `thinkthen`'s own dependencies to the root lock's versions (ADR 0047 item 1). If Polars forces another version, the builder stops and records the case.
- deny runs with the root `deny.toml`. If the Polars tree carries a license outside the root list, the root file cannot take it, since its `unused-allowed-license = "deny"` would fail the root run. The builder then stops. The fix is a binding `deny.toml` that adds only the named licenses, with `policy.py` checking that difference. That fix needs a re-score.
- The code reviewer checks each new dependency, the feature set, the lock, deny's result, and both pinned sha256 values, and the review record says so (repo `CLAUDE.md`).

## Budgets

- Production Rust in `libraries/polars/src`: at most four files and 450 nonblank lines. The tag's door measures about 210 before the error type and the options.
- Rust tests: at most eight files and 900 nonblank lines, the helper included. Three files hold one throttle test each.
- Scripts: `check.sh` and `setup-toolchain.sh` together at most 140 nonblank lines. Gate changes under `sdlc/scripts` at most 20 nonblank lines.
- Documentation: at most 180 net nonblank lines across `libraries/polars/README.md`, the Polars section of `sdlc/planning/libraries/rust.md`, the ADR 0047 Polars item, and `polars-plan.md`.
- The binding lock: at most 230 packages. The tag's lock held 222 with Polars' `lazy` feature and the stand-in.
- Ratchet: `libraries/polars/ratchet.json` equals the measured total. The root `sdlc/ratchet.json` does not change.

Stop and re-score before crossing a budget, adding a dependency beyond this list, touching `crates/thinkthen` or `conformance/`, changing the root toolchain, or adding a verb beyond the five.

## Exclusions

A Polars feature or module in `thinkthen`. A lazy `Expr` door or a Polars plugin. `filter`, `rank`, `find`, `relate`, and `recognize` over a column. Any change to 0106 or the Python wheel. Publication and the registry name. Any live or paid call.

## Dependencies

After 0086 (the public API with `EngineBuilder::from_env` and `throttle`), 0098 (`Question::kind`, `choose_labels`, `tag_labels`, `QuestionSet::members`, `value_json`), 0093 (ADR 0047, the registry, the `surfaces` rung, and the binding checks), and 0106 (the column contract it matches). 0117 has landed. Spike 257 (below) runs first.

## The spike before the build

Spike 257, in its own folder under `~/workspace/experiments/`, answers five questions about the Polars tree before any code. It builds a scratch crate on `thinkthen` at main with `polars` 0.55 and default features off.

1. Which features do the five methods need, and how many packages does the lock then hold?
2. Does deny pass with the root `deny.toml`?
3. Does the lock resolve `thinkthen`'s dependencies to the root lock's versions?
4. Does any Polars release build on Rust 1.93.1? If one does and it serves the five methods, decision 7 drops the second toolchain.
5. How long does a cold `check.sh` build take on Linux and on the Mac?

Its result can change decisions 1, 7, and 8 and the lock budget. The ticket records the result before the build starts.

## Routing

Builder: Claude (Opus subagent). Reviewer: a fresh Claude session for design and for code.

## Complexity

Contract 2; state and timing 2; reach 2; proof 3; cost of error 2; total 11. Final level: 3. The door holds no `unsafe` code, and every guarantee comes from the engine. The risk sits in the dependency tree and the toolchain.

## Review

- Design review: pending.
- Code review: pending.
