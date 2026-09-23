# One state per request caps table-scale classification

Status: Open

MotherDuck shipped `prompt_jev()` on 2026-09-21 (the clipping is in Ian's notes) and reports 2,484 rows a second at $0.50 per 100,000 rows on AG News. Our recorded contract sends one evidence string per request — records never share a request (`specification/annotate.md:102`) — with a fixed floor near 256 billed input tokens per request (`2026-09-20-live-probe-findings...md:76-85`) at $0.042 a million tokens and a documented limit of 1,200 requests a minute (`specification/records.md:140-143`).

The arithmetic cannot be reconciled with the public shape we built against. At one row a request, the ceiling is about 20 rows a second and about $1.08 per 100,000 rows. Their throughput implies roughly 124 rows per request at that rate limit; their price sits below our per-request floor. Either the endpoint accepts multiple states in one request through a form the public contract does not name, or a partner lane carries different rate and price, or both.

The question that settles it costs one metered probe or one TypeSafe conversation: does the endpoint accept multiple states per request, at what price, at what rate. If yes, the packing specification changes and the engine's batch spine already knows what to do with it — annotate packs questions per record today, and the same machinery packs records per request tomorrow. Until then, no claim of table-scale throughput should appear on any page of ours.

The probe is a paid call and is Ian's to order. What Ian can overturn: asking at all, or the priority.
