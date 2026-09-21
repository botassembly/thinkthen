# 0050: Apply the transform catalog ruling

Date: 2026-09-21

Status: landed

## Result

ADR 0015's trigger has fired. The repository has whole-run metrics, a row policy, and a reviewed-action monitor. Ten `.jq` files total 1,408 lines, the transform index holds seven executable Bash blocks, and nineteen how-tos are green.

The accepted release surface remains read-only `thinkthen transform list` and `thinkthen transform show NAME`. The tool will list or print selected transforms and will never run `jq`. Catalog membership, public names, byte identity, and package location wait for release preparation after the one-crate move.

The agent declines the separate `report` verb because the completed transforms produce several useful report shapes without a combined command. Ian can overturn that conclusion and the catalog timing.

The remaining order is explicit: record and address the build-team findings in ADR 0017, land Job 3's shared conformance cases, prove Job 2's DuckDB interrupt inside Python, then begin ADR 0017 section 8 step 1.

## Review and proof

Independent design review rejected the first proposal because it repeated an accepted architecture decision and prematurely promised that every current transform would ship in the catalog. The accepted rewrite applies ADR 0015 and leaves the catalog boundary to its implementation ticket.

Independent implementation review found that the plans skipped two required ADR 0017 jobs and attributed the final `report` verdict to Ian. The repair restored both jobs, named the verdict as the agent's decision, and made the roadmap, both plans, ADR 0015, the transform index, and the ticket agree. The same reviewer accepted the repaired result.

The recorded counts reproduce by command. `sdlc/scripts/lint`, all seven executable transform blocks, and `git diff --check` pass on the reviewed branch. No product code, transform behavior, command surface, recording, or live call changed.
