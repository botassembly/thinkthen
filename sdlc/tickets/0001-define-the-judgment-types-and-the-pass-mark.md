---
flow: build
priority: 10
opens: crates/thinkthen-core Cargo.toml Cargo.lock sdlc/ratchet.json sdlc/scripts/policy.py
---

# 0001: Define the judgment types and the pass mark

Status: ready

## Outcome

`thinkthen-core` gains the types for one yes/no judgment, the symmetric pass mark, and the JSON result of `specification/result.md`. Nothing reaches a network, a file, or the binary.

## Current Facts

The core holds a name constant and a version line. `specification/result.md` is settled for `decide if`. `sdlc/planning/rust-standards.md` names the types this ticket starts: a probability, a pass mark, and enums for every fixed set of choices. The core's `clippy.toml` bans files, the environment, sockets, clocks, processes, and dynamic JSON. Clippy ignores a ban it cannot resolve, so the dynamic JSON bans have never fired.

## Scope

- `Probability`: a finite number from zero to one. The constructor returns a result.
- `PassMark`: above 0.5 and at most 1. The constructor returns a result.
- `Condition`: non-empty text for a yes/no question.
- `Question`, `Answer`, `Policy`, `AssessmentStatus`, and `Assessment` as the specification describes them. Version one of each enum holds only what `decide if` needs. No variant arrives for a later verb.
- One total function assesses an answer under a policy. It follows the three rules in `result.md` exactly, with a no accepted when one minus the probability is at or above the mark.
- `DecisionResult` with `Meta` and `Usage`, serialized with `serde` to the exact shape in `result.md`. The schema string is a constant. Field order follows the document.
- Typed error enums with `thiserror`. No string errors.
- Dependencies added: `serde` with derive, `serde_json`, `thiserror`, and `proptest` for development. Update the accepted dependency sets in `policy.py` in the same commit.
- Prove the dynamic JSON bans fire now that `serde_json` resolves: plant a `serde_json::Value` access in the core, watch `lint` refuse it, remove it, and say so in the record.

Excluded: the plan, adapters, the binary, asymmetric marks, and every other verb.

## Acceptance

- Table tests cover the pass mark rules, including 0.9 and 0.1 at `--min-prob 0.9`, the unsure middle, and no policy.
- Property tests: every probability is assessed to exactly one status. An accepted yes and an accepted no never co-occur. A mark of `P` and a probability `p` give the same status as `1 - p` with the value flipped.
- A test serializes a result and compares it by value with the JSON example in `result.md`, copied into a test fixture string.
- Constructors refuse NaN, infinities, negatives, values above one, a mark of 0.5, and an empty condition.
- The whole ladder is green. The ratchet commit message says what grew and where duplication was sought.
- A second agent reviews the dependency additions and the public surface, and the record names what it checked.
