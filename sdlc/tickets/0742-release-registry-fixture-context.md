# 0742 — Run registry fixtures with the complete release context

Status: OPEN.

Milestone: 0.2.1

## Outcome

The release-registry self-test exercises every existing Go tag branch in the isolated configured gate without using live services or credentials.

## Changes

- Diagnose the four Go tag fixture failures in the combined core gate and repair the shared cause in `sdlc/scripts/release-registry-self-test.py`; change `sdlc/scripts/release-registry` only if the reproduction proves a production defect.
- Check every sibling fixture for the same missing or stale release context, preserving all existing success, refusal and mocked-command assertions.

## Keeps

- Public behavior, tag protection, publishing requirements, test limits and isolation remain unchanged.
- No live registry, tag, release, provider or publishing operation runs.
- Add no runner, verifier or new tooling.

## Proof

- Preserve failed M5 job 0709/1792513392676-35528 at b678c04a7a4a213bb2c1b7ca090a656bfe2541e9: release-registry-self-test reported 38/42, with four Go tag cases returning exit 1 before any mocked gh command.
- Reproduce the cause through the existing isolated test path, make all existing registry fixtures pass, and run the configured gate on the combined repaired revision before landing.

## Progress

- 2026-10-20 started
