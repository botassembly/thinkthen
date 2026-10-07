# 0441: Ship the OpenAI Decisions text backend in 0.2

Status: in progress. The native OpenAI text adapter, saved reply fixtures and cache identity corrections are on main with passing integration checks. Cross-surface adoption and the complete parity run remain with the binding and SQL owners.

Milestone: 0.2

Owner: builder; lane2, `ticket/0441-openai-decisions-backend`. Root owns paid calls, review and landing.
Ticket review: fresh whole-ticket review at 56b91f4f5 found only the qN identity gap. Other design/probe checks are reused; the original reviewer confirms this correction narrowly under root ownership.

## Outcome

OpenAI text judgments work through the existing named-backend door on all ten functions and supported SDK/SQL surfaces before 0.2 releases. One pure second adapter shares the existing engine, scheduler, readers, results and storage. Images require separately admitted route limits; text support cannot wait for image measurements.

## Evidence

- Starts from: The admitted saved OpenAI Decisions guide and official create reference fetched on 2026-10-06. PM messages `2026-10-06-pm-openai-decisions-api-text-support-is-must-land-for-0-2-plus-a-vision-status.md` and `2026-10-06-pm-openai-key-is-available-and-drop-the-cluster-idea.md` supersede the preview/point-release deferral. No calls or reply observations have been made by this design slice.
- Keeps: Released System One bytes, adapter name, text identities and offline fixtures; existing bare/generic compatibility doors, backend precedence, six error kinds, partial logical failures, secrecy, cancellation, count-only usage and zero-send replay. Preserve all previous seventeen asks, 0456 catalog completion and native lane0 ownership.
- Changes: [ADR 0122](../planning/adr/0122-openai-decisions-pure-adapter.md) amends ADR 0010/0017; [backends contract](../../specification/backends.md#openai-decisions-target-for-02) fixes selection, encoding and decoding. Add the `openai` built-in using `/v1/decisions`, `OPENAI_API_KEY`, and `gpt-6-luna`. Adopt settled 0442–0444 metadata/cache contracts without a second storage design. Update named-backend conformance, supported binding/SQL selection cases, the pure-adapter vendor-word policy boundary and the backend install page at implementation.
- Proof: Root first runs the [bounded probe](../planning/0441-first-probe.md); save actual bodies for adapter fixtures. Outside-in CLI/API and loopback cases pin all ten functions, expanded tag/find/recognize/relate, rich authored descriptions, false values, answer ordering/names/types, totals, missing/duplicate/extra answers, per-question refusals, model/usage, safe error paths, fixed-route retries, cancellation, request-byte/profile bounds, cross-adapter misses, record/replay and zero-send invalid input. Pin unchanged B at q2 after A versus q1 alone: equal per-question key, retained cache/replay observation ID and answer ID for unchanged logical scope/reading, raw original names kept, and only missing pieces sent. Retain existing parser/secrecy/cache/conflict regressions and released System One fixtures. Shared conformance covers named selection on every supported family and SQL surface. No paid gate or generated reply evidence.
- Defers: OpenAI image execution until admitted route bounds and image costing; unknown vendor question/choice maxima, global determinism, performance/accuracy ranking, proxy routing and 0.3 close-call choose behavior. No cluster function or recipe, new harness, receipt tooling, numerical source-line requirement or large benchmark.

## Order and ownership

Implement after 0442–0444 settle and land in the native integration lane. Design only in lane2 now; edit no core, source, result or cache files and merge no active native WIP. Root coordinates the adapter seam with lane0 before implementation. Keep one 0441 ticket: schema selection, adapter, saved exchanges, conformance and docs form one ownable outcome. File caps/ratchet apply to measured implementation, not PM's unreviewed line/fixture estimates. No new dependency is proposed.

The first probe diagnoses the contract, not delivery. Root reports observed counts and actual usage-priced cost, changes this same ticket for supported limits/tolerance if needed, obtains the original reviewer’s narrow confirmation and then implements. Access failure remains a 0.2 blocker, not permission to omit the backend. Root runs full tests/lint on the landing commit and applicable spec/surface gates, writes one short record at landing, and preserves release rehearsal and Ian's publication approval.

## Request-local correlation clarification

OpenAI assigns qN only after existing per-question lookup and missing-piece packing. Validate exact live sent names/order before projecting accepted answers. Persistent adapter-encoded questions and answers omit only their top-level correlation `name`; semantic fields and authored names remain intact. Stored answers validate against the name-free question. Transport position changes neither per-question key nor a reused observation/answer identity for unchanged state, scope and reading. Fresh paid observations still receive new IDs. Raw recorded request/reply bodies retain original names/order and validate against their own request before projection. [The cache contract](../../specification/cache.md#version-2-identity) specifies this pure adapter normalization; no store machinery or System One/v1 identity changes follow. Root is running the unchanged accepted probe; no new probe/check scope is added here.

## OpenAI images

The reference admits user-message input_text/input_image parts, inline base64 data URLs and at most 128 images per request. It does not settle total decoded/request/context limits or image token accounting for this SDK. Keep ADR 0121's JPEG/PNG validation, eight-image SDK bound and ordered original bytes. Refuse OpenAI images before lookup/send until route admission is complete; do not copy System One image wire fields, infer support from a hostname, resize silently or drop attachments. No image call belongs to this text probe. Other admitted vision routes remain owned by 0447/0448/0452.

## Retained choose and catalog decisions

Current choose abstains below the caller's winning-probability cut or at an exact top tie. The System One published confidence/top-probability relation for fixed option count remains documented; it supplies no OpenAI confidence formula or near-tie rule. A close-call none option stays a 0.3 idea. Once categories have written definitions, choose/tag already assign them; Ian dropped cluster on 2026-10-06. Existing0407/0414 and 0456 named questions/input declarations/catalog completion remain required, with no competing catalog implementation here.
