# 0459: Fail binding checks when copied C declarations drift

Status: ready

Milestone: 0.2

Owner: builder.

## Outcome

Every binding that copies the C ABI has declarations that match the public C header. Generation or an existing-gate check fails a mismatch in struct fields, layout, enum values or exposed function signatures. PHP's missing `thinkthen_result_row` prototype is corrected in the pushed family follow-up `06f295ace`; this ticket prevents future drift.

## Evidence

- Starts from: Ian's approved ask 3 in shared mailroom message `2026-10-07-pm-file-size-rule-file-cleanup-after-core-freeze-binding-layout-check-and-cache.md`. The reviewed SDK candidate carries the additive rank-member details getter; PHP's hand-copied declarations omitted `thinkthen_result_row`; `06f295ace` corrects it and exercises the real FFI refusal path.
- Keeps: The public C ABI, actual platform-dependent alignment and widths, existing bare/complete calls and memory ownership. Bindings that directly compile against the header or native Rust source retain that route.
- Changes: Inventory the actual copied declarations. Generate them from the header where practical or compare them at their real compiled/runtime boundary through the existing binding gates. Cover layout and function declarations, not just matching names. Keep platform facts derived from the actual C compiler and each language's real declarations; do not assume one platform's offsets everywhere. Batch identical work under existing family owners.
- Proof: Plant a mismatched field/layout, enum and function declaration and make the owning existing gate fail. Run actual C-backed binding checks with required toolchains; exit 77 cannot count as success. Existing installed consumers and secrecy/cancellation cases remain required. No separate per-language write-ups or receipt system.
- Defers: ABI redesign, new functions and unrelated package work. This correctness check is required in 0.2 and is independent of the optional timing of 0458.

## Dependencies and ownership

Use the settled public C header from 0426/0431 and coordinate copied files with 0427–0430. The PHP prototype correction is already pushed in the active family follow-up; the full drift check remains this ticket's outcome.

Reviews: accept
