# 0535: Give R, Ruby and JavaScript typed native requests

Status: OPEN.

Milestone: 0.2

Depends on: 0534

Reviews: revision fe9bf662f0234e8177eecd2a0f0adea96b36e5d0, accept

Reviews: revision 52ec7a54c991a88a1b5fd92354b1a7d3fd4daa4e, reject

Reviews: revision 250d8fa864e1c837b8a27b1868b69d6a5d7e1287, accept

Reviews: revision e89362a8bb5a0e4dd07c6350639e6e89d1e6dfd3, reject

Reviews: revision ceb3389a0637a555507a6f8a25fce4b47c399e2f, accept

## Outcome

R, Ruby and JavaScript calls hand native Rust request values to the engine. None of the three writes the request grammar in host code.

## Evidence

- Starts from: gap 1 of [the 0.2 closure review](../records/2026-10-11-0-2-closure-review.md). `r/thinkthen/R/requests.R:40-47`, `ruby/lib/thinkthen/session.rb:94` and `typescript/native.js:75` build `{schema, call}` with the input kinds and send it as JSON. [The binding guide](../../libraries/BINDING-AUTHOR.md) says native callers construct typed Request values without a JSON round trip.
- Keeps: every named call, argument, result type, error class and cancellation behavior that 0494, 0497 and 0498 qualified. R keeps its documented index conventions. Both JavaScript module forms keep working.
- Changes: one slice per language, in this order: Ruby, JavaScript, R. Each slice converts host values into the public Rust Request types in its native extension, deletes the host request assembly, and lowers its ceiling. Reuse the conversion 0534 adds where the Rust side is shared.
- Proof: each language's routine installed checks pass. One installed case per language shows that an invalid input raises the typed error with the message Rust admission gives.
- Defers: nothing.

## Native authored-definition API

Expose the existing authored-definition decoder so native host adapters preserve authored bytes and exact Rust admission errors without constructing a whole JSON request or duplicating the parser.

### Added public declarations

```text
fn RequestDefinition::from_authored_json(&str) -> Result<RequestDefinition, Error>
```

### Planned public declarations

```text
fn RequestDefinition::from_request_json(&str) -> Result<RequestDefinition, Error>
```

## Progress

- 2026-10-11 started
- 2026-10-11 landed 9dcdcc5c7; next: Ruby native requests and reviewed error corrections are landed; convert JavaScript next, then R.
- 2026-10-11 landed b071e50bc; next: Ruby and JavaScript native requests and reviewed error and feed corrections are landed; convert R to complete the ticket.
