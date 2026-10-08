# 0462: Review all 0.2 changes and sweep known bugs

Status: COMPLETE. Fresh area reviews and the historical sweep produced owned fixes in 0463–0472. Their implementation and Ian’s release hold remain open; see the consolidated record for scope and limitations.

Milestone: 0.2

Owner: builder.
Lane: claude-0 owns review coordination and sdlc/ changes; reviewers read main without edits.

Reviews: revision c26986771, accept

## Outcome

Every changed product area since rc/0.1.0-rc.1 has a fresh read-only review. Confirmed bugs have a fix or ticket with severity and a scope ruling. Open issues, deferred gaps, skipped or ignored tests and recorded failures have a disposition. Report what landed and what remains; do not claim absence of bugs from a passing suite alone.

## Evidence

- Starts from: main 7ea661c1e; inbox message 2026-10-08-pm-thinkthen-hold-0-2-for-tcga-results-a-bug-sweep-and-an-after-sprint-review.md, asks 2, 3 and 6; retained 29-consumer installed campaign and candidate failure history in 0425.
- Keeps: Existing behavior, distinct secrecy, ownership, cancellation, error and parser checks, the shared suite and actual installed-package evidence. Keep 0461 ready for the PM's ruling after TCGA results.
- Changes: Review core/engine, CLI, C, each SDK family, SQL, dataframes, MCP, packaging and release scripts for defects, dead code, duplication, stale documentation and specification drift. Reconcile independent findings and historical issue/test/failure inventories. Create bounded fix tickets for confirmed defects; record later feature gaps explicitly. Create one concise consolidated record for this ticket at landing.
- Proof: Fresh read-only reviewers by area, concrete file/line evidence and safe focused reproductions where useful. Reuse existing tests; no paid calls, hosted workflows, benchmarks, receipt framework or verification tools. Review the ticket before authorizing implementation. Code fixes receive their own review and applicable checks.
- Defers: Release management and publication require Ian's permission. TCGA clinical or biological interpretation belongs to the demo team. No new feature scope, broad file redesign or automatic deferral of user-visible bugs.

## Review and sweep order

1. Review core/engine; CLI, C and MCP; existing and foreign SDK families, SQL and dataframes independently. Review packaging/release scripts and public help/docs against the same baseline.
2. Sweep all open issues, deferred gaps, ignored/skipped tests and failed outcomes recorded since 0.1. Separate fixed historical failures, current defects, permitted platform exclusions and future features. A stale issue status alone is not a runtime bug.
3. Verify important findings, reconcile duplicates and assign tickets by real behavior change. Fix confirmed bugs in 0.2; move a defect later only with a written ruling. Later expansion remains later.
4. Report severity totals, coverage, fixes landed, ticketed work and remaining decisions to the PM. The review stays open until every area reports and the sweep is reconciled. The TCGA scope decision and Ian's release permission remain separate.
