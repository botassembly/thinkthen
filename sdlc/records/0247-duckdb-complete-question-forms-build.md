# 0247 DuckDB complete-question forms build

Status: Linux x86-64 implementation source `dc5b428b9f11cb5fddd9456b62e9ab07bbb070e3` built and selected installed proof passed on 2026-09-28. The preceding source slice `9a7c4fb8` held the same behavior with the FFI child at a policy-rejected basename; `dc5b428b` moved that child without changing its contents. Fresh independent High code/package review accepted candidate `795fe61a`; the coordinator integrated the unchanged runtime and proof files. Release qualification is not claimed. The branch includes main `75f756d1`, including the adjacent 0246 engine change, before its final source build. No provider, real user file, public page or other platform package was touched.

## Source and installed artifact

`cpp/build.sh` built the Rust bridge and C++ extension from pinned DuckDB v1.5.5 source/static archives under the lane-specific `/run/user/1000/thinkthen-codex-6.lock`. `release-pack --reuse x86_64-unknown-linux-gnu ... duckdb` packaged exactly that build. The final isolated paths and SHA-256 values are:

| File | SHA-256 |
| --- | --- |
| `databases/duckdb/bridge/target/release/libthinkthen_duckdb_bridge.a` | `85fdc84ff56906eb8166b0e7c6011201cc057a7da9e6a3f32d50a45b6a37b574` |
| `databases/duckdb/build/thinkthen.duckdb_extension` | `471c92a8766c9c9947f9cf4451377a27cf81a6fbd6eeca17adfb82f0843322da` |
| `databases/duckdb/build/0247-policy-package/thinkthen-duckdb-0.0.1-x86_64-unknown-linux-gnu.tar.gz` | `388e8465626f2bf4ec3c16431aa018e4730da621d3cc40b72a5dc85a13bfbd4a` |
| `databases/duckdb/build/0247-policy-installed/thinkthen.duckdb_extension` | `471c92a8766c9c9947f9cf4451377a27cf81a6fbd6eeca17adfb82f0843322da` |

The stock v1.5.5 CLI loaded the final extracted extension. `cpp/verify_package.py` passed its matching v1.5.5 stock-host checks and confirmed that the **unchanged extracted artifact** refuses stock v1.5.4. Those stock checks and all seven selected cases passed again on the final policy-corrected installed path. The build output is isolated in this worktree and remains untracked; it is review evidence, not a release upload. The conformance backend was rebuilt in this lane because its previously warm binary predated the current capture arm, although source was unchanged.

## Selected outside-in proof

The final extracted package passed seven selected `tools/verbs_suite.py` cases: `complete_question_files_keep_identity`, `complete_question_refusals_and_nulls`, `complete_question_rechecks_prepared_authority`, `verbs_answer_through_the_generic_arm`, `r1_1_score_maps_back_by_text`, `an_atfile_read_stops_at_one_mib` and `atfile_reads_through_the_callers_file_system`. The first new case wrote three real named question files under a temporary folder, called choose, score and tag through the installed extension and counted three actual listener sends. It compared all three captured request bodies with separately fixed canonical expectations, including ordered descriptions and saved model. It compared details' `question_sha256` with SHA-256 of separately specified canonical question JSON, including the saved profile, and verified the inline form reused the same identity. Results retained VARCHAR choice, DOUBLE score and VARCHAR[] tags. The three detailed rows were cache hits on those same calls; this is an identity check, not a claim that every repeated call is cached.

The refusal case covered synthetic private-key content in a malformed file, blank, invalid-UTF-8, over-1-MiB and missing files, a valid file of the wrong verb, malformed inline JSON, plain text without members, a mixed invalid chunk, typed NULL and caller file-access denial. Each error retained Local or Usage as specified, the private marker was absent from every message, typed NULL rows remained NULL, and the listener counted zero sends. A prepared file call sent once under allowed access; after `SET enable_external_access=false`, its later execution returned Local with no second send. A separate zero-budget statement returned Deadline with zero sends. The retained choose-with-list `@file` refusal and `/dev/zero` large-file refusal still passed. The latter proves rejection, not a one-byte-over read bound.

Rust formatting, strict offline bridge Clippy, four bridge unit tests, DuckDB source checks, the three derived ratchets, ticket/page checks and diff checks passed. The Rust/bridge and C++ parent files remained below the 500-nonblank-line cap: `ffi.rs` 496 after its explicit child path and `thinkthen.cpp` 435. The first shared-policy run found that the private child held unsafe exports in a filename other than `ffi.rs`. The implementation moved that coherent child to `bridge/src/ffi/complete_listed/ffi.rs`; it did not change its parser, request or error path. The shared policy passed after this correction, and strict Clippy plus the seven selected installed cases passed on the rebuilt final package. No full conformance suite, stress campaign, provider request or cross-platform rebuild was run.

