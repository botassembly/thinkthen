# Yellow routine deadline analysis

The completed checkpoint 0691/1792518201049-861514 tested 52a2e2c48a377315b095e4bef6d4fe2e0b48c6ee. Package and lint passed. Routine nextest reported 2,165 run, 2,156 passed, nine timed out and 78 skipped. The original log remains in Yellow's job-runs directory. No unchanged retry, deadline adjustment or quiet window was used.

The nine terminations occurred near the configured five-second boundary with two workers. The inspected test and product sources were unchanged from the tested commit. These failures show a qualification gap, without demonstrating a wrong answer, deadlock or machine-contention cause.

## Three workload classes

Seven cases prepare or modify real SQLite fixtures: two facade cases, two store cases, image replay corruption, malformed stored answers and old-store refusals. The store fixture seeds production transactions; its unfinished-write case inserts 400 large rows and copies the database and journal. Image replay checks twelve mutations, while the native refusal tables combine twenty mode/format cells and eight malformed-answer cells. Production store creation uses synchronous FULL and commits writes. Sources: engine/facade_tests/contract_tests.rs, engine/store/tests.rs, tests/images/replay.rs, tests/native_store.rs and engine/store.rs in the core crate.

The relate ceiling case constructs 64,770 logical questions, produces 162 requests and parses their bodies. Its historical purpose includes detecting quadratic planning through a child deadline. Sources: crates/thinkthen-cli/tests/backend/relate/ceiling.rs and sdlc/records/0123-build-relate-fits-the-backend.md.

The speed fixture runs one committed table and three planted variants. Ten rows per table each launch a command and a status process, making eighty CLI launches for a complete run. Sources: crates/thinkthen-cli/tests/backend/speed.rs and probes/speed/measure.py.

## Repeated cause and evidence boundary

0605 records earlier routine deadlines and a historical trace with 898 synchronization calls totaling 4.637 seconds. That trace does not identify Yellow's current bottleneck. The shared structural risk is that one external wall-clock cutoff includes fixture preparation, durable writes, process startup and assertions. Several cases aggregate substantial work, while the configuration describes timing headroom measured on arm64.

No failing case inspected here combines a fake clock with real I/O. The completed log contains no per-phase timing or stacks proving where the SQLite cases stopped. CPU activity establishes some progress, without excluding a localized wait. Contention, a shared SQLite defect and a product stall remain unproved.

The smallest proposed corrections preserve behavior and assertions: create independent legacy fixtures in one transaction and copy valid seeds for refusal tables; retain the large planning regression in the existing long suite with a smaller routine boundary case; run one complete speed table and restrict each planted fault to its relevant row. Genuine production durability checks remain intact. These are recommendations from the analysis, not changes made by this checkpoint.

The failed Yellow run cannot qualify x86. The authorized Gator check on repaired source can resolve that acceptance gap. This evidence provides no reason to require an unchanged Yellow rerun, a quiet machine or a deadline increase.
