# 0104: Fresh package before the check

Status: landed. A fresh review of `62c89764` accepted it (`sdlc/records/0104-review.md`) and kept the plant in `lint`.

## Result

- `sdlc/scripts/package` removes `target/package/thinkthen-*.crate` before `cargo package`. A fresh `CARGO_TARGET_DIR` would also avoid the stale file. It would rebuild the whole dependency closure on every run, so deleting one file is the simpler fix.
- `sdlc/scripts/lint` writes 4 MiB of random bytes to the crate path before every package run. The fresh crate is 463385 bytes, so the plant is always the longer file.

## Red and green

| Run | Package script | Exit | Last lines |
| --- | --- | --- | --- |
| `package` over a planted 2,000,000-byte crate | `origin/main` | 2 | `gzip: stdin: decompression OK, trailing garbage ignored`, `tar: Child returned status 2` |
| `package` over a planted 4 MiB crate | fixed | 0 | `package: ... unpacked catalog pass` |
| `lint` | `origin/main` | 2 | the same `gzip` and `tar` lines |
| `lint` | fixed | 0 | |

After the red run, `target/package/thinkthen-0.0.1.crate` was still 2,000,000 bytes. `cargo package` had written the new crate over the start of the old file.

## Checks

- `sdlc/scripts/lint` with `THINKTHEN_API_KEY` and `THINKTHEN_BASE_URL` unset: exit 0.
- `sdlc/scripts/spec` with both unset: exit 0, `demos: 21 green, 0 red`.
- `sdlc/scripts/live` did not run. No Rust code changed.
