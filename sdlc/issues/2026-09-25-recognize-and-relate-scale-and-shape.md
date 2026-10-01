# recognize and relate: scale

Status: open for items 4 to 6, all later features. Item 3, the both-ways edge shape, moved on 2026-09-30 to `closed/2026-09-30-relate-pair-planner-loses-precision-on-the-beatles-bench.md`, because one relate ticket after 0304 slice 4 settles both. Shortened 2026-09-30. Ticket 0123 fixed items 1 and 2. The recognition-guidance Quick Fix fulfilled items 7 and 8. Git history holds their text. Ian can overturn any fix below.

Milestone: later

Priority: ranked in `../planning/issue-priorities-2026-09-30.md`. Owner: a future ticket after 0304 slice 4.

The relation planner and the request splitter serve both commands. Each open item is about how that path grows with the input or how its output reads.

## 4. relate takes one set, and a grown set re-asks every question

`relate` reads one complete entity set. Every request state carries the complete entity array, so adding one entity changes every request, misses the cache, and bills again. No form takes two inputs or marks rows as new.

The fix:

- Say now, in `specification/relate.md` and the database pages, that adding any entity re-bills the whole run.
- Design one form that marks a subset as new, from a second input or a column. It asks only questions that touch a new entity and keeps old digests where the method allows. A two-set run is the same form with one side marked new.
- Measure "ten new songs against 184 old ones" before and after.

Done when a second run that adds ten entities sends only questions touching those ten, and the specification states what it re-bills.

## 5. Block before pairing when the set is large

Same-kind pairs grow with N². At 255 entities an either rule asks 32,385 questions. Experiment 225 counted 5, 14 and 43 false edges at 10, 50 and 200 records. Nothing narrows candidates first.

The fix: an optional blocking step in the shared planner. A key column, a text match, or one cheap question per entity puts entities in groups, and pairs are asked only within a group. The dry run reports questions with and without blocking. Recognize keeps its path.

## 6. The 255 refusal on every surface

The command's refusal now names the actual count, the 255 limit, the pair arithmetic and a split or narrow remedy (`8e452e45b`, [record](../records/qf-relate-boundary-guidance.md)). The other hosts' diagnostics and a 254, 255 and 256 case in the shared conformance set remain.
