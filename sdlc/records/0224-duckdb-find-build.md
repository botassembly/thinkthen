# 0224 DuckDB ordered find: first Linux C++ slice

Status: source implementation and selected local proof on `ticket/0224-duckdb-find`; exact packaged artifact and independent native review are pending in this record. This is Linux x86-64 C++ source and host work. Apple Silicon, Linux ARM64 and Intel macOS find packages, actual macOS 15 execution, PostgreSQL and the full 0224 outcome remain open.

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

Pending a source-frozen release build and selected installed checks. The prior baseline hash above is historical and does not prove this source.
