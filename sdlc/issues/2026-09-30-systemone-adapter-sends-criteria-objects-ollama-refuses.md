Status: Open. Found 2026-09-30 in experiment 415 (`~/workspace/experiments/415-ollama-nimble-check/RESULTS.md`), the same family as `2026-09-29-systemone-adapter-sends-null-criteria-liquid-d1-refuses.md`. Merge consideration applies.

# The systemone adapter sends criteria descriptions as JSON objects that Ollama refuses

## The problem

thinkthen serializes a criteria description as whatever the question gave it: a string, an object (`{"what": ..., "not_for": ..., "examples": [...]}`), an empty object, or null. TypeSafe's hosted System One accepts every form. Two other backends accept less:

- Liquid d1 refuses null descriptions on a noul question (422, `questions.q1.criteria.false`; the earlier issue).
- Ollama 0.35 (nimble, tev1) refuses any object-valued description with 400, `score criteria must be an array of descriptions`. Null values pass. Verified 2026-09-30 against `http://localhost:11434/v1/systemone`: `[{},"good",{"what":"excellent"}]` fails; `["poor","good","excellent"]` passes; `{"Monday":null,"Tuesday":"..."}` passes.

`thinkthen check` therefore reports three criticals against Ollama (choice, score, mixed) although the bench's own request shapes, plain strings, all pass.

## The fix candidate

One serialization rule for every backend: a description renders as its plain string when it is one, renders the object's own text when the question supplies one form, and an absent or empty description is omitted entirely. No nulls and no objects on the wire. TypeSafe loses nothing it reads today. A fixture set covering string, object, empty, and absent descriptions on all three question types pins the behavior.

## Done when

`thinkthen check` against Ollama 0.35 serving nimble reports no critical finding, and against Liquid d1 the one-sided noul passes. The adapter's fixtures cover every description form on noul, choice, and score.
