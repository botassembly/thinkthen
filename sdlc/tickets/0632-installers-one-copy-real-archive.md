# 0632: Serve installers from one copy and test them on real archives

Status: OPEN.

Milestone: 0.2.1

Phase: 2

Depends on: 0633

## Outcome

- `site/public/install.sh` and `site/public/install.ps1` are deleted from the tree. The site build copies the root files into its output, and the copy check in `versions` is deleted.
- An installer check serves the real archives and checksums that `release-pack` built for the candidate SHA from the local stand-in server, through `THINKTHEN_INSTALL_BASE` and `THINKTHEN_INSTALL_API`. It installs them with `install.sh` (Linux and macOS) and `install.ps1` (Windows) and runs the installed command.
- The fake-binary archive cases stay only for edge cases that a real archive cannot produce: a bad checksum, a version mismatch, a symlinked folder.

### Proof

- On the M5 through pm job, the installer check installs the real macOS archive, and `thinkthen --version` prints the candidate version.
- The same check passes in the pinned arm64 container on the M5 for the aarch64 musl archive, and in the pinned x86_64 container on Yellow for the x86_64 musl archive.
- `install.ps1` on Windows is proven on the candidate in 0687.
- The built site serves byte-identical installers.

## Evidence

The repo tracks `install.sh` and `install.ps1` twice, at the root and under `site/public/`. `sdlc/scripts/versions:167-180` fails the build when the copies differ. `sdlc/scripts/installer-test` serves archives from a local stand-in server, but each archive holds a fake `thinkthen` script that only prints a version (see its header). No routine check installs a real release archive and runs the installed command.
