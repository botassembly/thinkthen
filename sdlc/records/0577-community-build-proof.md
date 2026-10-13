# Local community build proof

The submitted PR source `d5b1598c7efb293c1dc1e64b87298b4d5badb813` reproduces the missing-root-manifest configure failure with the pinned community vcpkg toolchain, exit 2. This is the failure before compilation, not a passing build. The earlier 0415 offline rehearsal omitted vcpkg and does not establish community CI success.

The corrected builds include the actual 0.2 stack integrated from `a65f09ef9` at `ce0fc75172c2e837fc6d2d67cf339aa8b2555c7a`. The final community adapter changes end at `a7bedc5df1ac9ac009266ad8a7b5fec5dc1d26f7`; later commits add documentation and the separate package README verifier. Every build uses Rust 1.95.0, commit `59807616e1fa2540724bfbac14d7976d7e4a3860`, from the repository toolchain file. Linux uses GCC 14.2.1 and CMake 4.0.2. Preparation, compilation and SQL tests use the pinned community commands and vcpkg toolchain; each gate has its own retained log and atomic exit.

## Pinned inputs

| Host ref | DuckDB source | CI tools source | vcpkg source |
| --- | --- | --- | --- |
| 1.5.6 | `069cc9f9b5be802405797faecc284961b07c70ef` | `3fd6109fc01555673b7c7123eba8d0d143fe8ce4` | `84bab45d415d22042bd0b9081aea57f362da3f35` |
| v2.0-cyanoptera | `d4355d01c59362d57036ad377735dd5340276a5b` | `a68570fde10384547868301a12327d5c67909527` | `cd61e1e26a038e82d6550a3ebbe0fbbfe7da78e3` |

The root `vcpkg.json` matches DuckDB's actual extension-template manifest at `a733bb150523d49b88ce882302b413d79c71c9b8`. Its OpenSSL dependency and overlays remain intact. Linux preparation and compilation run in separate native manylinux_2_28 containers; the shared Cargo/Rust homes preserve fetched inputs. The ARM Linux container runs natively on Apple Silicon, without emulation. The local Docker images record their actual digests in the ignored evidence folder; the moving base-image tags do not establish identical image bits to the failed public run.

## Completed gates

| Local platform | Host ref | `make release` | `make test_release` | Pinned hello_world | Proposed hello_world |
| --- | --- | --- | --- | --- | --- |
| Native Linux x86-64 | 1.5.6 | 0 | 0: 35 assertions, 2 files | 0: records = 1 | 0: records = 1 |
| Native Linux ARM64 | 1.5.6 | 0 | 0: 35 assertions, 2 files | 0: records = 1 | 0: records = 1 |
| Native Linux x86-64 | v2.0-cyanoptera | 0 | 0: 2 files, zero skips | 0: records = 1 | 0: records = 1 |
| Native Linux ARM64 | v2.0-cyanoptera | 0 | 0: 2 files, zero skips | 0: records = 1 | 0: records = 1 |
| Native macOS ARM64, supplemental | 1.5.6 | 0 | 0: 35 assertions, 2 files | 0: records = 1 | 0: records = 1 |
| Native macOS ARM64, supplemental | v2.0-cyanoptera | 0 | 0: 2 files, zero skips | 0: records = 1 | 0: records = 1 |

The Mac runs macOS 26.4, Apple Clang 21.0.0 and SDK 26.5, with CMake 4.0.2 and the same Rust pin. This differs from the community macOS 15 runner and establishes supplemental local proof. These local Mac measurements cover native ARM only. The 2.0 CLI reports `v2.0.0-dev85888` at `d4355d01c5`; this is the pinned preview, not a released 2.0 version.

A comparison of 1,113 build-input files against the ticket source finds zero differences for both x86 snapshots, ARM 1.5.6 and both Mac snapshots. ARM 2.0 differs only in `extension_config.cmake`: its build predates the later legacy-only JSON declaration inside `NOT COMMAND duckdb_extension_statically_link`. That condition is false on the pinned 2.0 host, which already includes JSON. Its tested C++ repair is `054ebf06051ab42fdea0d1930433efda7f8fce91`, and its test-runner repair is `6e785fbe2cb66d64b38477486f0c11ea407c41e8`. No unchanged adapter was recompiled to obscure this source distinction.

The pinned community `doc_test` job has `if: false`, independent of the earlier Linux failure. Both descriptor queries run locally without a key or model call on all six cells above. That proves the examples, not the disabled documentation generator, public workflow or signing/publication. The inspected generator also needs public statistics and httpfs; it was not substituted with the example check.

## Installed behavior and mechanical checks

The copied final x86 1.5.6 extension passes 183 existing cases across twelve suites: portable, plan, find, verbs, images, files, relate, databases, rank, rank sets, settings and signal. The stock Python host is DuckDB 1.5.6 with the existing pinned NumPy 2.5.3 prerequisite. Each child uses fresh cache/configuration folders and the loopback backend; no paid backend is called. Installed assertions remain unchanged.

The retained backend comes from checkpoint `5b65d99345f0633f0350aa97bd297d179b9c9bac`. Its `conformance/backend`, `conformance/cases.json` and six embedded image reply fixtures have zero source diff against this lane. The binary and its build target were used read-only. The first installed attempt lacked NumPy and failed one portable case; the final complete run after installing the pinned prerequisite passes every suite, with each exit inspected separately.

Formatting, policy, catalog and Python compilation with warnings treated as errors pass. The final policy check also covers the package README verifier. The separate README boundary fixture passes nine CLI cases using the canonical public README committed at `317763b15`: exact directory/archive bytes pass; changed text, an extra member, a missing legal file and README symlinks fail; legacy repositories retain their existing validation. Footer/legal/checksum rejection code and installed assertions were not weakened. These checks cover the community adapter and package text.

## Artifact SHA256

| Artifact | SHA256 |
| --- | --- |
| Linux x86-64 1.5.6 | `22d1c43562e82069e9d92427fa7a6ae341ce91968d6fc777128e5cb454ad4b97` |
| Linux ARM64 1.5.6 | `d456d94a6f23f4f3e25f505a81e7e0770128db1bcb89abc2e64d9dcd1f96a684` |
| Linux x86-64 2.0 preview | `d1a95f370e2ca52e0474f15aff0e599c9d142c2dae68b399978891623989edce` |
| Linux ARM64 2.0 preview | `95e81902110b0cde52849f12e7a667a36a5d2a0b245d0947639043069090fae9` |
| macOS ARM64 1.5.6 | `e2c05626ee530d0d6cf9ba69f13cce51a5711bceb0a882ef299d2d95aaa74018` |
| macOS ARM64 2.0 preview | `0113698e0d8bdc496f736caea200469fee0c163a3d4adb4537c5b4ce91b1c195` |

The PR description, DuckDB README, descriptor and API/LTS recommendation are drafts under `sdlc/records/0577-*`. Both descriptor refs are reviewed-candidate placeholders. No community PR/fork was changed, no public workflow was dispatched and nothing was published.
