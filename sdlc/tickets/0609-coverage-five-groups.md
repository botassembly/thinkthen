# 0609 — coverage-five-groups

Status: OPEN.

Milestone: 0.3

## Outcome

The coverage report states one denominator of product code without tests or generated code, and reports five groups: reached from outside tests, reached only by internal tests, reached by both, unobserved, and collections that failed or are missing. An incomplete collection is named as incomplete and prints no bare percentage. Wrapper-language coverage, such as Java's or Python's, is reported separately.

## Evidence

- Source: the read-only build-efficiency audit of 2026-10-16, at Ian's request. Timings come from the 0543 release campaign's phase records, 22 revisions from 2026-10-14 to 2026-10-16. Re-measure on main before acting.
- The report already uses LLVM source coverage, includes subprocesses through continuous profiles, covers native code through 29 installed-language collections, and states its denominator.
- The denominator includes 8 generated files (`libraries/c/src/**/*_generated.rs`).
- It reports per-collection counts, unit-only lines and unobserved lines, but not the outside-only, both, or failed-or-missing groups.
- A failed suite stops the run, so no report records it. A missing-tool collection still prints bare percentages for the rest.
- A coverage run took 3,612 and 3,769 seconds on 2026-10-12: about 29 minutes of build and staging and 34 of measuring.
- Cost before: about 63 minutes per run, with three groups missing. Expected after: the same run reports all five groups; build time falls with ticket 0602.

## Scope

- Changes: `sdlc/scripts/coverage_report.py`, `sdlc/scripts/coverage`, `sdlc/scripts/coverage_installed_*.py`.
- Keeps: every existing collection.

## Proof

The five groups add up to the stated denominator, and a planted failing collection shows as incomplete with no percentage. After landing, measure again and record the result here.
