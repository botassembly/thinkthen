# 0612: Compute each call's answer in Rust and carry it in the final packet

Status: OPEN.

Milestone: 0.2.1

Phase: 1

Depends on: 0626

Reviews: revision 72fb2e5, accept

Reviews: revision 84e5458, accept

Reviews: revision 167571c, accept

## Outcome

- The final packet of every call carries the call's settled answer, computed in Rust: the bare value (scalar, collection or aggregate, following the caller's input shape), the probability, the pick or selected index, and the resolution state. Resolution separates an unresolved answer from a failed answer.
- The public Rust result types expose these fields.
- The Rust library reads these fields and does no projection of its own. The consumer's own projection code in `libraries/rust/consumer` is deleted.
- The result schema in `crates/thinkthen/src/public/results/complete.schema.json` describes the new fields, generated from the Rust types.
- Whether a call reports "yes" odds, "picked" odds or items becomes a property of the Rust types, recorded in the schema. It is no longer a table in Python.
- No template or language change is part of this ticket. The generator work is 0637; each language's adoption is its own phase 3 ticket.
- If review finds this ticket large, split it: rows and scalars first, then rank, recognize and relate items.

### Proof

- On the M5 through pm job, the Rust library consumer (`libraries/rust/consumer`) replays `conformance/cases.json` against the loopback backend. Each of the ten functions shows the expected value, probability, pick and resolution in the final packet, for single and collection inputs.
- Unresolved, failed and null answers stay distinct in the packet for the partial annotate case and for `19-find-none`.
- The schema generated from the Rust types matches the committed schema.

## Evidence

Rust sends rows and aggregates, and each language then works out the call's answer itself. The rules for that live in the code generator, not in Rust. `sdlc/generators/results/templates/host_answers.py:28-30` hard-codes that decide and filter report "yes" odds, choose and find report the "picked" odds, and rank, recognize and relate report items. Lines 41-63 of the same file derive the resolution, selection and pick paths. Each language template then carries its own engine that applies those rules to raw packets. Go does it in `libraries/go/owned_results_generated.go:4775` (`settle`). The Python, Ruby, TypeScript, Swift, PHP, Objective-C, C++, Ada, COBOL, Dart, C# and JVM outputs do the same, built from about 18 templates under `sdlc/generators/results/templates/`. Python, Ruby and TypeScript link Rust directly, yet they still compute the answer in the host language. When a call's answer rule changes, every template must change with it. The binding guide warns against exactly this (`libraries/BINDING-AUTHOR.md`: "Moving handwritten rules into per-language templates does not remove duplication").

The Rust library's outside consumer (`libraries/rust/consumer/src/main.rs`, `consumer/src/fixture.rs`) also repeats per-function projection that `Call::value()` and `Call::probability()` already provide.

0626 rewrites the same result schema and serializers, so this ticket follows it.

## Progress

- 2026-10-18 started
- 2026-10-19 next: Fresh review accepts72fb2e5c9 with native settlement, snapshot ownership and serializer fixes plus PM-authorized all-host declaration support; regeneration, native schema equality, C and native clippy, serialization contracts and the2012-item API inventory pass, and the full main closing gate remains after the accepted pairs.
