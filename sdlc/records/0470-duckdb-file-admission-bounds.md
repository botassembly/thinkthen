# Bounded DuckDB file admission

DuckDB file calls feed owned descriptors into the native bounded session while authorized file handles stay on the calling thread. Static declarations are admitted before content reads. Native closure and cancellation stop further reads, allowing only the documented one-descriptor race. Completed prefixes, locations, duplicates and host file authority remain intact. Whole-set operations retain their existing result-memory contracts; this is not a total-memory guarantee.

Fresh design and implementation reviews recorded in the ticket accept the ownership and admission changes. Review repaired lost path-shape admission by reusing shared native preparation rather than restoring a C++ validator. The deterministic installed cases check static malformed paths, zero reads and sends before admission, bounded unread suffixes, completed-prefix reader failures, CSV/TSV authority and held-provider cancellation. The latest DuckDB qualification also passes all ten file functions after correcting recognition settlement.

The installed extension from the reviewed DuckDB source has SHA-256 `6eb0440002bd2a9d59063716798f2d259bda2640e360350d91922f08295536fa`. Its focused file and cancellation evidence is in the DuckDB lane's `target/0495-close/`. Root routine evidence covers all 1996 ordinary Rust cases, one doctest and 21 external consumer cases; lint and script checks pass with unchanged earlier checks reused. Large-input, full parity and platform checks remain candidate work. No memory-exhaustion campaign or paid call ran.

## What the build taught us

A correct provider request count does not establish bounded file reads. Count content access independently and preserve declaration refusal before opening valid-prefix files. Cancellation must release host endpoints without waiting for a held provider, while native owned state handles its own settlement.
