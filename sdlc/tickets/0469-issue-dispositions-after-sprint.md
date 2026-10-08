# 0469: Reconcile stale issue statuses with landed fixes

Status: OPEN. The sweep found obsolete open statuses, duplicate headers and closed issues in the open folder.

Milestone: 0.2

Owner: builder.

Reviews: revision fa00d9a4e, accept

Reviews: revision 1b36ccab50434e84809165afa5b09218c5bae2ef, accept

## Outcome

The issue list separates fixed defects, remaining obligations and later features using verified landing references. This changes records only and does not claim new runtime qualification.

## Evidence

- Starts from: 0462 historical sweep of all 56 root issues. SQLite model routing, named backends, Polars deadline fixtures and exact-commit release routing have landed fixes but obsolete open descriptions. Run-facts and release/install issues also retain superseded obligations. Four closed files remain in the open folder; link/navigate have duplicate headers and the parked demonstration lacks a milestone.
- Keeps: Every genuine remaining obligation, historical failure evidence, explicit later-feature rulings and all release holds. Keep legacy 0.1 failures distinct from current 0.2 evidence.
- Changes: Verify each resolved portion against source and landing records, close or narrow its issue through pm, move closed files and normalize duplicate metadata. Link known current bugs to their new 0.2 owners. Leave true future features later; do not erase or silently defer a runtime bug.
  Claim `sdlc/issues/**` and `sdlc/records/0469*`.
- Proof: Fresh ticket and whole-change review, existing issue/ticket/link checks and exact fix references. Compare scope before and after; no build, paid call, workflow or new status checker.
- Defers: No reformat of the whole ledger, historical receipt reconstruction or new verification machinery. Fixes to runtime behavior have their own tickets.
