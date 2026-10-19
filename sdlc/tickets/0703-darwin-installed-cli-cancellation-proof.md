# 0703 — Prove installed command cancellation on macOS without Linux signal metadata

Status: OPEN.

Milestone: 0.2.1

Phase: 1

Reviews: revision 349a19a, accept

Reviews: revision 94a6a34, accept

## Outcome

- The installed command's held-call cancellation check runs honestly on macOS and proves its supported cancellation behavior, including signal handling, held-provider release, terminal draining, facts and absence of a later-input suffix.
- The fixture uses a valid observation or synchronization boundary on macOS rather than assuming Linux process metadata exists. Linux keeps its existing signal-consumption assertions.

## Changes

- Repair the platform assumption in `sdlc/scripts/conformance/cli_parity.py` and any directly shared process fixture needed for the same assertion.
- Inventory the same process-signal-state assumption in existing cancellation fixtures and fix the affected class through the existing helpers.
- Establish that the chosen macOS observation really changes when the signal is consumed; a constant-zero process field is not evidence.

## Keeps

- Keep cancellation supported on macOS and preserve the existing public names, behavior, error kinds, facts, request counts, held-call lifecycle and final drain assertions.
- Do not weaken the signal-consumption barrier, silently skip the case, substitute a fixed sleep, raise deadlines, use a debugger or ptrace, or add a test-only back door to an installed release artifact.
- Reuse existing fixture and installed-package entrypoints. Add no runner, parser or evidence verifier.
- A change to public behavior, configuration or the approved proof requirement goes to the PM before implementation.

## Proof

- Through a pm job on the M5, the installed command's cancellation-held-call case passes against loopback with the actual chosen macOS synchronization observation and all existing terminal assertions.
- A targeted fixture check demonstrates that the observation cannot falsely acknowledge an unconsumed signal, and existing Linux assertions remain intact.

## Evidence

- 0636 M5 job `1792441871889-70440` at source `a8527005c4685a46f870f497c5c0605009262752` passed fresh production packaging, installed C static, session and blocking checks, installed Rust checks, and 242 of 243 installed command cases. The remaining `cancellation-held-call` case failed at `cli_parity.py:312-316` because Darwin has no `/proc/<pid>/task/*/status`.
- Apple's `adv_cmds/ps/keyword.c` maps the displayed pending field to `extern_proc.p_siglist`; the examined XNU `bsd/kern/kern_sysctl.c` fills caught and ignored signals but does not populate that pending field. Treating a zero from `ps` as an acknowledgement can falsely pass.
- The PM approved running the unchanged cell on Linux from the same source for 0636 and required this separate macOS fixture repair; that scope ruling does not withdraw macOS cancellation support.

## Progress

- 2026-10-19 started
