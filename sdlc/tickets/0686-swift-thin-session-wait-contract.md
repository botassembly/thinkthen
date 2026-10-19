# 0686: Delete Swift call projection and session loop

Status: OPEN.

Milestone: 0.2.1

Phase: 3

Depends on: 0688, 0727

## Outcome

- Changes: `libraries/swift/Sources/ThinkThen/CallProjection.swift`, `libraries/swift/Sources/ThinkThen/OwnedSession.swift`, `libraries/swift/Sources/ThinkThen/CallResult.swift`, `libraries/swift/Sources/ThinkThen/InputsGenerated.swift`, `libraries/swift/Sources/ThinkThen/ResultsGenerated.swift`, `build-tools/results/templates/swift.py`, `build-tools/results/legacy/swift.txt`, `conformance/library-surface/swift.json`, `libraries/swift/README.md`, `libraries/swift/README-PUBLIC.md`, `libraries/swift/Tests/fixtures/owned_consumer.swift`, `libraries/swift/Tests/fixtures/smoke.swift`.
- Swift calls `thinkthen_run_call` and reads the final answer, probability, pick and resolution from the canonical Call JSON that `thinkthen_call_result_json` returns.
- `CallProjection.swift` is deleted, with the `decideResultScalar`-style projections that `swift.py` generates.
- The `Session` class in `OwnedSession.swift`, `Client.start`, the two `Client.execute` session drivers, `sessionCheck` and the private closed, reader and producer error text are deleted. One call runs `thinkthen_run_call` on a dispatch queue worker, and cancelling a Task fires a `thinkthen_call_cancel` token and returns promptly while the provider is held.
- `libraries/swift/Sources/ThinkThen/CallProjection.swift` and `build-tools/results/templates/swift.py` leave this language's exception file `build-tools/results/legacy/swift.txt`.
- The public surface change updates the Swift entry in `conformance/library-surface/swift.json` (`Session`, `Call`, `SessionBoundaryFailure`) in the same change. Both READMEs drop the `Session` paragraph. The ten site function samples keep their call shape and need no change.

### Proof

- On the M5 through `pm job run --machine m5 0686 SHA --memory 8 -- sh libraries/swift/check.sh`, with `THINKTHEN_ARTIFACT` naming the Apple package built at the same revision by the release route (`release-pack` for the C archives, `libraries/swift/build-xcframework.sh`, then `release-pack --apple-wrappers aarch64-apple-darwin OUT swift`), the installed Swift consumer runs all ten named calls against a recorded backend. A held-provider case shows other async work progressing and Task cancellation returning before the provider is released.
- `python3 build-tools/results/answer_rules.py` passes with the two Swift exceptions removed. Before the change it fails on those files.
- The Linux Swift run happens at the 0619 container checkpoint.

## Evidence

- After 0727 lands, this language's answer-rule exceptions live in `build-tools/results/legacy/swift.txt` and its surface entry in `conformance/library-surface/swift.json`. This ticket edits only those two files, so it does not collide with other languages' call tickets.
- `libraries/swift/Sources/ThinkThen/CallProjection.swift` matches rows by string keys such as "kind" and "aggregate", and `recognitionProbabilities` pairs kept proposals with spans. The same rule is hand-written in PHP and Ruby. The 0637 answer-rule check lists it and `build-tools/results/templates/swift.py` in `LEGACY_FILES` in `build-tools/results/answer_rules.py`.
- The generated calls in `InputsGenerated.swift` (for example `Client.decide`) call `execute(Request(...))` and then a generated projection such as `decideResultScalar()`.
- `Session.push` and `Session.read` in `OwnedSession.swift` run blocking `thinkthen_session_push` and `thinkthen_session_next_packet` on `DispatchQueue.global`, and `sessionCheck` reads the per-thread `thinkthen_session_error_message`. Whether cancellation always unblocks a waiting `next_packet` is uncertain. 0638 landed `thinkthen_run_call` with independent result and error owners and a cancel token, so no host calls those session functions.
- `libraries/swift/check.sh` on Darwin needs the final installed package and starts no native build.
