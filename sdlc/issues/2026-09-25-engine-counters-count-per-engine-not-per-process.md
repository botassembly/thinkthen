# Engine counters count per engine, not per process

Status: Open

Filed by ticket 0111 on 2026-09-25.

## What happens

`Engine::usage()` returns the sends, cache answers, and tokens of that one engine and its clones. A second engine built in the same process starts from zero. Ticket 0084 describes `Counters` as a process total, and tickets 0095 and 0111 planned on that.

## Why it matters

A binding that rebuilds its engine when a setting changes loses the old engine's counts. The PostgreSQL binding keeps one engine per settings plan and adds their counters itself. Ian's per-process request cap of 2026-09-25 depends on that sum, so every SQL surface repeats the same bookkeeping.

## What would fix it

Either count in one process-wide place, as 0084 says, or change 0084's text to say the counters belong to each engine. The owner of the public API decides.
