---
flow: build
priority: 120
opens: libraries/polars sdlc/planning/adr/0047-bindings-are-unpublished-crates-over-the-public-api.md sdlc/planning/libraries/rust.md sdlc/planning/polars-plan.md
---

# 0120: Build the Rust Polars surface

Status: landed 2026-09-25 in the surface batch, at batch head `03580733` on `ticket/surface-batch`. Integration record: `sdlc/records/surface-batch-integration.md`. Owner: Claude.

## Outcome and authority

A Rust program that holds Polars data can ask ThinkThen questions of it. `decide`, `choose`, `score`, and `tag` run over a text `Series`, and `annotate` runs over a `DataFrame`. Each is one engine call over the whole column. Each goes through the caller's own `thinkthen::Engine`, its throttle, and the same batch path a slice of strings takes.

Ian ruled on 2026-09-24 that Rust Polars and Python Polars are both in 0.1 and that a Rust Polars surface ticket joins the surfaces (`sdlc/planning/one-line-plan-2026-09-24.md`, "Ian's rulings, afternoon of 2026-09-24"). That ruling overturns draft ADR 0047 item 8. That item deferred the Rust `Series` door past 0.1. The same afternoon he named the width setting the throttle. ADR 0017's amendment "the width is called the throttle" makes the Rust setting `EngineBuilder::throttle`. ADR 0017's ruling of 2026-09-21, "the data frame is Polars", governs the Python side.

Ian can overturn every decision below except his two rulings.

## Does Python Polars need Rust Polars?

No. Ticket 0106 (design accepted on `origin/ticket/0106-port-python-polars`) reads a Polars column through the Arrow C stream capsule, `__arrow_c_stream__`. Its Arrow layer uses only the standard library and pyo3. It adds no Rust dependency. It builds on neither `pyo3-polars` nor the `polars` crate, and the wheel never imports Polars. Experiment 213 measured that door at 12 to 14 ns a record, reading the producer's own buffers (the workspace's `experiments/228-polars-experiments/NOTES.md`).

Rust Polars needs nothing from Python Polars either. It calls the `polars` crate's own Rust types and holds no pointer code.

So the two doors share no Polars crate. Building the wheel on the `polars` crate was considered and dropped. It would add the Polars dependency tree and a second Rust toolchain to the wheel. It would tie the wheel to one Polars Rust minor version. 0106's zero-copy and throttle proofs already hold without it.

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
   - `Error` exists because 0084 gives no outside crate a way to build a `thinkthen::Error`. `kind()` returns `Usage` or `Defect` for the door's own refusals and the engine's kind otherwise. `Display` gives the message. `source()` gives the engine error. `Debug` is log-safe: it names a column or a dtype and never a text. Each refusal test asserts the `Debug` text beside the pinned sentence.
