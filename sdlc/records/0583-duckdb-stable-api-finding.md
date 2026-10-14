# 0583: Test one DuckDB build across supported versions

Ticket: 0583
Date: 2026-10-14

Review: accept. A fresh read-only reviewer reproduced all five compiler checks, both positive controls, six upstream hashes and baseline source copies. No findings required changes.

Experiment thinkthen/0583 ran on the Beelink on 2026-10-14. ThinkThen baseline: 0e46dc2daf53bba7ad40165e67068948607f7bec. The experiment treated the repository as read-only and made no repository edits. The shared coordinator separately modified the six experiment tickets, as checkout-state.log records; these are not experiment mutations. Input copies were taken with git show at the exact baseline revision.

## Decision and stop rule

Keep the approved C++ builds and leave 1.4 unsupported. The named blocker is duckdb_scalar_function_set_bind. DuckDB 1.4.5 exposes its declaration in duckdb.h, but the stable extension API function-pointer table does not expose it. That distinction matters: an ordinary client declaration does not make a call available through the stable extension binary interface. Building against a later stable API exposes the call but raises the minimum API beyond 1.4.

This meets the ticket stop rule, the first missing required C API call. No ThinkThen port, product build, SQL test run, stock-host load, or publication was attempted. No model calls were made. No active release build, worktree, or cache was used.

## Existing evidence and the requirement

The earlier experiment 253 report was read before running probes. It established settings and filesystem issues but did not establish a single cross-version artifact. Accepted ADR 0081 already explains why the product migrated to C++ and requires scalar bind behavior and statement lifecycle. Baseline portable.cpp defines Bind at line 39, retains the caller context in PortableBind, and passes Bind into the native scalar registration at line 395. scalar_owner.hpp uses ClientContextState::QueryBegin and QueryEnd. A replacement that simply registers an execution callback does not preserve that mechanism or prove the existing SQL contract.

Baseline ticket 0577 already records Ian's approval to retain C++ builds for 1.5.6 and 2.0 and skip 1.4. This result supports that scope. The baseline checkout does not contain that team's newer build proof records, and none of its active work was consulted or touched.

## New information useful to the 0.2 listing

The scalar bind call became stable in DuckDB 1.5.6. The exact v1.5.6 duckdb.h history beside the declaration says unstable v1.3.0 and stable v1.5.6. Its declaration is gated by DUCKDB_API_VERSION_AT_LEAST(1, 5, 6). The 2.0 prerelease headers carry the same stable band. Therefore the older blanket statement that stable C extensions cannot bind scalars is outdated for 1.5.6 and later. The coordinator was notified immediately.

This does not prove a complete migration is viable for 1.5.6 and 2.0. Query lifecycle, interrupt behavior, settings, file permissions, prepared statements, nested types and all SQL tests would still require a separate narrowly scoped investigation. The experiment stops at the 1.4 blocker instead of expanding into that work. No claim is made that any lifecycle API is missing in the newer headers.

## Bounded probes and results

Six public upstream files were downloaded from raw.githubusercontent.com/duckdb/duckdb: duckdb.h and duckdb_extension.h at v1.4.5, v1.5.6 and v2.0-cyanoptera. upstream-manifest.json contains exact URLs, byte counts and SHA-256 digests. The prerelease branch is mutable; retained bytes and hashes identify this observation, not a final 2.0 release. Source rights: public DuckDB source, MIT. No clinical, production, licensed or model data was used.

Run `python3 experiments/479-thinkthen-duckdb-c-api/check_api.py` to reproduce the five syntax-only compiler checks from retained headers. compile-results.json contains every exact command, exit code and elapsed value; each case folder has its probe source and compile.log.

| Header revision | Target stable API | Result |
| --- | --- | --- |
| v1.4.5 | default v1.2.0 stable table | Missing duckdb_scalar_function_set_bind, compiler exit 1 |
| v1.5.6 | v1.4.0 | Missing duckdb_scalar_function_set_bind, compiler exit 1 |
| v1.5.6 | v1.5.6 | Table member present, compiler exit 0 |
| v2.0-cyanoptera | v1.4.0 | Missing duckdb_scalar_function_set_bind, compiler exit 1 |
| v2.0-cyanoptera | v1.5.6 | Table member present, compiler exit 0 |

Each check takes about 0.02–0.03 seconds. These are table-surface checks, not linked or loadable extension binaries. The first attempt accidentally let an indirection macro expand inside a member access; those logs are retained as attempt-1-macro-expansion.log and are not used as evidence. The corrected probe undefines that macro and directly checks the API table member. Positive controls establish that the corrected probe recognizes the call when it is available.

