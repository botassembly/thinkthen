# Split tests between the fast gate and the release QA suite

Status: closed 2026-09-30. Replaced by ticket `../../tickets/0335-gate-and-release-test-split.md`, which answers the three asks and carries the remaining slices. The Beatles Bench team's feedback of 2026-09-30 set the split three ways: this repository keeps mechanics against recordings with no network; the public Beatles Bench publishes accuracy, cost and speed; the private release QA suite runs live checks, installed packages, every surface, edge cases, a real-prose reading set, and pass or fail on release candidates. The accuracy and timing item under "What moves to the release suite" belongs to the Beatles Bench under that split.

## The change

Keep this repository's gate fast, network-free and focused on mechanics. Move the checks that need the live service, an installed package, or every surface times every function to the release QA suite, which runs on release candidates and releases.

## Why

- Ian wants fast, efficient tests on both sides and no duplicate tests. The release suite may take ten times as long, because it runs only on candidates.
- The standard test script skipped the Polars, C interface and package checks, and landings broke them. The full package run on main then found two broken package tests and a Zig panic on a zero deadline (status of 2026-09-30).
- The release suite now runs the command line against the live service. It holds ten command pages with 153 checks. Its run against main `c22512868` caught two user-visible changes: `--dry-run` became `--plan`, and `--plan` gained a second line with the whole-input count. Both match the specification. They show the kind of change it is built to notice.

## What the gate keeps

- Unit tests where logic is dense: question parsing, thresholds, digests, batch packing, record framing and the wire encoder. Not one per function.
- Outside-in tests of the command and each library against saved replies, for mechanics: exit codes, messages, refusals, batching, cache and recording rules, secrecy.
- One replay smoke test per binding in the standard `test` rung, not per function. It should load the installed-shape package, ask one recorded question, and check the answer. This closes the blind spot above at low cost.
- The contract cases in `conformance/`, as today.

## What moves to the release suite

- Every surface times every function, on Linux and macOS.
- Live and cache-off checks, including real batching counts and cache hits against the service.
- Installed-package checks from the release artifacts.
- Accuracy against labeled sets, and timing per function and binding, compared with the last release.
- Regression checks for harvested edges that only a live service or an installed package can show.

## What the release suite asks of this repository

1. List the tests that the move makes redundant, and retire them in the lead's own tickets.
2. Name a release-candidate commit before each release, so the suite runs against a pinned build.
3. Keep the specification the contract. The suite takes every expected exit code and message from it.
