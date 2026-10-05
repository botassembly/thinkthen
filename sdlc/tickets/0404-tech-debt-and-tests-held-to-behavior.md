# 0404: Remove the proof spiral

Status: in progress. Permanent build-rule cleanup is prepared first; machinery, records, tests and stale statuses remain.

Milestone: 0.2

Lane: 0 for standing rules. Later slices use a free lane under the current team note.

## Outcome

1. Permanent rules require one review of the whole ticket or slice, full tests and lint on the landing commit, and one short record per ticket at landing. A second review is allowed only for a substantial fix touching data loss, credentials, money, memory safety or user-visible correctness, or for a new dependency. Retain file caps, source ratchets and release rules.
2. Delete machinery that verifies proof artifacts. Keep plain recipe replay and sample-output comparison. Retire obsolete paid-check harnesses and receipt dumps.
3. Collapse release-ticket records and rename tests by behavior. Merge duplicate tests while keeping every distinct regression.
4. Status lines say what works and what remains in at most two sentences, based on actual ancestry and evidence.

## Evidence

- Starts from: Ian's 2026-10-05 cleanup ruling and the docs handoff, “Remove the proof spiral root and stem,” sections 1–4. Main `8ef1415b5` already lands the default of eight. The older cleanup plan and ticket preparation still impose duplicate review, checkpoint and record work.
- Keeps: file caps and ratchets; the second dependency reviewer; release rules in `worktrees.md`, `release-process.md` and `quality-plan.md`; spend guards `live`, `live-tokens` and `live-test`; secrecy, child-process, cancellation, memory-safety and installed-package checks; release rehearsal and Ian's approvals. Ticket 0119 stays separate. Product contracts and behavior stay unchanged.
- Changes: first rewrite `AGENTS.md`, preparation, ticket and record guidance, Rust standards and the old plan to remove conflicting rules. Delete `recipe-proof.py`, its receipt-count pins, `manual-0399-launch.py`, `sdlc/manual/0399/`, the two 0402 D receipt manifests and C checkpoint manifest. Replace binding-proof tree hashes with sample runs and output comparisons. Remove ticket enforcement of a separate build-lessons section. Inspect script self-tests for overlap; delete ticket-named ones only when another check guards their release behavior. Consolidate 0402, 0401 and 0380 records; keep designs where code points at them. Delete `0405-surface-cells.json` and keep its readable matrix and report. Rename or merge `*_0399`, `*_0400`, `*_0401` and `numbers_0402.rs`; merge duplicate rank cases and repeated safety cases without dropping distinct failures. Rewrite statuses for 0377, 0380, 0381, 0382, 0399, 0402, 0416 and 0419. Add no inline-unit-count pin, proof checker or performance campaign.
- Proof: run the smallest checks that exercise each changed behavior. Compare recipe and binding sample output; run retained regressions after merges. Use one fresh whole-slice review, then full tests and lint on the landing commit. The final short record gives commits by kind, lines removed and the harm each kept check guards.
- Defers: recipe publication and Windows building wait until the standing-rule slice lands, then resume separately. External hosting, native Windows runs, release rehearsal, paid calls and release approval need their existing authority. Other product cleanup and new performance checks stay outside this ticket.
