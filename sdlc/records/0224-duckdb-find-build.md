# 0224 DuckDB ordered find: first Linux C++ slice

Status: selected Linux x86-64 C++ source and installed package proof from source commit `ba7a090eb6c5898002d0a0a081e5e01cecae402d` on `ticket/0224-duckdb-find`; fresh independent High native review accepted `fb9009aa`. SQLite and PostgreSQL are already landed. Apple Silicon, Linux ARM64 and Intel macOS find packages, actual macOS 15 execution and the full 0224 outcome remain open.

## Source and ownership

The C++ binder registers direct-only volatile 2/3/4 argument overloads and retains the caller's `StatementOwner` and session snapshot. One `Value` list reader follows DuckDB selection, distinguishes a SQL NULL list from a NULL child, and preserves each original position. It validates every nonempty row in a chunk before the first send, without deduplicating equal members. The bridge copies the question, list and settings before detaching its worker, creates one plain `Question::find`, and uses the shared `Engine::find_with` with one cancellation token and actual-attempt `SendBudget`. The private frame carries original indexes and probabilities; the C++ decoder checks the count, byte length, order, finite probability range and selected index before publishing the typed struct. `RustReply` frees both successful and failed bridge buffers, including decoder refusal. `thinkthen_try_details` was not changed.

## Red, green and selected proof

With the prior installed Linux extension saved at `/tmp/thinkthen-0224-duckdb-find/baseline.duckdb_extension` (SHA-256 `75b6d298e1f2ed2e4f952ea84c41e82eb7a8cdd2a4253e9ca05f668dd3d1423a`), selected conformance cases `18-find-second` and `19-find-none` failed because `thinkthen_find` was absent. After the source build, both pass with one captured complete request body equal to independent corpus bytes, an observed digest from that body and the served URL equal to the independent corpus digest, one listener send, and literal original-index/result/probability assertions. `find_suite.py` passed duplicate-second, first-real tie, none tie, top SQL NULL, empty list, NULL child, blank, size/count bounds, 2/3/4 arities, exact typed result, held cancellation, and spent statement deadline with zero sends. The invalid later row in a chunk produced no send from an earlier valid row. No paid provider or broad suite ran.

The existing `PackedReplies` counted responder was reused for tie outcomes; the existing shared backend capture arm was reused for canonical wire bodies. No test-only product hook or copied planner was added. The new tests protect the native list reader, original-index result decoder, and held owner boundary that the shared Rust tests and prior DuckDB verbs cannot observe. No existing scaffold test was deleted or consolidated; the old conformance find `NOT_RUN` reason was removed because its two actual cases now run.

## Measured source

DuckDB Rust source is 6,364 nonblank lines versus 6,192 before (+172); C++ is 1,688 versus 1,532 (+156); Python is 3,401 versus 3,289 (+112). The additions are the private owned find bridge, typed C++ adapter, bounded capture helper and selected host proof. I checked `listed_result.cpp`, `nested_result.cpp`, `tools/verbs_suite.py` and the SQLite find adapter for reusable readers and result assertions. Their deduplicating/list-null or host-specific behavior does not preserve this ordered native find contract; `PackedReplies` and the shared capture backend were reused instead. `thinkthen.cpp` gained only a registrar line; `ffi.rs` holds only the necessary exported C ABI wrappers because repository policy forbids unsafe exports in its private child. It is 494 nonblank lines, under the 500-line cap. The three measured ratchets equal their source totals. No public Rust API or shared source grew.

## What the build taught us

DuckDB's `Value` list preserves equal children and dictionary selection, but the existing listed-result helper erases the distinction between a top-level NULL and a NULL child. The find adapter therefore needs its own small reader, and the selected installed test must assert both branches with zero sends. A raw selected-index-only bridge reply would leave the candidate order unproved; the checked frame and literal installed result cover every original position and probability. The offline policy check initially rejected unsafe exported functions in the private child, so the C exports stay in `ffi.rs` while owned find work stays in the child. The release archive remains a separate proof from a local C++ build; its exact source and installed hashes are recorded below after packaging.

## Exact package and installed result

