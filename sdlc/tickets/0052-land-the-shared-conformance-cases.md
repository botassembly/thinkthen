---
flow: build
priority: 37
opens: conformance crates/thinkthen-core/tests sdlc/planning
---

# 0052: Land the shared conformance cases

Status: proposed

## Outcome

One language-neutral file fixes the cases that the command and every future surface must answer alike. An offline Rust test validates present product semantics through the production grammar, digest, wire adapter, and answer rules. Future engine faults remain explicit fixture contracts until the merge ticket supplies their runner.

## Current facts and decisions

Experiments 205 and 207 produced twenty cases and an offline validator. The file carries useful evidence, but it names experiment paths, treats helper operations as verbs, omits `rank` and `find`, tests only decisions inside `annotate`, and copies product rules into Python. Five exchanges were shaped rather than captured. It has no `local`, `deadline`, or `defect` case.

Product recordings intentionally store only successful decoded exchanges. A backend refusal cannot pass through `--replay`. The fixture must preserve that rule.

This ticket makes these decisions. Ian can overturn any of them.

1. `conformance/cases.json` is the single case contract. The initial file has twenty-five cases: the twenty experiment cases with a mixed `annotate` case, plus `rank`, `find`, `local`, `deadline`, and `defect`. The schema does not freeze that number. Its declared count must equal its array length, and coverage must include all eight verbs and all six error kinds.
2. `verb` is always one of the eight verbs. Bulk form, details, repeated calls for usage, cancellation, deadlines, and fault injection live under explicit operation arguments.
3. The shared `rank` expectation is an ordered list of input indexes beside their yes probabilities. The shared `find` expectation is the selected input index or null beside every candidate probability in stable input order, with `none` last when present. A host maps indexes back to its own container. ADR 0017 records these meanings; the command keeps its current printed records.
4. A success exchange stores the request body as a JSON string, so the test compares its UTF-8 bytes with the production encoder. The response remains structured and must pass the production decoder.
5. `synthetic_contract` means the exchange came from a stub or was shaped to the accepted wire contract. `captured` requires a public, stable repository recording named by a relative path, and the embedded request and response must equal it. The validator recursively refuses headers and credential fields. No case points into an experiment.
6. A fault case names one accepted error kind and a deterministic future injection. This ticket validates the schema for `local`, `deadline`, and `defect`; the merge-ticket runner validates their behavior. Faults never masquerade as product recording entries.
7. A pure-core integration test validates the schema, unique ids, required coverage, question and set grammar, request bytes, replies, model agreement, expected bare answers and details, provenance, and privacy. It calls production Rust and contains no second canonicalizer or probability formula.
8. This ticket lands the fixture and validator. The section 8 step-1 merge ticket adds the command runner over its private engine boundary. Successes use embedded exchanges; faults use deterministic in-process injection. No experiment builder or duplicated Python grammar enters the repository.

## Scope

Add `conformance/cases.json`, its README, and one pure-core integration test. Amend ADR 0017 with the host-neutral `rank` and `find` meanings, successful-exchange and injected-fault split, and runner timing. Update both active plans. Change the test rung only if Cargo does not already discover the integration test.

Excluded: network or paid calls, a command runner, public engine functions, the one-crate move, bindings, database extensions, dependencies, product recording changes, and experiment builders.

## Acceptance

- The initial `thinkthen.conformance/1` file declares its actual length, has unique ids, and covers all eight verbs and all six error kinds without helper operations under `verb`.
- Mixed `annotate`, `rank`, and `find` carry the host-neutral expectations above.
- Every success request is a string whose bytes equal production encoding. Every response passes production decoding and reproduces the expected bare answer and details.
- Every exchange has valid provenance. `captured` content matches its public repository recording; synthetic content says so. No header or credential field occurs anywhere.
- Future `local`, `deadline`, and `defect` cases validate as schema contracts and are not claimed as executed behavior.
- Focused mutations prove rejection of a wrong expected answer, wrong request byte, duplicate id, unknown verb, missing error-kind coverage while count still matches, credential field, and malformed mixed set.
- ADR 0017 and both plans put the first command runner in the merge ticket and keep Job 3 before that merge.
- The focused test, all four repository rungs, and `git diff --check` pass with the key unset and no network request.

## Dependencies

Ticket 0051 and the experiment 205/207 conformance evidence.

## Complexity

- Contract: 2
- State and timing: 0
- Reach: 2
- Proof: 2
- Cost of error: 1
- Total: 7
- Minimum level floor: none
- Final level: 3
- Reasons: the fixture becomes a compatibility contract for ten surfaces. The proof covers every verb and error kind, exact request bytes, and hostile mutations without copying product semantics.
- Selected model: `gpt-5.6-sol` with medium reasoning.

Re-score if implementation needs a product hook, public type, dependency, or network call.

## Review

Independent design review rejected the first proposal because `rank` and `find` lacked host-neutral meanings, parsed JSON could not prove request bytes, provenance and future-fault limits were vague, and the case count looked permanent. This rewrite fixes those boundaries and keeps the runner in the merge ticket.
