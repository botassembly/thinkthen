# Immutable proxy records

`Service` owns one SQLite connection for one tenant. `admit_question` takes an owner's essential ask and an existing request JSON definition without input records. It calls `SdkRequest::prepare` with the built-in stub engine to admit the function without executing it. `SdkCall::definition` exposes only immutable recording metadata and shared typed-answer validation. `attach_wording` checks that metadata before linking a wording explicitly. No similarity rule merges questions.

Question hashes cover a tagged canonical JSON encoding of essential ask, function and fixed domain. Wording hashes include the question link and admitted wording, descriptions and presentation order. Thresholds remain execution settings. Evidence hashes distinguish text from structured JSON and preserve array order. JSON object order is canonical through the workspace's sorted JSON maps. Caller identifiers never replace content hashes.

Evidence is the materialized selected value. The store never applies a request pointer to it. Annotation recording must admit each named member separately; the store refuses composite annotation definitions because different members can select different evidence. Pointer provenance and operation fan-out belong to later recording work.

`set_gold` accepts the existing `thinkthen::Answer`, validates it against the shared admitted definition and original evidence, then uses an immediate SQLite transaction for expected-current comparison, immutable assertion, audit event and pointer update. Separate connections serialize writers. A stale correction returns `proxy.conflict`; prior assertions remain readable. History reads use a fixed page bound and an assertion cursor. Record size and lock-wait boundaries live in code.

The SQLite dependency provides transactions and durable storage that the standard library does not provide. Its bundled SQLite keeps the store independent of an operator's system SQLite version. The connection uses foreign keys, WAL and full synchronous commits. The reviewed API is [rusqlite Connection](https://docs.rs/rusqlite/0.40.2/rusqlite/struct.Connection.html) and [Transaction](https://docs.rs/rusqlite/0.40.2/rusqlite/struct.Transaction.html).

This library does not execute calls or expose a server. Authentication, transport completion framing, operation and exchange recording, routes, budgets, cache, datasets and curation interfaces belong to the following design outcomes.
