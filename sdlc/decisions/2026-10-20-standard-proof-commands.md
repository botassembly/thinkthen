# Proofs run through standard commands, and failed runs name their cause

Ian, 2026-10-20, after a root-cause review of the day's failed check runs: about three in four failed on test setup or on how the run was started, not on the product. The review traced this to proofs written as prose, developers working out run commands afresh, and test setup changing while in use. Ian approved these fixes.

## Rules

1. **Three standard commands.** Every proof uses one of them, named exactly in its ticket:
   - Targeted check: `pm job run --machine MACHINE NNNN SHA -- sdlc/scripts/job CHECK [ARG...]`, as `sdlc/scripts/README.md` says.
   - One surface in the Linux container: `pm job run --machine m5 NNNN SHA --memory 16 -- sdlc/scripts/linux-check surface PATH` for arm64, and the same with `--machine gator --memory 7` for x86_64 (Yellow is the fallback). Until 0740 lands, the existing `linux-check` checks.
   - Landing and checkpoint: the configured gate through `pm ticket land`, and `sdlc/scripts/test-full-cases --checkpoint` at checkpoints.

   Nobody types a hand Docker command, a hard-coded image tag or a hand-built PATH. A proof that needs something these commands lack is a ticket against the command, not a workaround.
2. **One developer brief.** Every developer brief carries the exact standard command lines for its ticket, copied from the ticket. A developer never re-derives an invocation.
3. **The developer fixes its own failed check.** When a check fails, the developer who wrote the change repairs it with its context intact. Reviews stay fresh and independent.
4. **The test setup holds still during language work.** After the 0688 freeze, changes to the image, the tool manifest's shared parts, `linux-check`, `job` or the gate land alone and ahead of other work, only when a language needs them, as pm requires for gate-definition changes.
5. **Every failed run names its cause.** The progress line that records a failed run starts its reason with one word: `product` (a real defect in shipped code), `setup` (test fixtures, tools, image or environment), or `invocation` (the run was started wrong). The PM counts these at each checkpoint, and a rising `setup` or `invocation` share gets a ticket against its cause.

Root cause, for the record: the test machinery is still large and tied to particular machines, the problem rule 5 of [the 0.2.1 ruling](2026-10-18-0-2-1-rebuilds-the-basis.md) names. These rules contain the cost while it shrinks.
