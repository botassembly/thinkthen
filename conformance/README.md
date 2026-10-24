# Shared conformance cases

Each JSON file describes one call. Every SDK runs every file through its public API and compares its serialized rows with `expected`. The Rust driver is `crates/thinkthen/tests/conformance.rs`. Existing Rust edge-case tables remain in place. Requests use the public SDK shape: a tagged question, function-specific options and raw records. Client settings live beside the request.

| Field | Meaning |
| --- | --- |
| `client` | Client settings admitted before the call, described by `../schemas/client.schema.json` |
| `request` | The complete request described by `../schemas/request.schema.json`, consumed by every driver |
| `cancelled` | Optional test control that cancels before executing the request |
| `replies` | Ordered canned System One reply rules described below |
| `expected` | Complete ordered rows, or `{"error":"stable.code"}` when admission rejects the call |
| `requests` | Exact number of backend requests |

A reply rule supplies `answer`, a literal System One answer object. Optional `contains` matches a substring of the wire question's instructions. Optional `criterion` requires that name in the wire question's criteria. Both conditions must match. The first matching answer rule supplies each question's reply. An unmatched question receives null and fails normal backend reply validation.

A rule with `status` and `body` instead fails an entire request when any question matches its conditions. This supports isolated failure cases without relying on worker arrival order. Rules contain no credential headers. The library uses no retries in these cases.

Compare every row field, including absent fields, order and detail. Remove only `message` from row errors and annotate-entry errors before comparison; preserve all caller ID data: stable codes are the error contract. A failed row stays in the row list. An admission error becomes the top-level error expectation. Settings and question validation belong to the library.

Build the unpublished test backend with `cargo build -p loopback --bin loopback`. Start `loopback CASE.json` with standard input open. Its first standard-output line is its free loopback port. Use `http://127.0.0.1:PORT/v1` as the backend base URL and a synthetic key. Send `count` plus a newline on standard input to read the current request count as one output line. Send `shutdown` plus a newline, or close standard input, to exit. Wait for the exit and close the process pipes after each case. The executable belongs only to the unpublished test fixture package.

The `public-contract/` cases cover owning boundaries that do not use the remote reply driver. `clients.json` retains raw settings JSON and an empty credential environment. `sql.json` covers SQL settings and policy admission. `identity.json` pins canonical question bytes, execution hashes and selected stub answers through the Rust client; `construction/stub.json` supplies complete stub rows to every SDK. `diagnostics.json` runs through shared Rust and installed SQL hosts. `boundaries.json` covers raw records, saved toolbox parsing, transport limits, CLI input, files and record-store errors through their public Rust or CLI boundaries. Those tests retain exact diagnostic messages only where escaping is the contract.
