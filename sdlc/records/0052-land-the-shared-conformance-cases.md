# 0052: Land the shared conformance cases

Date: 2026-09-21

Status: landed

## Result

`conformance/cases.json` is now the shared, language-neutral behavior contract. Its initial twenty-five cases cover all eight verbs and all six public error kinds. The cases include mixed `annotate` questions across `decide`, `choose`, `score`, and `tag`; stable `rank` and `find` results expressed as input indexes and probabilities; exact request strings; decoded bare and detailed answers; one verified captured exchange; and explicit synthetic provenance for shaped exchanges.

Successful cases run through the production question grammar, request encoder, response decoder, digest, answer rules, ranking, and find selector. Fault cases name deterministic future injection points. `local`, `deadline`, and `defect` remain schema contracts until the one-crate merge adds the private engine runner. The fixture makes no network call and widens no public API.

ADR 0017 now fixes the host-neutral `rank` and `find` meanings and separates successful recorded exchanges from injected faults. Both active plans put the private command runner in the merge ticket. Section 8 step 1 remains next. Ian can overturn the shared result shapes, provenance split, or runner timing.

## Review and proof

Independent design review rejected the first ticket for ambiguous bulk results, weak provenance rules, parsed request bodies, and a case count that looked permanent. The rewritten ticket fixed those boundaries and was accepted.

Independent code review found seven proof gaps across two rejection passes. Bulk cases could omit their result, common credential fields could escape inspection, one mutation stopped at the header, a handwritten JSON compactor duplicated library behavior, a lint exception was too broad, `x-api-key` was missing, and successful cases could carry fault metadata. The implementation now rejects each condition with focused hostile mutations. The same reviewer accepted the final repair.

The Rust ceiling rose by 736 lines for the offline validator. The implementation checked for duplication and kept product rules in existing production APIs. No dependency was added.

On the rebased final branch, `install`, `lint`, `test`, and `spec` all exited zero with `THINKTHEN_API_KEY` unset. The test rung passed 438 tests and two doctests. The spec rung passed 26 specification checks, seven transform checks, and all nineteen green how-tos. The focused conformance test passed both the fixture and hostile-mutation checks. No network or paid call ran.