4. **What each method does.**
   - `decide_series` calls `decide_many_with` once over the column's borrowed strings. It returns a `Boolean` series under the input's name. A null means "not sure".
   - `choose_series`, `score_series`, and `tag_series` put the question in a one-question set and call `annotate_with` once. 0095 rules this bulk form for these three verbs. `choose_series` returns a `String` series with a null where nothing fits. `score_series` returns `Float64`, the position from 0 to K−1. `tag_series` returns `List(String)`.
   - Amended 2026-09-25 after the build: a failed row in `choose_series`, `score_series`, or `tag_series` ends the call with the engine's `Backend` error, as in `decide_series`. The series asks a one-question set, and the engine refuses a reply with no usable answer (`specification/backends.md` line 93, `specification/annotate.md` line 48). A failed row never becomes a null. The coordinator accepted this for 0.1 on 2026-09-25, and Ian can overturn it.
   - A failed row in `decide_series` ends the call with the engine's `Backend` error, and the good answers of that column are lost. `decide_many` yields one `Result` per row, and a failed row's `Err` carries only a message. The door cannot build the failed marker from it. The frame's decide column differs: it widens, because `annotate_with` returns the marker. Ian can overturn this. The other choice is to send decide columns through one-question `annotate_with` as well. That choice gives up the sealed `DecisionQuestion` argument and needs its own band path.
   - The question given to `choose_series`, `score_series`, or `tag_series` must be of that method's kind, read through `Question::kind`. Labels known at run time come from `Question::choose_labels` or `tag_labels`, or from `Question::from_json`. A wrong kind refuses with `Usage` before any request, with the sentence in decision 5. `decide_series` takes a sealed `DecisionQuestion` with no kind getter, so it relies on the engine's own refusal of a non-decision question (0084).
   - A typed `ChooseQuestion<C>` or `TagQuestion<C>` cannot reach `choose_series` or `tag_series`. 0084 has `into_choose` and `into_tag` from `Question` to the typed form and nothing back. A caller with typed questions puts them in a set through `QuestionSetBuilder::choose` or `tag` and calls `annotate_frame`, and the README shows that path. A generic `C: Choice` form of the two methods would widen the surface, and Ian can ask for it.
   - `annotate_frame` reads the `on` column, calls `annotate_with` once, and returns the caller's frame with one new column per question, in `QuestionSet::members` order. The caller's columns come back unchanged. A Polars column clone shares its buffers.
   - A question set whose members carry `on` pointers cannot run over one text column. The builder records what `annotate_with` does with such a set. If the engine does not refuse it, the builder stops and files an issue. The issue asks the library to refuse such a set or to expose each member's pointer, where only a pointer other than the empty root counts. The door adds no refusal the public API cannot support.
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
9. **The column contract, shared with 0106.** A frame's new columns follow the Python door. A decide column is `Boolean`, a choose column `String`, a score column `Float64`, and a tag column `String` holding the JSON array text. When any row's answer to a question failed, that question's whole column widens to `String`. A widened cell holds its member's JSON text from `AnnotatedRecord::value_json`, lifted unchanged as a `serde_json` `RawValue`. `thinkthen` already turns on serde_json's `raw_value` feature. So a decision reads `true` or `false`, a number and a tag array read as the engine wrote them, and a failed cell reads as the marker, such as `{"failed":{"kind":"backend","cause":"missing_probability"}}`. Two cells differ from the raw text: a choice holds its plain label with no quotes, and a not-sure or nothing-fits cell stays null. Nothing re-serializes a parsed value, so key order and number text stay the engine's. A frame built in Rust then reads the same as one built in Python. The issue `sdlc/issues/2026-09-25-public-library-api-gaps.md` holds this table and asks 0106's builder to take widened cells from the same source. Whichever Polars door lands first writes the table as the Polars item of ADR 0047, and the other cites it.

## How it shares code with 0106

Neither door depends on the other, and no helper crate joins them. ADR 0047 item 1 bars one binding depending on another. Item 7 puts host-neutral code in the public API. Every piece that does not touch a host type is shared through `thinkthen`:

- The same engine calls: `decide_many_with` for decide columns, one-question `annotate_with` for choose, score, and tag, and `annotate_with` for frames.
- Column order and column kinds from `QuestionSet::members`, and the question's kind from `Question::kind`.
- The failed marker's bytes and every widened cell's text from `AnnotatedRecord::value_json`, the engine's one serializer (0095). The door lifts each member's raw text out of that JSON. No binding keeps its own table of failure causes. The tag's Python door serialized the stand-in's `Failed`, and the real `Failed` has no serializer. So 0106 needs the same source, and the ADR 0047 Polars item names it for both doors.
- Test tools: the 0092 backend with 0117's delay arm, `round`, and `wait`, through its Rust library as a dev-dependency. The shared cases in `conformance/cases.json`.

What each door keeps is host conversion. Python reads Arrow C buffers through `unsafe` code. Rust reads `polars` types through safe calls. No type exists that both could share. The written column contract (decision 9) and one refusal sentence keep the two alike.

## Evidence

- Starts from: spike 255's plan, whose risk 5 covers this ticket's toolchain, and tag `surfaces-wave7-frozen-2026-09-24b` (`9df8bae9`), `libraries/rust/src/polars.rs` (the feature-gated Series door over the stand-in, with its `decide_column`, `choose_column`, `score_column`, `tag_column`, and `annotate_frame`) and `libraries/rust/tests/polars_door.rs` (eight tests). They passed under the stand-in's null backend with Rust 1.95 in the freeze run (`sdlc/records/surfaces-freeze-2026-09-24.md` at that tag: Rust, 39 tests). Experiments 213 to 216 (the workspace's `experiments/228-polars-experiments/NOTES.md`) measured the throttle gate (then called the width gate) through a column, the fork with a warm Polars pool, and the Polars 0.55 toolchain need.
- Keeps: one crossing per column through the batch path, answers in input order, null as "not sure" in a decide column, the null-row and non-text refusals, the caller's frame columns unchanged, and the widening of a failed question's column to text.
- Changes: the door leaves the stand-in for the public `thinkthen` API in its own crate. `choose`, `score`, and `tag` over a column become one `annotate_with` call. At the tag they made one engine call per row, the same defect as index rows R1-24 and R4-23 in Python. Every method takes `CallOptions`. The tag's methods took none. The caller's engine value replaces the tag's inherent methods on a stand-in engine. Frame tag columns become JSON text to match the Python door. At the tag the Rust frame wrote `List(String)`. Tests leave `ENGINE_NULL` for the loopback backend.
- Proof: the acceptance tests below, each with its planted bug, run by `libraries/polars/check.sh` in the `surfaces` rung. The equality of a Series call with a slice call at throttle 8, in wall time and in requests in flight, is the proof Ian named for Python Polars on 2026-09-21. This ticket applies it to Rust.
- Defers: a lazy `Expr` door. `filter`, `rank`, `find`, `relate`, and `recognize` over a column. The nearest score level in bulk. The registry name and publication. More than one Polars minor version per release. A `List(String)` tag column in frames for both doors. Windows.

