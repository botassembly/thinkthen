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