## Growth, reuse and limits

Measured ratchets changed from Rust `6364` to `6497` (+133), C++ `1688` to `1784` (+96), and Python `3525` to `3622` (+97). The Rust child parses full questions and maps public `Details::value()` to the existing result codec; its extra parent line names the policy-compliant FFI child. The C++ child resolves and validates a full chunk under the session owner, groups first-seen texts and decodes through existing `DecodeListed`. The Python child keeps real installed identity and refusal cases outside the already 418-line verb suite. I checked `listed.rs::set/run`, `listed_result.cpp`, `scalar_owner.cpp` and the existing verb suite for removal or reuse before raising the counters. The list route's `QuestionSet`/`Annotated` input cannot retain a complete question's model/profile; changing it would alter the existing three-argument form. File resolution and result decoding are reused. The small C++ validation helper is shared by bind and execution, and the Rust child adds only the separate `Judgment` encoding required by public details.

The selector chooses C++ for Linux x86-64, Linux ARM64, macOS ARM64 and macOS x86-64. Linux x86-64, native Linux ARM64 in M5's Docker VM, native M5 macOS ARM64 and translated Intel macOS on M5 have installed 0247 package proofs. Native Intel hardware, actual macOS 15 execution and the eventual Linux ARM64 release runner remain separate. The original shared-conformance issue stays open for recognize/relate file mapping and final coverage accounting. DuckDB README wording remains in the ticket for the documentation owner; this build did not edit a public page.

## Native M5 macOS ARM64 installed package

The source was the exact landed main commit `aabdeaa5c8f2bff1ba32696fc2acec1b4f533e1e`, checked out separately at a dedicated M5 worktree under `worktrees/` in the workspace. No DuckDB product or proof source changed after the accepted Linux candidate. Before building, M5 reported macOS 26.4 on arm64, load 2.83 on 18 logical CPUs, 94% memory free, 441 GiB disk free and a second disk-I/O sample of 0.08 MiB/s. The prior 0231 worktree and the Ruby/R lane's outputs were left untouched.

The build reused pinned local DuckDB v1.5.5 source, its 21 static archives, project-local CMake 3.31.10, stock v1.5.5 CLI and Python host, stock v1.5.4 negative host and the official Rust 1.95.0 ARM64 toolchain. It fetched or rebuilt no DuckDB dependency. Under `/tmp/thinkthen-codex-6.lock`, `cpp/build.sh` used separate `CARGO_TARGET_DIR` and `THINKTHEN_DUCKDB_CPP_BUILD` paths beneath `databases/duckdb/build/0247-m5`. `release-pack --reuse aarch64-apple-darwin ... duckdb` then packaged that artifact in the same isolated folder. Both commands exited 0. A same-source `conformance-backend` was built in its own target folder with offline Cargo.

| M5 artifact under `databases/duckdb/build/0247-m5` | SHA-256 |
| --- | --- |
| `cargo/release/libthinkthen_duckdb_bridge.a` | `dfe1aad3004e87cf79e667396e09c10e1f56a770d2f958aea52e40ee02791c62` |
| `package/thinkthen-duckdb-0.0.1-aarch64-apple-darwin.tar.gz` | `46f7b4857fb7844aceea02f16b3a18c5d47e8f0525ff652a922ced7d34a47e70` |
| `installed/thinkthen.duckdb_extension` | `994000e80af242e19a67f713d645f77cd3b8c0f0811021f76c8cd596975f496d` |
| `backend-target/debug/conformance-backend` | `aa515b150e8d5428b062ef9711b22d4e93c5f614de38c546b80b3d0e5fd55103` |

The extracted extension is byte-identical to the just-built extension. `lipo` reports one arm64 Mach-O; `otool` reports `minos 15.0`; a raw-byte scan found zero build-account home-prefix occurrences. The package step validated the pinned platform/version footer. Stock v1.5.5 CLI loaded the extracted extension and returned `BOOLEAN` for a typed NULL decision. `cpp/verify_package.py` exited 0: the matching stock v1.5.5 Python host loaded it, that host refused a changed-footer copy, and the **unchanged extracted artifact** was refused by stock v1.5.4. This checks deployment metadata on macOS 26.4, not execution on macOS 15.

