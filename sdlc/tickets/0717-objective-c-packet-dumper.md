# 0717: Replace the Objective-C fixture consumers with one packet dumper

Status: OPEN.

Milestone: 0.2.1

Phase: 3

Depends on: 0676, 0688

## Outcome

- Changes: `libraries/objective-c/checks/FoundationCases.m`, `libraries/objective-c/checks/FoundationResultsConsumer.m`, `libraries/objective-c/checks/FoundationDump.m`, `libraries/objective-c/checks/foundation_installed.py`, `sdlc/scripts/coverage_installed_managed.py`, `sdlc/scripts/coverage-checks.py`.
- One Objective-C packet dumper over the public API, following the 0642 contract, replaces `checks/FoundationCases.m` and `checks/FoundationResultsConsumer.m`, which are deleted. `checks/foundation_installed.py` builds the dumper in its installed consumer package and compares its output through `conformance/children` in place of the `native_cases` adapter.
- `checks/FoundationConsumer.m` stays for the held-provider and cancellation cases.
- The Objective-C binary names in `sdlc/scripts/coverage_installed_managed.py` and `sdlc/scripts/coverage-checks.py` follow the change.

### Proof

- On the M5 through `pm job run --machine m5 0717 SHA --memory 8 -- sh libraries/objective-c/check.sh`, with `THINKTHEN_ARTIFACT` naming the Apple package built at the same revision by the release route, the installed dumper's output matches the 0642 oracle for every shared case. Before the change the check runs the `Cases` adapter and the `Results` fixture reader instead.
- Done once by hand and not committed: changing one expected value in the oracle fails the Objective-C comparison with a diff.

## Evidence

- `libraries/objective-c/checks/foundation_installed.py` builds four executables (`Consumer`, `Cases`, `Public`, `Results`) in a scratch SwiftPM package. `Results` reads `conformance/fixtures/complete.json` through the private `TTResultPrivate.h`, and `Cases` serves `native_cases(adapter, consumer="objective-c")` from `conformance/children`.
- `sdlc/scripts/coverage_installed_managed.py` names the Objective-C binaries `Consumer` and `Cases`. `sdlc/scripts/coverage-checks.py` runs `checks/foundation_installed.py` in its fixture. `conformance/test_parity.py`, `sdlc/scripts/release-registry-self-test.py` and `site/src/data/repo.mjs` also name `checks/foundation_installed.py`, so the driver keeps its name.
- 0642 defines the dumper contract and the oracle. The dumper is written once against the run-call results from 0676.
- Split from 0675 on 2026-10-19 so packaging and the test move land separately.