On a 16-CPU Linux x86-64 host at load 1.04 and 21 GiB available memory, `flock -o /run/user/1000/thinkthen-codex-3.lock env CARGO_NET_OFFLINE=true RUSTC_WRAPPER= THINKTHEN_DUCKDB_CPP_BUILD=/tmp/thinkthen-0224-duckdb-find/cpp-build sh databases/duckdb/cpp/build.sh` exited 0. The isolated CMake output and lane lock did not serialize other independent builds. After source commit `ba7a090e`, `flock -o /run/user/1000/thinkthen-codex-3.lock sdlc/scripts/release-pack --reuse x86_64-unknown-linux-gnu /tmp/thinkthen-0224-duckdb-find/package duckdb` exited 0 using that unchanged built extension. Exact SHA-256 values:

| File | SHA-256 |
| --- | --- |
| Rust bridge static archive, `bridge/target/release/libthinkthen_duckdb_bridge.a` | `1c07548cb18bfac167cf4fb64640ffdaad4cf29ccaebb82eed273d81c006d094` |
| `thinkthen-duckdb-0.0.1-x86_64-unknown-linux-gnu.tar.gz` | `faa9a07a9866e0c76aa27e7775085478f14458211daaece734e10967c5369aeb` |
| Extracted `thinkthen.duckdb_extension` | `1f8ff27cf7025897068d53de1c350f99199943e4a6ef25f4e74a07bbc4333626` |
| Built `build/thinkthen.duckdb_extension` | `1f8ff27cf7025897068d53de1c350f99199943e4a6ef25f4e74a07bbc4333626` |

The extracted file is 73,309,806 bytes and contains zero raw-byte occurrences of the builder's home path. The archive contains ThinkThen and DuckDB licenses, dependency inventory and notices. Against that extracted file, selected conformance IDs 18 and 19 passed 2/2. The actual captured complete bodies were:

```json
{"state":"[{\"id\":\"u001\",\"evidence\":\"First passage.\"},{\"id\":\"u002\",\"evidence\":\"Second passage.\"},{\"id\":\"u003\",\"evidence\":\"Third passage.\"}]","model":"jev-1.13.0","questions":{"q1":{"type":"choice","instructions":"Which passage answers the question?","criteria":{"u001":null,"u002":null,"u003":null,"none":null}}}}
{"state":"[{\"id\":\"u001\",\"evidence\":\"First passage.\"},{\"id\":\"u002\",\"evidence\":\"Second passage.\"}]","model":"jev-1.13.0","questions":{"q1":{"type":"choice","instructions":"Which passage answers the question?","criteria":{"u001":null,"u002":null,"none":null}}}}
```

Case 18's actual body SHA-256 was `6f2db9a5d733178470a4f58ebc2495dd66ebe2609c6c1bec8f534e9e8a6a77fe`; with served URL `http://127.0.0.1:44639/case/18-find-second/capture/v1/systemone`, its recording digest was `4f877e1a9785b9de1bca06a4c332cc63266cd11722330035519571870206cfbd`. Case 19's actual body SHA-256 was `d2e0ca3f5223f468512160e22784684fbc812306fa948b17dfc11d9098970a20`; with served URL `http://127.0.0.1:33005/case/19-find-none/capture/v1/systemone`, its digest was `58d9e7278cc515ad6b207163871465731becdbefbfb6b389dfd548c66c926f5e`. Each actual body and digest matched independent corpus bytes and its computed digest for that same ephemeral URL; each listener counted exactly one send. Literal expected SQL results covered the second original member and none, including all ordered candidate probabilities.

The same extracted extension passed `original_duplicate_and_ties`, `null_empty_and_invalid_units_do_not_send`, and `held_find_and_spent_statement_budget` 3/3: original duplicate and tie indexes, full typed struct, NULL and empty SQL results, invalid member/count/byte/deadline zero sends, held cancellation, and spent statement zero sends. `cpp/verify_package.py` exited 0: stock DuckDB v1.5.5 loaded it, a changed-version footer was refused, and the unchanged file was refused by the genuine stock v1.5.4 host. The prior baseline hash above is historical and does not prove this source.
