# 0439: Document Linux R installation

Status: COMPLETE.

Opened as: 2026-10-11. Linux R installation guidance and installed replay passed review, full tests and lint at 20681a8a8. Typed result/2 and image adoption remain with 0431.

Milestone: 0.2

Owner: builder.
Ticket review: accepted 2026-10-06; blocking findings corrected.

## Outcome

A Linux R reader can install the supported package using complete documented prerequisites and exercise a named call.

## Evidence

- Starts from: Current main c64b71859; 0405 audit is historical and predates 0377/0420. PM message `2026-10-06-pm-0-2-is-not-done-every-sdk-consistent-and-the-sdk-ready-for-the-proxy.md`, asks 8.
- Keeps: Preserve existing bare calls and generic JSON compatibility doors, backend selection from 0377, six error kinds, cancellation, secrecy, count-only usage, and zero-send strict replay. Reuse the Rust engine, C boundary and native file reader; add no host cache, scheduler or second parser.
- Changes: Document supported R-universe/source-package routes as verified by actual package tooling, prerequisites, native library loading and a saved-answer example. Cross-link the R index convention and typed source helpers.
- Proof: Run the documented install/import in the existing Linux package consumer; replay the example without network calls to a model. Check public text and links. Do not add a new unsupported packaging route. Focused checks passed: existing R smoke/native load and loopback count, bounded offline source install and exact README replay, 20 install-check tests, site build/internal links and R public text. Fresh root review corrected source-helper references and kept changes within R installation scope. Full tests/lint and the short landing record remain for the lander.
- Defers: Proxy service/screens, unrelated features and Windows Node/C#/JVM packaging remain outside this documentation outcome. R images remain required in 0.2 and owned by 0431/0447; this ticket defers only their implementation here.

## Dependencies and ownership

0412 owns index documentation; 0431 owns typed R carriers. This ticket owns Linux installation instructions and the corresponding existing package smoke.
