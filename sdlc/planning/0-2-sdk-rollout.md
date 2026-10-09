# 0.2 SDK rollout

The [thin, first-class binding ruling](../decisions/2026-10-09-thin-first-class-bindings.md) sets the target. Rust owns every rule, and each language owns only its idiom. Every surface meets the caller acceptance and its language section in [the binding author guide](../../libraries/BINDING-AUTHOR.md). All binding architecture belongs to 0.2. Scope is frozen to this work, confirmed bugs and TCGA blockers. The [ticket cleanup ruling](../decisions/2026-10-09-rewrite-binding-tickets-in-place.md) split the family work so independent lanes can claim it. The [0521 assessment](../records/0521-surface-contract-assessment.md) explains the completion boundaries.

This plan gives the build order. Read status and lane ownership through pm.

## Order

| Step | Tickets | What it gives and what it waits on |
| --- | --- | --- |
| 1 | 0468, 0487, 0475, 0476, then 0510 | Finish the open behavior work. 0510 follows 0475 and 0476. Send TCGA a named main build once 0475 and 0476 land. |
| 2 | 0511 request grammar, 0513 shared result generation, 0503 session, 0517 package design, 0515 dead code | The foundations. 0511, 0513, 0517 and 0515 wait on nothing and run in parallel. 0503 follows 0511. Keep one writer per shared seam and preserve the pure-core boundary. |
| 3 | 0512 CLI and MCP, 0519 SQL, 0470 DuckDB feed | 0512 and 0519 follow 0511. 0470 consumes 0503's session. |
| 4a | 0494 R, 0496 Python, 0497 Ruby, 0498 TypeScript, 0527 Rust and Rust Polars | The direct-to-Rust surfaces. They wait only on 0511 and 0513, so they run beside the pilot. 0495 DuckDB follows 0470 and 0519. |
| 4b | 0516 C# pilot | Follows 0503, 0511, 0513 and 0517. It proves the session pattern, async waiting, cancellation, cleanup and a local NuGet install for every C-interface family. |
| 5 | 0504 JVM, 0522 Dart and Flutter, 0523 Swift, 0524 Go, 0525 C++, 0526 PHP, 0518 Objective-C | The C-interface families. Each follows 0516 and 0517 and owns its packaging slice. Run them in parallel lanes. |
| 4c, then 5 | 0505 C and Zig, then 0528 Ada and 0529 COBOL | 0505 follows 0503 and 0513 and does not wait for the pilot. Ada and COBOL follow 0505. |
| 6 | 0501 inventory and final assembly, 0383–0385 Windows packages, 0484 test pruning, 0467 documentation | 0501's inventory slices follow each host packaging slice, and its final assembly follows all of them. Windows tickets consume the final artifacts. |
| 7 | 0508 final review, then the full installed run | One read-only review across the final code. Fix confirmed defects, then run every installed consumer on the authorized machines. |

The critical path runs 0511 → 0503 → 0516 → the C-interface families → 0501 → 0508. Keep it staffed first. Give free lanes to step 4a and to 0505 while the pilot runs.

## Standing rules

- Use the existing lanes. Open a fourth lane only during step 5, and only when memory allows.
- Keep one writer for the shared Request, schema, header and session paths.
- Preserve the frozen 0.1 C ABI and its accepted legacy behavior. Raise any conflict before changing it.
- Write and check Windows work locally. The Windows checks for 0383–0385 and the native cache checks for 0474 and 0480 run once, in the first release candidate, after every local check passes. The PM gets Ian's permission before that candidate starts.
- The PM settled 0461's unknown-window rule on 2026-10-09: keep routes under byte limits, report the window as unknown and let the provider's refusal stand.
- No paid calls are needed. No tag, candidate, hosted workflow, rehearsal or publication follows from this plan without Ian's permission.
