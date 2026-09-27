# Calibration profile preparation

Read-only preparation at `fc587946` on 2026-09-27, verified by the coordinator from local experiment 2029. Accepted SQL settings tickets 0149/0157 are pinned at `2999814e0013926c2e14bd7b3927374c4b2c4775`. This note corrects build assumptions without changing 0203's outcome. Independent review is pending.

## Current behavior and the missing paths

- Runtime profile selection already exists. `public/settings.rs::EngineBuilder::profile` saves the path; `build` parses it and sets `Settings.profile`. The old design snapshot predates landed 0148. Saved calibration identity remains missing: `public/question.rs::Question::from_json` has no profile field, and `public/results.rs::Details::of` supplies `None` to both `Run.tuned_for` and `question_sha256_with_profile`.
- Adding that field also touches `public/builders.rs`: label `finish` and score `build` construct `Question` directly. Initialize programmatic questions to no saved profile, as the accepted design requires. The original opens list missed this file. Search all `Question`/`Self` literals and conversions together before editing; wrappers that retain a whole question differ from constructors that name its fields.
- `public/set.rs::QuestionSetBuilder::build` passes member name, core question and threshold into `core/question_set.rs::from_parts`, which creates an unprofiled set. Preserve the accepted member refusal and top-level set rule. Python frame shortcuts and R's one-by-one fallback need their separate accepted proofs.
- Python `Engine::ask` in `libraries/python/src/engine.rs` already returns `Details::to_json` for details. A fix to the shared writer may need only Python test changes there. Inspect this route before claiming another product file. Ruby `question_text`, R `.tt_text`, typed label conversions and question-file parsing remain the exact routes named by the accepted design.
- The plural `PreparedRequests::with_profile` takes a fourth request ceiling after 0154. Singular `PreparedRequest::with_profile` still takes three arguments. Preserve both limits and profile checks; do not copy one signature to the other. Active 0167 is changing this module and active 0171 is changing `Run`, so refresh these narrow facts before implementation.

## Scope and useful proof

At the snapshot, nonblank source counts are `public/question.rs` 474, `core/question_set.rs` 469, R `calls.rs` 464, `public/results.rs` 455 and Ruby `thinkthen.rb` 428. Plan coherent reuse or extraction where the intended change needs it. Do not scatter code merely to fit a cap. Measure actual growth for review.

The partial digest values in the original report are prose observations. They are not the complete independent expected value required by 0203. Keep the accepted single shared fixture with pinned complete digest and exact optional warning pair. Existing profile-limit and warning-occurrence tests protect different properties; do not replace them with duplicated assertions or derive the expected digest from the same function under test. Follow the existing per-surface adapters and count actual loopback sends for refusals. Use focused functional checks during the build and retain valid evidence for the related-ticket integration checkpoint under current AGENTS.

Full implementation still waits for 0149. Current main includes 0148, 0154, 0155 and 0170; pending accepted settings are not landed code. Refresh DuckDB adapters after 0201/0149 and result constructors after 0171. The broader library batching work stays outside 0203.

## What this preparation caught and missed

The helper identified stale settings prerequisites and useful file headroom. Coordinator verification caught its inaccurate combined request-signature description and its claim that all builder routes held questions opaquely. A literal search found the missing label and score initializers. Carry only verified facts into the build brief. Evaluate this preparation again against the completed build; no runtime speedup or product completion is claimed.
