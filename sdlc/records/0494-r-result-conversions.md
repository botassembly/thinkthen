# R owned native calls

R now exposes one family of ten named functions through generated request sessions and typed results. The old judging endpoints, complete-call grammar, copied readers, workers and unused shared-host dependency are deleted. An ordinary engine owns settings and execution instead of the hidden default engine. The README maps the old API to the replacement.

Fresh read-only review accepted `d46e894f3546713a6daf22f6f6899cefc99477b7`. Installed checks cover all ten functions, presence versus null, typed failures and facts, files, images, original positions, cache and replay, cancellation, answer identities and safe printing. The 31 routine shared cases, five backend cases, feed cancellation, wrapper identities and ten saved-response producer cases pass. Full parity and platform execution remain candidate work.

The review and installed checks exposed a completion-position bug: R observations used native request positions instead of the caller's original row positions. The repair changes observation mapping without changing native result indexes. Existing fixture assertions now compare numeric values without imposing R integer storage, preserve native duplicate-identity refusal and count concurrent requests without assuming arrival order. Exact request bodies and output order remain checked.

Strict R Clippy, generator freshness, formatting, policy, source ratchets and installed symbol isolation pass. Evidence lives in lane 1 `target/0494-close/`, including `final-install.log`, `final-clippy.log`, `final-policy.log`, `installed-symbols.log` and the individual repaired installed checks. The initial `installed.log` contains the completion-position failure; it is not evidence of a final pass. The fresh reviewer also ran a small installed invalid-source and removed-export probe.

The reviewed source archive has SHA-256 `51fc00df41d0e9da35f3b09e2e094b554e7e562183c80a131a93f6c3ccadef3c`. The binary archive has SHA-256 `963ac994399a118279d43dc853d20ee91b50edaab0ab74664314bfb8d8142f01`; its native library matches the installed file at `345a366f53936aea457bd2784722edd8013c48d334ca0f4ccc93fbe608be0763`. Source packaging rewrites vendored Cargo paths and removes trailing blank lines. It no longer copies the unused host package.

Source, fixtures and packaging add 261 lines and delete 2940, a net reduction of 2679. The binding Rust ceiling falls from 4557 to 3485 and the R ceiling from 3193 to 1942, a total reduction of 2323. Core routine evidence is reused while the core remains unchanged; no paid, load, large-input or candidate campaign ran for this closure.

## What the build taught us

Installed package tests catch both packaging drift and host representation errors. Native result indexes and a caller's original row positions serve different purposes; map observations at the host boundary while retaining native indexes. Concurrent providers may arrive in any order, so assert request content and final output order separately.

Retiring a compatibility API must also retire its dependency and package-copy path. Keep generated ownership, error and presence behavior; remove the second grammar instead of maintaining it beside the replacement.
