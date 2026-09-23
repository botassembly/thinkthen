# 0081: Build `relate` over the shared relation planner

Status: Option A ruled; design ready for Sol re-review.

## Trial record

The Luna-first route is Luna Max because this ticket crosses command, shared planner, request state, replay, and failure boundaries. Independent Sol rejected the first design before implementation.

Sol found four substantive gaps: no shared name-and-kind endpoint owner, false wildcard and method-H claims, no exact mixed detailed-result shape, and stale file and line budgets that also forbade required recognition regressions.

This remediation amends ticket 0081 and `sdlc/planning/relate-design.md`. It assigns one generic relation entity and edge owner, scopes wildcard expansion and H-state corrections into 0081, requires recognize regression proof, gives actual owner budgets, and records Ian's Option A ruling.

Ian chose ordered question entries under `answer.questions` on 2026-09-23. The shape matches `recognize --details`, keeps each entry complete and ordered, and uses request digests for audit and comparison without public question ids. `value` remains accepted edges only. Exact JSON tests must cover choice, yes/no, rejection, and failure entries.

No product code changed. No surface file, live call, or paid call occurred. No focused or complete gate ran. No targeted Sol repair, reopened defect, or elapsed start-to-accept time is recorded. The design now proceeds to Sol re-review.
