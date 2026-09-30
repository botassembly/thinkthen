# 0305: Measure the test suite

Status: landed.

## Outcome

A record, `sdlc/records/0305-test-suite-measurement.md`, that lists per test binary its time, test count, and kind. It names the slowest tests, duplicate tests, tests that pin exact output below the command line, and surface package tests that a shared `conformance/` case set could replace. It proposes a cut list and a faster suite.

## Proof

Commands and timings in the record.

## Evidence

- Starts from: the 2026-09-29 review's 30 failing tests and 84 s full run.
- Keeps: every test and gate; this ticket only measures.
- Changes: adds the measurement record.
- Proof: `sdlc/records/0305-test-suite-measurement.md`.
- Defers: the cuts, to later cleanup tickets.
