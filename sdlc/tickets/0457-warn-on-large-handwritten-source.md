# 0457: Warn on large hand-written source and retain a hard limit

Status: COMPLETE. The permanent source rule warns at 500 and fails at 1,000. Reviewed C documentation and Swift carrier moves preserve declarations and behavior; full tests, lint and specification checks passed.

Milestone: 0.2

Owner: lane 2 builder on `ticket/0457-source-size-policy`.

## Outcome

The standing instructions and existing source policy warn at 500 nonblank lines and fail at 1,000. The same rule covers Rust and hand-written source in every binding language. A warning requires an explanation in the change's commit; it does not force a mechanical split.

## Evidence

- Starts from: Ian's approved ask 1 in shared mailroom message `2026-10-07-pm-file-size-rule-file-cleanup-after-core-freeze-binding-layout-check-and-cache.md`. The existing 500-line hard stop encourages files cut just below the cap.
- Keeps: The measured total in `sdlc/ratchet.json`, existing behavior and risk checks, and the existing policy runner. Generated source, vendored dependencies and build output retain their existing exclusions.
- Changes: Own `sdlc/scripts/policy.py`, `AGENTS.md`, `sdlc/planning/rust-standards.md`, `libraries/c/include/thinkthen.h`, `libraries/c/DESIGN.md`, `libraries/swift/Sources/ThinkThen/NativeViews.swift`, `libraries/swift/Sources/ThinkThen/NativeQuestionViews.swift`, `libraries/swift/ratchet.swift.json`, `sdlc/scripts/release-pack` and `sdlc/scripts/release-go-cpp-pair`. Move copied question-definition carriers and their conversions without changing public declarations or result ownership. Update both actual release archive inventories. Replace the conflicting standing rule in AGENTS and Rust standards. Extend the existing policy's source inventory to hand-written binding languages. Define 499 as below warning, 500 through 999 as warning, and 1,000 or more as failure. Explain warnings in commits without adding a prose checker or a second count system.
- Proof: Extend the existing policy runner with threshold, binding-language inclusion and generated/vendor/build exclusion cases. Run policy and its existing self-tests. Actual large files warn without blocking; a planted 1,000-line file fails. No per-language proof records. The focused threshold/inventory/exclusion cases, formatting, C Clippy and three version/shared/static export tests passed after the C documentation move. Header content outside comments and compiler-preprocessed C output match the baseline; strict C++ syntax passed. Existing measured ratchets remain 161603 (workspace Rust), 15083 (C Rust) and 2644 (C .c). The Swift aggregate is measured at 3312 after adding the new module import. Actual SwiftPM/native consumer compilation, the retained helper behavior and local archive secrecy passed. The archive test found that the condensed header had removed the release tool’s recognized version comment; restoring that existing marker leaves declarations unchanged. Whole-change review and final gates follow the focused checks.
- Defers: Source reshuffling belongs to 0458 after core freeze. This ticket changes no public behavior, C layout, cache format or release authority.

## Dependencies and ownership

Apply the standing rule before the bounded file cleanup. Keep source totals measured. The implementation uses the existing policy machinery and one whole-change review.

Reviews: accept

Landed: qualified source 2febe18b46f96971b582d605091a7518610238fb.
