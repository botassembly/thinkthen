---
flow: build
priority: 231
opens: databases/duckdb/tools/setup.sh databases/duckdb/tools/version.env databases/duckdb/cpp/archive-sha256.txt databases/duckdb/cpp/CMakeLists.txt databases/duckdb/cpp/build.sh databases/duckdb/check.sh databases/duckdb/cpp/verify_package.py databases/duckdb/cpp/verify_interrupt.py databases/duckdb/tools/settings_suite.py sdlc/scripts/release-pack sdlc/scripts/release-smoke databases/duckdb/README.md sdlc/planning/databases/duckdb.md sdlc/issues sdlc/records sdlc/tickets
---

# 0231: Ship the DuckDB C++ extension on the other release platforms

Status: design for fresh independent review. Owner: Codex. The Linux x86-64 implementation already landed under 0201, 0149 and 0157. This ticket adds Linux ARM64 and macOS x86-64/ARM64 package paths and their native proof before 0.1; it changes no SQL API. Exact implementation files need a new claim after review.

## Outcome and authority

Every advertised 0.1 DuckDB archive contains the accepted C++ extension for DuckDB v1.5.5, built from the corresponding official static inputs. An installed archive on its own target proves bind-time refusal, query lifetime, same-session warm/settings behavior and request-size behavior before that target's register 51/72 and 0149/0157 remainder closes. A target that lacks a verified archive or native installed check fails release smoke. It never falls back to the old C API package under the same release name.

Ian approved the C++ migration and the staged Linux x86-64 path in [ADR 0081](../planning/adr/0081-duckdb-cpp-api.md). Ticket 0201 owns its SQL behavior and ADR 0080 owns the query budget. The 2026-09-28 work plan prioritizes required release platforms. The coordinator may approve target-specific implementation details within those outcomes; Ian can overturn the supported-platform or outward release choice in ticket 0128. This ticket adds no new architecture decision and does not authorize a workflow dispatch, publish or host setup outside the normal release path.

## Starting evidence and retained behavior

The [platform preflight](../records/2026-09-28-release-platform-preflight.md) identifies the current route. `sdlc/scripts/release-pack::part_duckdb` selects C++ only for `x86_64-unknown-linux-gnu` and sends the other three targets to the old C API. `databases/duckdb/tools/setup.sh` accepts only `Linux:x86_64`; `cpp/build.sh` stages only one fixed target; `cpp/CMakeLists.txt` has ELF-specific linker arguments; `check.sh` refuses macOS before toolchain validation. The older `src/tables.rs::Warm::finish` still uses an environment engine. Its successful compilation is not a caller-session proof.

The accepted 0201 [build record](../records/0201-duckdb-cpp-api-prerequisite.md) proves the Linux x86-64 C++ archive in stock v1.5.5 CLI and Python, including v1.5.4 refusal, prepared file permissions, query lifecycle, SIGINT and relations. The [0149 build](../records/0149-duckdb-integration-build.md) proves same-session warm/scalar settings and total send budget on an installed Linux C++ archive; the [0157 build](../records/0157-build.md) proves request size on that target. Preserve their SQL names, types, NULL/error rules, caller file authority, engine totals, split rules and diagnostic boundary. Do not repeat the Linux proof solely because this ticket adds platform selection.

## Pinned input availability

