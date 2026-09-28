# 0218 root-container functional-check build

Status: focused build complete on the 0218 source branch, awaiting fresh independent code review and landing. Design ACCEPT at `487f0cdd` is recorded in [the design-review handoff](0218-design-review.md). This record closes no issue on its own.

## Result and source boundary

`sdlc/live-test` now starts only its storage-failure case through `/usr/bin/python3` with child-only `RLIMIT_FSIZE` equal to the initialized scratch ledger's byte length. The child ignores `SIGXFSZ` and execs the copied `sdlc/scripts/live`; that script reaches its existing `os.write` append handler and prints exactly `live: the charge may have been recorded; inspect --status before retry`. The test independently asserts exit 1, exact warning, unchanged ledger bytes with `cmp`, and absence of the job marker. It keeps the ledger regular and private. The other dummy-key cases still use the ordinary runner. No real ledger or product source was read or changed.

`sdlc/scripts/test-full-cases --run` now checks the Linux real UID with `id -ru`, verifies `prlimit` exists, and reads `/proc/self/status`'s `CapEff` to reject effective capability bits 21 or 24. A missing or malformed capability field refuses. These checks precede the heavy-lock source and Cargo. The script's `--list` branch and non-Linux path are unchanged, as is the routine `sdlc/scripts/test` path. The existing `backend/interrupt.rs` direct-test assertion and its product behavior needed no edit; its old prerequisite diagnostic already names root, and the gate now gives a clear earlier refusal.

The scripts grew by 55 insertions and five deletions at this checkpoint: 32 changed lines in `sdlc/live-test` and 28 added lines in `test-full-cases`. `sdlc/ratchet.json` counts Rust source, so no ratchet value changed. The run-result capture was shared rather than copied; the Linux guard lives next to the full gate's entry point. No new helper file, dependency, product hook or retained test was removed.

## Focused proof

| Boundary | Result |
| --- | --- |
| Shell syntax and ordinary scratch ledger | `sh -n` passed for both changed scripts; `sdlc/live-test` passed all local dummy-key cases as UID 1000, including exact warning, bytes and no job marker. |
| Root scratch ledger | Existing `thinkthen-ruby-builder:local` image, UID 0, `--network none`, read-only checkout and executable `/tmp` tmpfs: `sdlc/live-test` printed `all cases passed`. A first local image was unsuitable because its default scratch mount was `noexec` and it lacked the script's required `/usr/bin/git`; those attempts did not establish a product failure. |
| Root full-gate boundary | In that root container, `test-full-cases --run` exited 2 with `RLIMIT_NPROC does not bind real UID 0; run --run as a nonroot user`, before Cargo or the heavy lock. `--list` printed the canonical 54-case plan as root after read-only host `jq` and its two libraries were mounted into the existing image; the image did not itself include `jq`. |
| Missing tool | Under UID 1000, a temporary `PATH` contained the needed `uname`, `id` and `awk` commands but no `prlimit`. `--run` exited 2 with only `prlimit is required for the Linux RLIMIT_NPROC case` on stderr and empty stdout. No Cargo or lock was reached. |
| Existing no-send witness | `flock -o "$THINKTHEN_HEAVY_LOCK" cargo test --locked --offline -p thinkthen --test backend interrupt::a_carrier_that_cannot_spawn_is_a_defect_and_sends_nothing -- --exact` passed 1/1 as UID 1000. That test retains exact exit 70, stderr, empty stdout and zero listener connections. |

No full `test`, `test-full-cases --run`, stress, repeated or high-load campaign, provider call or real live-ledger run occurred. A privileged nonroot capability probe and a real non-Linux host were unavailable; the shell source checks the named bits, and non-Linux behavior remains the original path. This is focused proof, not a full gate pass.

## What the build taught us

The precise UID matters: `id -u` reports effective UID, while the nproc exemption concerns real UID, so the guard uses `id -ru`. The first root image's missing hardcoded Git path and non-executable scratch mount were test-environment limits, not ledger behavior. An existing image with those prerequisites let the whole local scratch test pass under root. The copied-script warning is unique to the append error path; exact warning plus unchanged ledger and absent job marker give independent assertions. The direct interrupt case remained useful without editing it. Review should inspect the shell capability parsing and the child-only signal/limit inheritance; remaining acceptance belongs to fresh code review.
