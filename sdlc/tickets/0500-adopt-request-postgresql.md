# 0500: Move PostgreSQL onto the shared request contract

Status: OPEN.

Milestone: 0.2

Reviews: revision 95fade3863f777142ecd49d4abca0efcf1790cce, accept

## Outcome

Adopt shared Request and generated results in PostgreSQL through named typed public calls.

## Evidence

- Starts from: PM architecture asks2/5 requires one ticket per direct binding; current PostgreSQL adapter repeats admission/result construction.
- Keeps: privileged question-file loading and existing client evidence-reader workaround; all ten functions, input/result/error and cache/replay behavior.
- Changes: Depends on0491, 0502 and the reviewed generation decision. Translate host arguments into Request, decode generated types, and remove the old copy after parity. Claim `databases/postgresql/**` and its installed typed consumer cases. Preserve absent versus null and tolerate permitted unknown result fields.
- Proof: Full shared cases execute through the installed host's typed interface, including files/images where supported, context/options, original positions, facts, failures, invalid-input zero sends and cancellation. Raw JSON pass-through is insufficient.
- Defers: Proxy and changing platform rulings without evidence. Size: medium surface migration.

## Retained planning diagnostic

Keep `thinkthen_plan` on the existing shared native `Engine::plan_with` planner. This diagnostic sends no judgment request; preserve its admission, file authority and output, and prove zero sends. Do not replace planning with execution or duplicate estimation in PostgreSQL. The migration covers every executing public function; this explicit diagnostic ruling requires no additional Request planning API.

Migrated rank calls use the shared Request blank-input diagnostic, `the evidence is empty or blank`, in place of PostgreSQL's prior `evidence is text, not white space`. Preserve Usage classification, zero sends and secrecy. Record and test this intentional wording change; add no adapter-specific whitespace validator.

## Progress

- 2026-10-08 started
