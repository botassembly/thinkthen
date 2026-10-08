# Adopt shared team skills

The PM's 2026-10-08 cleanup message authorizes this documentation Quick Fix. The repository instructions now use workspace role skills and pm status. [The preservation decision](../decisions/2026-10-08-preserve-planning-rulings.md) retains accepted cache/type/API choices, benchmark fixture provenance, capacity boundaries, 0.2 scope/order and Ian's release hold, with existing contracts as their source.

Removed `work-plan-2026-09-27.md`, `cleanup-2026-09-30.md`, `bench-handoff.md`, `handoff-to-the-build-team-2026-09-21.md`, `handoff-to-the-architect-2026-09-21.md` and `build-team-response-to-handoff-2026-09-21.md` from `sdlc/planning/`. Historical Markdown citations now name their original source commit. Parked spreadsheet/language-analysis clients, hard-case/stability analysis, unresolved caller controls and alternate adapters/schema generation through four `pm idea add` entries. Existing issues retain their evidence and status.

Checks: `pm lint` reports zero findings; its light flow skips link checks. A targeted inspection checked 514 local Markdown targets and 25 archived targets in changed documents, with no new missing target. Four historical source links were already missing in the base records: `decide_edge.rs`, both old `annotate_schedule.rs` files and `engine/request/tests.rs`. Existing ticket/issue backtick citations remain historical evidence. New prose matches none of the 35 private names in the external list. The instructions fit the existing size check, and `git diff --check` passes. The lane table and pm configuration are unchanged. pm reports build `94e514c`; the configured mailroom is `agents`. Product code, configuration and tests have unchanged inputs, so their existing evidence applies. Removed the owned 0471 lock scratch and retained warm builds.

Review: the coordinator will add the fresh read-only review verdict and any correction here before landing.

## What the build taught us

Remove duplicated status without copying it into a decision. Preserve product rulings through their owning contracts and pin historical evidence to its original source. The configured lane table needs the upstream layout migration before its duplicate prose can be removed.