The official [DuckDB v1.5.5 release](https://github.com/duckdb/duckdb/releases/tag/v1.5.5) and its [release API](https://api.github.com/repos/duckdb/duckdb/releases/tags/v1.5.5) list the inputs below. The preflight downloaded each **static ZIP only into memory**, verified its SHA-256 against the release API digest, and hashed its contained archives. It did not install a toolchain or execute an archive. All three ZIPs contain `libduckdb_static.a` and the other DuckDB archives; Linux ARM64 contains 22 `.a` files, each macOS ZIP 21. The macOS sets lack Linux's `libduckdb_jemalloc.a`. The complete per-archive digests must enter target-specific manifests during implementation; the existing `cpp/archive-sha256.txt` is for Linux AMD64 alone.

| Target | Official static ZIP SHA-256 | Official matching CLI ZIP SHA-256 | Archive count |
| --- | --- | --- | --- |
| `aarch64-unknown-linux-gnu` | `static-libs-linux-arm64.zip` `ea6a34cb49ec2db5ed23d9e8311237c53c32abf9cdbf5dd608c4176c3dd8bfeb` | `duckdb_cli-linux-arm64.zip` `02163197027a42149147364d31fa67cac82108517a4be43304a1cc226eaef07a` | 22 |
| `x86_64-apple-darwin` | `static-libs-osx-amd64.zip` `a27d36fa1247a3ffa1692e7aa0bf4ea4d1e0ee51da7c4df7a5db5217357b1b4d` | `duckdb_cli-osx-amd64.zip` `47cbda17c5d4643a58833617dfae649a6a8722d7e54435a08161b98ac1c4e832` | 21 |
| `aarch64-apple-darwin` | `static-libs-osx-arm64.zip` `d79ec66b8a4054b866faada82e9e31f859a713c555b3f1c4b71c4a43d3273e9c` | `duckdb_cli-osx-arm64.zip` `da5177b8869c4ed8c65d514fb47a8ed0f6fa7427f103304932d5e83851e46abd` | 21 |

Keep the exact C++ source commit `d8cdaa33fda8df955cc76ef58a280f68f4cd43fa`, stock Python DuckDB v1.5.5, and negative stock v1.5.4 host rule. The release API supplies asset digests, not proof that the linked extension loads or that the release runner has a compatible C++ ABI. Record the fetched asset digest and every extracted archive digest in the implementation commit. Check the actual archive set, source commit, host architecture, Rust target triple and DuckDB footer platform before building or packing. On macOS verify Mach-O architecture and minimum OS version as part of artifact inspection. On Linux verify ARM64 ELF architecture and the release's glibc floor. No universal macOS extension is implied by a platform-specific static ZIP.

## Build slices

1. **Inputs and selection.** Give `tools/version.env` an explicit four-target map of official CLI and static ZIP names and SHA-256 values, retaining the existing Linux x86-64 values. Make `tools/setup.sh` select only its actual `uname` kernel and chip, reject an unsupported or mismatched host before download or compilation, and verify both ZIPs, extracted CLI, the pinned C++ source commit, and every extracted static archive. Use a portable digest helper or host `shasum -a 256` on macOS. Keep the archive manifests per target with one shared verifier; the number and names must match, including the macOS absence of jemalloc. Never validate one target with another target's manifest. The pinned older host also needs a matching official v1.5.4 CLI asset and digest per target before the version-refusal check can run.
2. **Link and stage.** Make `cpp/build.sh` derive and validate the actual host target and stage only to `build/artifacts/cpp/$TARGET`, with no x86-64 alias. Keep the Rust bridge locked and offline after setup. In `cpp/CMakeLists.txt`, retain the Linux archive group and ELF options only on Linux. Give macOS an explicit static archive link and required system libraries/frameworks based on the pinned upstream source and actual native link diagnostics; no Linux `--start-group`, `--exclude-libs`, `dl` or `pthread` flags go to Apple `ld`. Check C++/Rust architecture and ABI before copying the extension. A missing symbol, duplicate runtime, incompatible deployment target or loader failure is a stop, not a reason to skip a package.
3. **Package and check.** Route each of the four target triples through `cpp/build.sh` in `sdlc/scripts/release-pack::part_duckdb` and remove the C API release fallback for those targets. Package the same ThinkThen/DuckDB licenses and selected dependency inventory as Linux x86-64. Make `--reuse` require the matching staged target and refuse an absent or mismatched extension. `databases/duckdb/check.sh` accepts macOS only after its matching pinned toolchain is ready, then runs the existing stock CLI, Python and installed archive verifiers. Preserve `release-smoke`'s missing-file and exit-77 failures. Align these entry points with 0128 Phase 3 runner setup; this ticket does not own `release.yml`, PostgreSQL, Ruby or other surfaces.

## Proof and closure

Source-selection checks run here before native hosts are available: one table over the four real target triples verifies the selected asset names, digest-manifest identity and stage paths; wrong `uname`/triple and an absent or cross-target artifact fail before packaging. Those are selector/refusal checks, not an emulated host, compiler or installed extension. Run format, shell/Python syntax, package-path and touched-source policy checks. Run focused Linux x86-64 regression checks only where shared selection or CMake behavior changed. Preserve already accepted full Linux product proof.

For each new target, build from the pinned source and static ZIP on that native runner. Record its source SHA, ZIP and contained archive hashes, bridge static library and final archive hash, target/ABI inspection, stock v1.5.5 CLI and Python versions, and fresh-prefix installed `release-smoke` with zero not-run. Select the existing package verifier's exact stock v1.5.4 rejection, bad constant zero-send bind, prepared permission change, late signal, multi-expression/chunk query budget, same-session warm/scalar cache hit against another session, request-size split and total send budget, plus relation cases affected by the platform. Prove fixed defect text and no diagnostic payload from the changed package where 0226's old C API source proof no longer describes that package. An installed package's no-fault check does not substitute for the synthetic source child. Do not run stress, paid calls or a full old campaign merely for this port.

Close each target's 0201/0149/0157 and register 51/72 rows only after its own native archive and selected host proof pass independent code review. A passing source check, cross-compile, or Linux x86-64 archive leaves that target open. When all three pass, 0128's four-runner rehearsal remains the release proof for the exact final source and workflow. The later 0226/0227 language package remainders have their own owners; this ticket must not silently close them.

## Evidence

- Starts from: ADR 0081, ADR 0080, staged 0201/0149/0157 Linux C++ source and installed proof, the 0128 Phase 2 release-pack/smoke record, and `sdlc/records/0231-platform-preflight.md` with official target input availability. No new platform build or provider call ran for this design.
- Keeps: v1.5.5 C++/Rust bridge and all shipped DuckDB SQL behavior, the verified Linux x86-64 package, exact source and per-target asset pins, release smoke's not-run failure, and 0128's manual release authority.
- Changes: target-specific pinned inputs/manifests, portable host selection, platform-specific CMake link, exact target staging, four-target C++ release-pack routing, and native installed checks for the three new targets.
- Proof: four-target selector/refusal table, per-target asset/archive digests, architecture inspection, stock-host installed checks with v1.5.4 rejection and selected 0201/0149/0157 regressions, fresh independent review, then the 0128 rehearsal over final artifacts.
- Defers: 0128's workflow and other macOS surfaces, 0226/0227 non-DuckDB package remainders, DuckDB grouped SQL find, a new secret type, Windows and any later DuckDB version.

## What the build taught us

Pending implementation and review. Record changes to the input map, native link assumptions and proof selection before landing.
