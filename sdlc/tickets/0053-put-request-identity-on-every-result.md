---
flow: build
priority: 37
opens: crates specification conformance sdlc/planning
---

# 0053: Put request identity on every result

Status: landed

## Outcome

Every detailed result carries `meta.requests`, an ordered list of the recording digests for the logical requests that produced it. A caller can join a saved result to its recordings whether one request or several made the result. Bare output and every request byte stay unchanged.

## Current facts and decisions

`DecisionResult` metadata carries the question digest but not the request digest. `annotate` carries one `request` digest inside each detailed named answer, but its aggregate metadata carries no list. The recording layer already computes the required digest from the adapter name, resolved URL, and exact request bytes.

Ian approved the revised build order and plural request list on 2026-09-21. The product-side reply adds five later requirements. None changes this ticket. Ian can overturn the decisions below.

1. Detailed results use `meta.requests`, always an array. A one-request result has one element. No singular `meta.request` field enters the contract.
2. Order is logical construction order: record, evidence group, then future chunk. Concurrent completion cannot change it. A retry adds no element. Separate logical requests with the same digest each retain their place.
3. The existing per-answer `request` field under detailed `annotate` answers stays. It names the exact request for that answer. `meta.requests` names the requests for the whole result.
4. `decide`, `choose`, `tag`, `score`, `filter`, `rank`, and `find` each make one logical request per detailed result today. `annotate` lists its evidence groups in question-set group order.
5. The common asking path creates one private prepared request after encoding. It holds the exact body and its production `recording::Digest`. Replay, cache locking, live send, recording, and the returned answer all use that identity. No path recomputes the digest, and `annotate` no longer encodes once for identity and again for sending.
6. `Meta::new` and `AnnotateMeta::new` are public today. Their constructor changes are part of this ticket and receive the required independent code review. This ticket adds no public engine verb.
7. A dated ADR 0017 amendment fixes the field, ordering, duplicate rule, retry rule, and retained per-answer field before the settled result specification changes.

## Shared conformance shape

The document gains one canonical `backend_url`, initially the built-in default. Every embedded exchange in this version uses that resolved URL. A later profile case may add an explicit case-level override through a separately reviewed schema change.

Every expected answer's `details` gains `requests`, an array of lowercase recording digests:

- `single`, `filter`, `rank`, and `find` answers carry the one digest for the exchange named by their existing `exchange` index.
- Every answer in one `annotate` result carries the complete ordered list of that result's exchange digests. Annotate exchange index `N` maps to question-set group `N`. A new synthetic multi-group annotate case stores two exchanges in that group order and pins the aggregate list. A command scheduling test releases the two responses in reverse completion order and proves that the serialized result still follows logical group order.
- The validator derives each value with `recording::Exchange::digest` from `backend_url` and the exact request string. It contains no copied adapter name or digest formula.

This duplicates a short list across the named answers of one annotate result. The duplication is intentional: each expected detailed answer remains independently usable by a future host runner.

## Scope

Add the ordered request list to `Meta` and `AnnotateMeta`. Add the private prepared-request flow and return its existing digest beside the reply. Thread it into single-question, `find`, and `annotate` results. Amend ADR 0017. Update the result specification, examples, shared conformance schema and validator, active plan, and accepted handoff response.

Excluded: partial-question failures, record-mode output changes, backend profiles, threshold warnings, cache defaults, the one-crate move, public engine verbs, `recognize`, `relate`, and C.

## Acceptance

- Every detailed result from all eight commands carries `meta.requests` as an array of recording digests.
- A one-request result carries exactly one digest. It equals the filename digest that `--record` or `--cache` uses for the same exchange.
- A multi-group `annotate` result lists digests in question-set group order even when replies finish in reverse order. Each named answer keeps its matching `request` field.
- Retries do not add list entries. Replay and live execution report the same digest. Equal logical requests retain separate list positions.
- One prepared request identity flows through replay, cache locking, live send, recording, and the result. Focused proof fails if any path derives a different digest or `annotate` encodes twice.
- Shared conformance expectations validate request identity through the production recording digest and the canonical URL. The multi-group case pins aggregate mapping and order.
- Exact output tests pin the new JSON field. The existing first-command behavior remains pinned: one text passed to `decide` without `--details` prints one bare Boolean.
- Request bodies, recording filenames, bare output, diagnostics, exit codes, ordering, and network behavior do not change.
- No dependency, public engine verb, network call, or paid call enters this ticket.
- The focused tests, all four repository rungs, and `git diff --check` pass with the key unset.

## Dependencies

Ticket 0052, the approved response in `sdlc/planning/build-team-response-to-handoff-2026-09-21.md`, and the product-side reply in `sdlc/issues/closed/2026-09-21-the-product-sides-reply-to-the-build-teams-response.md`.

## Complexity

- Contract: 2
- State and timing: 1
- Reach: 2
- Proof: 2
- Cost of error: 1
- Total: 8
- Minimum level floor: none
- Final level: 3
- Reasons: one field reaches every detailed result, two public constructors change, and annotate must preserve logical order under concurrent completion. The digest implementation already exists.
- Selected model: `gpt-5.6-sol` with medium reasoning.

Re-score if implementation changes a request, introduces another digest implementation, adds a public engine verb, or needs network evidence.

## Review

Independent design review rejected the first proposal because it omitted the required ADR amendment, left the conformance mapping and URL unspecified, excluded public constructor changes that the scope requires, and could not enforce its claim that the digest is computed once. This rewrite defines those boundaries and requires one prepared identity through every path.

Independent code review found one blocker: the source ceiling was three lines below the measured total inherited from the rebased starting point. The first final lint then found that the two expanded public constructors exceeded the six-argument rule and that the annotate scheduling test file exceeded its size limit. The repair groups replay status and ordered identities in one public `RequestMeta`, moves the two identity tests to their own module, and keeps both lint rules intact. The first final spec run found historical probe rows that predate request metadata. The replay check now documents and removes `meta.requests` from both historical and current rows while it compares every other result field. The same reviewer accepted all repairs and found the prepared-request flow, eight command paths, annotate ordering, conformance contract, and public metadata change coherent.
