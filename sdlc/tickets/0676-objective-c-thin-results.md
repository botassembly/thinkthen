# 0676: Delete Objective-C value mapping and use the run call

Status: OPEN.

Milestone: 0.2.1

Phase: 3

Depends on: 0688

## Outcome

- Changes: `libraries/objective-c/Sources/Foundation/ThinkThen.m`, `libraries/objective-c/Sources/Foundation/ThinkThenFoundation.h`, `libraries/objective-c/Sources/Foundation/TTResult.h`, `libraries/objective-c/Sources/Foundation/TTResult.m`, `libraries/objective-c/Sources/Foundation/TTResultPrivate.h`, `libraries/objective-c/Sources/Foundation/TTResults.g.h`, `libraries/objective-c/Sources/Foundation/TTResults.g.m`, `build-tools/results/templates/objc.py`, `build-tools/results/templates/objc_calls.py`, `build-tools/results/answer_rules.py`, `conformance/library-surface.json`, `libraries/objective-c/checks/FoundationCases.m`, `libraries/objective-c/checks/FoundationConsumer.m`, `libraries/objective-c/checks/foundation_installed.py`, `libraries/objective-c/README.md`, `libraries/objective-c/README-PUBLIC.md`, `site/examples/functions/*/objective-c.m`.
- `ThinkThen.m` calls `thinkthen_run_call` for every named call.
- `TTCall` exposes the final answer, probability, pick and resolution from the canonical Call JSON that `thinkthen_call_result_json` returns, through generated accessors.
- The hand-written number conversion in the `objc.py` template and the `TTCallValue` and `TTCallProbability` projection that `objc_calls.py` generates are deleted.
- `TTSession`, the `TTTask` tick loop and their private closed, reader and producer error text are deleted. Async calls run the call on one dispatch queue worker and cancel through a `thinkthen_call_cancel` token.
- `build-tools/results/templates/objc.py` and `build-tools/results/templates/objc_calls.py` leave `LEGACY_FILES` in `build-tools/results/answer_rules.py`.
- The public surface change updates the `objc` entry in `conformance/library-surface.json` (`TTSession`, `TTTask`, `TTCall`) in the same change. Both READMEs and the ten site function samples change with the result accessors.

### Proof

- On the M5 through `pm job run --machine m5 0676 SHA --memory 8 -- sh libraries/objective-c/check.sh`, with `THINKTHEN_ARTIFACT` naming the Apple package built at the same revision by the release route (`release-pack` for the C archives, `libraries/swift/build-xcframework.sh`, then `release-pack --apple-wrappers aarch64-apple-darwin OUT objective-c`), the installed consumers run all ten named calls against a recorded backend, and `FoundationResultsConsumer` matches `conformance/fixtures/complete.json`, including absence, null, false and failure.
- `python3 build-tools/results/answer_rules.py` passes with the two Objective-C exceptions removed. Before the change it fails on those files.
- The Objective-C packet dumper and the 0642 oracle comparison belong to 0717.

## Evidence

- `libraries/objective-c/Sources/Foundation/ThinkThen.m` drives `thinkthen_session_new_with_surface`, `thinkthen_session_try_push`, `thinkthen_session_try_read`, `thinkthen_session_wait` and `thinkthen_session_result_view` through `TTSession` and `TTTask`. Its errors come from the per-thread `thinkthen_session_error_view`. 0638 landed `thinkthen_run_call` with independent result and error owners.
- `build-tools/results/templates/objc.py` converts decimal text to `NSDecimalNumber` by hand, and `objc_calls.py` generates `TTCallValue` and `TTCallProbability` from `host_answers.call_results`. Both templates are in `LEGACY_FILES` in `build-tools/results/answer_rules.py`.
- Objective-C maps numbers and answers by hand, and that mapping thrashed whenever the core result shape changed: five commits on decimal-range number conversion (6e7b37620, 4a22a18ea, 86b7857dc, 2d512ee33, a44dc2295), about eight on the filter answer accessor (e0c9359d3 through 0d16adfe8), and repeated reshaping of call value and probability (d8e403036, c32c2ad3f, 1d5c7285c).
- `checks/foundation_installed.py` needs the assembled Apple package with `CThinkThen.xcframework` and starts no native build.
