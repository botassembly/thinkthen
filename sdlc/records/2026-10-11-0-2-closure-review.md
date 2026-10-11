# 0.2 closure review, 2026-10-11

The PM sent three fresh read-only reviewers over the tickets the coordinator closed on 2026-10-10. They read tickets, records, landing commits and code. They ran no builds or tests. This record keeps their findings. New tickets 0534 to 0543 own the fixes.

## Sound closures

0503, 0515, 0516, 0519, 0499, 0470, 0533, 0494, 0498, 0505, 0522, 0523, 0524, 0525, 0527 and 0528 meet their outcomes. All three 0512 risks found before closure are fixed: input no longer truncates (`cli/asking/native_reader.rs:54-75`, 0dda145d5), annotate admits every record (`public/complete/annotate/streaming.rs:73-77`, 17770ab66), and rank with details renders from the kept original (`cli/asking/native.rs:419-451`, 065ee48c4).

The 2,429-line raise of the Rust source ceiling at 5c9a07a2a is a recount. Shared host source moved into the root count, and each library's ceiling fell to match.

## Gaps

1. Python, R, Ruby and JavaScript build the request grammar in host code and send it as JSON: `python/thinkthen/_calls.py:210`, `python/thinkthen/__init__.py:145-149`, `ruby/lib/thinkthen/session.rb:94`, `typescript/native.js:75`, `r/thinkthen/R/requests.R:40-47`. The binding guide says native callers construct typed Request values without a JSON round trip. Python also keeps untyped `**legacy` keyword passthrough and host-side `UsageError` checks (`__init__.py:149-299`). Python file reading parses a JSON selection with the `thinkthen_host::source` parser before the native reader (`python/src/files.rs:6,44`).
2. PHP builds its request by string concatenation and knows the input grammar (`php/src/session/operation.php:20-34`).
3. Old names remain. The JVM keeps the `door` package, `thinkthen-door.jar`, a 197-line `jvm/door/thinkthen/Json.java` and release self-test expectations for removed classes (`sdlc/scripts/release-managed-pair-self-test.py:127`). The COBOL README has no old-to-new mapping. "Door" wording remains in `swift/Sources/CThinkThen/include/thinkthen.h:2-9`, `go/thinkthen.go:1,31`, `cpp/include/thinkthen/json.hpp:20` and `python/src/files.rs:1`. The upgrade guide says Foundation "remains unrun".
4. 0495 moved only the complete and file paths of DuckDB to the shared request. About 13 routes still call the engine directly and build JSON, in `databases/duckdb/bridge/src/ffi/scalar/ffi.rs`, `portable_many/ffi.rs`, `portable/ffi.rs`, `listed.rs`, `images/ffi.rs` and `complete_listed/ffi.rs`.
5. `Engine::finish_usage()` (`public/engine.rs:234`) stays public and drops a usage write failure. The C JSON usage-status routes (`libraries/c/src/call.rs:92`) duplicate the typed exports and are not 0.1 exports.
6. 0512 edges: four `cli_reader` branches remain (`public/pull.rs:186,303,378`, `public/complete/recognize/streaming.rs:78`). The branch at `pull.rs:186` changes single-input CLI behavior with no test. Plain rank of a released unit returns `Defect` (`cli/asking/native.rs:427,443`). A poisoned reader lock returns None (`native_reader.rs:55,71`).
7. Rust request code repeats itself: 27 hand-written result-kind checks (`public/frame/typed.rs:84-416` and others), two framing decoders (`public/request/source.rs:90-114`, `public/request/framing.rs:53-85`), `table/framed.rs:26-30` resetting another module's fields, and a stop-at-first-error iterator written five times.
8. The installed-consumer drivers copy each other. The usage-lock test exists with two handshakes (`ada/checks/usage_installed.py:180-221`, `jvm/tests/usage_installed.py:40-75`) and six more copies. Languages load each other's fixtures by path (`csharp/tests/backend.py`, `cpp/fixtures/session_cases.py`). 0530 fixed one authored-input bug five times, once per language (slices Y, Z, AB, AJ, AL). The 0530 record reads as a status log.
9. 0508 landed before 19 of its 25 dependencies closed. No fresh review covers the later migration landings. Ruby's last five code commits (`2c71db0db` to `d97af8365`, 5,139 lines removed) and six 0529 release-script commits have no review on their tickets.
10. 0518 closed without its installed Foundation consumer compiling or running anywhere. 0474 and 0480 left Windows cache checks that no open ticket names.
11. Stale text: the Status lines of 0461, 0467, 0468 and 0425, the evidence of 0474 and 0480, and "remains" lines in the 0496, 0504, 0524 and 0529 records. `pm ticket land --tidy-status` retires the closed Status lines. 0543 owns the Windows checks those lines mention. The closed tickets' evidence and the records' "remains" lines stay as history and need no ticket.

## Kept for after 0.2

Rust test code is about 1.4 times product code, and 514 tests remain inside `crates/thinkthen/src`. The Rust Polars typed tests repeat one refusal check five times. These go to an idea with the trigger "after 0.2.0 ships", because test size does not change product behavior.
