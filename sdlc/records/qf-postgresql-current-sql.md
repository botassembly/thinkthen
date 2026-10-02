# Quick Fix qf-postgresql-current-sql: the PostgreSQL archive holds only the current version's SQL

Status: built in lane claude-4; fresh review accepted. Ian can overturn the rule that the source control file names the packed version.

## Why

Checkpoint 5 on `f65faea4e` failed in the surfaces publish's release smoke:

```
release-smoke: FAIL thinkthen-postgresql16-0.1.0-x86_64-unknown-linux-gnu.tar.gz (the builder's home or the test probe is in extension/thinkthen--0.0.1.sql )
```

`cargo pgrx package` writes into `target/release/thinkthen-pg16/` and never clears it. `check.sh` copies that tree over `thinkthen-pg16-shipped/` without clearing it. Lane claude-4 had built the extension before the 0.1.0 bump, so both ignored trees still held `thinkthen--0.0.1.sql`. `package.sh` copied every `thinkthen--*.sql`, so the 0.1.0 archive packed the stale 0.0.1 file. That file was built with the test probe, and the smoke's scan caught it.

## Change

- `databases/postgresql/package.sh` reads `default_version` from the source `thinkthen.control`. It packs the tree's control file and `thinkthen--VERSION.sql` only. It fails with a clear message when the tree's control file differs from the source copy or when the version's SQL is missing.
- `databases/postgresql/runtime.sh`: `runtime_install` copied every `thinkthen*` file from the extension folder into the test server. It now copies the control file and `thinkthen--$EXT_VERSION.sql` only, on Linux and macOS. The check then runs against the same files the archive ships.

The stale files stay in lane claude-4 as the reproduction. Nothing deletes them.

The other packers in `sdlc/scripts/release-pack` already name one file: the wheel, gem and npm pack by `$V`, the DuckDB extension and SQLite library by fixed path, the crate and R package from fresh scratch folders, and the source wrappers from listed sources.

## Checks

- On main (`f65faea4e`) in lane claude-4: `release-pack --reuse x86_64-unknown-linux-gnu OUT postgresql` packed `extension/thinkthen--0.0.1.sql`, `extension/thinkthen.control`, `extension/thinkthen--0.1.0.sql` and `lib/thinkthen.so`. `release-smoke OUT` printed the checkpoint's failure line exactly.
- With the fix, same lane, same stale trees: the archive holds `extension/thinkthen.control`, `extension/thinkthen--0.1.0.sql` and `lib/thinkthen.so`. `release-smoke OUT` printed `postgresql: 13 passed, 0 failed` and `surfaces: pass databases/postgresql thinkthen-postgresql16-0.1.0-x86_64-unknown-linux-gnu.tar.gz`. Its only failures were the eight "DIR holds no file" lines for the parts not packed.
- Planted trees in scratch: a tree without `thinkthen--0.1.0.sql` failed with `package.sh: no thinkthen--0.1.0.sql in ...; rebuild the tree`. A tree whose control file named 0.0.1 failed with `package.sh: .../thinkthen.control differs from thinkthen.control; rebuild the tree`. Both exited 1.
- `STEPS="shipped_lacks_probe no_home_in_library find_signatures_are_owned_and_private an_update_cannot_grant_public examples" bash databases/postgresql/check.sh`: 5 passed, 0 failed, with the stale file still in both trees.
- `sdlc/scripts/lint` exit 0. `CARGO_NET_OFFLINE=true python3 sdlc/scripts/policy.py` exit 0.

## Deferred gap

`libraries/r/tools/make-tarball.sh` lists `thinkthen-*.crate` in the target's `package/` folder. `release-pack` and the R check give it fresh target folders. A run against a shared target holding an older crate would fail loudly on two names, so it ships nothing stale.
