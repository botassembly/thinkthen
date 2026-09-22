# NOTES, the Rust surface lane

Commands and output as they happened. All checks run against the null
backend offline or the loopback stub on port 8213 at 300 ms.

## 2026-09-21 — the lane lands

The crate `thinkthen` in `libraries/rust/`: an `Engine` value from the
environment or settled settings, every verb in the ruled shape, the text
form of a first argument through the one file grammar, the six error kinds
as `Result<_, Error>` (Rust's own error class), and the stand-in behind
one dependency line. Null suite, conformance slice, and wire suite all
green through `./check.sh` and the root `scripts/check_surfaces.sh`.

```
$ ENGINE_NULL=1 cargo test --quiet
slide: 1 passed; verbs: 13 passed; wire: 3 passed (skipped under null)

$ ENGINE_NULL=1 cargo run --quiet --example conformance
16 ok, 3 skip (07/20 backend needs the wire; 17 needs the disk cache),
1 diverge (18, the pre-fired token, on record in DIVERGENCES.md)

$ ./check.sh   (stub on 8213, delay 300 ms)
null suite green; conformance slice green; wire suite 3 passed
$ ../..//scripts/check_surfaces.sh   (stub up)
"not landed: libraries/rust" is gone; every landed check green
```

## Findings, filed rather than worked around

1. **The slide's own chain does not compile.** As drawn, the Rust slide
   calls `Question::decide("...")` and chains `.band(0.2, 0.8)` on the
   `Result` the constructor returns, so the sample fails to build. The
   fix on the slide is one character, a `?` after the closing paren; the
   alternative is a contract change, `Question::decide` returning the
   builder before validation so the chain reads as drawn. The slide
   should change or the contract should, and this is the first finding
   for the slide owner. The test runs the sample with the one-character
   fix and a comment marking it.
2. **`DecideBuilder` cannot finish with the grammar's default cut.** The
   builder finishes only through `.cut` or `.band`, so a text-only first
   argument has nowhere to land. This surface builds the text form by
   serializing `{"decide": text}` through `Question::from_json`, the one
   grammar, which keeps the digest honest but spends a `serde_json`
   dependency to do it. The contract could offer the constructor and
   drop the workaround from every binding.
3. **`Settings.address` is carried and unspent.** An engine built with
   `from_settings` keeps using the environment's address: the stand-in
   reads the address once per process and never reads the setting. A
   dead-address test through a settings-built engine therefore cannot
   run, and the wire suite proves the not-retryable classification with
   the stub's own 422 refusal instead. The real engine must honor the
   setting, because a database extension will point its own engine value
   at its own address.

## Stand-in quirks the tests now encode

- The null `choose` rule weights an option by its own name, so a
  refund-named option wins; the test says so in a comment.
- The null `score` rule answers exactly three levels; a two-level score
  question fails the core's distribution check. The verb test uses three
  levels and this note records why.
- The null `tag` rule weights a label by its own name (0.72 refund,
  0.55 maybe, 0.03 else), matching the conformance numbers.

## What is unchecked here

- No packaging run yet: `cargo add thinkthen` on the slide is the future
  registry shape; this crate carries `publish = false` at 0.0.1 until
  the packaging rehearsal.
- The conformance runner checks the fields the file's `expect` carries
  for each verb; a `rank` and `find` case do not exist in the file yet,
  so those arms ran only in the verb tests.

## 2026-09-21 — recognize and relate

The two new functions on this surface, from the brief in `sdlc/issues/2026-09-21-update-for-the-library-team-recognize-and-relate.md`. What landed, in `src/lib.rs`: `Engine::recognize` and `Engine::relate` plus their `_opts` forms (cancel and deadline ride both), the re-exported contract types (`Recognize`, `Relate`, `Kind`, `RelationRule`, `Entity`, `Relation`, `Recognized`, `Edge`, `MAX_RELATE_RECORDS`), and two free functions that are this surface's one conversion: `byte_range(entity, text)` and `name_in(entity, text)`. The contract counts offsets in code points; Rust slices bytes; the emoji case is the strict proof:

```
$ ENGINE_NULL=1 cargo test --quiet
recognize: 7 passed   (the emoji case among them: byte_range == 14..24, name_in == "Maria Chen",
                       byte 10 is inside the 😀 and 14 starts the name)
```

The conformance runner grew `recognize` and `relate` arms (`examples/conformance.rs`): all 41 recognize cases and 3 of the 4 relate cases answer `ok`, every entity's name slices out of the text in byte indexing in the runner itself, and `./check.sh` exits 0 end to end.

### Findings, filed not worked around

1. **The slide's `located_in` is not in the recording.** The Rust section of `recognize-surfaces.md` asks `works_for` and `located_in` over the deck sentence; the recording behind that sentence covers `works_for` and `based_in`. The call as drawn answers the usage kind naming the coverage (test `the_slide_ask_meets_the_recordings_gap`); the deck call stays the real-engine acceptance, and the recognize package needs a `located_in` recording on the deck sentence for the stand-in to prove it.
2. **Two recordings answer one input.** `71-relate-R03-persubject-10` and `72-relate-R04-pairs-10` carry the same records and the same question, because the package recorded both the per-subject form and the ruled pairs form over one block. The stand-in prefers the pairs form, which `relate-design.md` rules; R03's five-edge expectation is the non-ruled form's answers and cannot be met by the ruled shape. The runner records it as a divergence (green check, honest line); the conformance owner decides whether R03 stays as a case, moves to the package record, or gains a distinguishing marker.
3. **Contract ergonomics: the slide's constructor does not exist as written.** The acceptance writes `Recognize::kinds([...])` as an associated function; the contract ships `Recognize::new().kinds([...])` (a builder method), so the slide's exact line does not compile and the tests use the builder. One token fixes it on either side; the finding is filed rather than worked around in this crate.
4. **`relate` per the recordings carries no kinds.** A rule that names a kind, or a `kind_field`, answers the usage kind from the stand-in, because the recordings hold kind `*` only. Real-engine data.

### Vocabulary sweep, run at close

Against `repos/mktg/products/thinkthen/vocabulary.md`, "The words for numbers":

```
$ grep -rniE "certainty|likelihood|cutoff|gray zone" src/ tests/ examples/ README.md NOTES.md check.sh
(no hits)

$ grep -rn "confidence" src/ tests/ examples/ README.md NOTES.md check.sh
(no hits; the doc comment on `recognize` names the restriction without the word)

$ grep -rniE "accuracy|calibrated" src/ tests/ examples/ README.md NOTES.md check.sh
(no hits)

$ grep -rn "score" src/ tests/ examples/
src/lib.rs: the verb's own name and the re-exported types; tests/verbs.rs and examples/conformance.rs use Question::score and the "score" verb arm.
```

Every `score` hit is the ruled verb, never a probability's name. The number on an entity was the interim `number` field, renamed to `strength` on 2026-09-21 (commits 78204cb, 14f418e, 2356976); the number on a relation is `probability`.

### Unchecked here

- The wire for the two functions: they answer from the recordings, so `check.sh`'s wire suite stays the other verbs' suite (stub up on 8213).
- The backend and defect kinds through `recognize`/`relate`: the stand-in's replay answers usage, deadline, and cancelled offline; backend needs the wire and defect is not fabricable.
- R03's per-subject case (finding 2).

## 2026-09-21 — the name number is settled: `strength`

Ian settled the open item in `sdlc/issues/2026-09-21-one-rule-for-every-number-the-tool-prints.md` ("The comparison came back"): the number on a recognized name is the field `strength` — ours, computed, defined once in the manual, with its parts under details. The relation number stays `probability`; the vendor's `confidence` passes through under details only. The contract, stand-in, conformance file, and validator already carried the rename (commits 78204cb, 14f418e); this surface followed: the `recognize` doc comment in `src/lib.rs` now states the settled rule instead of the open item, and the conformance example's entity comparison in `examples/conformance.rs` reads `strength`. Rerun: `./check.sh` with the stub on 8213 — conformance slice green for the Rust surface, wire suite 3 passed.

## 2026-09-21 — the rulings wave (languages lane)

Ruling 2: `tests/defect.rs` constructs a contract `Error` with kind `defect` and asserts the surface carries it whole — kind `Defect`, `retryable` false, the message, and the Display word `defect`. Part of `cargo test`: 1 passed.

Ruling 1 aftermath: `tests/recognize.rs`'s spec uses `source`/`target`.

Prose fix: the interim-`number` line above is history now; the settled field is `strength`.

## 2026-09-21 — the shapes from lane B item 2 (languages lane)

The three shapes `e44d492` landed in `contract/` and `standin/`, carried
into this surface, plus the fast-backend deadline proof and the examples
file. Commands and output as they happened.

**`Details.requests` and `failed_questions` (0053, 0054).** The typed
`Details` already carries both; the null suite now pins them:

```
$ ENGINE_NULL=1 cargo test --quiet --test verbs
test result: ok. 15 passed; 0 failed
```

`details_carries_the_audit_trail` asserts one 64-figure digest and
`failed_questions == 0`; `annotate_preserves_the_good_answers_and_marks_the_failed_one`
asserts the typed marker `Annotated::Failed(Failed { kind, cause })`
serializes to the ruled
`{"failed":{"kind":"backend","cause":"missing_answer"}}`, the neighbour
keeps its `Decision(Yes)`, and `failed_questions(&rows)` counts one.
`the_record_row_is_the_ruled_shape` pins `Row`/`rows_json`. The crate
re-exports `Cause`, `Failed`, `FailureKind`, `Row`, `rows_json`, and
`failed_questions` so a caller spells the shapes in Rust's own types.

**The record row (go-ahead item 4).** The conformance example builds
`Row<Option<bool>>` from the surface's own outputs and checks its
serialization where the cases carry rows (`05`, `06`, `19`).

**Conformance, offline: 73 and 74 run green.**

```
ok       73-details-carries-requests
ok       74-annotate-preserves-good-answers
conformance slice green for the Rust surface
```

Case 73 checks the audit's identity fields (the null backend's own rule
cannot reproduce its recorded probability); case 74 checks the typed
marker and its count.

