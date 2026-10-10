# SQLite inline relation admission

SQLite converts inline relation text to the saved native definition and calls `Relate::from_json` for rule admission. It applies no separate rule-count restriction. Empty and duplicate rules receive the native usage refusal. SQLite retains its text-array decoding and inline spelling diagnostics.

The [thin-binding ruling](2026-10-09-thin-first-class-bindings.md) assigns grammar, limits and validation to Rust. The [relation contract](../../specification/relate.md) admits ordered inline rules through native validation. [Ticket 0109 decision 11](../tickets/0109-port-the-sqlite-surface.md) imposed a four-rule cap on the former positional table-and-column interface. [ADR 0105](../planning/adr/0105-one-call-shape-for-sql-and-data-frames.md) requires shared native arguments and a clean break from positional slots; ticket 0284 replaced that interface with `(query, rules)`. The remaining cap contradicts the approved shared grammar because the JSON definition path already admits the same rule set. The 0511 raw-row admission slice left inline behavior unchanged for that slice; it did not preserve the cap as a ruling.

This reviewed API detail implements 0511 and changes no SQL columns, row admission, read-only query authority, or public Rust declarations. Ian can overturn it.
