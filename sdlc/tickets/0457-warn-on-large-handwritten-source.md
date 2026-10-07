# 0457: Warn on large hand-written source and retain a hard limit

Status: in progress. Policy thresholds and standing instructions are implemented. The C header is 985 nonblank lines after moving extended schema prose to its existing online reference without changing declarations. Swift native views remain 1110 nonblank lines and are the sole policy hard failure; their repair waits for the named core-freeze checkpoint.

Milestone: 0.2

Owner: lane 0 builder on `ticket/0457-source-size-policy`.

## Outcome

The standing instructions and existing source policy warn at 500 nonblank lines and fail at 1,000. The same rule covers Rust and hand-written source in every binding language. A warning requires an explanation in the change's commit; it does not force a mechanical split.

## Evidence

- Starts from: Ian's approved ask 1 in shared mailroom message `2026-10-07-pm-file-size-rule-file-cleanup-after-core-freeze-binding-layout-check-and-cache.md`. The existing 500-line hard stop encourages files cut just below the cap.
- Keeps: The measured total in `sdlc/ratchet.json`, existing behavior and risk checks, and the existing policy runner. Generated source, vendored dependencies and build output retain their existing exclusions.
- Changes: Own `sdlc/scripts/policy.py`, `AGENTS.md`, `sdlc/planning/rust-standards.md`, `libraries/c/include/thinkthen.h`, `libraries/c/DESIGN.md` and the deferred `libraries/swift/Sources/ThinkThen/NativeViews.swift` repair. Replace the conflicting standing rule in AGENTS and Rust standards. Extend the existing policy's source inventory to hand-written binding languages. Define 499 as below warning, 500 through 999 as warning, and 1,000 or more as failure. Explain warnings in commits without adding a prose checker or a second count system.
- Proof: Extend the existing policy runner with threshold, binding-language inclusion and generated/vendor/build exclusion cases. Run policy and its existing self-tests. Actual large files warn without blocking; a planted 1,000-line file fails. No per-language proof records. The focused threshold/inventory/exclusion cases, formatting, C Clippy and three version/shared/static export tests passed after the C documentation move. Header content outside comments and compiler-preprocessed C output match the baseline; strict C++ syntax passed. Existing measured ratchets remain 161603 (workspace Rust), 15083 (C Rust) and 2644 (C .c). Full policy truthfully fails only the held Swift repair; whole-change review and final gates follow both repairs.
- Defers: Source reshuffling belongs to 0458 after core freeze. This ticket changes no public behavior, C layout, cache format or release authority.

## Dependencies and ownership

Apply the standing rule before the bounded file cleanup. Keep source totals measured. The implementation uses the existing policy machinery and one whole-change review.

Reviews: accept
