# 0303: Green main

Status: ready. Lane claude-1. Plan: `sdlc/planning/cleanup-2026-09-30.md`.

## Outcome

`cargo test --locked --offline --workspace --all-targets` passes on main under both umask 022 and umask 002. Every later landing in the cleanup phase runs it.

## Evidence so far

A review of `cac92d2c6` on 2026-09-29 found 30 failures under umask 022 and 39 under umask 002, out of about 1,250 tests. Most are exact-output tests that later commits broke: `meta.attempts` in `--details`, an extra plan line, transform byte counts, help text, and demo output. `engine/deadline_tests.rs:446` fails every run because `engine/request.rs:167` now reads the key before the folder lock. Doctests need `--cfg thinkthen_internal_doctest`. `config.rs:248` warns on other-write only; `cli/asking/folders.rs:113` also warns on group-write.

## Changes

- Decide for each failure whether the code or the test is wrong, from the specification. Fix the wrong one.
- One folder-permission rule, shared by both call sites: warn on other-write only.
- Make the full suite one command in `sdlc/scripts/test`, or point `test` at `test-full-cases --run`.

## Retained behavior

No public output changes except where the specification already says the new output is right.

## Proof

The full suite passes twice, under umask 022 and 002. `policy.py` passes.

## Deferred gaps

Package gates of the surfaces belong to later tickets.

## Evidence

- Starts from: 30 failures under umask 022 and 39 under 002 at `cac92d2c6`; record 0305.
- Keeps: public output the specification already requires.
- Changes: stale tests, the folder-permission rule, the routine test script.
- Proof: full suite green under both umasks; `policy.py`.
- Defers: surface package gates.
