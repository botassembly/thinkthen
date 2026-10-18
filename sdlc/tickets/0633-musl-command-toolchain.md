# 0633: Build the static Linux command with a real musl toolchain

Status: OPEN.

Milestone: 0.2.1

Phase: 2

Depends on: 0619

## Outcome

- The pinned Linux image provides a musl C toolchain for both Linux architectures, and the image's tool manifest lists it.
- `release-pack` builds the musl command with that toolchain only. The clang fallback, the `CC_<target>` probing and the `SQLITE_DISABLE_LFS` flag are deleted.
- `release-pack` refuses to build the musl command without the declared toolchain.

### Proof

- In the pinned arm64 container on the M5 and the pinned x86_64 container on Yellow, both through pm job, `release-pack` builds the musl archive for that architecture.
- On each of those machines, the installed command runs `thinkthen --version`, and a replayed call that uses the cache, in a plain Alpine container and in a glibc container.
- `ldd` reports a static binary.

## Evidence

`install.sh` installs the `x86_64-unknown-linux-musl` and `aarch64-unknown-linux-musl` commands. `sdlc/scripts/release-pack:210-221` builds them as follows: when no musl gcc is present, it compiles the bundled SQLite with clang against glibc's headers and adds `-DSQLITE_DISABLE_LFS`, because those headers otherwise name `open64` and `stat64`, which musl 1.2.4 and later no longer export. The shipped command's build therefore depends on which compiler the machine happens to have.
