# 0492 — generated-c-header

Status: OPEN.

Milestone: 0.2

Reviews: revision 95fade3863f777142ecd49d4abca0efcf1790cce, accept

## Outcome

Generate the committed C header from Rust-owned exports, layouts, values and ownership documentation.

## Evidence

- Starts from: PM architecture asks2/5; hand-kept thinkthen.h and0485 value-drift concern.
- Keeps: Frozen0.1 ABI and thinkthen_call; current layout/export tests and memory ownership.
- Changes: Adopt cbindgen with required dependency review. Generate header deterministically and fail on committed drift; remove independent value tables. Fold0485 into this proof. Depends on0491 and0482. Claim `libraries/c/include/thinkthen.h`, C export definitions, generator configuration and relevant checks.
- Proof: Regenerated header matches; compiler-visible layout, exported symbol and Rust/C value checks pass. A changed Rust discriminant changes or fails header generation.
- Defers: Premature retirement of carriers before typed consumers migrate. Size: medium shared contract change.
