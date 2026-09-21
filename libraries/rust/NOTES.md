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

Every `score` hit is the ruled verb, never a probability's name. The number on an entity is documented as the interim `number` field with the open comparison named; the number on a relation is `probability`.

### Unchecked here

- The wire for the two functions: they answer from the recordings, so `check.sh`'s wire suite stays the other verbs' suite (stub up on 8213).
- The backend and defect kinds through `recognize`/`relate`: the stand-in's replay answers usage, deadline, and cancelled offline; backend needs the wire and defect is not fabricable.
- R03's per-subject case (finding 2).

## 2026-09-21 — the name number is settled: `strength`

Ian settled the open item in `sdlc/issues/2026-09-21-one-rule-for-every-number-the-tool-prints.md` ("The comparison came back"): the number on a recognized name is the field `strength` — ours, computed, defined once in the manual, with its parts under details. The relation number stays `probability`; the vendor's `confidence` passes through under details only. The contract, stand-in, conformance file, and validator already carried the rename (commits 78204cb, 14f418e); this surface followed: the `recognize` doc comment in `src/lib.rs` now states the settled rule instead of the open item, and the conformance example's entity comparison in `examples/conformance.rs` reads `strength`. Rerun: `./check.sh` with the stub on 8213 — conformance slice green for the Rust surface, wire suite 3 passed.
