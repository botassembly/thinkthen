# Read recognition questions through the native observer

Ticket: 0477. Starting revision: `b8e5c8a60`.

The recognition specification explains current boundary, kind and edge question shapes, identifies their wording as unstable, and names tagged examples as the stable teaching input. Its runnable plan command displays first-record step-1 bodies. Later stages depend on accepted answers, so the plan supplies bounds rather than their actual questions.

The Rust observer recipe uses the existing native complete-record call and `CallOptions::observe`. It copies borrowed details inside the callback, groups them by original record index, stage and position, and prints actual question text, ordered options/descriptions, answers, failures, probabilities, request references and source metadata. Two occurrences of the saved generic receipt fixture demonstrate retained originals and distinct indices. The specification states completion, failure, stage, privacy, cache/replay and serialized-event limits.

## Checks

- `cargo test --manifest-path libraries/rust/Cargo.toml --locked --offline` passed through the existing example gate. The gate built every example, compared full pinned output, removed the observer recipe's key and counted zero connections at its saved loopback endpoint. Record 0 and record 1 each map boundary position 2 to the marked piece `42` and answer `BEGIN`; both complete and retain their original text.
- `cargo clippy --manifest-path libraries/rust/Cargo.toml --locked --offline --all-targets -- -D warnings` passed.
- `cargo fmt --manifest-path libraries/rust/Cargo.toml` produced formatted examples; the formatter check passed afterward.
- `CARGO_NET_OFFLINE=true python3 sdlc/scripts/policy.py` passed. Existing size warnings concern unchanged files.
- `node sdlc/scripts/ratchet.mjs libraries/rust/ratchet.json` passed at 408 nonblank Rust lines. The additional recipe and zero-send branch earn the increase from 284. The existing example gate supplies the output check; no new runner or trace framework was added.
- `mustmatch test spec/recognize.md` passed all six examples with configuration and usage isolated through the repository scratch helpers. The documented plan command also returned four step-1 questions.
- The external private-name list contained 35 patterns. The changed paths and content contained zero matches.
- `git diff --check` passed.

Builds used a bounded user scope with two Cargo jobs, an 8 GB memory cap and a two-core CPU quota. Warm lane builds remained below the 40 GB lane cap.

## What the build taught us

A Rust observer closure needs its `RecordObservation<'_>` argument annotated to satisfy the callback's higher-ranked lifetime. `to_owned()` retains the actual logical question after the callback ends. Debug formatting intentionally withholds question text, so callers must use `question().text()` and declared option readers to see it.

Recognition's `--plan` describes boundary bodies and later request bounds. It cannot display kind and edge questions before boundary answers select spans. The saved recording binds answers to its historical URL; the gate binds that exact loopback endpoint to count zero sends and fails if another process occupies it. These controlled replies establish replay and observation behavior, not model accuracy.

## Limits

No product API, source scheduler, Request carrier, header, CLI trace export, trace store, site proof tooling, paid call or release action changed. This recipe covers the native Rust observer. CLI details and serialized events retain their existing limits. Broader repository and release gates belong to the coordinator.
