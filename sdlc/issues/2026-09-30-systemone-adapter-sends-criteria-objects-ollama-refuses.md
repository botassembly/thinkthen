# The systemone adapter sends criteria descriptions as JSON objects that Ollama refuses

Status: open. Found 2026-09-30 in workspace experiment 415, the same family as `closed/2026-09-29-systemone-adapter-sends-null-criteria-liquid-d1-refuses.md`. Designed 2026-09-30 by ADR 0115; ticket 0339 landed the workaround on 2026-09-30: `Descriptions::Text` in `crates/thinkthen/src/core/plan.rs`, rendered by `sent` in `crates/thinkthen/src/core/adapters/systemone/request.rs`, and chosen by the `ollama` row in `crates/thinkthen/src/core/adapters/systemone/backends.rs`. Owner: upstream (Ollama), reported at https://github.com/ollama/ollama/issues/18718; ticket 0339 for the workaround; then a ticket deletes it.

Kind: debt

Pay when: Ollama accepts object-valued criteria descriptions. Watch the upstream issue.

Debt: 014

Severity: medium

Keeping it means Ollama users lose each description's `not_for` and `examples` detail. Ian ruled on 2026-09-30 that the Ollama rendering is a temporary Ollama-only workaround (ADR 0115). When Ollama is fixed, delete it and send descriptions as authored.

## The problem

thinkthen serializes a criteria description as whatever the question gave it: a string, an object (`{"what": ..., "not_for": ..., "examples": [...]}`), an empty object, or null. TypeSafe's hosted System One accepts every form. Two other backends accept less:

- Liquid d1 refuses null descriptions on a noul question (422, `questions.q1.criteria.false`; the earlier issue).
- Ollama 0.35 (nimble, tev1) refuses any object-valued description with 400, `score criteria must be an array of descriptions`. Null values pass. Verified 2026-09-30 against `http://localhost:11434/v1/systemone`: `[{},"good",{"what":"excellent"}]` fails; `["poor","good","excellent"]` passes; `{"Monday":null,"Tuesday":"..."}` passes.

`thinkthen check` therefore reports three criticals against Ollama (choice, score, mixed) although the bench's own request shapes, plain strings, all pass.

## Verified

The published OpenAPI 3.1.0 schema at `api.typesafe.ai/openapi.json` allows a criteria description of `string | object | array | null` on choice, `string | object | array` on score, and `string | object | array | null` on noul. A live `thinkthen check` against `api.typesafe.ai/v1` on 2026-09-30 (account with credit) passed every row, including the object-criteria probes. Ollama's refusal is a deviation from the published schema, not an untested corner. A record of both dialects sits in the awesome-thinkthen list at `apis/system1.md`.

## The fix candidate

Replaced by ADR 0115: flattening for every backend would drop `not_for` and `examples` that TypeSafe reads, so each backend names its own dialect instead. The original candidate, kept for the record: One serialization rule for every backend: a description renders as its plain string when it is one, renders the object's own text when the question supplies one form, and an absent or empty description is omitted entirely. No nulls and no objects on the wire. TypeSafe loses nothing it reads today. A fixture set covering string, object, empty, and absent descriptions on all three question types pins the behavior.

## Done when

Ticket 0339 lands the workaround and leaves this issue open as debt. It closes when Ollama accepts object-valued descriptions and a ticket deletes the workaround. Until then, the original condition below holds for the workaround.

`thinkthen check` against Ollama 0.35 serving nimble reports no critical finding, and against Liquid d1 the one-sided noul passes. The adapter's fixtures cover every description form on noul, choice, and score.

## Gaps ticket 0339 left open

Ticket 0339 deferred these with the workaround. Each waits for a user who meets it; none blocks 0.1.

- Nobody has checked whether Ollama accepts an object `state` (the check's mixed probe and a pointer selection) or structured question text. The loopback mimic assumes the first. The machine that built 0339 runs Ollama 0.15.5-rc1, which has no System One endpoint, so its manual check stopped at `critical endpoint` (404).
- Ollama's hosted service is not in the key guard until it serves decision models.
- A re-pulled Ollama tag such as `nimble` keeps its cached answers until `--refresh-cache` or `--no-cache`.
- Bindings and SQL reach the `ollama` form only when ADR 0114 build slice 2 gives them a `backend` setting.
- `status` does not print the description form, and no backend has its own longer timeout.
