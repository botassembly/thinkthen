# Rewrite the binding tickets in place

## Ruling

Ian, 2026-10-09, after the 0521 surface assessment left most binding tickets with two to four stacked amendments: "You should go ahead and do all the cleanup of all the tickets, not the coordinator." He agreed that the tickets should be decomposed, reordered and rewritten.

## Reason

Stacked amendments made each ticket contradict itself. A reader had to work out which sentence still applied, and 0383–0385 still opened with a 0.3 deferral that later text reversed. Developer agents build from the first sentence they read. Git keeps every superseded word, so the ticket itself only needs the current outcome.

## The rule

- Each open 0.2 binding ticket holds one current Outcome and one Evidence list. Amendment sections are folded in and removed. Review lines and Progress lines stay unchanged.
- 0504 covers the JVM family only. Dart and Flutter (0522), Swift (0523), Go (0524), C++ (0525), PHP (0526) and Rust with Rust Polars (0527) have their own tickets, so independent lanes can claim them.
- 0505 covers the complete C session views with C and Zig. Ada (0528) and COBOL (0529) follow it.
- 0499 is rewritten as SQLite's remaining Request work. SQLite's SQL conventions stay with 0519.
- 0517 is the package design and can start now. Each migration ticket owns its own packaging slice. 0501 owns the shared inventory. 0530 assembles the final distributions after every migration.
- 0515 removes dead code and marks the frozen C exports. Each migration ticket removes its language's old public names after its replacement passes.
- Each migration ticket owns its README. 0467 is the cross-surface documentation pass after the migrations.
- Every rewritten ticket gets a fresh review before work under it continues.

## Replaces

For this cleanup, the PM skill's rule to keep superseded ticket text visible in amendment sections. The commit history holds the superseded text instead. The 0521 assessment's ticket amendments remain in force through the rewritten text.
