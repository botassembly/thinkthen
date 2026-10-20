# 0751: Share framed pull descriptors with owned sessions

Status: OPEN.

Milestone: 0.2.1

## Outcome

`RequestFeed::from_session_descriptors(name, descriptors)` accepts an iterator of `Result<RequestSessionDescriptor, RequestReaderFailure>` so direct bindings call `Engine::call` through the existing native session decoder and composition path.

- Changes: `crates/thinkthen/src/public/request/execution.rs`, `crates/thinkthen/src/public/request/composition.rs`, `crates/thinkthen/src/public/request/framing.rs`, `crates/thinkthen/src/public/request/session_input.rs`, `crates/thinkthen/src/public/request/native_feed.rs`, `crates/thinkthen/tests/request_session.rs`, `crates/thinkthen/tests/request_session/pull_descriptors.rs`, `specification/rust-api-0.2.md`.
- Reuse session framing, projection, shared attachments, source locations and `RequestReaderFailure::error`; retain its Producer and Io Local errors and Utf8 and InvalidInput Usage errors.
- Check admission, cancellation and deadline before advancing the iterator. Keep logical record positions, actual completed prefixes, decoder header/blank/EOF behavior and accounting.
- Keep the C header, packet schema, `Engine::call` signature and settled value tree unchanged. Stop for a PM ruling if the implementation requires changing them.
- Record the additive declaration and its authority in the Rust public API specification. The existing inventory derives declarations from that specification.

## Proof

- Run focused native tests on M5 through `pm job`, using `sdlc/scripts/job sh -c 'cargo test --locked --offline -p thinkthen --test request_call_transport --test request_session && CARGO_NET_OFFLINE=true python3 sdlc/scripts/policy.py && python3 sdlc/scripts/inventory'`. The coordinator records the exact job command before submission.
- Compare owned sessions and pull descriptors for all four reader failures, their safe messages, logical record positions and completed prefix. Check located records, lines/JSONL/CSV/TSV headers and blanks, table EOF, and zero advancement for a wrong name, invalid header or pre-cancelled call.
- Before review, merge current main and run the existing routine lint and test suite through M5 `pm job`. Inspect logs and fix concrete failures. No local builds, host installs, live calls, paid calls, Actions, publishing or new runner.

## Progress

- 2026-10-20 stopped at Ian’s request with implementation and regression fixtures pushed as uncompiled work in progress; no jobs started and no tests run.
