# 0524: Go owned native calls

## Change

This additive slice starts from e2382c091. `Client` exposes all ten named methods over the shared request session. Ordinary strings, definition maps and slices are converted at the host boundary; `Item` separates per-record fields and `FileRecords` selects the native reader. The host does not implement admission, cache identity, backend routing or judgment rules.

The Go target consumes the shared prepared result graph. Every object field becomes a generated typed accessor returning `Presence[T]`; absent, null and false remain distinct. Scalars keep exact JSON numbers. Union accessors check their native tags before converting. Packet and result bytes are copied before the C owner is freed; unknown fields survive round trips. Default formatting withholds result contents. `SessionError` retains generated failure details, the packet prefix and terminal facts.

Calls poll the nonblocking native door while other goroutines run. Context cancellation frees the owner promptly. Client cleanup cancels active sessions and stops their host readers. A terminal pointer stays nil while native settlement is pending; cancellation does not fabricate final facts.

Linux amd64 cgo linking selects the bundled static library through the module-relative native folder. The staged module contains the copied header and localized archive and builds without pkg-config, dynamic library paths or the source checkout. Existing compatibility files only change linker declarations. Source checks create their own exact native link and remove it before invoking existing scratch cleanup.

## Focused checks

The installed external consumer built from `target/0524/module-final`, without a source-tree module or native-library override. Its named calls cover all ten functions, retained typed facts after cleanup, false and authored null, missing fields and unknown-field round trips, original file paths and physical lines 1 and 3, native model/source metadata, a typed deadline failure and invalid and pre-cancelled inputs. Provider counts equal the successful calls' native request totals, so deadline, invalid and pre-cancelled cases add no sends.

Two held-provider checks prove another goroutine progresses and both context cancellation and client cleanup return before the fake provider is released. The consumer is statically linked and has no `libthinkthen` dynamic dependency. The final localized native archive SHA-256 is `0a2562b2847db10b12b8eb1c98febcec374758ffa13fddcdaa01aa04502a2608`.

The native build used the existing capped, offline release build. Generated Go freshness, Go formatting, installed-module vet, source policy and exact Go/Python ratchets are the bounded checks for this slice. Full shared parity, large-input tests and release actions were not run.

Review found that the source replay smoke still staged only the shared C library, while Linux amd64 now links the static archive. The Go smoke now runs the existing C localization helper on the archive resolved by `native_install`, placing `libthinkthen.a` in that helper's owned prefix before creating the source link. The existing one-surface replay smoke passed through this actual staging path: the seed sent one loopback request and the Go call reused its cached answer. The helper's existing offline build refreshed the debug C archive under the memory cap. The source link and its scratch prefix were removed on exit. Installed-module code did not change, so its checks above remain applicable. This repair adds one shell line; Go and Python source ceilings remain 7,753 and 898.

## Source size

Go source grows from 4,953 to 7,753 nonblank lines; 2,275 lines are generated accessors derived from the native graph. Fixture Python grows from 826 to 898. The exact generated output is exempt from the handwritten file-size ceiling and still counts toward its language ceiling. The existing large `thinkthen.go` file gains only conditional linker declarations; its compatibility implementation remains until retirement. Handwritten executable source adds 653 and removes 9 nonblank lines, counting the generator template, focused installed consumer and package helper. Reviewer acceptance of the increased ceilings is required.

## What the build taught us

The surface-attribution integration starts from `08141007d`. A copied installed-module regression observed `thinkthen/0.2.0 (c)` on both a successful Go call and a typed provider failure. The C constructor correctly identifies direct C callers; Go must select its own native surface through the additive counted constructor. `session.go` passes C-owned `go` bytes and retains their extent through the call. The existing installed consumer now captures the real User-Agent and checks retained success and failure facts after client cleanup. Facts have no surface field, so attribution is checked at the transport boundary.

The focused `surface` mode passed with exactly two loopback requests, each reporting `thinkthen/0.2.0 (go)`, and no dynamic native dependency. The copied module bundles the warm C build's localized static archive, SHA-256 `0a87559d7747530492a66ed6c6bde8ab3f4edaea613862b9f841dad2f3fcc502`; no source-path native override or Rust rebuild was used. Generated result types and the package helper are unchanged. This integration adds 21 nonblank Go lines and 13 nonblank fixture Python lines. The measured ceilings are 7,774 Go and 911 fixture Python lines. Growth requires reviewer acceptance before landing. Full parity and release qualification remain outside this bounded check.

Every cgo file's unconditional pkg-config declaration affected the installed package, even when the new client did not call its compatibility functions. All such declarations needed the same target selection. A cancellation signal stops intake before the blocked provider settles; the host must stop its reader separately instead of waiting for terminal facts. The held-provider cleanup test caught and fixed that delay.

## Remaining outcomes

This slice leaves compatibility APIs/readers in place. Full installed conformance, image and bounded producer adoption, other Unix assets, final module archive assembly under 0530 and compatibility retirement remain owned work. Go named sessions now identify their native Go surface. No release readiness or whole-ticket completion is claimed.
