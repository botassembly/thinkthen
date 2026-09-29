---
flow: build
priority: 301
opens: sdlc/issues/2026-09-29-systemone-adapter-sends-null-criteria-liquid-d1-refuses.md
---

# 0301: Omit null descriptions from System One noul criteria

Status: Proposed for fresh design review. This ticket changes the **wire** treatment of an explicitly null yes/no description; it does not change the accepted question-file grammar or create another backend adapter. The [preparation](../records/0301-noul-criteria-preparation.md) distinguishes the reported hosted refusal from local proof.

## Outcome and boundary

For a `noul` question, send only criteria sides with a non-null description, in `true`, then `false` order. Omit `criteria` entirely if neither side has a non-null description. Keep `Question::Decide`'s absent versus explicitly null values and its canonical question digest unchanged. This covers `decide` from typed or file input, its `filter`/`rank` and grouped consumers, and the fixed `check` probe through the one shared encoder. Structured string/object/list descriptions stay byte-for-byte as held. Keep `choice`'s null-valued option keys and `score`'s empty-object representation of null levels. Do not change response decoding, result thresholds, the System One URL, retry policy or public APIs.

This deliberately revises [backends.md](../../specification/backends.md)'s current promise that a present `noul` null travels as `criteria.true` or `criteria.false`. [question-file.md](../../specification/question-file.md) still accepts and distinguishes explicit null from an absent input key; clarify that wire omission does not collapse that input or its digest. The report from experiment 413 supports a specific compatibility correction, but local tests cannot prove that a hosted backend accepts the revised bytes.

## Small implementation and proof

- In `crates/thinkthen/src/core/adapters/systemone/request.rs`, use one local noul-criteria construction rule for `Decide` and expanded `Tag`: filter only `Json::Null` descriptions before deciding whether to emit `criteria`. Keep `Criteria` for `Choice` and the `Score` array untouched. No parser or generic serializer framework.
- Update exact assertions in `request.rs` and `crates/thinkthen/tests/question_file/structured.rs`: a true-only structured description with explicit false null emits just `true`; its false-only mirror emits just `false`; both absent and both explicitly null emit no `criteria`; both non-null preserve order and bytes. Reuse the existing choice and score checks. One of these cases must assert the complete expected request bytes rather than merely look for a missing substring.
- Update `specification/backends.md`, `specification/check.md`, `specification/fixtures/check/requests.jsonl` row 1, and `spec/decide.md`'s exact dry-run assertion. `spec/check.md` already compares the four emitted bodies to that fixture; keep the comparison. Extend `crates/thinkthen/tests/backend/check.rs` using its existing `Listener::serving` and `Listener::requests()` route to assert the live `noul` body and exactly four sends at the named address. This proves transport bytes, not provider acceptance. Review the existing request and fixture assertions before adding any duplicate test.
- Measure source growth in `sdlc/ratchet.json`. `request.rs` is 447 nonblank lines at this source pin, under the 500-line cap. The current codex-2 claim owns settings/parser and preview files, not this adapter file; coordinate any exact specification-page overlap before implementation. No shared conformance case currently embeds a `noul` request with a null criterion, so do not rewrite all 54 goldens or invent new response expectations. Run focused serializer, structured-question, check and affected spec proofs, then format, policy, pages, tickets and diff checks under offline Cargo, two jobs and the codex-7 lane lock.

## Request identity and remaining acceptance

`meta.question_sha256` and set question digests remain stable because the parsed question and canonical form remain unchanged. Any request that formerly included a `noul` null criterion gets different exact body bytes, therefore a different `meta.requests`/cache/recording digest. Existing recording files remain readable and valid for their old request; a new executable's strict replay of that logical question can miss the old entry. Do not rename or rewrite a captured provider exchange, add a legacy cache fallback, or relabel an old measurement as a new request. No-criteria, both-described, choice and score requests retain their identities.

After local review, the issue's hosted done condition still needs one separately authorized bounded `thinkthen check` against Liquid `d1:free`, with its four request bodies/statuses and finding count recorded safely. Do not infer acceptance from the loopback backend or repeat the 505-question experiment. A TypeSafe compatibility claim for the revised bytes also needs direct evidence if it is to be asserted. No provider call belongs to this preparation or an unapproved build.

## Evidence

- Starts from: [The open issue](../issues/2026-09-29-systemone-adapter-sends-null-criteria-liquid-d1-refuses.md), experiment 413's retained result, the current serializer and the settled `noul` wire sentence in `specification/backends.md`.
- Keeps: The question-file null input and canonical digest, non-null descriptions, no-criteria and both-described bytes, choice/score null rules, strict replay and existing recordings.
- Changes: Explicit null `noul` descriptions no longer serialize as null wire members; the fixed `check` noul body and matching exact-byte documentation change with them.
- Proof: A small serializer edge table, the compiled structured-file route, exact check fixture and loopback body/count, unchanged choice/score checks and the affected documentation comparisons.
- Defers: Fresh design/code review; one authorized hosted Liquid check for the issue's done condition, any TypeSafe acceptance claim for revised bytes, and unrelated model-quality measurements.

## What the build taught us

Complete after implementation. Record exact changed request identities, retained recordings, focused checks and any mismatch between the saved experiment report and a later hosted result.