## Error-index rows

The index (`sdlc/issues/closed/2026-09-23-surfaces-branch-error-index.md`) has no Rust Polars row. The tag's Rust door had the defect shapes of these Python Polars rows. 0106 owns the Python rows. This ticket proves the Rust form of each against the real engine through the loopback backend. The record plants each bug, shows its test red, removes the plant, and shows it green. "Counted" means the backend's count.

| Mirrors | Rust proof | Planted bug |
|---|---|---|
| R1-24 Polars half | `score_series` over 200 distinct texts at throttle 8 on the delay arm at 100 ms, with a 1 s deadline, returns `Deadline` near 1 s with at most 96 counted. | Pass `CallOptions::new()` to the engine in place of the caller's options. The call runs past 2 s and all 200 are counted. |
| R4-23 and R2-24 Polars halves | Covered by the R1-24 row. `CallOptions` has no getters, so the door cannot keep the deadline and drop the cancel token. It passes the caller's value whole or not at all. One pass-through test therefore covers the deadline, the cancel token, and the interrupt check. The R1-24 test stops a column mid-batch, and it stands as this surface's proof for the port guide's `18-cancel-mid-batch`. The per-row loop of the tag is caught by the throttle equality test below. | The R1-24 plant. |
| R1-3, with R2-12 folded in | A three-chunk `Series` built with `concat` and no rechunk, then sliced at offset 5, gives each row its own answer on the case arm. In safe Rust a slice is already applied to each chunk, so R2-12's own defect cannot occur here, and this test covers the slice beside the chunks. | Read `chunks()[0]` for every chunk. |
| R1-4 | `annotate_frame` returns the caller's `Categorical`, `Enum`, struct, and list columns with their dtypes and values equal to the input. | Cast the caller's `Categorical` column to `String` and leave it there. The dtype check turns red. |
| R2-28 | No test returns early. A missing toolchain or missing crate makes `check.sh` report "not run" and never "pass". Any failure after the probe step is a plain failure. | Two plants. Run `check.sh` with `HOME` set to an empty scratch folder, so it finds no toolchain: it prints "not run", and the rung counts no pass. Add a failing test: `check.sh` fails and does not print "not run". |

## Other acceptance

