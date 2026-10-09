# 0514: Rewrite the binding author guide for thin, first-class bindings

Status: OPEN.

Milestone: 0.2

Reviews: revision 4cd756859, accept

Reviews: revision b0a33b1f61a98a129c2374c2058383c57e65f637, accept

## Outcome

`libraries/BINDING-AUTHOR.md` states the 2026-10-09 rule: Rust owns every rule, each language owns only its idiom. It lists the ten first-class items with each language's expected form, and the checks that enforce them.

## Evidence

- Starts from: [the 2026-10-09 binding decision](../decisions/2026-10-09-thin-first-class-bindings.md). The current guide keeps results as plain host values and bans typed results.
- Keeps: Packaging and naming guidance that still holds.
- Changes: Replace the result rule. Add a table per language for calls, inputs, typed results, absence, errors, async and cancellation, cleanup, editor support, install and thinness. Claim `libraries/BINDING-AUTHOR.md`.
- Proof: A fresh reviewer applies the guide to C# and Dart and names the same target shape the decision gives.
- Defers: None. It lands before 0516 starts.
