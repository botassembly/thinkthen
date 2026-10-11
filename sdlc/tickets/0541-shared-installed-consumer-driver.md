# 0541: Share one installed-consumer driver across languages

Status: OPEN.

Milestone: 0.2

Depends on: 0530

Reviews: revision fe9bf662f0234e8177eecd2a0f0adea96b36e5d0, accept

## Outcome

Every language's installed checks use one shared fake backend, one case projection and one usage-lock runner. Each language keeps only its build command and its consumer program. The case projection passes authored question text through unchanged, so that bug cannot return in one language.

## Evidence

- Starts from: gap 8 of [the 0.2 closure review](../records/2026-10-11-0-2-closure-review.md). The usage-lock test exists with two handshakes in `libraries/ada/checks/usage_installed.py:180-221` and `libraries/jvm/tests/usage_installed.py:40-75`, plus six more copies. Languages load `libraries/csharp/tests/backend.py` and `libraries/cpp/fixtures/session_cases.py` by path. 0530 fixed the authored-input projection five times. `conformance/children/` is already the shared home.
- Keeps: every installed case and its expected result. The routine and full profiles.
- Changes: move the fake backend, the case projection and one usage-lock runner into `conformance/children/`. Point every language's check at them and delete the copies. Cut `sdlc/records/0530-source-package-inventory-repair.md` down to what was built and the lessons; pm holds the status.
- Proof: the routine installed checks pass for every language. A search finds no fixture loaded from another language's folder.
- Defers: nothing.
