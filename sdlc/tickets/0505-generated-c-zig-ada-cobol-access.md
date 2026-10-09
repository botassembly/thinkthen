# 0505: Generate binding access for C, Zig, Ada and COBOL

Status: OPEN.

Milestone: 0.2

Reviews: revision 95fade3863f777142ecd49d4abca0efcf1790cce, accept

Reviews: revision c79e3f65e05eb4f12fa46d8f7ab003d477258c9c, accept

## Outcome

Provide generated typed access for C, Zig, Ada and COBOL without hand-mirroring the post0.1 carriers.

## Evidence

- Starts from: PM architecture asks3/5; constrained-language readers and frozen C surface.
- Keeps: All languages, frozen0.1 ABI, COBOL documented representation limits and named typed access.
- Changes: After0492/0502/0503, review generated carriers versus generated JSON accessors per host, then replace post0.1 mirrors only when installed typed cases pass. Claim `libraries/c/**`, `libraries/zig/**`, `libraries/ada/**`, `libraries/cobol/**`, respecting0482 and generated-header ownership.
- Proof: C compiler and each installed host consumer read full typed fields and failures through shared cases. Raw JSON availability alone does not satisfy parity.
- Defers: Business policy and language removal. Size: large constrained-host migration.

## Revised code-reduction estimate

## 2026-10-09 amendment

The thin, first-class ruling fixes this outcome to generated Rust-owned layouts for C, Zig, Ada and COBOL, including 0513's Ada specs and COBOL copybooks. Follow 0511 admission and the reviewed shared interfaces. Preserve the frozen C ABI, lifetime rules and documented host limits. Remove manual post-0.1 mirrors only after installed typed cases pass. The earlier fallback estimate below is historical; measure actual changes. Review the amendment and narrow generated declarations and host glue per slice.

### Earlier fallback estimate

The fallback generates layouts while retaining host value copying and native lifetime handling. At main `26fc8c900`, `libraries/ada/src/thinkthen_c*.ads` contains 1,967 nonblank lines and the seven COBOL `tt-*.cpy` carrier copybooks excluding constants contain 699. Estimate at most about 2,700 hand-maintained lines becoming generated across these named files. This is an upper bound: package declarations, imports, constants and any retained compatibility code reduce the replaceable portion. Generated carrier lines remain, so net source deletion may be small or zero after generator code is added.

C already consumes the generated header. Zig can import it directly; its execution and owned-result code is not a disposable layout copy. The 0502 experiment did not establish a deletion estimate for those adapters. Preserve typed values, failure facts, representation limits and lifetimes; record actual manual code removed at landing. Do not apply the study's 25,000–35,000-line schema-reader estimate to this layout-only fallback.
