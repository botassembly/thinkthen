---
flow: build
priority: 20
opens: crates/thinkthen-core specification/fixtures sdlc/ratchet.json
---

# 0002: Translate a plan to and from systemone

Status: landed

## Outcome

`thinkthen-core` gains the plan and the `systemone` adapter of `specification/backends.md`: two pure functions, proven against the fixture files.

## Current Facts

Ticket 0001 supplies `Question`, `Answer`, `Probability`, and `Usage`. `specification/backends.md` settles the adapter contract and the `systemone` mapping. `specification/fixtures/systemone/` holds one request and response pair and three refused responses. The core cannot read files, so tests take fixtures through `include_str!`.

## Scope

- `Condition` refuses text that holds only white space, as `decide.md` now says. Ticket 0001 refused only the empty string. `Evidence` follows the same rule.
- `Evidence`, `ModelName`, and `Plan`: the evidence, the model name, and an ordered list of questions. The adapter names them `q1` onward.
- `AdapterKind`, an enum with one variant, `Systemone`. It parses from its lowercase name and refuses any other.
- `encode(plan)` returns the request bytes. `decode(plan, bytes)` returns the model that answered, one answer per planned question, and optional usage.
- Wire bodies are typed structs with `serde`. No dynamic JSON.
- A typed decode error tells apart: a body that is not valid JSON for this format, a missing answer, an answer of the wrong kind, and a probability out of range.

Excluded: HTTP, keys, retries, `choice`, `score`, and any second adapter.

## Acceptance

- `encode` of the plan in `if-urgent.request.json` equals that file by value.
- `decode` of `if-urgent.response.json` yields probability 0.92, the model name, and the usage.
- Each `refused-` fixture yields its own error variant.
- A response with no `usage` decodes. A response with unknown extra fields decodes.
- A property test round-trips: any valid condition and evidence survive `encode` and a parse of the bytes unchanged, including quotes, newlines, and non-ASCII text.
- The whole ladder is green, and a second agent reviews the public surface.