**The fast-backend deadline.** Rust has no signal handler, so the stop
gesture is a spent budget: `tests/deadline_fast.rs` runs a
two-million-record null batch (about 5.6 s deaf) with a one-second
deadline and requires the deadline kind within 1.5 s:

```
$ ENGINE_NULL=1 cargo test --quiet --test deadline_fast
test result: ok. 1 passed   (1.28 s)
```

**The examples file.** `examples.json` holds all ten functions keyed by
name, each a call and the answer the null backend gives;
`tests/examples.rs` requires every snippet to appear verbatim in its own
source (Rust cannot evaluate a string) and runs the compiled call beside
it, comparing the file's expected answer:

```
$ ENGINE_NULL=1 cargo test --quiet --test examples
test result: ok. 1 passed
```

**Full check, `./check.sh`:** every suite green including the new two;
the wire suite skips with no stub on 8213; exit 0.

## 2026-09-22 — the settle wave on the Rust surface: the Series door

Ian approved the Polars decisions ("approve the polars decisions. i want all details settled across all languages"), and the plan's shape is the Series door first. It is in, behind a feature flag, with its tests and docs.

**The feature.** `polars = ["dep:polars"]`, off by default: the core surface keeps building with the pinned 1.93.1 toolchain and no Polars. With the feature on the crate needs the 1.95 toolchain Polars requires (experiment 228's recorded cost: `sysinfo 0.39` refuses 1.93). `check.sh` runs the feature step with `RUSTUP_TOOLCHAIN=1.95` and `--test-threads=1` (the request-count equality reads the process-wide usage counter, so the two measurements must not race a sibling test).

**The methods** (`src/polars.rs`): `decide_column` (the whole column through the batch spine at the process gate, a boolean column whose nulls are "not sure"), `choose_column`, `score_column`, `tag_column` (text/number/list columns; the stand-in's contract carries no bulk form for these three, so they run one call a record — recorded, not hidden; `decide` is the spine), and `annotate_frame` (the caller's columns unchanged, one new column a question in the set's name order; a failed member widens that column to text carrying the ruled marker, the same widening the other frame doors use). A null row refuses naming the row; a non-text column refuses naming its dtype.

**Zero copy and the equality expectation.** Each row is a `&str` read out of the producer's own UTF-8 buffer — no copy, no per-row object. Docs state the equality: a column crosses at the same width as a slice — one crossing, 32 in flight, answers in input order, same request count. The tests prove answers, order, and the request-count equality against the slice form; experiment 213 measured the width itself (9.658 s vs 9.665 s, both 32 in flight).

**Green by command.** `./check.sh` exit 0: the null suite, the fast-deadline test, the examples, the new feature step (`8 passed`), the conformance slice, and the wire suite skipped without the stub. `cargo check` and the full default suite stay green on 1.93.1 with the feature off.

**Noted, not hidden.** The two new files are rustfmt-clean; the folder's pre-existing fmt diffs (other files) wait for the merge ticket, per `MERGE-NOTE.md` item 4, which already assigns fmt/clippy over the new folders to the merge.

## 2026-09-22 — the review fix wave (lane C, the Rust surface)

**Item 3 resolves to nothing to change, with the grep to prove it.** The
Rust surface is a native Rust API: no `extern "C"`, no `no_mangle`, no FFI
boundary for a panic to cross, and no `from_secs_f64` of its own — it takes
the contract's `Options` and never builds a `Duration` from a float. The
phase-1 checked conversion lives in the contract
(`deadline_from_seconds` / `Options::with_deadline_seconds`), so a Rust
host that wants the checked door calls it there. The five `deadline`
mentions in `src/lib.rs` are doc lines.

```
$ grep -rn 'extern "C"\|no_mangle\|from_secs_f64\|catch_unwind' libraries/rust --include="*.rs" | grep -v target
(no matches)
```

**The cross-side repair: the fixture opt-in.** The stand-in's annotate
partial-failure fixture is armed only by `ENGINE_SYNTHETIC_PARTIAL=1`
(standin commit `7acb3da`, finding 7). `tests/verbs.rs`'s
`annotate_preserves_the_good_answers_and_marks_the_failed_one` and
conformance case 74 both replay that record, so the null suite and the
conformance example failed at HEAD without the opt-in. `check.sh` now
exports the opt-in on those two lines, with a comment naming the reason.

```
$ ./check.sh                                        # exit 0
null suite: 15 passed (verbs included), examples 1, deadline_fast 1,
polars door 8, conformance slice ok, wire twin skipped (no stub on 8213)
```

The plain `cargo test` outside `check.sh` still needs `ENGINE_NULL=1`
(and the fixture opt-in for verbs); that is the pre-existing review note
about env-dependent suites, not this lane's change.
