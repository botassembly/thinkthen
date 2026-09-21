# 0053: Put request identity on every result

Date: 2026-09-21

Status: landed

## Result

Every detailed result now carries `meta.requests`, an ordered array of the recording digests for the logical requests that produced it. A one-request command reports one digest. `annotate` reports one digest per question group in question-set order, even when concurrent replies finish in another order. Equal logical requests retain duplicate positions. Each detailed annotate answer also keeps its existing singular `request` field.

One private prepared request now owns the encoded request bytes and their production recording digest. Replay, cache locking, live sending, recording, and result metadata share that identity. Annotate no longer encodes a request once for identity and again for sending. Bare output, request bytes, diagnostics, exit codes, retries, and network behavior did not change.

The shared conformance contract now contains twenty-six cases and a canonical backend URL. Its expected detailed answers use the production recording digest. The new two-group annotate case and command test pin logical order independently of completion order. ADR 0017 and the result, annotate, and recording specifications record the public contract. Ian can overturn the field name, ordering rule, duplicate rule, or retained per-answer field.

## Review and proof

Independent design review rejected the first proposal because it omitted the ADR amendment, underspecified the conformance URL and group mapping, overlooked public constructor changes, and could not enforce its single-identity claim. The rewritten ticket fixed those boundaries and was accepted.

Independent code review found one gate blocker: the source ceiling was three lines below the measured total inherited from the branch base. The first final lint then found two structural blockers: the expanded public constructors took seven arguments, and the annotate scheduling test file exceeded its 500-line limit. The repair groups replay status and ordered identities in one public `RequestMeta`, moves the two identity tests to their own module, and weakens no lint rule. The ceiling now equals the exact 24,717 lines. The same reviewer accepted both repairs and found no product defect. The review checked the prepared-request path, all eight commands, reverse annotate completion, duplicate logical requests, live and replay identity, exact request bytes, bare decide output, public constructors, specifications, ADR, and conformance schema.

The focused conformance tests, reverse-completion test, duplicate-position test, exact detailed-output test, and live/replay identity test passed with the key unset. The repository test rung passed 440 tests and two doctests. No network or paid call ran.
