# 0746 — A new advisory fails a lane only when the lane changes a lock

Status: OPEN.

Milestone: 0.2.1

Phase: 2

## Outcome

- Changes: `sdlc/scripts/lint`, `sdlc/scripts/linux-check`, `sdlc/scripts/README.md`, `sdlc/scripts/lint-self-test.py`, `sdlc/scripts/dependency-check.py`, `sdlc/tools/deny.json`, `sdlc/scripts/surfaces`, `sdlc/scripts/test-full-cases`, `sdlc/scripts/allow-list`.
- The landing gate's `cargo deny` step keeps checking bans and licenses on every run. It checks advisories only when the candidate changes a tracked `Cargo.lock` or `Cargo.toml` against its merge base with main.
- When a ticket's gate skips the advisory check, lint prints one line saying so and naming the merge base.
- The 0691 checkpoint run, the release workflow and `sdlc/scripts/test-full-cases --checkpoint` check advisories on every run, whatever changed. An advisory found there gets its own repair ticket, ordered first, as 0744 was.
- Shipped code still never carries a known advisory: the checkpoint and release checks are the bar for a candidate.

### Proof

- A lint self-test fixture shows three cases: an unchanged lock with a planted advisory passes the ticket gate and prints the skip line; a changed lock with the same advisory fails; checkpoint mode with an unchanged lock fails.
- The focused isolated fixture runs through `pm job run --machine m5 0746 SHA --disk 20 --memory 8 -- sdlc/scripts/job python3 sdlc/scripts/lint-self-test.py`; its synthetic Git repositories and controlled Cargo child require no Linux compiler and add no linux-check mode.
- `pm job run --machine m5 0746 SHA --disk 20 --memory 8 -- sdlc/scripts/gate` passes, including the registered lint fixture on the Linux side; lint, generation and cargo deny never run as native macOS checks.
- The landing candidate also passes `pm job run --machine gator 0746 SHA --disk 20 --memory 7 -- sdlc/scripts/linux-check lint test` in parallel with its configured gate while Gator is available.

## Evidence

- On 2026-10-20 one new TLS advisory (CVE-2026-111943) turned red the landing gates of 0743, 0745 and 0740 and the Gator checkpoint run within the hour. None of them changed a dependency. Every lane then waited on the standalone 0744 repair. Root-cause record: `sdlc/records/2026-10-20-five-delays-root-cause.md`.
- The step reads a locally fetched advisory database (`cargo deny fetch db` in `linux-check`), so main's gate can turn red with no commit.

## Progress

- 2026-10-20 started
