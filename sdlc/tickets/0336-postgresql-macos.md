# 0336: The PostgreSQL extension builds and confines file reads on macOS

Status: in progress

## Outcome

`cargo pgrx package` builds the PostgreSQL extension on macOS. A confined `@file` read under `thinkthen.file_directory` works there. macOS opens the file with `openat` and `O_NOFOLLOW_ANY` and reads the descriptor's path with `fcntl(F_GETPATH)`. Linux keeps `openat2` and `/proc/self/fd`. Closes `sdlc/issues/2026-09-30-postgresql-extension-does-not-build-on-macos.md`.

## Evidence

- Starts from: main `3911accfc`. Experiment 218's wave 4 M5 run saw seven E0425 errors in `databases/postgresql/src/ffi.rs`. The rank 3 investigation in `sdlc/planning/issue-priorities-2026-09-30.md` found the hidden second cause: `path_of` reads `/proc`, which macOS lacks, so every confined read would refuse.
- Keeps: every confinement refusal, its sentence and its exit code; the spelling check in `files.rs::beneath` that refuses any `..` before an open; the one-link and inside-the-base checks on the descriptor; the Linux `openat2` path and its plain-open fallback; the `files.rs` unit tests unchanged.
- Changes: `ffi.rs` gates `openat2`, `openat2_missing`, the Linux `open_beneath` and the `/proc` `path_of` to Linux. It adds a macOS `open_beneath` (base opened `O_RDONLY | O_DIRECTORY`, file opened with `openat` and `O_RDONLY | O_NONBLOCK | O_NOFOLLOW_ANY | O_CLOEXEC`) and a macOS `path_of` through `F_GETPATH`. `pgrx-package-locked.sh` adds `-Wl,-undefined,dynamic_lookup` to `RUSTFLAGS` on macOS. The PostgreSQL README names both platforms' opens and the check's macOS route. The PostgreSQL ratchet rises 41 lines to 3063.
- Proof: on Linux, the PostgreSQL check, `tickets`, `policy.py`, `lint` in a clean checkout and workspace Clippy pass. The file-open functions pass Clippy with `-D warnings` for `aarch64-apple-darwin` in a scratch crate. On the M5 (macOS 26.4, arm64), in a scratch copy removed afterwards: `cargo pgrx package` builds against zerobrew's PostgreSQL 16.14; `cargo test --lib` passes all 10 tests, including the three confinement tests; a scratch server loads the packed dylib without touching the keg and gives 12 of 12 expected results for a non-superuser role (three inside reads; refusals for `/etc/hosts`, two `..` spellings, a symlink out, a symlink directory hop, a final symlink to a file inside, a hard link to an outside file, a missing file and a directory).
- Defers: the full PostgreSQL check on macOS. The M5 has zerobrew, not Homebrew, so `check.sh` reports "not run: brew is missing". Homebrew's formula API still names 16.15 with both pinned bottle hashes, so `runtime-darwin.env` stays; experiment 218's 16.14 was zerobrew's keg. The macOS jobs of the release rehearsal are the check of record.

## Design notes

`O_NOFOLLOW_ANY` refuses a symlink at any step (macOS 11 and later; the release targets 15.0). macOS is deliberately stricter than Linux in two ways. A middle symlink that stays inside the folder reads on Linux and refuses on macOS. The folder needs read permission on macOS, where Linux's `O_PATH` needs only search permission. Both fail safe with the same refusal. The `..` refusal stays with the spelling check. A directory renamed out of the base during the open leaves a descriptor whose `F_GETPATH` path is outside, and the inside-the-base check refuses it.

The macOS link flag lives in the package wrapper. `policy.py` refuses a PostgreSQL build script and a `.cargo/config.toml`, and the check's `RUSTFLAGS` would replace a config file's flags anyway.
