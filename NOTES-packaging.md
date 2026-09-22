# Packaging rehearsal, 2026-09-21

The brief's item 5, run as two lanes, languages first and databases after; the database sections follow the language ones. build each surface's real installable package, install it in a clean container from the package file, and run one slide sample from the marketing deck (`repos/mktg/decks/2026-09-21-thinkthen-semantic-commands/surfaces.md`) exactly as drawn. Version 0.0.1 everywhere; the first release is 0.1.0 and is not this. Linux first; the macOS attempts by cross-compilation follow in their own section, recorded honestly. Every container is disposable, named `pkg211-*`, removed with `docker rm -f -v`; every image pull and removal is recorded here.

The cache default, stated once for all six packages: the ruled default folder is `$XDG_CACHE_HOME/thinkthen` on Linux and `~/Library/Caches/thinkthen` on macOS. The stand-in reads `THINKTHEN_CACHE` into the settings and writes nothing to disk today — no cache code path writes files yet — so the rehearsal could not observe a folder written, for any surface. The statement holds for the real engine that lands behind the same settings.

## Python

Build, from `libraries/python`:

```
$ maturin build --release
📦 Built wheel for abi3 Python ≥ 3.10 to libraries/python/target/wheels/thinkthen-0.0.1-cp310-abi3-manylinux_2_39_x86_64.whl
```

Clean container: `docker pull python:3.12-slim` (Debian glibc 2.41, Python 3.12.14). Install from the wheel file, not from the source tree and not `maturin develop`:

```
$ docker run --rm --name pkg211-python \
    -v /tmp/pkg211/python:/work \
    -v libraries/python/target/wheels:/wheels:ro \
    python:3.12-slim bash -c \
    "pip install --quiet /wheels/thinkthen-0.0.1-cp310-abi3-manylinux_2_39_x86_64.whl pandas \
     && cd /work && ENGINE_NULL=1 python slide_sample.py"
decide          -> True   (comment: True)
band decide     -> False   (comment: None)
filter          -> []   (comment: none drawn)
annotate        -> columns ['body', 'team', 'urgency', 'wants_refund']
  three new columns: ['team', 'urgency', 'wants_refund']
  first row: {'body': 'I renewed once this morning, but my card shows two charges.\nPlease refund the duplicate.', 'team': None, 'urgency': 1.7, 'wants_refund': True}

MISMATCH  band decide: the comment says None, the run gave False; against the three-bucket stub this evidence carries no keyword, so its probability falls below the band
findings are reported, not hidden; the slide changes, not the sample
```

The `band decide` mismatch is the sample's own recorded finding, unchanged by packaging: the stand-in's keyword rule puts that evidence under the band, and the comment is the real backend's answer. `decide` and `annotate` reproduce their comments against the stand-in.

Installed-package evidence, second clean container:

```
$ pip show thinkthen | grep -E '^(Name|Version|Location)'
Name: thinkthen
Version: 0.0.1
Location: /usr/local/lib/python3.12/site-packages
$ python -c 'import thinkthen; print(thinkthen.__file__)'
/usr/local/lib/python3.12/site-packages/thinkthen/__init__.py
```

Registry use: the wheel came from the local file; `pandas` came from PyPI inside the container. Nothing was published. Pull recorded: `python:3.12-slim`. Containers: `--rm`, none left.

## TypeScript

