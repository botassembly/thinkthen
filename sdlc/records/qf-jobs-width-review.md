ACCEPT

# Review of Quick Fix qf-jobs-width

Reviewer: a fresh read-only Opus session that did not write the work. Its first reply, on `f8c808fa`, listed findings. Its second reply, on `c921a5d3`, accepted the fixes. This page restates both.

It judged 4 the best-supported default. The three ADR claims, the retry claim, and the experiment author's recommendation match their sources. Every number on both pages matches its source and names the record that measured it. The public page names no private project or customer.

Findings, each fixed in the next commit:

1. The issue pointed at `sdlc/records/qf-jobs-width.md` before that file existed.
2. `specification/records.md` still said 4 is safe everywhere.
3. The pages stated that longer records stay under the limit at 4 as a measured fact, the decision called the long-record slowdown measured, and it dropped "about" from 6 to 10 percent. The pages now say the record reports it with no rate, and the decision marks the slowdown as unmeasured.

Note, not blocking: no test ties the `[default: 4]` help text to `Width::FALLBACK`.

Plant: `Width::FALLBACK = Self(3)` turned `the_first_explicit_width_wins_and_only_a_different_one_is_refused` red. Two width tests hung past 60 seconds. `decide_edge` passed 20 of 20.
