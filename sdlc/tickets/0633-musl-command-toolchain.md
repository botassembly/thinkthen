# 0633: Build the static Linux command with a real musl toolchain

Status: OPEN.

Milestone: 0.2.1

Phase: 2

Depends on: 0619

## Outcome

- Changes: `sdlc/scripts/release-pack`, `sdlc/tools.json`, `sdlc/linux/Dockerfile`.
- The pinned Linux image provides a musl C compiler for its own architecture, and `sdlc/tools.json` declares it in the group `release-pack` resolves.
- `release-pack`'s `part_command` builds the `*-linux-musl` command with that compiler only. The clang fallback, the `CC_<target>` probing and the `-DSQLITE_DISABLE_LFS` flag are deleted.
- `release-pack` refuses to build the musl command when the declared compiler is missing, and names it.

### Proof

- In the pinned arm64 image on the M5, through `pm job run --machine m5 0633 SHA --memory 16`, a `docker run --rm --label pm.repo=thinkthen --label pm.ticket=0633` of that image runs `sh sdlc/scripts/release-pack aarch64-unknown-linux-gnu OUT command`. It builds `thinkthen-VERSION-aarch64-unknown-linux-musl.tar.gz`, and `file` reports the unpacked `thinkthen` as statically linked. The same run with the musl compiler removed from `PATH` refuses with the compiler's name; before the change it falls back to clang and succeeds.
- The unpacked command runs `thinkthen --version` and one replayed call that uses the cache in a plain `alpine` container and in a plain glibc `ubuntu` container on the M5.
- The x86_64 archive is built and run the same way on Yellow at the 0619 container checkpoint.

## Evidence

- `install.sh` installs the `x86_64-unknown-linux-musl` and `aarch64-unknown-linux-musl` commands, and `sdlc/scripts/release-workflow installer-smoke` picks the same targets.
- `sdlc/scripts/release-pack` `part_command`, for a `*-linux-musl` target, checks `CC_<target>`, then `<arch>-linux-musl-gcc`, then `musl-gcc` for x86_64 only. When none is present it exports `CC_<target>=clang` and adds `-DSQLITE_DISABLE_LFS` to `CFLAGS_<target>`, because clang then compiles the bundled SQLite against glibc's headers, which name `open64` and `stat64`; musl 1.2.4 and later no longer export them. The shipped command's build therefore depends on which compiler the machine happens to have.
- 0619's image definition `sdlc/linux/Dockerfile` (on its branch) installs Ubuntu's `musl-tools`, which provides `musl-gcc` for the image's own architecture. `release-pack` does not accept `musl-gcc` for aarch64 today.
- The hosted `.github/workflows/gate.yml` also installs `musl-tools`; 0620 owns the hosted workflow steps.
