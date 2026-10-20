# 0742 — Keep constructor documentation within the public README contract

Status: COMPLETE.

Milestone: 0.2.1

Reviews: revision 22d5af5, accept

Landed: 3236f1f

## Outcome

The C public README preserves the owned-constructor error instructions and passes the existing public-page contract, so the release-registry self-test exercises every existing Go tag branch without using live services or credentials.

## Changes

- Preserve every word and code statement in `libraries/c/README-PUBLIC.md`; join the constructor-diagnostics paragraph to the adjacent caller-ownership paragraph so the page satisfies the existing under-40-line requirement.
- Leave the production README validation and all release-registry success, refusal and mocked-command assertions unchanged.

## Keeps

- Public behavior, tag protection, publishing requirements, test limits and isolation remain unchanged.
- No live registry, tag, release, provider or publishing operation runs.
- Add no runner, verifier or new tooling.

## Proof

- Preserve failed M5 job 0709/1792513392676-35528 at b678c04a7a4a213bb2c1b7ca090a656bfe2541e9: release-registry-self-test reported 38/42, with four Go tag cases returning exit 1 before any mocked gh command.
- Reproduce the cause through the existing isolated test path, make all existing registry fixtures pass, and run the configured gate on the combined repaired revision before landing.

## Progress

- 2026-10-20 diagnosis: the four Go fixtures correctly stopped at production README validation; 0741 added two lines to the previously 38-line C README, reaching 40, which the unchanged under-40 rule refuses; no missing fixture context exists.

- 2026-10-20 started