The extracted extension passed the seven named `tools/verbs_suite.py` cases above, seven of seven with zero failures, using the same-source loopback backend and temporary child HOME/XDG folders. They retain the fixed canonical request-body and digest checks, named-file secrecy and zero-send refusals, prepared authority recheck, legacy three-argument behavior, typed NULL, large-file refusal and existing result mapping. No provider, whole 54-case suite, full release gate, Intel package or Linux ARM64 build ran. The untracked M5 artifact remains a local proof, not a published package.


### M5 independent review

Fresh High package review accepted `a6877b43cfe475a702886436666e69dcbdebff48`. The reviewer verified the clean source, unchanged DuckDB diff, all four artifact hashes, archive-member identity, ARM64 architecture, minimum-OS metadata, raw-byte scan, 21 pinned static archives and stock-host versions. Independent selected execution returned the typed NULL result, passed version refusal and all seven named cases with an explicit remote zero status.

The retained build and backend commands each prepended `~/.cache/thinkthen-toolchains/rust-1.95.0-official/rustup/toolchains/1.95.0-aarch64-apple-darwin/bin` to their child PATH, where `~` denotes the build account's home. The reviewer inspected those real Cargo/Rustc 1.95.0 executables and the absence of toolchain overrides. The default SSH shell instead resolves Zerobrew tools; it does not describe the build environment. Build logs record success but do not independently print compiler paths, so the retained invocation establishes selection. The CMake intermediate differs because the macOS strip/footer stage follows linking; the packaged, archive-member and extracted bytes match.

## Native Linux ARM64 installed package

The exact source was integrated main `bfe0791ca9583f7169fca9ade0c2ebd47a37d51b`. Its DuckDB product and package source matched the accepted Linux and M5 candidate. A `git archive` of that commit was unpacked into the new case-sensitive Docker volume `thinkthen-0247-linux-arm64-build`; source extraction exited 0. The older `thinkthen-0231-linux-arm64-build` volume and the verified preflight toolchain directory were mounted read-only. M5 reported load 2.08 on 18 logical CPUs, 94% free memory and 441 GiB free disk before this build. Its Docker daemon reported `linux/aarch64`.

The container ran native `--platform linux/arm64` with networking disabled in the digest-pinned `manylinux_2_28_aarch64` image `c22ffd129ac99a8a42d1f2c2f4e88a9089288dd9ee987a7092da1c7dc48f27a9`. It reported `aarch64` and glibc 2.28. It reused the old volume's pinned DuckDB v1.5.5 C++ source at `d8cdaa33fda8df955cc76ef58a280f68f4cd43fa`, stock v1.5.5 CLI/Python host, stock v1.5.4 negative host and cached Cargo registry. The preflight mount supplied 22 static archives, including `libduckdb_static.a` SHA-256 `6fb859bd64db7ed3485f81aa339c30b0ce6c1dd46a4847cd5b9f605edf4ea585`. No DuckDB dependency was fetched or rebuilt.

`HOME`, `CARGO_HOME`, `CARGO_TARGET_DIR`, `THINKTHEN_DUCKDB_CPP_BUILD`, package path and backend target were isolated under `/work` in the new volume. The direct PATH prefix was `/pins/home/.rustup/toolchains/1.95.0-aarch64-unknown-linux-gnu/bin`; the container printed that path for both `rustc` and `cargo`, along with their official 1.95.0 versions. `cpp/build.sh`, `release-pack --reuse aarch64-unknown-linux-gnu ... duckdb`, and the same-source offline `conformance-backend` build each exited 0. Their containing Docker commands and the remote wrapper also returned 0. The prior input mounts stayed read-only.

| File under `thinkthen-0247-linux-arm64-build:/work/build` | SHA-256 |
| --- | --- |
| `cargo/release/libthinkthen_duckdb_bridge.a` | `2559468ffb601eed61787a630f25648f2461d0c7991d994517fd860e1b71309b` |
| `package/thinkthen-duckdb-0.0.1-aarch64-unknown-linux-gnu.tar.gz` | `53bfdf5ac3887503eda9cf5a43035bfc3b9b81d7fc5b5e1d65eefd4af2d2fd51` |
| `installed/thinkthen.duckdb_extension` | `3e70b8e9dbfa4106168f28a9d9db97bd1da9751ff5513f1ca0850ba5407f1c01` |
| `backend-target/debug/conformance-backend` | `1d751269c119f491f039d1860966bba1f4e7067e94b6d7e9702d4c0861bdcd46` |

The extracted extension matched the just-built extension byte for byte. `file` and `readelf` identified one AArch64 ELF; its highest required versioned glibc symbol was `GLIBC_2.28`. A raw-byte check found zero isolated container-home and build-account home-prefix occurrences. The package step validated the target and pinned footer. The stock v1.5.5 CLI loaded the extracted artifact and returned `BOOLEAN` for a typed NULL decision. `cpp/verify_package.py` exited 0: stock v1.5.5 Python loaded it, a changed-footer copy was refused, and the **unchanged extracted artifact** was refused by stock v1.5.4.

