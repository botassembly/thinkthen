# 0632: Install every packed command archive with the real installer

Status: OPEN.

Milestone: 0.2.1

Phase: 2

Depends on: 0633

## Outcome

- Changes: `sdlc/scripts/release-smoke`, `sdlc/scripts/installer-test`.
- When `release-smoke` checks a packed command archive in its default mode, it also installs that archive with `install.sh` from the loopback stand-in server through `sdlc/scripts/release-workflow installer-smoke`, as `--command` mode already does, and runs the installed command. The `surfaces` rung, the release suite and the hosted smoke job therefore all install the real archive.
- `sdlc/scripts/installer-test` keeps its fake-binary archives only for cases a real archive cannot produce: a missing or wrong checksum, a version mismatch, a symlinked folder or binary, an unsupported system, and the release-list fallbacks. A case that only proves a plain install of the latest release is deleted.

### Proof

- On the M5 natively: `pm job run --machine m5 0632 SHA --memory 8 -- sh -c 'out=$(mktemp -d) && sh sdlc/scripts/release-pack aarch64-apple-darwin "$out" command && sh sdlc/scripts/release-smoke "$out"'`. The output shows `install.sh` installing the packed archive and the installed `thinkthen --version` printing the checkout's version. Before the change the same command passes without ever running `install.sh`.
- `python3 sdlc/scripts/installer-test` passes in `lint` on the M5.
- The aarch64 musl archive in the arm64 container on the M5 and the x86_64 musl archive in the x86_64 container on Yellow are proven at the 0619 container checkpoint, through `sdlc/scripts/linux-check test-full-cases`. `install.ps1` on Windows is proven on the candidate in 0687.

## Evidence

- Narrowed on 2026-10-19: the single installer copy moved to 0710, which lands alone.
- `sdlc/scripts/release-workflow` has an `installer-smoke` step that copies the packed command archive and checksum under a loopback `loopback_server.py` folder, runs `install.sh --version` with `THINKTHEN_INSTALL_BASE` pointing at it, and checks the installed `thinkthen --version`. Only `release-smoke --command` and the hosted `.github/workflows/release.yml` smoke job call it.
- `sdlc/scripts/release-smoke` in its default mode, which `sdlc/scripts/surfaces` runs after `release-pack --reuse`, maps `thinkthen-VERSION-TARGET.tar.gz` to the `command` surface and runs only `command_check`, which unpacks the archive and runs the binary by path. No local routine or release-suite run installs the real archive with `install.sh`.
- `sdlc/scripts/installer-test` serves archives whose one file is a fake `thinkthen` script that prints a version (`archive` and `release`), and its `cases` table includes plain install cases beside the edge cases.
- 0633 changes how `release-pack` builds the musl archives that the Linux installs use.