Initial sandboxed downloads failed DNS resolution. The authorized network download succeeded. No package installation or compiler cache changes were needed. Machine load was about 11 before the checks; free workspace disk was 203 GB. The probe consumed no meaningful build load.

## Build count, time and size comparison

Stable C product build count: zero, stopped at API feasibility. Stable C product binary size and full build time: not applicable; no product binary exists. The five compiler probes are syntax-only and produce no binary. Current C++ packaging requires a separate build per supported DuckDB version and platform, as ADR 0081 and ticket 0577 say. For approved 1.5.6 and 2.0 support that is two builds per platform; adding 1.4 would add another. Actual C++ build time and binary size were not measured because the stop rule fired and repeating builds would consume 0.2 resources. No numeric savings estimate is justified.

## Artifacts and limits

This folder holds PLAN.md, OWNER, REPORT.md, check_api.py, compile-results.json, six upstream header snapshots, upstream-manifest.json, input-manifest.json, selected baseline source copies under inputs/, machine.log, compiler.log, checkout-state.log and five case folders with source, retained headers and raw compiler logs. No regenerable compiled output exists. No evidence was deleted; the coordinator owns recording the finding and later cleanup.

The report proves the first stable API blocker for the all-three-version goal. It does not claim that the existing SQL checks pass under any port, that a binary loaded on any version, or that a one-build 1.5.6/2.0 migration has been ruled out. A future experiment can test the narrower supported-version pair if its maintenance savings justify the work.

## Reproduction inputs

The reviewed experiment folder can be removed after this record lands. The following probe and source hashes retain the finding without retaining downloaded headers. Download each header from its URL into a case directory as `duckdb.h` and `duckdb_extension.h`, then run `cc -std=c11 -Werror -fsyntax-only -I CASE [VERSION FLAGS] CASE/probe.c`. VERSION FLAGS are `-DDUCKDB_EXTENSION_API_VERSION_MAJOR=1 -DDUCKDB_EXTENSION_API_VERSION_MINOR=M -DDUCKDB_EXTENSION_API_VERSION_PATCH=P`, using the target version in the table; omit them for the default case.

```c
#include "duckdb_extension.h"
#undef duckdb_scalar_function_set_bind
void probe(duckdb_ext_api_v1 *api) { (void)api->duckdb_scalar_function_set_bind; }
```

- [v1.4.5-duckdb.h](https://raw.githubusercontent.com/duckdb/duckdb/v1.4.5/src/include/duckdb.h), 190799 bytes, SHA-256 `cbe6a827284375c5a646b8f27dbd3a00808ce8738b5bbb49c190cb1e08540281`.
- [v1.4.5-duckdb_extension.h](https://raw.githubusercontent.com/duckdb/duckdb/v1.4.5/src/include/duckdb_extension.h), 95235 bytes, SHA-256 `514a5b639cd800e90f1556e8e039cff7f0b4a256e461754dd55caf79dc16db00`.
- [v1.5.6-duckdb.h](https://raw.githubusercontent.com/duckdb/duckdb/v1.5.6/src/include/duckdb.h), 287887 bytes, SHA-256 `e85f70155b5948c8575e59f66e60d43575cec8b252293534c9e8598eb313a293`.
- [v1.5.6-duckdb_extension.h](https://raw.githubusercontent.com/duckdb/duckdb/v1.5.6/src/include/duckdb_extension.h), 133437 bytes, SHA-256 `dfbb388b8815df5f279561694274d95119ac1eaf7be2881926dfaf606471fa9e`.
- [v2.0-cyanoptera-duckdb.h](https://raw.githubusercontent.com/duckdb/duckdb/v2.0-cyanoptera/src/include/duckdb.h), 290195 bytes, SHA-256 `8dbc9c71a10f0a75fcacc1a3f4f7c76093a84d232fc152222ee5b6e512ab4560`.
- [v2.0-cyanoptera-duckdb_extension.h](https://raw.githubusercontent.com/duckdb/duckdb/v2.0-cyanoptera/src/include/duckdb_extension.h), 134516 bytes, SHA-256 `a67312ee4c88b1c3ede25af0a74c253ce39354efe1c3beaf576c441451f84a36`.

The initial incorrect macro-expanded probe was discarded. The corrected negative cases each diagnose the absent `duckdb_scalar_function_set_bind` member; both positive controls compile. No full-build or size comparison was made because the first missing API call met the authorized stop rule.
