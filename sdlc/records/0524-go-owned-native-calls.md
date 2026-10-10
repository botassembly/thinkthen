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

## Source size

Go source grows from 4,953 to 7,753 nonblank lines; 2,275 lines are generated accessors derived from the native graph. Fixture Python grows from 826 to 898. The exact generated output is exempt from the handwritten file-size ceiling and still counts toward its language ceiling. The existing large `thinkthen.go` file gains only conditional linker declarations; its compatibility implementation remains until retirement. Handwritten executable source adds 653 and removes 9 nonblank lines, counting the generator template, focused installed consumer and package helper. Reviewer acceptance of the increased ceilings is required.

## What the build taught us

Every cgo file's unconditional pkg-config declaration affected the installed package, even when the new client did not call its compatibility functions. All such declarations needed the same target selection. A cancellation signal stops intake before the blocked provider settles; the host must stop its reader separately instead of waiting for terminal facts. The held-provider cleanup test caught and fixed that delay.

## Remaining outcomes

This slice leaves compatibility APIs/readers in place. Full installed conformance, image and bounded producer adoption, other Unix assets, final module archive assembly under 0530 and compatibility retirement remain owned work. The shared C session currently reports Rust surface attribution; Go preserves that native fact honestly until the common interface supplies binding attribution. No release readiness or whole-ticket completion is claimed.
