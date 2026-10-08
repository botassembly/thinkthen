# 0505 — generated-c-zig-ada-cobol-access

Status: OPEN.

Milestone: 0.2

## Outcome

Provide generated typed access for C, Zig, Ada and COBOL without hand-mirroring the post0.1 carriers.

## Evidence

- Starts from: PM architecture asks3/5; constrained-language readers and frozen C surface.
- Keeps: All languages, frozen0.1 ABI, COBOL documented representation limits and named typed access.
- Changes: After0492/0502/0503, review generated carriers versus generated JSON accessors per host, then replace post0.1 mirrors only when installed typed cases pass. Claim `libraries/c/**`, `libraries/zig/**`, `libraries/ada/**`, `libraries/cobol/**`, respecting0482 and generated-header ownership.
- Proof: C compiler and each installed host consumer read full typed fields and failures through shared cases. Raw JSON availability alone does not satisfy parity.
- Defers: Business policy and language removal. Size: large constrained-host migration.