Build, from `libraries/typescript` (napi CLI from the folder's own dev-dependencies):

```
$ npm run build
> napi build --release --platform --cargo-cwd ./addon --js loader.cjs --dts loader.d.ts .
$ npm pack
thinkthen-0.0.1.tgz
$ tar tzf thinkthen-0.0.1.tgz
package/loader.cjs
package/index.js
package/package.json
package/README.md
package/index.mjs
package/index.linux-x64-gnu.node
package/index.d.ts
```

Clean container: `docker pull node:22-slim` (Node v22.23.2). Install from the tarball, then run the deck's TypeScript sample exactly as drawn:

```
$ docker run --rm --name pkg211-typescript \
    -v /tmp/pkg211/typescript:/work -v libraries/typescript:/pkg:ro node:22-slim bash -c \
    "mkdir -p /app && cd /app && npm init -y >/dev/null && npm install --silent /pkg/thinkthen-0.0.1.tgz \
     && cp /work/sample.mjs . && ENGINE_NULL=1 node sample.mjs"
ThinkThenError: a choose reply carried no choice
    at invoke (/app/node_modules/thinkthen/index.js:104:11)
    at async Module.choose (/app/node_modules/thinkthen/index.js:144:11)
  kind: 'defect',
  retryable: false
```

**Finding, the deck's TypeScript sample cannot run as drawn.** The one-shape ruling of 2026-09-21 (`sdlc/issues/2026-09-21-one-shape-for-nine-surfaces-as-the-slides-show-it.md:32`) says "TypeScript takes one options object last: `tt.choose(question, text, { options })`, `tt.tag(question, text, { labels })`, `tt.rank(question, records, { top, signal })`". The binding implements the last object as call options only (`signal`, deadline) and builds the question from the first argument alone; a bare string first argument becomes a `decide` question, so `choose` and `tag` raise `defect` and `rank` ignores `top`. Host probe, null backend and wire stub both:

```
choose {options}-last  -> ERROR defect | a choose reply carried no choice
choose question-object -> "the refund team"
tag {labels}-last      -> ERROR defect | a tag reply carried no labels
tag question-object    -> []
rank {top,signal}-last -> 3 rows returned for { top: 2 }   (top ignored)
rank question-object   -> 3 rows
```

The transport works: `signal` in the last object reached the engine (the lane's cancel test used it). The gap is the merge of question options (`options`, `labels`, `top`) from the ruled last object into the question. This is a TypeScript surface defect, not a packaging defect; the packaging lane records it and does not fix it. The lane that landed the surface ran the deck section as it stood then (decide/band/filter/annotate); the current section (choose/tag/rank) is new, and this run is its first.

**The installed package answers, through the shape the binding implements** (question object first, call options last), in the same container:

```
$ ENGINE_NULL=1 node sample-supported.mjs
choose -> "the refund team"
tag -> []
rank[0] -> {"index":0,"record":"I want a refund for order 9","probability":0.97}
```

`tag` returning `[]` is the stand-in's rule (no option text carries a keyword); the deck's `["billing", "shipping", "urgent"]` is the real backend's answer. The package installs from the tarball, loads its prebuilt `.node`, and answers all three verbs. Registry use: the tarball came from the local file; nothing was published. Pull recorded: `node:22-slim`. Containers: `--rm`, none left.

## Ruby

Build, from `libraries/ruby` (no Ruby on this host; `build.sh` runs inside a locally built Debian trixie image, `ruby:3.4-trixie` plus `libclang-dev`, mounting the host's Rust toolchain read-only; the image is never pushed and is removed below):

```
$ ./build.sh
built: thinkthen-0.0.1.gem and lib/thinkthen/thinkthen.so
$ ls -la thinkthen-0.0.1.gem
-rw-r--r-- 1 root root 1718272 thinkthen-0.0.1.gem   (sha256 b05dd4d5b40d2cc8…)
```

Clean container, no Rust toolchain present, installing the gem from the local file into a clean `ruby:3.4-trixie` account:

```
$ docker run --rm --name pkg211-ruby \
    -v /tmp/pkg211/ruby:/work -v libraries/ruby:/pkg:ro ruby:3.4-trixie bash -c \
    "which cargo rustc gem ruby; gem install --local --quiet /pkg/thinkthen-0.0.1.gem \
     && gem list thinkthen && cd /tmp && ENGINE_NULL=1 ruby /work/slide_sample.rb"
/usr/local/bin/ruby
Successfully installed thinkthen-0.0.1
1 gem installed
thinkthen (0.0.1)
finding: score returned 1.7, the slide comment says 2.0; nearest level is 'Immediate.'
3 of 8 are complaints
I want a refund for order 9
The refund never arrived
maybe escalate this one
Where is my order?
Hello team
slide sample green
```

No `cargo` or `rustc` was found in the container; the gem carries the compiled extension and installs from the local file alone. The slide sample is the deck's Ruby section run as drawn: filter keeps 3 of 8, rank returns the deck's five, score returns 1.7 with "Immediate." nearest — the score-comment finding is the sample's own, already filed for the slide owner in the marketing repo. Registry use: none; the gem came from the local file. Pulls recorded this section: none new (`ruby:3.4-trixie` was already local from the surface's own work). The builder image `thinkthen-ruby-builder:local` was rebuilt by `build.sh` and removed after the gem was built: `docker rmi thinkthen-ruby-builder:local`.

## R

Build the source tarball, from `libraries/r`:

```
$ R CMD build thinkthen
* building ‘thinkthen_0.0.1.tar.gz’
```

**Finding fixed: the first tarball swept build junk.** The first `R CMD build` produced a 12.8 MB tarball with 3,586 entries, carrying `src/.cargo/` (a 51 MB registry cache), `src/entrypoint.o`, and `src/thinkthen.so`. The package had no `.Rbuildignore`. Added and committed (`67bd500`): `^src/\.cargo$`, `^src/.*\.o$`, `^src/.*\.so$`. The rebuilt tarball is 16,542 bytes with 19 entries and zero build-junk entries.

Clean container: `docker pull rocker/r-ver:4.3` (Ubuntu 22.04, R 4.3.3). Two R packages came from the distribution (`r-cran-jsonlite`, `r-cran-dplyr`); rocker's R hides `/usr/lib/R/site-library`, so `R_LIBS_SITE` is set. The Rust toolchain is mounted read-only from the host, the same pattern as the Ruby builder. Attempt A installs from the local tarball alone; attempt B installs from the source directory with the repository present, which is how R-universe builds:

```
$ docker run --rm --name pkg211-r \
    -v libraries/r:/tarball:ro -v ~/.rustup:/root/.rustup:ro \
    -v /tmp/pkg211/r:/work -v .:/src:rw rocker/r-ver:4.3 bash -c '...'
=== attempt A: install from the tarball alone (no repository present)
  failed to read `/tmp/contract/Cargo.toml`
  No such file or directory (os error 2)
make: *** [Makevars:13: rust/target/release/libthinkthen.a] Error 101
ERROR: compilation failed for package ‘thinkthen’
=== attempt B: install from the source directory inside the repository
* DONE (thinkthen)
=== the slide
slide sample green: 2 rows kept, NA dropped by filter, choose NA on a tie
```

**Finding, by design: the R source tarball is not self-contained.** `src/rust/Cargo.toml` depends on `../../../../../contract` and `../../../../../standin`, so the tarball installs only where the repository's layout exists beside it. R-universe builds from the repository in place, so the ruled path is unaffected. A standalone, CRAN-style tarball would need those two crates vendored into the package or published. The design comment in `tools/config.R` says this is deliberate (“never from a vendor tarball”); the rehearsal records what it means for installers.

The installed package ran the deck's R section as drawn: 2 rows kept, `NA` dropped by `filter()`, and the choose column `NA` on an exact tie — the sample's own assertions, green. macOS: no cross-compilation path exists for an R source package; R-universe's own builders are the macOS path, and the submission stays Ian's. Registry use: none; the tarball and the source install are local. Pulls recorded: `rocker/r-ver:4.3`.

## Rust

`cargo package --list` is clean — the manifest, sources, tests, examples, and the notes files only:

```
$ cargo package --list
Cargo.toml
Cargo.lock
NOTES.md
README.md
check.sh
examples/conformance.rs
src/lib.rs
tests/slide.rs
tests/verbs.rs
tests/wire.rs
```

**Finding fixed: the path dependencies had no versions.** `cargo package` refuses to package a crate whose dependencies carry a `path` without a `version` ("all dependencies must have a version requirement specified when packaging"). Added `version = "0.0.1"` beside the path in all three manifests — `libraries/rust/Cargo.toml`, `contract/Cargo.toml`, `standin/Cargo.toml` — which is the standard packaging fix and changes nothing for local builds (verified with `cargo check` and the surface's own `check.sh`).

**The unpublished-dependency reality.** `cargo package` resolves the whole graph even with `--no-verify`, so with `thinkthen-contract`, `thinkthen-standin`, and `thinkthen-core` unpublished, packaging needs those crates in a resolvable source. The rehearsal built one:

1. `cargo vendor --versioned-dirs /tmp/pkg211/vendor` collected the 61 registry crates.
2. The three unpublished crates were placed beside them with their `path` deps stripped to version-only, workspace-inherited fields (`edition.workspace`, `rust-version.workspace`, `[lints] workspace`) resolved to literals for `thinkthen-core`, and `.cargo-checksum.json` files generated (excluding the checksum file itself, a trap the first attempt hit).
3. `cargo package --no-verify --config 'source.crates-io.replace-with="vendored-sources"' --config 'source.vendored-sources.directory="/tmp/pkg211/vendor"'` produced the artifact:

```
$ cargo package --no-verify --config ...
    Packaged 12 files, 61.0KiB (16.7KiB compressed)
$ ls -la target/package/thinkthen-0.0.1.crate
-rw-rw-r-- 1 ian ian 17104 thinkthen-0.0.1.crate
```

`--no-verify` skips cargo's verification build; without it cargo would try to fetch the unpublished dependencies from crates.io. The packaged `Cargo.toml` carries version-only deps, exactly as publishing would write them.

**The scratch install.** A project at `/tmp/pkg211/rust-scratch` that has never seen the repository: `thinkthen = "0.0.1"` in its manifest, a `.cargo/config.toml` pointing crates.io at the vendored source, and the deck's Rust sample in `src/main.rs` — with the one filed fix, the `?` after `Question::decide`, which the lane's own record documents. The packaged `thinkthen-0.0.1.crate` was extracted into that source first, so the consumer resolves the artifact cargo built from the crate file.

```
$ ENGINE_NULL=1 cargo run
refunds: 1, review: 0
```

The sample's match arms ran against the packaged crate: the refund ticket lands in `refunds` (the stand-in answers Yes at 0.97), `review` stays empty.

macOS: the Rust package is source, not a binary artifact; cargo builds it on the target machine, so no macOS artifact is produced or needed here. Registry use: none; the `.crate` is local and was consumed from the local file. Pulls recorded: none new (cargo fetched from crates.io into the host cache, as every build does).

## C

Build, from `libraries/c`: the shared library, the static library, and the header beside them.

```
$ cargo build --release
$ ls -l target/release/libthinkthen.so target/release/libthinkthen.a contract/include/thinkthen.h
-rwxrwxr-x 4273760 target/release/libthinkthen.so
-rw-rw-r--  37126320 target/release/libthinkthen.a
-rw-rw-r--     5107 contract/include/thinkthen.h
```

Clean container: `docker pull ubuntu:24.04`, only `gcc` added from the distribution. The three files are copied in — no repository, no Rust toolchain, no build of the library — and the deck's C sample compiles with a plain `cc` and runs:

```
$ docker run --rm --name pkg211-c -v /tmp/pkg211/c:/work ubuntu:24.04 bash -c \
    'apt-get install -y gcc && cc ... slide.c -L. -lthinkthen -Wl,-rpath,\$ORIGIN -o slide && ENGINE_NULL=1 ./slide'
cc (Ubuntu 13.3.0-6ubuntu2~24.04.1) 13.3.0
	libthinkthen.so => /work/./libthinkthen.so (0x00007f8c89e00000)
slide ok: decide YES at 0.97, decide_many YES NO YES
```

The static archive is complete too — the same sample linked against `./libthinkthen.a -lpthread -ldl -lm` runs with the identical answer:

```
$ cc ... slide.c ./libthinkthen.a -lpthread -ldl -lm -o slide-static && ENGINE_NULL=1 ./slide-static
slide ok: decide YES at 0.97, decide_many YES NO YES
```

The C sample's answer classes match its slide comments under the stand-in: decide Yes at 0.97 (the comment promises 0.99 on the real backend), decide_many YES/NO/YES. Each surface's own section above carries its findings where a comment did not reproduce. Registry use: none; the three files were copied from the local build. Pulls recorded: `ubuntu:24.04`.

## macOS

Cross-compilation was attempted with `cargo-zigbuild` (already installed, v0.23.4; `zig` at `~/.local/bin/zig`) after `rustup target add aarch64-apple-darwin`. Results, honestly:

| Surface | Attempt | Result |
| --- | --- | --- |
| C | `cargo zigbuild --release --target aarch64-apple-darwin` | **Real artifact.** `target/aarch64-apple-darwin/release/libthinkthen.dylib`, Mach-O 64-bit arm64 dynamically linked shared library (5,380,512 bytes), and `libthinkthen.a` (ar archive, 24,660,224 bytes) |
| Python | `CARGO=cargo-zigbuild maturin build --release --target aarch64-apple-darwin` | **Real artifact.** `thinkthen-0.0.1-cp310-abi3-macosx_11_0_arm64.whl`; its `thinkthen/_thinkthen.abi3.so` is Mach-O arm64 |
| Rust | `cargo zigbuild --release --target aarch64-apple-darwin` | The crate compiles for the target, but the package is source: the `.crate` is platform-neutral and no binary artifact is part of it |
| TypeScript | `napi build --target aarch64-apple-darwin`, plain and with `CARGO=cargo-zigbuild` | **Failed.** napi-rs's own zig-linker script rejects the exported-symbols argument ("unsupported linker arg: /tmp/rustc…/list"); the plain path demands an Xcode SDK. No darwin `.node`; this one needs the Mac or a configured SDK |
| Ruby | `cargo zigbuild --target aarch64-apple-darwin` | **Failed.** `rb-sys` panics (`ruby not found`) without a Ruby toolchain to read config from; no macOS gem artifact from this box |
| R | not attempted | R source packages have no local cross path; R-universe's own builders are the macOS path, and the submission stays Ian's |

So the final macOS state, after both Mac visits: **eight of the nine surfaces carry a real darwin-arm64 artifact or a platform-neutral source answer, and R is the one by design.** C and Python (cross-compiled above), TypeScript (`libraries/typescript/dist/thinkthen-0.0.1-darwin-arm64.tgz`, built on the Mac), Ruby (`libraries/ruby/dist/thinkthen-0.0.1.gem`, built on the Mac), the three database extensions (SQLite `thinkthen.dylib`, DuckDB `thinkthen-osx_arm64.duckdb_extension`, PostgreSQL `thinkthen-pg16-0.0.1-darwin-arm64.tar.gz`, all built on the Mac), Rust (source by design), and R (source through R-universe, which builds its own macOS binaries; the submission stays Ian's).

Incidental note: the Ruby builder container ran as root and left root-owned artifacts under `libraries/ruby/target/`; a host rebuild of that folder needs a fresh `CARGO_TARGET_DIR` (the macOS attempt above used `/tmp/pkg211/ruby-cross`) or a chown this session cannot do without sudo.





## Databases

Added by the databases lane (DuckDB, SQLite, PostgreSQL), sharing this file with the languages lane's sections above. Same rules: one version 0.0.1, local installs from package files, disposable containers named `dbpkg211-*` removed with `docker rm -f -v`, samples run against the offline stand-in (`ENGINE_NULL=1`), no key, no paid call, nothing published. Tools added for these two lanes' cross builds: `cargo install cargo-zigbuild --locked` → cargo-zigbuild 0.23.4; zig 0.15.2 was already at `~/.local/bin/zig`.

Names become rows is not rehearsed here: that slide is the `recognize` slide, and `recognize` is not built yet; none of the three database samples shows the pattern, so that proof waits for the recognize wave.

### DuckDB

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

### SQLite

Package: `dist/thinkthen.so`, the loadable extension under the name the slide's `.load ./thinkthen` resolves. Built with the same glibc pin as DuckDB:

```
$ RUSTFLAGS="-L /usr/lib/x86_64-linux-gnu" cargo zigbuild --release --target x86_64-unknown-linux-gnu.2.28
    Finished `release` profile [optimized] target(s) in 22.30s
$ objdump -T target/x86_64-unknown-linux-gnu/release/libthinkthen0.so | grep -o 'GLIBC_[0-9.]*' | sort -V | uniq | tail -1
GLIBC_2.28
$ sha256sum dist/thinkthen.so
37ed53e547cdd7972668fdd3d2f8852eb30cc55d2b6821da86cfc53b054c3a34  dist/thinkthen.so
```

The `RUSTFLAGS -L` is load-bearing: the extension links the host's `libsqlite3` for its interrupt call (`#[link(name = "sqlite3")]`), and zig has no stub for it. The artifact still demands only `libsqlite3.so.0` at runtime, which every SQLite host has.

Container: `ubuntu:24.04`, already present on this box (digest `ubuntu@sha256:008173c23f95b170204355c12626cb5a965d779a7e1283b09e9cffbb1bf33ca3`); stock sqlite3 installed from the distro:

```
$ docker exec dbpkg211-sqlite sqlite3 --version
3.45.1 2024-01-30 16:01:20 e876e51a0ed5c5b3126f52e532044363a014bc594cfefa87ffb5b82257ccalt1 (64-bit)
```

3.45.1 is above the 3.41 support floor the surface states. `databases/sqlite/package.sh` is the whole flow (build, stage, container, sample, cleanup); the sample runs as drawn from the slide file itself:

```
== sqlite package: the slide sample, as drawn, in the container
5
1|i want a refund now
3|refund, please
4|maybe later
3
== sqlite package: cleanup
containers left under dbpkg211-*: 0
```

Warm says 5, the WHERE kept rows 1, 3, 4, the count says 3 — the shape the surface's own NOTES recorded, now proven from the packaged file in a clean container.

Two traps recorded for the next packaging run: `cargo zigbuild` writes to `target/<triple without the glibc suffix>/`, so the glibc pin belongs in the target argument only, never in the artifact path; and the first runs of both database containers omitted `ENGINE_NULL=1` and the sample failed `Connection refused` against the real address — the null backend is a container environment variable, and no key or paid call is ever involved in these rehearsals.

### PostgreSQL

Package: `dist/thinkthen-pg16-0.0.1-linux-amd64.tar.gz`, sha256 `86abd7a9860ab2fa4fe63e3820ebf30088904d4f779a6779e892944dc8c4ae67`, holding the pgrx package tree for PostgreSQL 16:

```
usr/lib/postgresql/16/lib/thinkthen.so
usr/share/postgresql/16/extension/thinkthen.control
usr/share/postgresql/16/extension/thinkthen--0.0.1.sql
```

`cargo pgrx package --pg-config /usr/bin/pg_config` is the build; the install lines ship in the tarball's README: copy the library and the two extension files into a PostgreSQL 16 tree, then `CREATE EXTENSION thinkthen;` per database.

Container: `postgres:16`, already present (digest `postgres@sha256:f1c3376c26f2609ab9f29f71f824103fe2fcd8ee0346485cb6122a4f93df6f94`). The image is Debian trixie, glibc 2.41, so the host-built extension (glibc 2.39) loads; an older PostgreSQL image on glibc 2.36 would refuse it, and pgrx does not produce a glibc-pinned artifact from this box — that exposure is recorded here as the open item, unchecked rather than fixed, unlike DuckDB and SQLite which carry the zig pin.

The sample runs exactly as drawn, extracted verbatim from the deck by the package script:

```
== postgres package: the slide sample, as drawn
 id |                                 body
  2 | maybe this is on our side, but the charge looks wrong. Can you check?

 id | team | urgency
  1 |      |     1.7
  2 |      |     1.05
  3 |      |     0.99
slide sample green: the maybe row reads, urgency ordered 1.7/1.05/0.99
```

`CREATE EXTENSION` is instant, `@refund.json` and `form.json` resolve from the backend's working directory (the fixtures are copied there, as the surface's own checks do), and the container is removed afterwards: `containers left under dbpkg211-*: 0`.

One trap the first run exposed and the notes keep: `VERSION` holds a comment line above the number, so `cat ../../VERSION` smuggled the comment into artifact names and README titles. All three database package scripts now use `tail -1 ../../VERSION`, and the mangled artifacts were rebuilt.

### macOS cross attempts (databases)

None of the three database surfaces produces a macOS artifact from this box today. Each attempt is recorded with its exact blocker, and all three are unchecked-until-the-Mac rather than pretended.

Prepared once: `rustup target add aarch64-apple-darwin` (rust-std for the pinned 1.93.1 toolchain; rustup recorded the component download). cargo-zigbuild 0.23.4 as in the preamble.

- **SQLite.** `cargo zigbuild --release --target aarch64-apple-darwin` compiles the Rust for arm64 macOS and stops at the link: `error: unable to find dynamic system library 'sqlite3' using strategy 'paths_first'. searched paths: .../cargo-zigbuild/0.23.4/deps/libsqlite3.tbd`. The stub set cargo-zigbuild carries has `libiconv.tbd` and `libcharset.tbd`, not sqlite3; zig 0.15.2's Darwin libc carries `libSystem.tbd` and nothing more. The blocker is the macOS SDK stub for `libsqlite3`, which the Mac has and this box does not. A minimal hand-written `.tbd` could force a link, but it would be a fabricated stub whose load behavior is untestable here, so it was not done; the artifact is unchecked.
- **DuckDB.** `DUCKDB_EXTENSION_NAME=thinkthen DUCKDB_EXTENSION_MIN_DUCKDB_VERSION=v1.5.5 cargo zigbuild --release --target aarch64-apple-darwin` fails at the link: `error: unable to find framework 'CoreFoundation'. searched paths: none`. The same class of blocker: the macOS SDK's framework stubs are absent (`ring` on Apple reaches for CoreFoundation), and zig's Darwin set here is libSystem only. When this is built on the Mac, the metadata append takes `-p osx_arm64`.
- **PostgreSQL.** `cargo pgrx package --pg-config /usr/bin/pg_config --target aarch64-apple-darwin` fails before any link, inside a dependency's C: `error: failed to run custom build command for \`ring v0.17.14\`` … `cc-rs: command did not execute successfully … "sccache" "cc" … "-arch" "arm64"`. pgrx drives the host `cc`, not a cross toolchain, and offers no cross path from this box.

The path when the Mac is available: `make release` (DuckDB, then the metadata append), `cargo build --release` plus the metadata append (SQLite), and `cargo pgrx package` (PostgreSQL) run natively there; or revive the zig route by pointing it at a real macOS SDK. Both are recorded as the path, not done.

## 2026-09-21, second pass: re-verified against the changed tree, uv beside pip (lane B item 7)

The first rehearsal predates the shapes work, the StringView reader, the cancel fixes, and the examples files. Every artifact below was rebuilt from this tree and reinstalled from its package file into a clean container; one examples-file set (ten functions) or slide sample ran per surface. Language containers are `laneb7-*`; the database sections ran the repository's own `package.sh` flows under their recorded `dbpkg211-*` names — the scripts are the rehearsal, and nothing was renamed for a prefix. Zero containers remain under `laneb7-`, `pkg211-`, or `dbpkg211-` (`docker ps -a` filtered).

**Python — wheel, pip and uv.** Rebuilt with `maturin build --release`. Container `laneb7-python-pip`, `python:3.12-slim`: wheel install plus the fixture, then all ten `examples.json` entries executed and compared — **10 ok, 0 mismatched, 0 failed** (the runner was written for this rehearsal; it is not committed). The uv proof, second container, same image: `pip install` of the wheel 1.74 s; `uv venv` 0.04 s plus `uv pip install` 0.06 s; the same runner through the uv venv — **10/10**; `uv` itself came from `pip` inside the container, the host was never touched (`grep -cE "uv|deno" ~/.zshrc` → 0). Install lines: `pip install thinkthen-0.0.1-cp310-abi3-manylinux_2_39_x86_64.whl`, or `uv venv` + `uv pip install <same wheel>`. Caveat: one pass each, warm image, local wheel — the uv win is a cold-start effect, real here and not benchmarked further.

**TypeScript — npm tarball.** `npm run build` + `npm pack` fresh. Container `laneb7-typescript`, `node:22-slim`: tarball installed into a scratch app, all ten examples run — **10 ok, 0 bad**, including `choose` and `tag` with the ruled `{ options }` / `{ labels }` last object, which raised `defect` in the first rehearsal and now answer — the fix since then holds through the packaged file. Install line: `npm install ./thinkthen-0.0.1.tgz`.

**Ruby — gem.** Rebuilt through `build.sh` (the builder image `thinkthen-ruby-builder:local` was rebuilt from the Dockerfile and removed after: `docker rmi` recorded). Container `laneb7-ruby`, `ruby:3.4-trixie`, no `cargo`/`rustc` present: `gem install --local --quiet` from the file, then `tests/examples.rb` — **10 of 10 examples ok**. Install line: `gem install --local thinkthen-0.0.1.gem`.

**R — source tarball.** `R CMD build thinkthen` under host R 4.3.3: the tarball is 20,342 bytes with 19 entries (the first pass's junk fix holds). Container `laneb7-r`, `rocker/r-ver:4.3`: attempt A, the tarball alone, fails exactly as designed — `failed to read /tmp/contract/Cargo.toml` — the recorded not-self-contained finding reproduces on today's tree; attempt B, the source directory inside the repository with the mounted rustup toolchain, installs into `rlib` and `examples.R` runs **10 of 10**. Install line: `R CMD INSTALL -l rlib <repo>/libraries/r/thinkthen`; the R-universe path stays the ruled one.

**Rust — crate.** The vendored source was rebuilt from this tree: 61 registry crates re-vendored, the four local crates re-staged with version-only deps and fresh checksums (`/tmp/laneb7/stage_vendor.py`). `cargo package --no-verify` produced 17 files, 30,025 bytes. The scratch consumer at `/tmp/pkg211/rust-scratch` (never seen the repository), pointed at the new vendor and the fresh crate: `cargo run` → **refunds: 1, review: 0**. Install line: `thinkthen = "0.0.1"` against the packaged `.crate` in a vendored source (the unpublished-dependency reality is unchanged).

**C — shared and static.** Rebuilt; container `laneb7-c`, `ubuntu:24.04` with only `gcc`: shared `cc slide.c -I. -L. -lthinkthen -Wl,-rpath,$ORIGIN` → `slide ok: decide YES at 0.97, decide_many YES NO YES`; static `cc slide.c -I. ./libthinkthen.a -lpthread -ldl -lm` → the same line. Install line: copy `libthinkthen.so`, `libthinkthen.a`, and `thinkthen.h`; link either way.

**The three databases — their own `package.sh` flows, green as before.** DuckDB in `duckdb/duckdb:1.5.5`: slide as drawn, `choose` NULL as recorded; SQLite in stock `ubuntu:24.04` sqlite3 3.45.1: warn 5, kept 1/3/4, count 3; PostgreSQL in `postgres:16`: slide green, urgency 1.7/1.05/0.99; each cleanup reports 0 leftovers. **One honesty caveat: at build time the working tree carried an uncommitted change in `databases/sqlite/src/lib.rs` (another lane's version-floor work in progress), so the SQLite artifact tested here included it — a rehearsal of the working tree, not of HEAD.** The other folders were clean.

**Ledger for this pass:** what broke — nothing new; two script bugs on my side (a missing `-I.` on the static C link, a lost dependency step in the first R rerun), both fixed and rerun; what held — every first-pass fix (Ruby builder cleanup, R junk fix, DuckDB/SQLite glibc pin, pgrx trap, `tail -1 VERSION`). New: **uv works beside pip, faster on a local wheel, and changes nothing in what gets installed.**

## 2026-09-22, the Mac lane: four real darwin-arm64 artifacts from m5

Ian made M5 available over Tailscale SSH (macOS 26.4, arm64, user imaurer), and this lane built the five missing macOS artifacts there as a guest: scratch under `~/tmp/thinkthen-macos/`, removed at the end; the scratch clone at `c3616dc`; nothing published; no key; no paid call. The guest's toolchain: cargo/rustc 1.95.0 (Homebrew - no rustup, so the repo's 1.93.1 pin is inert there; recorded as a known build-version difference), node v26.0.0, zerobrew's postgresql@16 and sqlite 3.51, Apple's `/usr/bin/sqlite3`, and system ruby 2.6.10.

**What landed, each verified by my own run, each beside its build log in the ignored `dist/`:**

- **TypeScript** - `libraries/typescript/dist/thinkthen-0.0.1-darwin-arm64.tgz` (1,426,192 bytes, `index.darwin-arm64.node` inside) with `macos-build.log`. Verified by installing the tarball into a scratch app and running the examples runner: 5 ok, 0 bad. **Build finding:** the link fails without `RUSTFLAGS="-C link-arg=-undefined -C link-arg=dynamic_lookup"` - the napi host symbols need dynamic lookup on macOS, and the repo's addon carries no macOS link flags. The repo should ship them (`.cargo/config.toml` in `addon/` with the two link args).
- **SQLite** - `databases/sqlite/dist/thinkthen.dylib` (3,656,432 bytes) with `macos-build.log`. `cargo build --release` produced `libthinkthen0.dylib` exporting `_sqlite3_thinkthen_init`; staged as `thinkthen.dylib`; **verified loading and answering in the guest's real sqlite3**: `ENGINE_NULL=1 sqlite3 :memory: ".load ./thinkthen" "select thinkthen_decide(...)"` -> `1`. Caveat recorded: Apple's `/usr/bin/sqlite3` refuses `.load` (extension loading omitted), so the proof used zerobrew's stock sqlite3 3.51.
- **DuckDB** - `databases/duckdb/dist/thinkthen-osx_arm64.duckdb_extension` (3,843,846 bytes) with `macos-build.log`. `make configure` wrote platform `osx_arm64` and `make release` appended the metadata footer (duckdb v1.5.5, abi `C_STRUCT_UNSTABLE`). **Verified in the official v1.5.5 macOS CLI** (downloaded to the scratch): `LOAD` with an **absolute** path (the hardened CLI refuses relative) and `thinkthen_decide` answered `true`.
- **PostgreSQL** - `databases/postgresql/dist/thinkthen-pg16-0.0.1-darwin-arm64.tar.gz` (1,789,187 bytes, dylib + control + sql) with `macos-build.log`. `cargo pgrx init --pg16` against zerobrew's pg_config, then `cargo pgrx package --pg-config ...` with the same dynamic-lookup flags. **Verified in a scratch cluster** on port 5499: `CREATE EXTENSION thinkthen` and `thinkthen_decide('{"decide": "..."}', '...')` returned `t`. The shared extension dirs were restored afterward (`ls | grep -c thinkthen` -> 0); no service was touched; no Postgres was running.

**Blocked, named:** **Ruby** - the guest has only system ruby 2.6.10 and the gemspec requires >= 3.1; zerobrew carries no ruby. Unblock: install a modern ruby on the guest (zerobrew/brew ruby or ruby-install) and run `build.sh` with it. Everything else about the gem was proven on Linux.

**Guest bookkeeping:** the scratch directory was removed (`rm -rf ~/tmp/thinkthen-macos`); the one install that outlives it is pgrx's home at `~/.pgrx` (config plus its empty data-16), created by `cargo pgrx init` and justified as necessary for the package build - removal is `rm -rf ~/.pgrx`. The duckdb CLI download and the clone's npm/venv/pip work all lived inside the scratch and are gone; zerobrew's packages were used, not changed; no service was started beyond the scratch Postgres, which was stopped and deleted; no postgres process runs now.

**Repo findings this lane hands to the surfaces:** the two macOS link-flag sites (TypeScript addon, pgrx crate) belong in the repo so no future macOS build needs magic environment; the DuckDB `LOAD` path caveat belongs in the dist README beside the artifact; and the apple sqlite3 caveat belongs in the SQLite README's macOS note.


## The Mac visits, closed 2026-09-22

Two visits to guest `m5` (macOS 26.4, arm64, over Tailscale SSH), each under the guest rules: scratch under `~/tmp/`, every install recorded with its removal, nothing published, no key, no paid call, no project data touched.

**First visit** closed TypeScript, SQLite, DuckDB, and PostgreSQL, each with its build log beside the artifact in `dist/macos-build.log`: the TypeScript `.node` in its `.tgz` (the scratch install ran the examples runner), the SQLite dylib loaded in a real `sqlite3` CLI, the DuckDB extension loaded in the official v1.5.5 macOS CLI (the `osx_arm64` metadata footer present), and the pgrx PostgreSQL package. All four artifacts verified Mach-O arm64 on this box.

**Second visit** closed Ruby - the fifth - with this recipe for the next build:

1. `ruby-build 3.4.10` compiled into the scratch (`RUBY_CONFIGURE_OPTS="--disable-install-doc --with-out-ext=psych"`; ruby-build built OpenSSL 3 into `ruby/openssl` itself). psych is excluded from core because macOS carries no libyaml; libyaml 0.2.5 is built from source into the scratch and psych 5.2.2 is then built from the Ruby source tree's `ext/psych` and installed by hand - RubyGems cannot install anything without psych, so the usual `gem install psych` is the chicken-and-egg this sidesteps.
2. `rustup` into the scratch (`--profile minimal --no-modify-path`); the workspace pins 1.93.1.
3. Sources rsynced from the worktree (`crates/`, `contract/`, `standin/`, `libraries/ruby/`, `conformance/`, `VERSION`, and the root `Cargo.toml`/`Cargo.lock`/`rust-toolchain.toml` - the root manifest is required because `thinkthen-core` inherits `edition.workspace`).
4. `RUSTFLAGS="-C link-arg=-Wl,-undefined,dynamic_lookup"` on the cargo build: on Darwin the Ruby extension must resolve `rb_*` symbols at load, not link time. This is now carried in the repo at `libraries/ruby/.cargo/config.toml` for both darwin targets, so the next build needs no environment magic.
5. The extension file is `lib/thinkthen/thinkthen.bundle` - macOS Ruby loads `.bundle`, not `.so`; the gem carries both names (the `.bundle` that loads, the `.so` the gemspec lists), so **the repo should teach `build.sh`/the gemspec the platform name (darwin -> `.bundle`) before the next macOS build**.
6. Suite on the guest, null backend: `cargo test --lib` 2 passed; `test_surface.rb` 32 runs, 99 assertions, 0 failures; `test_cancel_fast.rb` the raise holds (1.056 s); `test_fork.rb` the child answers; `examples.rb` 10 of 10; `test_conformance.rb` the slice green; `slide_sample.rb` green. Then the gem installed into the scratch gem home and a fresh `ruby -e 'require "thinkthen"'` outside `-I lib`: `decide` true, `recognize` 3 entities, first "Maria Chen". Every line is in `libraries/ruby/dist/macos-build.log`; the gem was re-verified Mach-O arm64 on this box, both member files.

**R - answered, not installed.** The guest carries no R: `R`/`Rscript` absent from PATH, no `/Library/Frameworks/R.framework`, no Homebrew. Nothing was installed. R's macOS path is the source tarball through R-universe, which builds its own macOS binaries; a local macOS R would be a verification nicety, not a distribution blocker. If ever wanted: Homebrew plus `brew install r` (a system-level change to Ian's machine) or the CRAN installer.

**Removals, both visits:** the whole second visit lived in `~/tmp/thinkthen-macos` - Ruby, rustup, libyaml, gems, sources - removed with `rm -rf ~/tmp/thinkthen-macos` (verified gone; `~/tmp` back to 0 B), plus `/tmp/ruby-build.*` and the gemcheck dirs. From the first visit, `~/.pgrx` remains with its recorded removal (`rm -rf ~/.pgrx`). No user-level install was made in either visit.
