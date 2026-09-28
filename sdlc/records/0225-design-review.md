# 0225 design review

Status: **ACCEPT**. Fresh independent Medium reviewer session `01a0e695-9381-7070-a807-bcc3be1f739f` reviewed clean source `1e4936a7e8b311e3c912f19aa4bdf8e5eeba3f35` and reported no blocking finding. The coordinator accepted the routine register 31 correction within Ian's authorized queue outcome. [Ticket 0225](../tickets/0225-bound-usage-lock-waits.md) and [ADR 0097](../planning/adr/0097-bound-advisory-usage-lock-acquisition.md) now authorize implementation; they do not claim runtime proof or closure.

The reviewer independently traced `update`'s blocking usage lock, `finish`'s wait, `Drop`'s join, and the writer's released queue mutex. That mutex boundary lets finish or drop publish one deadline for the writer's nonblocking lock attempts. The deadline must cover **every** taken month, including months after an earlier successful write. Existing writer failure clears pending deltas and wakes finish; in-memory totals and run facts remain distinct from disk persistence. Existing CLI warning remains before final `--facts` with unchanged result code. The review accepted the explicit limit: foreign usage-lock acquisition is bounded, while arbitrary filesystem I/O and status reader waits are not. It endorsed one held-lock compiled command case and retention of ordinary released-lock durability and warning-before-facts cases. No reviewer build, test, provider or live-ledger operation occurred.

## What the build taught us

Preparation found that the accepted write-behind queue has a separate destructor join and a multi-month pending vector. Those details set the small design's actual boundary. The later code review accepted the implementation of that deadline and cleanup; the build record preserves the actual proof.
