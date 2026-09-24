FINDINGS

Review of ThinkThen ticket 0099 at c1bf5b84 (branch ticket/0099-port-branch-adrs), compared against tag surfaces-wave7-final (f6a7faea).

1. sdlc/planning/adr/0041-one-deadline-spelling-across-every-surface.md:88. The amendment makes the refusal depend on ticket 0095's acceptance, and it says main has no public deadline door "until then". Ticket 0095 is design-only, and its design was already accepted on 2026-09-24 (0095 branch, line 9). The methods ship when ticket 0086 builds them (one-line-plan-2026-09-24.md:40, 0095 line 13). A reader of main today would think the door exists, but it does not. The ticket's own wording ("conditional on 0095's acceptance") caused this. Fix: "Ticket 0095 designs `deadline_seconds` and `deadline_millis`, and ticket 0086 builds them. Once 0086 lands, they refuse a budget above 4,294,967,295 seconds (`u32::MAX`) as `usage` before anything is sent. Until then, main has no public deadline door. This rule binds 0095's design and 0086's tests." Add the same wording to the build record's first bullet.

Checks that passed:
- git diff between the tag and each ADR shows only the three path fixes (0041:4-5, 0042:5, 0043:30-31) plus one new 2026-09-24 amendment at the end. The accepted text and the 2026-09-23 amendments are byte-identical.
- The 0041 amendment matches the 0095 ticket (branch origin/ticket/0095-binding-contract-members, lines 21, 32-33, 72). The owners are `CallOptions::deadline_seconds` and `CallOptions::deadline_millis`. A budget above 4,294,967,295 seconds is `usage`. The bool refusal and the clamp stay in each binding. Ticket 0095 is not merged to main yet. The ADR cites it by number only, so no path breaks.
- The 0042 and 0043 amendments name "queue item 3" of one-line-plan-2026-09-24.md. That item is the per-surface ticket list, and R and PostgreSQL appear in it. The rulings are unchanged.
- Paths: the three branch-only paths name the tag, and each exists at the tag. `sdlc/records/0070-…` and `sdlc/records/0074-the-r-interrupt-window.md` exist on main and are byte-identical to the tag, so leaving them as they are holds. ADR 0017 and one-line-plan-2026-09-24.md exist.
- No private project or customer is named. The new text has no cleft sentences and no trailing glosses. `git diff --check origin/main HEAD` is clean. `sdlc/scripts/lint` exits 0. `sdlc/scripts/spec` exits 0 with 21 demos green.

Note (not blocking): main's sdlc/records/0075 lacks the tag's 2026-09-23 update paragraph, the one that says ADRs 0041 and 0043 took items 1 and 2. On main, 0075 still lists those rulings as awaiting ADRs, and 0043:4 cites it. A follow-up could port that paragraph. It is outside 0099's stated work.
