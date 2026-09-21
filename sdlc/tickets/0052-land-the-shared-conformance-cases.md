---
flow: build
priority: 37
opens: conformance crates/thinkthen-core/tests sdlc/planning
---

# 0052: Land the shared conformance cases

Status: proposed

## Outcome

One language-neutral file fixes the cases that the command and every future library and database surface must answer alike. An offline test validates the file through the production question grammar, digest, wire adapter, and answer rules. No runner reaches a backend, reads a key, or invents a second product grammar.

## Current facts and decisions

Experiments 205 and 207 produced one twenty-case file and an offline validator. The useful evidence is real, but the file is not ready to become the contract. It names paths outside this repository, marks five exchanges as shaped rather than captured, uses `details`, `usage`, `cancel`, and `decide_many` as verbs, omits `rank` and `find`, tests only two decisions inside `annotate`, and has no case for `local`, `deadline`, or `defect` errors. Its Python validator copies product grammar and digest logic that already live in Rust.

Product recording entries intentionally store only successful decoded exchanges. A backend refusal cannot pass through `--replay`, and weakening that rule would make the conformance fixture less trustworthy.

This ticket makes these decisions. Ian can overturn any of them.

1. `conformance/cases.json` is the single source of truth. It starts with the twenty experiment cases, replaces the annotate case with one mixed `decide`/`choose`/`score`/`tag` set, adds `rank` and `find`, and adds deterministic `local`, `deadline`, and `defect` faults. The file has twenty-five cases and covers all eight verbs and all six error kinds.
2. `verb` is always one of the eight product verbs. Bulk form, details, repeats for usage, cancellation, deadline, and fault injection live in explicit arguments. They are not extra verbs.
3. A successful case embeds the exact System One request and response needed to replay it. Each exchange says `captured` or `synthetic_contract`. Synthetic data must satisfy the same wire decoder and is never described as a live measurement. Cases carry no path into an experiment.
4. A failure case names one of the six error kinds and the deterministic fault the future runner injects. Backend, local, cancellation, deadline, and defect cases do not masquerade as product recording entries.
5. A Rust integration test in the pure core owns validation because the production parser, canonical digest, adapter, and answer rules already live there. It checks the file's schema, exact count, unique ids, eight-verb and six-error coverage, question and set grammar, request byte identity, reply shape, model consistency, expected bare answer and details, provenance, and absence of credential or header fields.
6. This ticket lands data and validates its meaning. It does not build a temporary command runner around private modules. The section 8 step-1 merge ticket adds the first command runner over its private engine boundary. Successful cases then use their embedded exchanges; faults use deterministic in-process injection. Every later surface reads the same file.
7. No experiment builder or duplicated Python grammar enters the repository. `conformance/README.md` documents the schema, provenance, runner rules, and the command that validates the file.

## Scope

Add `conformance/cases.json`, its README, and one pure-core integration test. Amend ADR 0017 section 7 and both active plans to state the successful-replay and injected-fault split and the runner timing. Update the test rung only if Cargo does not already discover the integration test.

Excluded: network or paid calls, a command runner, public engine functions, the one-crate move, bindings, database extensions, new dependencies, product recording changes, and generated experiment scripts.

## Acceptance

- The single file has schema `thinkthen.conformance/1`, twenty-five unique cases, all eight product verbs, and all six error kinds. `verb` contains no helper operation.
- The mixed annotate case exercises all four question types. Rank and find each have a successful case with the output shape ADR 0017 assigns them.
- Every success exchange is marked `captured` or `synthetic_contract`, passes the production adapter, carries no header or credential, and reproduces the expected bare answer and details.
- Every fault case names a deterministic future injection and one accepted error kind. None is written as a product recording entry.
- The validator uses production Rust types for question parsing, digests, request encoding, response decoding, and answer reading. It contains no second canonicalizer or probability formula.
- A focused mutation test proves that a wrong expected answer, wrong request, duplicate id, unknown verb, missing error kind, credential field, and malformed mixed question set each fail validation.
- ADR 0017 and both plans put the command runner in the merge ticket and keep Job 3 before that merge.
- The focused conformance test, all four repository rungs, and `git diff --check` pass with the key unset and no network request.

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
- Reasons: the fixture becomes a compatibility contract for ten future surfaces. It is local data, but the proof must cover every verb, every error kind, wire bytes, and hostile mutations without copying product semantics.
- Selected model: `gpt-5.6-sol` with medium reasoning.

Re-score if implementation needs a new product hook, public type, dependency, or network call.

## Review

Pending independent design review.
