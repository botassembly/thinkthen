Status: Open. Filed 2026-09-29 from experiment 413's `RESULTS.md`. Ticket 0301 is implementing the correction accepted in [ADR 0110](../planning/adr/0110-noul-null-criteria-wire-omission.md).

# The systemone adapter sends null criteria that Liquid's d1 refuses

## The problem

The fixed `thinkthen check` probe supplies an explicit `"false":null` description. The System One encoder retains that value and sends `"criteria":{"true":"The text says the box was intact.","false":null}`. Experiment 413 reports that Liquid's d1 at `https://api.liquid.ai/decisions/v1/systemone` refuses this with status 422 and `param: questions.q1.criteria.false`. The original filing reports that TypeSafe accepts the body and that other question forms passed; separate raw check bodies and statuses were not retained for independent verification.

The original filing incorrectly said that a missing CLI side becomes null. Reviewed preparation at `46925b681` confirmed that an absent `--true` or `--false` side is already omitted. The affected case is an explicitly null description, including the fixed check probe. The [preparation record](../records/0301-noul-criteria-preparation.md) traces both paths and the existing exact R and TypeScript assertions.

## Accepted correction

ADR 0110 omits only explicitly null `noul` criteria members and omits the criteria object when neither member remains. It preserves input grammar, canonical question digests and the existing `choice` and `score` null rules. The changed request bytes get the exchange identity of those bytes; they can reuse an existing absent-side entry or miss an old null-bearing entry. Existing captured exchanges are not rewritten. Equivalent hosted answers are not established by this wire change or by a local fixture.

## Done when

`thinkthen check --url https://api.liquid.ai/decisions/v1 --model d1:free` reports no critical finding, and the adapter's fixtures cover explicit-null `decide` criteria. The local proof also retains absent-side, both-described, choice and score behavior, exact check bodies and plan totals. Hosted acceptance still needs its separately authorized bounded check; design acceptance and loopback proof do not close this issue.
