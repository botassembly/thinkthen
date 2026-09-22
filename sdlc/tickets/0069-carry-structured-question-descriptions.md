---
flow: build
priority: 69
opens: crates/thinkthen/src/core crates/thinkthen/src/cli crates/thinkthen/tests specification spec sdlc/scripts sdlc/ratchet.json sdlc/planning
---

# 0069: Carry structured question descriptions

Status: accepted

## Outcome and authority

Question files and annotate entries carry structured descriptions through the existing Rust parser, canonical digest, request encoder, and result path. Ian authorized bounded tickets within the engine plan and explicitly settled levels, tag requests, and types in rulings 10–12 of `../issues/2026-09-21-the-question-file-cannot-carry-typesafes-structured-fields.md` at `90a544f`. This ticket follows landed 0065/0068 and precedes private controls and recognition. ADR 0037 amends the settled question contract. Typed library builders stay with the library team.

## Design

Reuse the ordered `core::json::Json` tree behind validated description types; never add a second JSON parser or weaken core policy. Question text accepts nonblank strings, objects, or arrays. Criteria descriptions accept strings, objects, arrays, or null; top-level numbers/booleans fail locally with exit 5, but nested finite numbers/booleans are valid. Empty objects/arrays are values, not absence. Preserve existing string validation and CLI precedence. Explicit null true/false is a present null criterion, distinct from an absent key; null or blank option/tag descriptions remain no description.

Score retains its string-list form and adds the existing ordered name-to-description map form. Names retain current count, uniqueness, blank/control validation and key result probabilities and `answer.level`. The model receives map values in order, including null; never substitute the name for a null map value. Map string descriptions must be nonblank, while the existing printable-name checks apply to names. Objects/arrays inside a levels list fail with a diagnostic naming the map form. Preserve legacy list canonical bytes; canonical map levels contain ordered names and descriptions, ensuring description changes change the question digest. Result `question.levels` still lists names. Bare score values and highest-probability level selection do not change.

Tag keeps today's exact request when question text is a string and every normalized description is a string or absent. If the question or any description is structured, every expanded tag instruction is `[QUESTION,{"label":NAME,"description":DESCRIPTION}]`, in that key order, with description omitted when absent. Preserve the existing true-criterion mapping as well; structured descriptions pass through unchanged there. No JSON is interpolated into a sentence. One structured description switches the whole logical tag question, including sibling labels. Annotate uses the same expansion.

Use the same compact serializer for descriptions in wire requests and canonical questions, retaining object key order and existing finite-number normalization. Formatting alone does not change digests; changed structured contents or key order do. Existing string-only request and canonical fixtures must remain exact. Profile request-byte limits continue to count the complete encoded body, including JSON punctuation, keys, and escaping. No token estimate enters validation.

Publish `specification/question-file.schema.json` with the structural question-file grammar and reusable entry definitions. `--dry-run` continues to use the production parser and complete semantic validation, not a second runtime schema interpreter. Document semantic checks beyond JSON Schema, CLI replacement of optional lists, and duplicate-member rejection. A gate-run schema parity test uses Python's installed `jsonschema` (4.10.3 observed; Draft 2020-12 support required) against a shared valid/invalid structural corpus also exercised by the production parser/dry-run. Add an explicit install-rung prerequisite check for this test-only tool, with no runtime/Cargo dependency. Never fetch schemas in gates. Recommend `{what, not_for, examples}` without requiring those keys.

## Scope and exclusions

Allowed: directly affected core question/text/description types, parser/resolver, wire encoder, digest and callers; necessary CLI conversions and help; focused core/integration/property tests; the schema and its local test/corpus wired into the test rung; relevant specification pages, executable question-file examples, ratchet and ticket/ADR/queue records. Record-carried `--options` keeps its existing string-only grammar; do not widen it incidentally. `find` stays string-only and keeps its generated request. No changes to answer calculations, engine scheduling/transport/cache/recording/usage, old recordings, probes, workflows, security policy tables, libraries, databases, site, shell object-building flags, paid calls, or accuracy claims. Preserve existing comments; do not add or remove source comments.

## Acceptance

- Observe focused red tests before implementing. Cover object/array instructions for every question kind, structured choice/tag descriptions, true/false boundaries including explicit null, named score descriptions, and mixed annotate entries. Check exact ordered wire and canonical JSON for new forms and unchanged historical string fixtures/digests.
- Pin score map names in detailed results and probability keys, criteria values in order, and unchanged numeric answers. Test CLI list/boundary overrides and structured question text with dynamic string-only options.
- Pin both tag branches, whole-question switching from one structured label, absent/null descriptions, and unchanged string-only request bytes.
- Refuse null question text, top-level number/bool slots, nonstring list levels with map-form advice, invalid names/counts, duplicate nested members, nonfinite nested numbers, and unknown/inapplicable keys. Errors and Debug must not echo description contents. Use existing counted-listener helpers to prove representative invalid descriptions and over-limit structured requests send zero requests on live paths, including annotate.
- Prove whitespace-equivalent structured files share a digest, changed descriptions/order change it, and question sets inherit the same rules. Test exact request-size boundary and one-byte overflow under dry-run and live preparation. Keep parser property coverage.
- Validate the published schema offline against its declared draft and a shared structural corpus, and execute dry-run examples. Explicitly document schema limitations rather than claiming full equivalence with semantic validation.
- Run policy, exact ratchet, formatting and focused tests before code review. Search for duplicate parsing/serialization before any ceiling increase and record why each increase earns its lines. Independent review checks the public grammar, compatibility, schema tool and size. Coordinator runs all four local gates sequentially after review, records results, then lands and pushes. Actions stays disabled.

## Complexity

Contract 1; State/timing 0; Reach 1; Proof 2; Cost of error 1; Total 5. Minimum floor: none. Final level: 2. Reasons: explicit product rulings, pure description conversion and one public grammar, deterministic wire/digest compatibility proof; no storage or cache invalidation change. Selected implementation: `swe2-implementer` (`swe-2-high`), Ian's approved bounded-worker routing. Independent design/code: separate `sol-reviewer` sessions (`gpt-5-6-sol-medium`). Stop and re-score if new public semantics, state changes, or safety-policy changes become necessary.

## Review

Independent design review: ACCEPT. The reviewer found the ticket bounded and source-feasible, accepted the level-2 SWE-2 implementation route, and highlighted score input-form provenance and exact legacy tag/request bytes as the main implementation risks already covered by acceptance. The read-only review ran no commands; code review and coordinator gates remain pending.
