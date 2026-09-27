---
flow: build
priority: 207
opens: sdlc/tickets/0207-sqlite-recognize-relations.md
---

# 0207: Give SQLite the full recognize relation result

Status: proposed for fresh design review; no product API is authorized by ticket 0206. Owner: Codex. Ian can overturn the SQL spelling after reviewing its query shape.

## Outcome

An SQLite caller must be able to ask one recognize question with relation rules over the original text and receive the complete entity and relation result. Keep the existing entity-only `thinkthen_recognize` table and row-based `thinkthen_relate` behavior. The new route must use the same public recognize engine call and request bodies as the settled `specification/recognize.md`, including offsets, strengths, full source and target entities, relation thresholds, and no-edge results. Do not emulate recognition with two unrelated model judgments.

## Evidence

- Starts from: Ticket 0206's bounded case-42 comparison and `sdlc/records/2026-09-27-shared-case-gap-audit.md`. `databases/sqlite/src/tables.rs::Recognizer::ask` explicitly rejects relation specs. The standalone `Relater` reads only row ID, name, and kind. A generic-arm run over case-42 entities produced two relation requests whose states had `entities` and `relation` but no original text; neither body matched the one captured case-42 relation request. Its two 0.9 generic edges were not case-42 answers. A direct case-arm attempt with a stripped recognize spec failed after three requests with backend 500; it did not prove an equivalent composition.
- Keeps: The existing two table-valued functions, their input and error behavior, all nine named relation skips until their shared arms pass, and the 54-case corpus as the only expected-answer source.
- Changes: Propose one JSON-valued SQL form, tentatively `thinkthen_recognize_document(body, spec)`, that calls `Engine::recognize_with` once with the full relation-bearing spec and returns its complete `{"entities":...,"relations":...}` value. An alternative table-valued result is acceptable only if it preserves that one-call request and complete output without forcing a second model judgment. Fresh design review settles the name, NULL behavior, and projection shape before source claims.
- Proof: Run shared cases `42` through `50` through the installed extension against their captured arms. Compare exact request bodies or digests, request counts, complete entity spans and strengths, directed edges and probabilities, threshold and no-edge variants, then remove only passing skips. Add one focused malformed-spec and no-send refusal at the SQL boundary if its shape introduces a distinct error route. Keep the full-functional route separate from stress.
- Defers: SQL find in all three databases to its existing issue, question-file forms to the shared proof issue, and batching B13e. No runtime implementation or broad gate belongs in ticket 0206.

## Build after design acceptance

Claim only the reviewed SQLite source, export contract, README, runner and check files, plus the relevant proof record. Preflight the fixed extension symbol count, 500-nonblank-line ceiling, ratchet, tool versions and existing optional-feature matrix. Reuse `worker::run`, `Recognize::from_json`, the captured relation arms and existing loopback backend. Preserve an explicit `not run` for all nine relation cases until the new SQL path passes each one. Run focused format, policy and relation checks, then full integration only at the related-ticket batch checkpoint. Require a fresh code review before landing.

## Source preflight for the next design review

Read at main `30bf6e81`. `databases/sqlite/src/tables.rs::Recognizer::ask` refuses `/recognize/relations`; `Relater::ask` constructs a separate relate spec from rows. The case-42 comparison in `sdlc/records/0206-prove-equivalent-conformance-paths.md` saw two relate request bodies without original text and no matching captured body. This is an absent SQL operation, not merely an untested equivalent query. `crates/thinkthen/src/public/recognize.rs` already exposes `Engine::recognize_with` and `Recognized::to_json`; the new SQL route can use that public result without inventing another recognition planner. `databases/sqlite/src/scalars.rs::register` registers scalar functions as direct-only and volatile. `databases/sqlite/check.sh` checks one `sqlite3_thinkthen_init` export. Preserve that ABI and the existing two table routes.

Before implementation, amend this ticket after an independent design review to settle the public SQL name and exact argument forms, including full recognize JSON and `@file` access under SQLite's authority. State body NULL, spec NULL, wrong SQL type and malformed spec outcomes, the JSON projection, and whether a spec without relations is accepted. Identify the exact source, registration, README, runner, conformance-skip and check files to claim. Keep one *public recognize call* distinct from one backend send: recognition may issue multiple requests, so compare each captured arm's body sequence and count rather than asserting one send. Keep all nine skips until cases 42–50 pass their own arms, including no-edge and threshold outcomes. A malformed spec needs a listener-backed zero-send refusal. The selected runner and `databases/sqlite/tests/conformance.py` provide the smallest outside-in proof after a rebuilt pinned SQLite 3.50.0 extension. No product API choice is made by this preflight.
