# 0496 — adopt-request-python

Status: OPEN.

Milestone: 0.2

## Outcome

Adopt shared Request and generated results in Python, pandas and Python Polars through named typed public calls.

## Evidence

- Starts from: PM architecture asks2/5 requires one ticket per direct binding; current Python, pandas and Python Polars adapter repeats admission/result construction.
- Keeps: native engine handles, mapping access, dataframe row identity; all ten functions, input/result/error and cache/replay behavior.
- Changes: Depends on0491, 0502 and the reviewed generation decision. Translate host arguments into Request, decode generated types, and remove the old copy after parity. Claim `libraries/python/**` and its installed typed consumer cases. Preserve absent versus null and tolerate permitted unknown result fields.
- Proof: Full shared cases execute through the installed host's typed interface, including files/images where supported, context/options, original positions, facts, failures, invalid-input zero sends and cancellation. Raw JSON pass-through is insufficient.
- Defers: Proxy and changing platform rulings without evidence. Size: large surface migration.
