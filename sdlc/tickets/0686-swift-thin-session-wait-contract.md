# 0686: Delete Swift call projection and session loop

Status: OPEN.

Milestone: 0.2.1

Phase: 3

Depends on: 0688

## Outcome

- Swift calls the single run-call entry point and reads the packet's final answer, probability, pick and resolution.
- `CallProjection.swift` is deleted.
- The session loop in `OwnedSession.swift:15-292` and its private closed, reader and producer error text are deleted. One call runs the run call on a dispatch queue worker, and cancelling a Task cancels through the handle and returns promptly while the provider is held.
- Its files leave the 0637 answer-rule check's list of exceptions.

### Proof

On the M5 through pm job, the installed Swift package consumer runs all ten named calls against a recorded backend. A held-provider case shows other async work progressing and Task cancellation returning before the provider is released.

## Evidence

- **Hand-written result shaping.** `libraries/swift/Sources/ThinkThen/CallProjection.swift:47-97` matches rows by string keys such as "kind" and "aggregate". `recognitionProbabilities` pairs kept proposals with spans. The same rule is hand-written in PHP and Ruby.
- **Blocking waits.** `OwnedSession.swift:75,101` runs blocking `thinkthen_session_push` and `next_packet` on `DispatchQueue.global`. Whether cancellation always unblocks a waiting `next_packet` is uncertain. With the run call, no host calls these functions directly.
