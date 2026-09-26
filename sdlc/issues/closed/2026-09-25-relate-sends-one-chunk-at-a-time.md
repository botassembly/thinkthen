# Relate sends one chunk at a time

Status: Closed 2026-09-26 by ticket 0143.

Ticket 0118 asked a DuckDB relate over 16 rows, under throttle 8, to hold 8 counted requests on a loopback backend. It holds 1. The DuckDB surface is not the cause. Main's engine sends relate's requests one after another.

## Measured behavior

- On the held arm with `SET thinkthen_throttle = 8`, a 16-row `thinkthen_relate` with the bare rule `knows` reached 1 counted request and waited there. The backend's count never rose to 8.
- `Engine::relate` loops over the prepared relations one at a time (`crates/thinkthen/src/engine/facade/relate.rs:116`).
- `Engine::ask_chunks` sends each chunk and waits for its reply before it sends the next (`crates/thinkthen/src/engine/facade.rs:257-276`).
- `PreparedRequests::with_profile` puts every question of one relation in one chunk unless a backend profile's limit splits it (`crates/thinkthen/src/engine/prepared_request.rs`). With no profile, a relation is one request.

The throttle therefore never binds a relate. A relate with several rules, or with a profile that splits a relation, takes the sum of its requests' latencies.

## Options

1. Send one relation's chunks under the engine's throttle, as the decide batches do, and keep the answers in chunk order. This helps only when a profile splits a relation.
2. Send the relations of one relate at the same time under the throttle. This helps any relate with more than one rule.
3. Leave it. One relation is one request without a profile, so most relates gain nothing.

## Recommendation

Option 2 first, then option 1, when a relate with several rules or a splitting profile shows up in a real workload. Neither is a surface change. Ticket 0118's acceptance line now proves the throttle reaches relate through the conflict sentence.

Ian can overturn the recommendation or the priority.