The extracted package passed the same seven named `tools/verbs_suite.py` cases above, seven of seven with zero failures and container and remote exit 0. They used the same-source loopback backend, fake key, child-isolated HOME/XDG folders and Docker's disabled external network. The prior 0231 package and its inputs were unchanged. This proves native Linux ARM64 execution inside M5's Docker VM. It does not qualify the later `ubuntu-24.04-arm` runner, Intel macOS or macOS 15 execution. No provider, full 54-case suite or release dispatch ran.

## Translated Intel macOS installed package

The exact source was integrated main `dfb00fcd6573a45d2ef6345761b352e9b928eda8`, checked out in a separate clean, detached M5 worktree under the workspace `worktrees/` directory. Since the previous package pin, main added the separately High-reviewed 0248 usage-storage changes in `engine/usage/{storage,tests}`; the DuckDB product and release-pack files did not change. This build therefore checks the complete current core with the accepted 0247 forms. M5 reported macOS 26.4, load 1.18 on 18 logical CPUs, 95% memory free, 440 GiB disk free and an idle second disk-I/O sample before building.

The child ran under `arch -x86_64 /bin/sh`: it printed `uname=x86_64` and `sysctl.proc_translated=1`. Its PATH began with `~/.cache/thinkthen-0231-macos-intel/rust/rustup/toolchains/1.95.0-x86_64-apple-darwin/bin`, and the child printed that exact `rustc` and `cargo` path plus their official 1.95.0 versions. The retained Intel input root supplied verified DuckDB v1.5.5 source `d8cdaa33fda8df955cc76ef58a280f68f4cd43fa`, 21 static archives, x86-64 stock v1.5.5 CLI/Python, stock v1.5.4 negative host and CMake 3.31.10. Only its cached Cargo registry was copied to the new worktree's `CARGO_HOME`; no DuckDB dependency or toolchain was rebuilt or installed. `CARGO_TARGET_DIR`, `THINKTHEN_DUCKDB_CPP_BUILD`, package path, backend target and temporary output all lived beneath `databases/duckdb/build/0247-intel` in the new worktree.

Under `/tmp/thinkthen-codex-6.lock`, `cpp/build.sh` and `release-pack --reuse x86_64-apple-darwin ... duckdb` each exited 0. A same-source offline `conformance-backend` build also exited 0. The containing translated children and remote wrapper reported zero status.

| Intel artifact under `databases/duckdb/build/0247-intel` | SHA-256 |
| --- | --- |
| `cargo/release/libthinkthen_duckdb_bridge.a` | `293e6d646e1e4e0148a0e34c2cc902db78632d9f50018b211b7ff7e709ebae0d` |
| `package/thinkthen-duckdb-0.0.1-x86_64-apple-darwin.tar.gz` | `3ccab9d3164ef22c7f14a4fd7015b3d4f38cb5272b21af834efb6f9da1d0ee06` |
| `installed/thinkthen.duckdb_extension` | `7dbe5fbf3f92d49a0354768ab6854fb85dfa7271f0758b89a387ebe39f6f6241` |
| `backend-target/debug/conformance-backend` | `3f953b3cf917fd8b5c5e10901a75ce04e35805f383e21a44e6ea034ec9c7f414` |

The extracted extension matched the just-built extension byte for byte. `lipo` reported one x86-64 Mach-O; `otool` reported `minos 15.0` and `LC_ID_DYLIB @rpath/thinkthen.duckdb_extension`; a raw-byte scan found zero build-account home-prefix occurrences. The package step validated the pinned `osx_amd64` footer. Stock x86-64 v1.5.5 CLI loaded the extracted artifact and returned `BOOLEAN` for a typed NULL decision. `cpp/verify_package.py` exited 0: stock v1.5.5 Python loaded it, that host refused a changed-footer copy, and stock v1.5.4 refused the **unchanged extracted artifact**. The translated host check returned 0.

The same seven selected `tools/verbs_suite.py` cases passed, seven of seven with zero failures and translated and remote exit 0, against this extracted artifact and the same-source loopback backend. They retain the independently fixed request bytes and digest, caller file-authority recheck, file secrecy and zero-send refusals, typed NULL, legacy list overload and result mapping. No provider, full 54-case suite, release dispatch or native Intel/macOS 15 runner was used. This proves translated x86-64 execution on M5/macOS 26.4 and a macOS 15 deployment field. It does not prove native Intel hardware or actual macOS 15 execution.
