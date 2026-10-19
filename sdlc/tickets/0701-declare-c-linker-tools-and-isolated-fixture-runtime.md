# 0701 — Declare installed C linker tools and isolate fixture runtime dependencies

Status: OPEN.

Milestone: 0.2.1

## Outcome

- The existing tool resolver supplies a declared, pinned `pkg-config` implementation for installed C static-link checks on supported native and Linux hosts. Selecting the C group supplies the real executable; a resolved-tools marker alone is not proof of availability.
- Isolated release fixtures retain their required shell and operating-system runtime commands through the existing fixture helpers. Fixtures that deliberately hide a compiler or another tested tool still prove that refusal. Normal fixtures no longer fail because an unrelated runtime command disappeared from a hand-kept fake PATH.
- The existing Linux fixture entrypoints are checked together with independent real exit codes before the complete container gate, so its next run does not discover these setup omissions one script at a time.

## Changes

- Declare the installed C linker tool in `sdlc/tools.json` through an existing provider and update only the necessary existing resolver or image installation paths in `sdlc/scripts/tools.py` and `sdlc/linux/install.py`.
- Fix and consolidate the affected isolated PATH setup in `sdlc/scripts/*self-test*` and its existing fixture helper. Inventory the release fixture entrypoints already called by `sdlc/scripts/test`; use those commands rather than introducing another runner or a second command inventory.
- Identify any additional shared helper before editing it and coordinate with the active conformance and generator lanes.

## Keeps

- Keep public APIs, tool pins other than the newly declared dependency, test deadlines, publishing guards, and every existing diagnostic, archive, source, checksum, privacy, and zero-send assertion.
- Keep deliberate missing-tool tests meaningful. Add no host system installation, new runner, verifier, selector, generic preflight, or weaker platform skip.
- Build and test only through pm jobs on an authorized build machine. Preserve the earlier failed receipts. Changed image inputs invalidate the corresponding prepared image.

## Proof

- Through a pm job, the real C tool-group resolver finds and verifies the declared implementation, and the existing installed C static and blocking-call checks run against a freshly packaged library on the supported host.
- The existing Python release-container fixture that failed with `sh: not found` passes, along with the other isolated release fixtures, with their actual missing-tool negative cases still failing as required.
- Run the existing Linux fixture commands with independent exits, then the configured Linux/native gate on the reviewed source. Do not infer a pass from an unrun stage.

## Evidence

- 0636 job 1792434815438-79593 built production archives and passed installed Rust checks, but installed C stopped before its matrix because `installed_cases.static_flags` could not find `pkg-config`; both 0636 and 0637 verified that the manifest has no declaration for it.
- 0619 job 1792435252854-29497 passed Linux lint and the core and consumer tests, then `release-python-self-test.py` failed because its isolated PATH omitted `sh`. The preceding Swift registry failure had the same missing-fixture-runtime cause and was repaired separately.

## Progress

- 2026-10-19 started
- 2026-10-19 next: PM requires every isolated fixture to start from the manifest resolved PATH and remove only the tool under test, with no hand-built PATH; convert every remaining fixture occurrence in the same repair, preserving fake-command precedence and all missing-tool negative cases before the batched source proof.
- 2026-10-19 next: Native batch1792436492837-38650 at8d58a653b ran20 entrypoints with15 passes and5 failures, proving both declared pkgconf commands while exposing protected-PATH stat errors, a missing manifest lint group that triggered an unintended provider subtree which was reaped, and an undeclared Python3.12 README launcher, so one shared repair batch precedes fresh review and both image rebuilds.
- 2026-10-19 next: Main-only native candidate19dcedb8d contains the reviewed-outcome runtime fixes without0619 ancestry or image changes, while focused job1792439226125-14081 passed all six affected entrypoints at container source3ad566785, so this partial landing proceeds before0637 and0692 and0701 remains open for fresh installed C proof, both image rebuilds and Linux fixture and configured-gate proof.
- 2026-10-19 next: Fresh component review accepts7505ff138 and M5 job1792439722339-53628 verifies both real pkgconf commands plus11 tools and27 README checks with earlier identical fixture passes reused, so the main-only partial fix is ready for one closing gate and landing while the ticket remains open for installed C, images and Linux proof.
