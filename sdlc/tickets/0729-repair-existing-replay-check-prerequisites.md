# 0729 — repair-existing-replay-check-prerequisites

Status: OPEN.

Milestone: 0.2.1

## Outcome

- Changes: `sdlc/scripts/settings`, `demos/46-find-unused-recording-entries/`.
- Restore the existing replay checks against the current documented interfaces and recorded fixtures. Keep every diagnostic, negative assertion, recorded answer key and product behavior unchanged.
- Make the settings self-test distinguish internal fixture environment from public settings and plant its R refusal on the current implementation, retaining an effective red/green check.
- Update the unused-recording demonstration's inputs and expected output to the current unchanged recording keys, preserving its distinction between used and unused entries.
- Add no runner, checker, toolchain pin change or live provider call.

## Proof

- Through an M5 pm job, reproduce the settings self-test and demo46 failures before the repair, then pass those checks after it.
- Replay the existing complete spec and demos checks with 0708's accepted stored-answer change included; collect all remaining failures without weakening checks.
- A fresh reviewer checks the whole cause and accepts the ending source; closing lint and test run once with the wave-two batch.

## Evidence

0708 M5 job1792448675157-4945 stopped the full spec check at an undocumented internal `THINKTHEN_DUCKDB_EXTENSION` variable and an R `builder.timeout(` plant whose source target no longer exists. Those files are unchanged from main. Job1792448933045-34915 passed demos through45, then demo46 expected an obsolete unused count and recording key. The 0708 offline comparison preserved every existing answer key.
