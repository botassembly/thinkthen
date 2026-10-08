# 0494 — adopt-request-r

Status: OPEN.

Milestone: 0.2

## Outcome

Adopt shared Request and generated results in R through named typed public calls.

## Evidence

- Starts from: PM architecture asks2/5 requires one ticket per direct binding; current R adapter repeats admission/result construction.
- Keeps: R indexing and engine ownership; all ten functions, input/result/error and cache/replay behavior.
- Changes: Depends on0491, 0466 and the reviewed generation decision. Translate host arguments into Request, decode generated types, and remove the old copy after parity. Claim `libraries/r/**` and its installed typed consumer cases. Preserve absent versus null and tolerate permitted unknown result fields.
- Proof: Full shared cases execute through the installed host's typed interface, including files/images where supported, context/options, original positions, facts, failures, invalid-input zero sends and cancellation. Raw JSON pass-through is insufficient.
- Defers: Proxy and changing platform rulings without evidence. Size: medium surface migration.