- **One process for each throttle test.** The throttle is process-wide under 0077, and the shared rules put each throttle test in its own process. Cargo runs each file under `tests/` as its own process. So the R1-24 deadline test and the Series-equals-slice test each sit alone in their own file, one test per file. The other files build engines with no throttle and never hold a request.
- **Series equals slice, the proof Ian named.** In `tests/throttle_equality.rs`, each run gets its own backend, its own cache folder, and its own engine at throttle 8. 0077 accepts a second engine with the same throttle. The 200 texts are distinct, because the engine asks once for equal texts in one call. On the delay arm at 100 ms, `decide_series` over the 200 texts and `decide_many_with` over the same 200 `&str` each finish in about 200 / 8 × 0.1 s = 2.5 s. Each counts 200, their answers are equal, and the two wall times fall within 5 percent of each other. On the held arm, again with a backend per run, each reaches exactly 8 in flight through `wait(8)`, and the count still reads 8 after 300 ms. `score_series` over 200 distinct texts on a third held backend also reaches exactly 8. Plants: loop `decide` per row in `decide_series`, and loop one `annotate_with` per row in `score_series`, as the tag did. Each holds one in flight, and its wait returns 1.
- **The caller's engine carries the call.** The throttle test above counts on the backend named by the caller's `base_url`. Plant: call `default_engine()` in the door. The environment's address is a closed loopback port, and the test's backend counts 0.
- **Shared cases over a Series.** These cases in `conformance/cases.json` run twice on the case arm, once through the `thinkthen` slice form and once through the door, and give equal values: 01 to 12, `17-annotate-mixed`, `17-annotate-partial`, 27, 28, 32, 33, and 34 to 39. The test reports each other case as not run, with its reason. 13 to 16, `18-find-second`, 19, 26, and 31 use verbs the door does not carry. `18-annotate-two-groups` reads two JSON pointers per record, pointer extraction stays command-only under 0084, and the door reads one text column. 20 to 25 need a fault injection, and 0095 gives a binding no public fault hook. 29 and 30 pass a question as JSON text or a file, and the door takes only built questions. 40 reads counters, and the door adds none. 41 to 52 are recognize and relate cases. Plant: map a not-sure answer to `false` in `decide_series`. The band case turns red.
- **Failed marker.** On the case arm, shared case `17-annotate-partial` fails one question with cause `missing_probability` beside good answers. `annotate_frame` widens that question's column to `String`, and the failed cell equals the literal `{"failed":{"kind":"backend","cause":"missing_probability"}}`, pinned in the test. The good cells equal their pinned literal texts. `score_series` over every text of the malformed arm `/arm/malformed/missing_probability` ends with `Error::Engine` of kind `Backend`, and the test pins its message (amended 2026-09-25, decision 4). Plants: write `null` for a failed cell, and parse the member and write it back through a sorted map. The first gives a null, and the second gives `cause` before `kind`.
- **A failed row in a decide column.** `decide_series` over every text of `/arm/malformed/invalid_probability` returns `Error::Engine` of kind `Backend`, and the test pins its message. Plant: write a failed row as null and keep going. The call returns `Ok`, and the test turns red.
- **Refusals.** Each sentence in decision 5 is pinned whole, and each counts zero requests. Plants: drop the null check, and drop the kind check in `score_series` and pass it a decide question. Each call reaches the engine, and the count is nonzero.
- **No paid backend.** `check.sh` reads the real `HOME` first, to find the toolchain folder and cargo's crate cache, and it never changes `HOME`. It then runs `unset THINKTHEN_API_KEY`, sets `THINKTHEN_API_KEY` to a fake value, and sets `THINKTHEN_BASE_URL` to a closed loopback port. `XDG_CACHE_HOME` and `THINKTHEN_CACHE` point at scratch folders. The test helper `tests/common/mod.rs` refuses to build an engine unless the environment holds exactly that fake key and a loopback address. Every engine it builds names its own test backend with `base_url` and its own cache folder with `cache_at`. Plant: delete both the `unset` line and the fake-key line, and run `check.sh` with a sentinel key set. The helper sees the sentinel, and every engine test turns red before any engine is built.
- **No `unsafe`.** The crate's lint table equals the binding table of ADR 0047 item 3, and the crate has no FFI module. 0093's `lint` check already refuses `unsafe` outside an FFI module, so it refuses any `unsafe` here. If that check does not reach a binding with no FFI module, the builder extends it within the gate budget. This ticket adds no second scanner.
- **The README example builds.** The README's example builds an engine on `EngineBuilder::from_env()` with `base_url`, `model`, `throttle`, `max_requests`, `cache_at`, and `cache_bytes`, then calls `decide_series`. It is a `no_run` doctest, so it compiles and sends nothing. The text names `default_cache` and `no_cache` as the other two cache setters.

## The check it adds to the gate ladder

