# 0054: Preserve good answers when one question fails

Date: 2026-09-21

Status: landed

## Result

An otherwise valid multi-question backend reply now keeps every usable answer and marks each failed logical question. Bare `annotate` output uses `{"failed":{"kind":"backend","cause":CAUSE}}`. Detailed output carries the question, the same failure, and the request identity. It omits `value`, `answer`, and `threshold` for that failed entry. `null` still means not sure.

Detailed metadata always reports `failed_questions`. A partial run completes later groups and records, prints no separate diagnostic for the marked failures, and exits 6. A later whole-run failure retains its established exit code and stopping boundary. A reply with no valid logical answer remains a backend failure at exit 4. One bad wire member fails one logical `tag` question.

The adapter now requires the exact planned answer-name set before salvage. Malformed replies, invalid model identity, unexpected answer names, transport failures, and recording failures still fail the whole request. Raw recordings replay the same good answers, failure markers, count, and exit code without another request. Request bytes and recording identities did not change.

The core holds one ordered list of answered-or-failed outcomes. It has no second successful-answer list that could shift question positions. The shared conformance suite retains its ordinary successful mixed-annotate case and adds a separate partial case, for twenty-seven cases total. ADR 0007 assigns exit 6. ADR 0017 records the shared result shape. Ian can overturn the marker, cause list, metadata field, or exit code.

## Review and proof

Independent design review rejected the first proposal because it changed the approved detailed key, selected too low a review level, and did not prove that an unexpected answer name overrides salvage. The revised design fixed all three and was accepted.

Independent code review rejected the first implementation for three reasons. Its old `answers()` accessor silently removed failures and could shift question alignment. The conformance edit replaced the ordinary all-success annotate case. The compiled-output test checked fragments and did not pin all metadata or the complete failed-entry key set. The repair removed the ambiguous accessor, made every one-question caller match exactly one answered outcome, restored the successful case beside a new partial case, and added a structural JSON assertion for model, usage, failure count, aggregate and per-answer request identity, null, and the exact failed-entry keys. The same reviewer reran the focused proofs and accepted the repair.

The first hosted spec run found one integration gap: historical probe rows predated `meta.failed_questions`. The replay checker now sets that historical-only metadata aside symmetrically, as it already does for later metadata fields. The ticket's live-versus-replay integration test still pins the new count exactly.

After rebasing onto current main, all four repository rungs passed with the key and base-address variables unset. The lint rung reported the exact 25,319-line ceiling. The test rung passed the Rust suites, conformance validator, shell self-tests, transforms, local listener tests, and doctests. The spec rung reproduced every recorded probe and finished with 19 green demos and no red demo. `git diff --check` passed. No paid or external backend call ran.
