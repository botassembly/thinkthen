# Packaging rehearsal, 2026-09-21 (database surfaces)

The brief's item 5 for the three database surfaces: package each extension exactly as a user receives it, install it in a clean disposable container, run that database's slide sample from `repos/mktg/decks/2026-09-21-thinkthen-semantic-commands/surfaces.md`, and record what broke. One section per surface, commands and output as they happened.

- One version everywhere: 0.0.1 (`VERSION`; the first release is 0.1.0 and is not this).
- Everything is built on this Linux x86-64 box from the `surfaces` worktree. Nothing is published to any registry or service; every install is from a local file.
- Every container is disposable, named `dbpkg211-*`, and removed with `docker rm -f -v` at the end of its section; the count left is printed.
- Samples run against the stand-in's offline backend (`ENGINE_NULL=1`): no key, no paid call, no wire. The loopback stub would be needed only by samples that demand wire shapes; none of the three database samples does.
- **Names become rows is not rehearsed here.** That slide is the `recognize` slide, and `recognize` is not built yet; none of the three database samples shows the pattern, so that proof waits for the recognize wave. The conditional in the brief resolves to not-applicable, and this is the honest record of why.
- Cache default, stated for each package: per the contract's settings, the real engine's cache home is `$XDG_CACHE_HOME/thinkthen` (falling back to `~/.cache/thinkthen`) on Linux and `~/Library/Caches/thinkthen` on macOS, with the 100 MB cap; `THINKTHEN_CACHE` or an engine setting wins over both. The stand-in implements no disk cache (`cache_answers` stays zero), so these rehearsals write no cache anywhere; the statement is the contract's, for the real engine.
- Tools added for the cross builds (user level, no sudo): `cargo install cargo-zigbuild --locked` → cargo-zigbuild 0.23.4; zig 0.15.2 was already present at `~/.local/bin/zig`. Recorded again where each is used.

## DuckDB

Container image: `duckdb/duckdb:1.5.5`, the official image at the pinned version. Pulled once this session:

```
$ docker pull duckdb/duckdb:1.5.5
Status: Downloaded newer image for duckdb/duckdb:1.5.5
docker.io/duckdb/duckdb:1.5.5
```

Image digest recorded during the rehearsal: `duckdb/duckdb@sha256:69b2f746c2d179c0e8d47914f2cff6a9af863372732822fb84e88047f4f2ebf1`.

### What broke first, and the fix

The straight build (`make release`) produced an extension against this box's glibc 2.39. It loaded on the build host but **refused to load in the official image**:

```
IO Error: Extension "/pkg/thinkthen.duckdb_extension" could not be loaded:
/lib/x86_64-linux-gnu/libc.so.6: version `GLIBC_2.39' not found (required by /pkg/thinkthen.duckdb_extension)
Encountered errors while executing init file "/pkg/load.sql". Exiting.
```

The official image's glibc predates 2.39. Any user on an older distribution would hit the same wall, so this is a real packaging defect, not a rehearsal artifact.

Fix, now the package's build path: pin the glibc baseline with zig.

```
$ DUCKDB_EXTENSION_NAME=thinkthen DUCKDB_EXTENSION_MIN_DUCKDB_VERSION=v1.5.5 \
    cargo zigbuild --release --target x86_64-unknown-linux-gnu.2.28
    Finished `release` profile [optimized] target(s) in 1m 29s
$ objdump -T target/x86_64-unknown-linux-gnu/release/libthinkthen.so \
    | grep -o 'GLIBC_[0-9.]*' | sort -V | uniq | tail -1
GLIBC_2.28
```

The metadata footer is appended exactly as the Makefile does it, over the zig-built library:

```
configure/venv/bin/python3 extension-ci-tools/scripts/append_extension_metadata.py \
  -l target/x86_64-unknown-linux-gnu/release/libthinkthen.so \
  -o dist/thinkthen.duckdb_extension \
  -n thinkthen -dv v1.5.5 \
  -evf configure/extension_version.txt -pf configure/platform.txt \
  --abi-type C_STRUCT_UNSTABLE
```

- Stage: `dist/thinkthen.duckdb_extension`, sha256 `d7dea0c2d7dfe85f0b85d74fbcf996c6f7bf018e42426bf18416ae89d3e3d1ae`.
- The ruled install line for the local package: `duckdb -unsigned` then `LOAD '/path/to/thinkthen.duckdb_extension';`. The community form (`INSTALL thinkthen FROM community; LOAD thinkthen;`) awaits the signed community build; nothing was published.
- `dist/` also carries a README with the same two lines and the version-pin warning.

### The rehearsal, green

`databases/duckdb/package.sh` is the whole flow (build, stage, container, sample, cleanup). Output of the final run, key lines:

```
== duckdb package: clean container from the official image
image present: duckdb/duckdb@sha256:69b2f746c2d179c0e8d47914f2cff6a9af863372732822fb84e88047f4f2ebf1
== duckdb package: the slide sample, as drawn, in the container
-- Loading resources from /pkg/load.sql
│ duckdb v1.5.5 │
┌───────┬────────────────────────────────────────────────┐
│     1 │ I was charged twice and I want a refund today. │
│     2 │ Please refund my shipping label fee.           │
└───────┴────────────────────────────────────────────────┘
│     1 │ NULL    │   ... choose column, all rows NULL
== duckdb package: cleanup
dbpkg211-duckdb
containers left under dbpkg211-*: 0
```

The sample runs exactly as drawn; the `choose` column reads NULL against the offline stand-in, the same divergence the surface's `NOTES.md` already records (the stand-in cannot distinguish options). The glibc-2.28 rebuild is the packaging fix that remains for the real artifact; this rehearsal is its first proof.

One trap for the next person: `set -e` plus `docker start -ai` exits non-zero when the sample errors, so cleanup must be a `trap` (the script now has one), not a final line. The first run left `dbpkg211-duckdb` behind because of it; it was removed by hand and the trap added.