No new rung. `libraries/polars/check.sh` joins the surface registry (0093) as the tenth surface. The `surfaces` rung runs it. Its steps, in order: the probe `cargo fetch --locked --offline`, the environment of the paid-backend bullet, `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, and `cargo test`. The `fetch`, `clippy`, and `test` calls pass `--locked` and `--offline`. `cargo fmt` takes neither flag. Only a failed probe reports "not run". The root toolchain is 1.95 since the amendment below, so the probe covers the toolchain, and `check.sh` has no toolchain step (amended 2026-09-25). The probe's line names the one `cargo fetch` to run on a networked machine. Every later failure is a plain failure. `lint` runs the ADR 0047 checks for this binding: the manifest, the lock, the lint table, the profile copy, deny, and `libraries/polars/ratchet.json`.

## Dependencies and second review

- New to main: `polars` (0.55 line) and its tree, in the binding's lock only. `serde_json` with `raw_value` for lifting cells from `value_json`, at the root lock's version. `conformance-backend` as a path dev-dependency.
- The binding lock must resolve `thinkthen`'s own dependencies to the root lock's versions (ADR 0047 item 1). If Polars forces another version, the builder stops and records the case.
- (Re-scored 2026-09-25, see Review: deny now runs with `libraries/polars/deny.toml`.) deny runs with the root `deny.toml`. If the Polars tree carries a license outside the root list, the root file cannot take it, since its `unused-allowed-license = "deny"` would fail the root run. The builder then stops. The fix is a binding `deny.toml` that adds only the named licenses, with `policy.py` checking that difference. That fix needs a re-score.
- The code reviewer checks each new dependency, the feature set, the lock, deny's result, and both pinned sha256 values, and the review record says so (repo `CLAUDE.md`).

## Budgets

- Production Rust in `libraries/polars/src`: at most four files and 450 nonblank lines. The tag's door measures about 210 before the error type and the options.
- Rust tests: at most eight files and 900 nonblank lines, the helper included. Two files hold one throttle test each.
- Scripts: `check.sh` and `setup-toolchain.sh` together at most 140 nonblank lines. Gate changes under `sdlc/scripts` at most 20 nonblank lines.
- Documentation: at most 180 net nonblank lines across `libraries/polars/README.md`, the Polars section of `sdlc/planning/libraries/rust.md`, the ADR 0047 Polars item, and `polars-plan.md`.
- The binding lock: at most 230 packages. The tag's lock held 222 with Polars' `lazy` feature and the stand-in.
- Ratchet: `libraries/polars/ratchet.json` equals the measured total. The root `sdlc/ratchet.json` does not change.

Stop and re-score before crossing a budget, adding a dependency beyond this list, touching `crates/thinkthen` or `conformance/`, changing the root toolchain, or adding a verb beyond the five.

## Exclusions

A Polars feature or module in `thinkthen`. A lazy `Expr` door or a Polars plugin. `filter`, `rank`, `find`, `relate`, and `recognize` over a column. Any change to 0106 or the Python wheel. Publication and the registry name. Any live or paid call.

## Dependencies

After 0086 (the public API with `EngineBuilder::from_env` and `throttle`), 0098 (`Question::kind`, `choose_labels`, `tag_labels`, `QuestionSet::members`, `value_json`), and 0093 (ADR 0047, the registry, the `surfaces` rung, and the binding checks). 0117 has landed. Spike 257 (below) runs first.

## The spike before the build

Spike 255 (the workspace's `experiments/255-thinkthen-python-polars-spike/PLAN.md`) already names this ticket among the decisions it can change. Its risk 5 measures the toolchain floor of Polars 0.55 and asks whether Python Polars should build on `pyo3-polars`. It has no report yet. Its answer settles decision 7's toolchain. If 255 moves 0106 onto `pyo3-polars`, the two doors then share one `polars` version and one toolchain pin, and this ticket's pins follow 0106's.

Spike 257, in its own folder under the workspace's `experiments/`, answers only what 255 does not. It builds a scratch crate on `thinkthen` at main with `polars` 0.55 and default features off.

1. Which features do the five methods need, and how many packages does the lock then hold? Does that set turn on serde_json's `preserve_order`? If it does, the sorted-map plant of the failed-marker test needs another form.
2. Does deny pass with the root `deny.toml`?
3. Does the lock resolve `thinkthen`'s dependencies to the root lock's versions?
4. How long does a cold `check.sh` build take on Linux and on the Mac?

The two spikes can change decisions 1, 7, and 8 and the lock budget. The ticket records the result before the build starts.

## Spike finding, 2026-09-24 (spike 257)

Spike 257 ran on beelink in the workspace's `experiments/257-thinkthen-rust-polars-spike/`, with its plan, scripts, and raw logs there. The tool refused a `REPORT.md` file in that folder, so this note is the report. A scratch crate on `thinkthen` at main (`ffcd7c91`, path, default features off) made every Polars call the five methods need, with a fake answer in place of the engine. It did not call the engine, because 0086 has not landed. No paid backend ran: a fake key and a closed loopback port only. Spike 255 had no Polars result when this ran.

**This finding changes the accepted design, and the ticket needs a re-score before the build.** It changes decisions 7 and 8, adds a dev-dependency, and needs a binding `deny.toml`.

1. **Toolchain.** No Polars 0.55 release builds on Rust 1.93.1. `polars-compute` 0.55.0 and 0.55.2 use `array_windows`, which 1.93.1 calls unstable. The `dtype-categorical` and `dtype-struct` features also pull `sysinfo` 0.39, which needs 1.95. Polars 0.54.4 is the newest release that builds on 1.93.1, and its door calls and tests pass there. A scratch copy of main with the root pin at 1.95 passed deny, fmt, clippy, doc, test, and doctest, so nothing in the spine breaks on 1.95. That run skipped `sdlc/live-test` and the Python and shell self-tests. The recommendation: `polars` 0.54.4 on the repo pin 1.93.1. Decision 7 then drops the 1.95 toolchain, `setup-toolchain.sh`, `toolchain.sha256`, and the missing-toolchain "not run" state. The R2-28 row keeps the failed-probe "not run" and loses the empty-`HOME` plant. The cost: the door supports Polars 0.54, one minor behind current, and a Rust caller on 0.55 cannot pass its `Series` in. When a later Polars minor is needed, the measured root bump to 1.95 is the lever. If spike 255 moves 0106 onto `pyo3-polars` 0.28, which needs `polars` 0.55.1 or later, this ticket follows 0106 to 0.55 and 1.95 under the rule above. Ian can overturn this choice.
2. **Features and lock.** The five methods need no Polars feature. Decision 8 becomes `polars = { version = "=0.54.4", default-features = false }`, and the README names Polars 0.54. Only the R1-4 test needs `Categorical`, `Enum`, and struct columns. Turning those features on in `polars` pulls `polars-io` and `polars-ops`, and the lock grows to 346 packages. A dev-dependency on `polars-core` `=0.54.4` with `dtype-categorical` and `dtype-struct` gives the tests those types at 169 packages, inside the 230 budget. That dev-dependency is beyond the ticket's list. serde_json's `preserve_order` stays off, so the sorted-map plant keeps its form.
3. **Deny.** Advisories and bans pass with the root `deny.toml` (advisory database of 2026-09-23). Licenses fail on four crates, for 0.54.4 and 0.55.2 alike: `foldhash` 0.2.0 (Zlib), `slotmap` 1.1.1 (Zlib), `xxhash-rust` 0.8.18 (BSL-1.0), and `ar_archive_writer` 0.5.3 (Apache-2.0 WITH LLVM-exception alone, a build dependency of `psm` under `stacker`). This is the stop the ticket names. The narrowest fix is a binding `deny.toml` with four per-crate `exceptions` entries and the root `allow` list unchanged.
4. **Shared versions.** With the binding lock seeded from the root `Cargo.lock`, 73 of the 76 shared crate names hold the root's versions, and every `thinkthen` dependency keeps its root version. The three others are additions: `getrandom` 0.4.3 and `r-efi` 6.0.0 through `uuid`, and `signal-hook` 0.4.4 through `polars-error`. A fresh resolve drifts `thiserror` to 2.0.21 and `zerocopy` to 0.8.58, so the builder seeds the lock from the root lock. New duplicates: `getrandom` (0.2.17, 0.3.4, 0.4.3) and `r-efi` (5.3.0, 6.0.0). The `hashbrown`, `syn`, and `windows-sys` duplicates already exist in the root lock.
5. **Build cost.** Cold `cargo fmt --check`, `cargo clippy --all-targets -D warnings`, and `cargo test` at `-j 4`, sccache off, under the heavy-build lock: 63 s and an 885 MB target folder for 0.54.4 on 1.93.1. 0.55.2 on 1.95 took 59 s and 886 MB. The Mac was not measured. The lever is the spike's `scripts/cold-build.sh` on the M5.

## Amended 2026-09-24: current Polars on a repo-wide Rust 1.95

The owner decided this after spike 257 (the finding above). Ian can overturn it. Where this section conflicts with the text above, this section wins, and the text above stays as history.

The decision: 0120 targets the current Polars release, 0.55.x. It does not take the spike's 0.54.4 recommendation. A Rust Polars crate forces its users onto its own Polars version, so a pin one minor behind would lock them out of current Polars. The spike moved only the `channel` in a scratch copy of main's `rust-toolchain.toml` to 1.95, and deny, fmt, clippy, doc, test, and doctest all passed (`logs/07`). It left `rust-version` at 1.93.1.

1. **Decision 7: the repo moves to Rust 1.95.** One Quick Fix moves the root `rust-toolchain.toml` and the workspace `rust-version` to 1.95. It lands right after 0086 and before any surface build. The Quick Fix is the first run with `rust-version` 1.95. Clippy reads `rust-version` as the minimum Rust version, and the resolver picks versions by it, so the Quick Fix runs clippy and a lock check under the new value. It also runs the steps the spike skipped: `sdlc/live-test` and the Python and shell self-tests. It updates `sdlc/planning/libraries/rust.md`, whose line 35 says the repo pins 1.93.1. This crate then builds on the repo's own toolchain. `setup-toolchain.sh`, `toolchain.sha256`, and the per-crate toolchain folder leave the design. `check.sh` has no toolchain step, and its only "not run" state is the failed probe. The crate's own `rust-version` names 1.95, equal to the root workspace value. The binding is its own Cargo workspace, so it cannot inherit the root value.
2. **Decision 8: Polars 0.55.x, pinned exactly.** The manifest reads `polars = { version = "=0.55.N", default-features = false }`. N is the newest 0.55 release the build verifies. The spike measured 0.55.2. The five methods need no Polars feature. The README names Polars 0.55. The builder seeds the binding lock from the root `Cargo.lock`, because a fresh resolve drifts `thiserror` and `zerocopy` off the root versions (spike finding 4). The builder confirms that serde_json's `preserve_order` stays off at 0.55, so the sorted-map plant keeps its form.
3. **A `polars-core` dev-dependency.** Only the R1-4 test needs `Categorical`, `Enum`, and struct columns. Those features on `polars` pull `polars-io` and `polars-ops`, and the spike measured 329 packages at 0.55.2 (`logs/04-lock-counts.log`). So the tests take `polars-core` at the same exact version as `polars`, with `dtype-categorical` and `dtype-struct`, as a dev-dependency. The spike measured 163 packages with that shape at 0.55.2. The `policy.py` manifest check for this binding requires the `polars` and `polars-core` pins to be equal. Its plant sets unequal pins and fails.
4. **A binding `deny.toml`.** `libraries/polars/deny.toml` equals the root file with the `allow` list unchanged and four per-crate `exceptions`. Each entry carries its reason as a comment:
   - `foldhash` (Zlib): the default hasher of `hashbrown` 0.17. `polars-arrow` and `polars-compute` use that `hashbrown`.
   - `slotmap` (Zlib): reached through `polars-async` under `polars-core`.
   - `xxhash-rust` (BSL-1.0): a direct dependency of `polars-core`.
   - `ar_archive_writer` (Apache-2.0 WITH LLVM-exception, with no plain Apache-2.0 offered): a build dependency of `psm` under `stacker` under `polars-utils`. It runs at build time only.
   The chains come from the spike's deny logs at 0.55.2 (`logs/08-deny-try-03-polars-0.55.2-1.95-facade.log`) and 0.54.4 (`logs/08-deny-try-05-polars-0.54.4-1.93.1-devcore.log`), and the builder confirms them at 0.55.N. The root `deny.toml` admits only licenses that are permissive and carry no copyleft term, and it asks for a record behind any widening. Zlib, BSL-1.0, and Apache-2.0 WITH LLVM-exception each meet that test: each lets anyone use, change, and ship the code with a notice kept, and none requires shared source. The binding `deny.toml` states that sentence in its header comment, and the build record repeats it. Each exception names its crate and license, so a new crate under the same license still fails. The binding's deny call uses this file. `policy.py` parses both files and checks that their settings differ only by these four entries. Comments do not count, so the binding file keeps its own header and reasons. One planted fifth entry fails the check. The root `deny.toml` does not change. The earlier stop clause on a license outside the root list is answered by this item.
5. **Budgets, re-scored.**
   - Scripts: `check.sh` at most 80 nonblank lines. The setup script leaves.
   - Gate changes under `sdlc/scripts`: at most 30 nonblank lines (re-scored to 70 on 2026-09-25, see Review). `policy.py` holds the deny-file check, the equal-pin check, and their plants. The registry gains one entry.
   - The binding lock: at most 200 packages, measured at 0.55.N with the `polars-core` dev-dependency. The spike measured 163 at 0.55.2. The record gives the count.
   - The binding `deny.toml`: at most 90 nonblank lines. The root file measures 59, and the four entries with their reasons and the header sentence add the rest.
   - Production Rust, Rust tests, and documentation keep their budgets. The documentation budget now covers the four exception reasons in the README.
   - Cold build: the spike measured 59 s and an 886 MB target folder for 0.55.2 on 1.95 on Linux. The record gives the Mac time from the spike's `scripts/cold-build.sh` on the M5.

Changes that follow from the five items:

- The R2-28 row: the empty-`HOME` plant leaves with the toolchain folder. It keeps two plants. The first runs the probe with `CARGO_HOME` pointing at an empty scratch folder. `check.sh` then prints "not run", and the rung counts no pass. The second adds a failing test. `check.sh` then fails and does not print "not run".
- The paid-backend bullet: `check.sh` still leaves `HOME` alone. It no longer looks for a toolchain folder.
- Dependencies: after 0086, then the 1.95 Quick Fix, then 0098 and 0093. Spike 257 has run. New to the binding lock beyond the list above, from the seeded 0.55.2 lock (`logs/13-compare-seeded-055.log`): `polars-core` as a dev-dependency; `getrandom` 0.4, `r-efi` 6, and `signal-hook` 0.4 through Polars; `rand` 0.10 and `rand_core` 0.10 in place of 0.9; and `chacha20` 0.10. `getrandom` 0.3 drops out. The lock also holds two `getrandom` versions (0.2 and 0.4), two `syn` versions (2 and 3), and two `windows-sys` versions. None of these crates is in `thinkthen`'s own normal dependency tree. In the root lock `rand` arrives only through `proptest`, and `signal-hook` sits behind the `cli` feature. ADR 0047 item 1's version rule therefore still holds.
- If spike 255 moves 0106 onto `pyo3-polars`, both doors share one Polars 0.55 version and the repo toolchain, and the pins move together.
- Four sentences above describe the dropped toolchain and no longer hold. The code reviewer checks no sha256 values. The case against a `polars` feature on `thinkthen` loses its toolchain reason and keeps its others. Building the wheel on the `polars` crate would add no second toolchain. Spike 255 no longer settles decision 7's toolchain.

## Routing

Builder: Claude (Opus subagent). Reviewer: a fresh Claude session for design and for code.

## Complexity

Contract 2; state and timing 2; reach 2; proof 3; cost of error 2; total 11. Final level: 3. The door holds no `unsafe` code, and every guarantee comes from the engine. The risk sits in the dependency tree and the toolchain.

## Review

- Design review: `sdlc/records/2026-09-24-design-review-0120.md`. Five rounds of fresh read-only reviewers found 22 items, and every one is answered. The sixth round accepted it. The amendment after spike 257 took three more rounds, recorded in the same file, and was accepted at `3318a7ba`.
- Build record: `sdlc/records/0120-build-rust-polars.md`.
- Re-score 2026-09-25: the gate-change budget under `sdlc/scripts` rises from 30 to 70 nonblank lines. The code review asked for the admitted wasm-only lock packages, the binding `deny.toml` in `surfaces --registry`, and the rule that a binding holds a `deny.toml` only where `policy.py` checks it. The coordinator approved the re-score on 2026-09-25, and Ian can overturn it.
- Code review: pending.

## Spike finding, 2026-09-24 (spike 255)

Spike 255 answered its risk 5 in the workspace's `experiments/255-thinkthen-python-polars-spike/`. The full note is in ticket 0106 on its branch. What matters here:

1. **0106 stays off the `polars` crate.** Python Polars reads the same zero-copy buffer through the public `__arrow_c_stream__` capsule as through pyo3-polars. The spike keeps 0106's Arrow stream door. "The two doors share no Polars crate" holds.
2. **Decision 7 holds.** pyo3-polars 0.28.0 with polars 0.55.2 stops on Rust 1.93.1, because `sysinfo` 0.39 needs 1.95. It builds on 1.95. This matches spike 257. pyo3 0.29.2 builds and runs on 1.95 too, so the repo-wide move costs the Python surface nothing.
3. **A working set, if the doors ever share a crate.** pyo3 `=0.29.2` with `abi3-py310`, pyo3-polars `=0.28.0`, polars `=0.55.2`, and Rust 1.95 read Python Polars 1.30.0, 1.40.0, and 1.44.2 zero copy. With `thinkthen` and pyo3-polars' `derive` feature, the lock held 262 packages, and a cold build took 238 s at `-j 4` with sccache off.
4. **The exclusion of a plugin holds on measured grounds.** A plugin expression runs, but Ctrl-C during `collect()` surfaced only when the plugin call returned, 2.5 s after the signal.
