# Quick Fix qf-installer-test-macos-targets: the installer test runs on macOS

Status: built in lane claude-3 from main `3a5e32b83`. Ian can overturn the platform table.

## Why

Release rehearsal run 36916614435 passed every surface on both Mac smoke jobs. Both then failed in `sdlc/scripts/installer-test`, which the release smoke step has run since `1aaa099a1`. The test derived the archive target from the machine name alone and knew only the Linux machine names. Apple Silicon stopped with "no case table for machine arm64". Intel Mac expected `thinkthen-0.1.0-x86_64-unknown-linux-musl.tar.gz` in every case, while `install.sh` correctly asked for `thinkthen-0.1.0-x86_64-apple-darwin.tar.gz`, so 30 of 36 cases failed on the fake server's 404.

## Change

- The test keys its platform table on `uname -s` and `uname -m`, with every pair `install.sh` accepts. Linux pairs name the static musl archive. macOS pairs name the Rust target. These names match `release-pack`, the tap step in `release-workflow`, and `release-smoke-command-test`.
- Seven new cases run under dash and bash with a stand-in `uname`, one per accepted pair. Each serves only that platform's archive and pins the whole success output. Every host therefore proves all four published archive names, including Intel macOS, which no local host can run.
- The test resolves its temporary folder. macOS places it under the `/var` link, and `install.sh` prints the `pwd -P` path. Without this, 18 of 50 cases held on the M5.
- Every earlier case keeps its exact expectations; only the archive name follows the host's target. No case is skipped on macOS.
- `install.sh` already names the macOS targets the release publishes (`x86_64-apple-darwin`, `aarch64-apple-darwin`), so it is unchanged.

## Checks

- Linux x86_64: `installer-test` 50/50 cases hold under dash and bash.
- M5 (Darwin arm64, Python 3.9.6): the main version stops with "no case table for machine arm64"; the new version holds 50/50. The scratch folder was removed.
- Mutation: changing `install.sh`'s `Darwin/x86_64` target to the Linux musl name fails both shells' `Darwin x86_64` case (48/50).
- `sdlc/scripts/lint` exit 0. `CARGO_NET_OFFLINE=true python3 sdlc/scripts/policy.py` exit 0.
- Not run: the release workflow. The next rehearsal shows the Mac smoke jobs.
