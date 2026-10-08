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
- The working-tree `git diff --check` passed. The reviewer checked the revision range and found generated trailing spaces preserved in the exact question transcript; that range is not whitespace-clean. This changes no question or executable behavior.

Builds used a bounded user scope with two Cargo jobs, an 8 GB memory cap and a two-core CPU quota. Warm lane builds remained below the 40 GB lane cap.

## What the build taught us

A Rust observer closure needs its `RecordObservation<'_>` argument annotated to satisfy the callback's higher-ranked lifetime. `to_owned()` retains the actual logical question after the callback ends. Debug formatting intentionally withholds question text, so callers must use `question().text()` and declared option readers to see it.

Recognition's `--plan` describes boundary bodies and later request bounds. It cannot display kind and edge questions before boundary answers select spans. The saved recording binds answers to its historical URL; the gate binds that exact loopback endpoint to count zero sends and fails if another process occupies it. These controlled replies establish replay and observation behavior, not model accuracy.

## Limits

Fresh read-only review accepted `eeee618da3d0b0fd6d8a53b6510af24267535ce3`, with the transcript whitespace qualification above.

No product API, source scheduler, Request carrier, header, CLI trace export, trace store, site proof tooling, paid call or release action changed. This recipe covers the native Rust observer. CLI details and serialized events retain their existing limits. Broader repository and release gates belong to the coordinator.

## Flagged-span recipe extension

The extension from PM ask 12 composes the existing `choose` function with caller-supplied bounded candidate spans. `specification/recognize.md` documents the current span as `keep`, Unicode coordinate validation, not sure handling and caller ownership of applying a result. The generic caller-defined fixture adds a saved question and one authored `fixture` answer. Its caller script validates two candidates against the unchanged original text, invokes strict CLI replay without a key and maps the selected label back to candidate metadata. The script prints a proposal and applies no edit. The existing executable recognition page checks that proposal alongside the original recognition result.

The extension checks passed separately:

- `mustmatch test spec/recognize.md` passed all six blocks with repository scratch configuration and usage helpers. The extended fixture block pins the original recognition result and the proposed [8,13) amount span.
- A focused replay check bound the saved loopback endpoint, ran the caller script without a key and counted zero connections. It checked the exact selected label and proposed span. A separate invalid-coordinate check moved a candidate start by one scalar and confirmed refusal before the command was invoked.
- `cargo test --manifest-path libraries/rust/Cargo.toml --locked --offline` passed the existing example gate, including the unchanged native observer transcript and its zero-send assertion, against the extended recording.
- `python3 sdlc/scripts/pages` and its `--self-test` passed. `CARGO_NET_OFFLINE=true python3 sdlc/scripts/policy.py` passed with size warnings on unchanged source files. `git diff --check` passed.

The extension taught us that the ordinary choice question file accepts objects as option descriptions, so each candidate can carry its text, kind and original coordinates without an extra API. The `é` prefix demonstrates why Unicode scalar positions must remain distinct from UTF-8 byte positions. A selected label still requires caller validation and acceptance; a saved controlled answer establishes software behavior, not revision accuracy. No product code, API, revise mode, PM state, paid call or release action changed.
