# 0535: Give R, Ruby and JavaScript typed native requests

Status: OPEN.

Milestone: 0.2

Depends on: 0534

## Outcome

R, Ruby and JavaScript calls hand native Rust request values to the engine. None of the three writes the request grammar in host code.

## Evidence

- Starts from: gap 1 of [the 0.2 closure review](../records/2026-10-11-0-2-closure-review.md). `r/thinkthen/R/requests.R:40-47`, `ruby/lib/thinkthen/session.rb:94` and `typescript/native.js:75` build `{schema, call}` with the input kinds and send it as JSON. [The binding guide](../../libraries/BINDING-AUTHOR.md) says native callers construct typed Request values without a JSON round trip.
- Keeps: every named call, argument, result type, error class and cancellation behavior that 0494, 0497 and 0498 qualified. R keeps its documented index conventions. Both JavaScript module forms keep working.
- Changes: one slice per language, in this order: Ruby, JavaScript, R. Each slice converts host values into the public Rust Request types in its native extension, deletes the host request assembly, and lowers its ceiling. Reuse the conversion 0534 adds where the Rust side is shared.
- Proof: each language's routine installed checks pass. One installed case per language shows that an invalid input raises the typed error with the message Rust admission gives.
- Defers: nothing.
