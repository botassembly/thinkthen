# `annotate --dry-run` under a profile prints the unsplit request

Status: Closed 2026-09-26 by ticket 0161 (`sdlc/records/0161-build-annotate-reads-what-it-names.md`). Under a profile, `--dry-run` prints the first chunk a live run sends, with `request_count` and `group_requests`. Filed 2026-09-26 by the queue owner from local experiment 273, report 03, finding 2-3. Blocks 0.1 under goal 4: a reviewer approves a request the tool never sends.

## What happens

With a profile that caps questions per request, `annotate --dry-run` prints one request that holds every question. The live run splits the set and sends several requests. The report printed 1,500 questions under a profile of 100. Live, a 250-question set under a 100-question profile showed one request in the dry run and sent 3.

`annotate.md` says the dry run "prints the first request". How-to 16 says it "shows the exact request". Neither holds under a profile, and the chunk count, and so the cost, stays hidden.

## Checked on main

Verified: `crates/thinkthen/src/cli/annotate/plan.rs:59` calls `facade::split` and discards the chunks. Line 62 then prints `plans.first()`, the unsplit group. The live counts come from the report.

## What would fix it

Print the first real chunk, and print the chunk count for each group. Ticket 0154 touches splitting, so it may carry this. The queue owner decides.

## Done when

Under a profile, the dry run prints the first request the run would send and the number of requests per group, and a test pins both.
