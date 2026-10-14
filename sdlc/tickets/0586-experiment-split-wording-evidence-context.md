# 0586: Experiment: split each question into wording, evidence and context

Status: OPEN.

Milestone: 0.3

## Outcome

Learn, for each of the ten functions, whether the engine can name the wording, evidence and context parts of every wire question it sends and rebuild the exact bytes from them, so the database can store parts apart and reuse rules beyond `exact` work.

## Evidence

- Starts from: `crates/thinkthen/src/core/digest.rs`, `crates/thinkthen/src/core/adapters/systemone/`, `crates/thinkthen/src/engine/store/`, `specification/cache.md`, `specification/request.schema.json` and the recordings under `specification/fixtures/`.
- Keeps: No product code changes, no paid model calls, no publishing, and no agent or machine time taken from 0.2 work. Today's exact cache key.
- Changes: `sdlc/records/0586*` only. Prototype code stays in an owned numbered experiment folder with `OWNER` reading `thinkthen/0586`.
- Method: for every recorded request of every function, split each wire question into wording, evidence and context with a digest each, then rebuild the request from the parts.
- Proof: every rebuilt request is byte-identical to the recording, and changing only the evidence changes only the evidence digest. The record names each function or step where a part cannot be separated, such as recognize's second step, relate's pairs, rank's packed batches or find's candidate lists, and why.
- Stop rule: all ten functions pass, or every failing case is named with its reason.
- Decision it changes: PRD 01's rule that an unsplit part stays empty and serves only `exact` reuse.
- Machine: the Beelink.
- Defers: the real split goes into the 0.3 database ticket.
