# 0615: Declare each check's tools in one manifest and drop hand PATHs

Status: OPEN.

Milestone: 0.2.1

Phase: 1

Reviews: revision 885024b, accept

Reviews: revision 04b84c5, accept

Reviews: revision e09063b, accept

Reviews: revision 9b34e0e, accept

Reviews: revision 2ce4f6a, accept

Reviews: revision 4d4407f, accept

Reviews: revision 3860e7e, accept

Reviews: revision 49a3d03, accept

Reviews: revision 79424ae, accept

## Outcome

- One tracked manifest names every tool each gate, check and library gate needs, with its pinned version where one is pinned, and the platforms on which each library is supported.
- Tools come from one of two places, never from a machine folder. On Linux, the pinned container from 0619 holds them. On macOS, one resolver reads the manifest and installs the pinned versions into a job-owned tool folder under the pm job checkout, then builds the PATH for the run from that folder and the system folders only.
- Before any work starts, the resolver refuses with one message that lists every missing tool.
- The `pm.json` tickets gate calls `sdlc/scripts/lint` and `sdlc/scripts/test` through the wrapper from `ticket/0543-landing-login-environment`, with no PATH text and no machine folder.
- `sdlc/scripts/test` and every `libraries/*/check.sh` and `databases/*/check.sh` stop editing PATH and stop reading absolute tool variables. They take tools from the resolved PATH.
- No tracked file names a home folder or a machine-specific install folder in a PATH.
- The fixtures' inline tool lists are 0617's work.

### Proof

- On the M5 through pm job, the native macOS `test` resolves every tool from the manifest into the job-owned tool folder, starting from a shell whose PATH holds only the system folders, and passes. The container half of the gate is proven in 0619.
- The same run with one manifest tool hidden fails before any build or test step, and its message lists that tool.
- A grep over tracked files finds no `PATH=` that names a home or machine folder, and no `THINKTHEN_*_BIN` tool variable.

## Evidence

Checks find their tools through PATHs written by hand, so a missing tool turns up only when a run fails on a new machine. `sdlc/pm.json` (the `ticketsGate` line) hard-codes one machine's folders, `/opt/zerobrew/opt/rustup/bin` and `$HOME/.local/share/thinkthen-0589/...`. `sdlc/scripts/test` prepends Ruby to PATH by hand after sourcing `libraries/ruby/toolchain.env`. Library gates find their own tools in their own ways: absolute `THINKTHEN_*_BIN` variables (`libraries/go/check.sh:15-20`, `libraries/cpp/check.sh:13-17`), a Flutter under `$HOME/.local/opt/flutter` (`libraries/dart/check.sh:8`), `/usr/bin/php8.3` (`libraries/php/check.sh:9`), the variable mapping in `libraries/jvm/toolchains.py:34`, R's pinned toolchain lookup, and a Node under `~/.cache/thinkthen-toolchains` (`libraries/typescript/check.sh:62`). Commit `7f1d4b7fb` shows the result: `mktemp`, `sha256sum` and `shasum` were added to tool lists only after a run failed without them. Release record 0543 shows the same class returning across rc.13 to rc.17.

The M5 login shells (`-lc` and `-lic`) both lack rustup and prettier and select Python 3.9.6, so the main landing gate still names this machine's tool folders. An accepted, unmerged portable gate wrapper sits on `ticket/0543-landing-login-environment` (9ec101b94); it uses the real login shell and keeps the `lint && test` exits, but it cannot pass until tool discovery is solved once. This ticket builds on that branch.

## Progress

- 2026-10-18 started

## Amendment, 2026-10-18

All-language lint needs GNU gcc for the Ada spec dump, which macOS lacks, so lint runs in the 0619 container and native macOS `test` proves 0615. 0617, 0618 and 0619 may build on the accepted 0615 head before it lands; the four land as one gate batch.
