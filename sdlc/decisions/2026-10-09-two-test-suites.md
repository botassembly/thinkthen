# Two test suites: routine behavior tests and release tests

Ruled by Ian on 2026-10-09.

## Ruling

In Ian's words: "In general, we should be doing outside-in tests. We shouldn't be doing proof spirals. We shouldn't be doing red-green and keeping the tests around forever. It should be red-green-remove. Get rid of any tests that aren't valuable to outside levels. We just want the outside-in tests, the behavior tests. We want them at the Rust code level, the CLI level, and every library level, but they shouldn't be calling the API." And: "Anything that's trying to stress or load test the system, fine. I just don't want to do it after every ticket. We'll need to have two suites here."

## What it means

- Tests state behavior where a caller meets it: the public Rust API, the CLI, and each language library's installed public surface. Every test runs against the local fake backend. No test calls a model API.
- Developers follow red, green, remove. Unit tests written to get a hard step working are scaffolding. They are deleted before landing unless they are one of the four kinds that stay in `skills/outside-in-tests.md` in the agents repo.
- The routine suite holds behavior tests, edge-case tables, contract checks and regression tests. No routine test runs longer than 5 seconds on a quiet machine. A developer runs the tests for what they changed. The coordinator runs `sdlc/scripts/lint` and `sdlc/scripts/test` once when a ticket closes, and reuses that run while the relevant code is unchanged.
- The release suite holds the large-input boundary cases, tests that build another package inside the test, the library-only feature pass, the full shared cases, the full per-language parity runs and the load runner. It runs at a release candidate through `sdlc/scripts/test-full-cases --run`, `sdlc/scripts/package` and `sdlc/scripts/test-stress --run`.

## Why

On 2026-10-09 the coordinator ran the whole Rust suite, then a second library-only pass, then the full C# parity suite after a narrow fix. The load slowed the machine Ian works on. No load test ran. The cost came from rules that asked for full tests on every landing commit, from about ten multi-megabyte and nested-build cases in the routine suite, and from about 510 in-source unit tests left from red-green without the remove step. Rust test code had grown to about 1.3 times Rust product code.

## Options considered

- Keep one suite and mark slow tests ignored. This alone leaves the per-landing rule in place, so full runs continue.
- Two suites with a per-landing whole-suite run. This still repeats the routine suite on every slice of a ticket.
- Two suites, focused checks per change, the routine suite once per ticket closure, and the release suite at candidates. Chosen.

## What it replaces

- `sdlc/planning/rust-standards.md`: "Run full tests and lint on the landing commit."
- `libraries/BINDING-AUTHOR.md` and the agents repo's `skills/developer.md`: "the coordinator runs full gates."
- The Debt 028 library-only pass in `sdlc/scripts/test`. `sdlc/scripts/package` still runs the library-only tests in the release `crate` job, so an ungated test is caught at the candidate.
- Ticket 0484 carries the moves and deletions.
