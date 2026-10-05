# 0403 A source and pin preparation handback

2026-10-05. Preparation candidate only; **A is incomplete**. Fresh independent High review of this source and these pins is the next prerequisite. No stock native build or load has run, and B's combined repository archive has not started.

The base is main `26174408f620c1771e70e6fbe495eab4e6c07e67`; the portable proof source is `8dd2fb63d781faa62c7b9ea754dd9c77a5302641` on `ticket/0403-duckdb-extension-for-dbt-v2`. The final record commit adds this handback and receipt inventory without changing executable inputs. The [complete accepted design](../planning/0403-accepted-design.md) is retained byte-for-byte at SHA-256 `904eb42d8922e1d9c0b4bd3c8090d1805b27a341381779df62f480634773b9bb`, with its [High acceptance](0403-design-acceptance.md). Integration uses the current base rather than the design's older main.

## Official prerequisites prepared

The [asset inventory](0403-a-pin-assets.json) records official URLs, independent downloaded SHA-256 values, release-provided digests, exact static member counts, header hashes, CLI binary hashes, Python wheels, and retained local paths. All four targets have assets for both versions; none is omitted or substituted.

| Input | Prepared evidence |
| --- | --- |
| v1.5.5 source | `d8cdaa33fda8df955cc76ef58a280f68f4cd43fa`; 14,601 tracked raw Git members verified in the existing source cache |
| v1.5.4 source | `08e34c447bae34eaee3723cac61f2878b6bdf787`; 14,539 tracked raw Git members verified in an owned local clone |
| CLI archives | Eight official ZIPs, downloaded and hashed; each extracted binary hash verified without executing it |
| Static archives | Eight official ZIPs; two versions across Linux amd64/arm64 and macOS amd64/arm64; 172 static archives plus eight headers |
| Python | Eight official CPython 3.13 wheels: DuckDB 1.5.4 and retained NumPy 2.5.3 for all four targets; downloaded and hashed without installing or importing |

Each Linux static ZIP contains 22 archives and one header; each macOS ZIP contains 21 archives and one header. Four new 1.5.4 linker manifests derive from the actual official members. The four existing 1.5.5 manifests remain unchanged and were independently checked against downloaded official bytes. Source tags and release digests come from [DuckDB's official releases](https://github.com/duckdb/duckdb/releases/tag/v1.5.4); wheels use the official PyPI metadata and files service.

Preparation was separate from offline gates and held shared cache mutation locks. Existing older-host paths, build folders, cached source, bytecode and other lanes were retained. Foreign target assets were prepared locally; no foreign host was used. The conventional Linux newest venv exists; the older venv does not. Python module versions, native platform queries and executable readiness remain unqualified. Missing prerequisites are exit 77, never a passing native receipt.

## Candidate behavior

`DUCKDB_VERSIONS` is the sole supported list. Bounded literal readers reject malformed, duplicate, unknown and incompletely pinned entries. All version/target combinations must have complete pins before selection. Defaults remain the first entry; the newest seven-field `setup --inputs` contract retains its four literal target expectations.

Build routing separates source, static archives, CMake output and final canonical artifacts by version and target. Source identity and exact static membership/digests are checked before and after CMake. Footer verification admits the processed final file; a successful default build alone updates the legacy convenience alias. Older selection cannot relabel or overwrite a newer artifact. C++ API source and core Rust APIs are unchanged; no header workaround or stable C migration is asserted before real compilation.

Checkout readers now select the canonical version/target artifact: harness, check scripts, Python/Node consumers, site smoke SQL, release container preparation, source checks and notices. Release-pack still produces A's existing single default-version root-member archive. It refuses an inherited older selection or missing canonical output even if a legacy alias or obsolete output exists. B's repository layout, publication text and combined archive remain future work.

## Portable receipts and limits

The [receipt manifest](0403-a-preparation-receipts.json) freezes the exact code, runtime, scripts, logs and baseline inventories. Owned scratch and logs remain at `/tmp/thinkthen-0403-prep-JNKtAc` for review.

| Focused check | Result |
| --- | --- |
| Selector/input/reader/scratch-packer/fake-build cases | 10 cases, zero failures, zero errors; counted loopback requests **0** |
| Exact source ratchets | All 53 match; root Rust 118,107; DuckDB Rust 4,062; C++ 1,762; Python 4,742 |
| Rust formatting | Root and bridge checks exit 0 |
| Ticket and syntax checks | Tickets exit 0; eight shell and two Node syntax checks pass |
| Raw source guards | 7,003 tracked members match raw HEAD bytes, modes and links before/after; 2,975 existing executable files, 107 bytecode files and 23 specifically enrolled cache/output directory links preserved |
| Policy | Baseline and candidate both exit **1** with identical output; overall focused exit **1**, no policy PASS |

The inherited policy finding is `site/scripts/recipe-proof.py` omitting `Connection: close` in a `BaseHTTPRequestHandler`. It reproduces on the starting commit's raw source snapshot; both outputs check 254 resolved packages. This preparation preserves the reviewed 0402 recipe proof source and records the outstanding finding without weakening policy.

Fake Cargo/CMake cases exercise real build routing and artifact admission without compiling or loading native code. Planted source mutation during CMake is refused afterward. Other plants cover static input corruption and missing/extra members, source modes/links, assume-unchanged and skip-worktree masking, and Git filter masking; the filter marker is never run by the guard. Scratch cleanup refuses the lane before its first actual deletion. Prior failed and cancelled preparatory attempts are counted in the manifest and do not qualify as passes.

The focused runner uses a private HOME and separate empty npm configurations, offline Cargo/UV, cached Node 22.22.3 and Rust 1.95.0, jobs 2, systemd memory 12G/swap 1G, identical lane HELD lock plus shared toolchain/cache/Cargo locks, and nonlogin Bash. Its Python exec resets SIGPIPE and SIGXFSZ to their defaults. No accepted 0400 runner or source-proof hash was changed or copied.

Python grows 518 measured nonblank lines, from 4,224 to 4,742. Growth covers one shared bounded pin reader, shared input validation/preparation and retained behavioral regression fixtures. Raw Git validation extends the existing archive-tree helper; consumers reuse these helpers rather than duplicating a checker. Rust and C++ source totals do not grow.

## Preserved scope and remaining proof

Reviewed Docs 0402 A/B/C/D source, 350 saved proof entries, six DRAFT recipes, 50 measured carriers, producer bounds, aliases, glossary and public 0.1.2 install remain unchanged. Approved DuckDB core advances while recipe publication and none wording wait on independent experiment reports; this is the documented dependency workaround, not a 0402 completion claim or authorization for an experiment. The documented 0.2 dbt route remains dbt v1 with DuckDB 1.5.5. An unsigned stock 1.5.4 load cannot qualify dbt v2's custom driver or its signed-extension requirement.

After fresh source/pin review, A still needs separately compiled binaries for both versions loading in their real matching stock CLI/Python hosts; full newest and selected older suites; symmetric cross-version and patched-footer refusals; scratch repository version selection; old-format package installed smoke; and actual site replay using final default canonical output. Native C++ ABI, stock input/runtime compatibility and release packaging remain unproved risks. Rebase and fresh review are needed if main advances before proof or landing.

Full test/spec/surfaces/stress, canonical 350 replay, Windows native/tag, remote hardware, CI dispatch, four-target release rehearsal, release/publication, provider calls, experiments and 0415 external submission/signing/spend did not run. The coordinator names broader checkpoints; foreign native and release rehearsal authorization remains separate.

Ian can overturn the A/B split and remaining proof selection in the accepted design. This handback makes no A landing or native qualification decision.
