# Notes

The port notes of ticket 0107. The tag `surfaces-wave7-frozen-2026-09-24b` keeps the earlier notes as history. Newest entry last.

## 2026-09-25: the port onto the real engine

- **Tried:** the addon as its own Cargo workspace at `libraries/typescript`, over `crates/thinkthen` by path with default features off. The lock started as a copy of the root lock, and cargo pruned it offline.
- **Saw:** 77 packages. `cargo deny` passes advisories, bans, licenses, and sources on it.
- **Tried:** a worker thread per call, with the result sent back through a `ThreadsafeFunction`. `detach()` fires the call's `CancelToken` and aborts the function, and `index.js` calls it on every settle path.
- **Saw:** an aborted call rejects at once, and the child process exits while the held reply is still open. A settled call lets Node exit too.
- **Tried:** `node:test` with a child process per call, each on its own loopback backend with a fake key.
- **Saw:** a child inherits `--input-type` from the parent's `execArgv`. `fork` and `Worker` now pass `execArgv: []`.
- **Saw:** the whole cases file passed inline to a child hit `E2BIG`. The child now reads the file itself.
- **Saw:** a cache folder holds a `.thinkthen-backend.json` marker and belongs to one backend address. An engine that points at a second backend uses `cache: false`.
- **Saw:** `maxRequests: 2` on a three-record `decide_many` sends two records, then refuses. `EngineBuilder::max_requests` documents that streaming order. The ticket expected zero sent.
- **Saw:** `find` takes `{ none: true }` since ticket 0150, so cases `18-find-second` and `19-find-none` run.
- **Saw:** the engine's throttle sentence carries no `thinkthen: ` prefix. The command adds that prefix.
- **Tried:** `ratchet.mjs` skips only `target`, so `npm ci --offline --prefix target/npm` holds `node_modules`, and `tsc` runs from there.

## 2026-09-28: batching, described labels, and call facts

- **Saw:** default Max packs distinct rows and coalesces duplicate questions in one body. The old exact-body corpus needs local `batch: 1`; the captured-body test checks Max separately.
- **Saw:** a bare score list stays a string list in the request. An object map with a null description becomes an explicit empty-object level in the request. The two forms cannot share one normalization.
- **Saw:** recognition accepts a structured kind description through its parser, but its token-position request names the kind without quoting that description. The test checks the real body and its digest instead of assuming the description appears in the prompt.
- **Saw:** a worker may start before a Rust account exists. A pre-account usage error has no facts; a failed sent request has final facts with zero completed records. A prompt abort carries one receipt, and observing it keeps the same worker alive for its final report.
