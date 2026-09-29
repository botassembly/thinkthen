# 0260 first proof slice: portable batch identity

Status: candidate proof passed focused checks; fresh code review and landing remain. This completes the CLI plus C-door checkpoint in ticket 0260, not register 64's every-applicable-surface criterion. Source began at accepted design `f68d9496` and claimed main `94bdf96d`; the coordinator owns later host assignments.

## What was added and observed

`specification/records.md` now states the selected-value spelling and preserves ADR 0048's content-cut and exact-request cache rules. The one batching corpus fixes five text values, literal compact UTF-8 spellings, hash heads, cuts, ordered members, close reasons, three complete System One bodies and fixed-address exchange digests. Its separate structured JSONL arm fixes nested object order, Unicode/escapes, integer/float distinction, negative zero and exponent spelling, with two literal bodies/digests. A fixture file's final newline is excluded from the expected transmitted bytes.

The core batch test compares actual selected record bytes, heads, cuts, membership, closure, bodies and digests against those literals. The compiled CLI sends the five texts from both line records and JSONL whose `é` came in as `\u00e9`. A counted listener received exactly three requests with the fixed body bytes; all five detailed rows named their independently expected dynamic-URL digest and ordered group. The structured CLI arm sent exactly two fixed bodies with its numeric content cut and three ordered rows. The public C JSON door passed the five texts as a JSON array through the compiled C driver, returned five ordered rows, and reached its listener exactly three times with the same fixed body bytes and dynamic-URL request identities. The synthetic listener replies only supply accepted yes results; they are not the asserted batch oracle. No provider was called.

An independent Python standard-library SHA-256 calculation over literal fixture bytes checked the five heads and three canonical exchange digests before the Rust tests ran. The escaped spelling `"caf\u00e9-5544"` has head `72afc19814d82dd9`, which is not a cut; parsed JSONL reaches `"café-5544"` with head `3db0138d194cf000`, which is a cut. That counterfactual changes membership if a host hashes the wrong spelling. Core and CLI tests establish that the current accepted routes normalize it before hashing.

## Focused checks and growth

- Root core filter `core::batch::tests::portable`: 2 passed. Compiled CLI backend filter `batching::portable`: 2 passed. C door filter `cases::portable`: 1 passed. Each listener case uses loopback only, no cache and no retry.
- Root strict Clippy on the library and backend integration target, and C workspace strict Clippy on all targets: passed. Rust format ran on the root and C workspaces. `CARGO_NET_OFFLINE=true python3 sdlc/scripts/policy.py` passed after clearing this environment's blocked `sccache` wrapper. Ticket evidence check, diff check and both ratchets passed. No full `test`, `spec`, `surfaces`, stress, live or platform campaign ran.
- Root Rust ratchet rises from 98,806 to 99,132 nonblank lines (+326): the 160-line core oracle test, 164-line outside-in CLI proof and two module declarations. C ratchet rises from 3,762 to 3,848 (+86): its 84-line outside-in door proof and two-line module declaration. Existing `core/batch/tests.rs` and backend `batching.rs` stay at 457 and 467 nonblank lines, below their 500-line caps. The C door's existing `cases.rs` is already larger than 500 and gained only a module declaration. The implementation searched the existing core batch fixtures/tests, CLI listener and C driver for reuse. Those helpers supplied the planner, capture and transport; no second listener, serializer, hasher or conformance framework was added. The literal fixture is shared by both host tests instead of copied into each.

The first unprivileged CLI listener attempt could not bind local loopback (`Operation not permitted`); the authorized offline loopback rerun passed. A first CLI assertion treated `meta.requests` as objects, but its actual public shape is a string array; correcting the assertion changed no product behavior. The five-text candidate and structured cut matched the fresh core. No accepted-domain wire/cache mismatch arose and no High-review stop was triggered.

## Finite remaining host proof

The original criterion requires the shared corpus to be exercised at every applicable public surface. The current first slice proves CLI and C only. The next related batch must consume the same five-text corpus through these existing runner/check routes, with ordered five-row acceptance, three counted sends and fixed bodies/request identities, or a stated unsupported-domain reason:

| Remaining package or route | Required boundary |
| --- | --- |
| Public Rust library | Five-text bulk call under Max through its public API. |
| Python, including its supported column/frame doors | Public bulk and each advertised frame conversion preserve the five strings. |
| TypeScript, Ruby, R | Each public bulk call preserves text and selected Max settings; Ruby's object-to-text convenience does not establish structured-object parity. |
| Rust Polars feature | Its public column route forwards the same string values. |
| SQLite, PostgreSQL, DuckDB | Deterministic ordered five-text SQL route under each host's own NULL, transaction and aggregate-order rules. |
| PHP, C#, JVM Java/Kotlin/Scala, Dart, Swift, Zig, Go, C++, Ada, GNU Objective-C, COBOL | Each of the 11 C-door packages needs a public forwarding call with counted requests and returned digest identities against the shared C corpus; JVM has three language callers. The one C corpus supplies the native body oracle without copying request files or implementing another hasher. |

Keep each package's existing installed-target and release-runner qualifications as separately owned evidence. A source route or one Linux package does not prove another target. The existing `conformance/cases.json` runners commonly force batch 1, so a new case there alone cannot meet this Max content-cut criterion. Register 19's answer/settings parity and the separate repeated-wire-text issue remain separate. This first slice measures neither answer accuracy nor provider costs.
