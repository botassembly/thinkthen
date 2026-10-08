# 0455: Local MCP calls over the native engine

The local `thinkthen mcp` server exposes the ten named functions over stdio. One native engine owns parsing, question/file loading, image admission, cache, record/replay and complete results. EOF, cancellation and broken output stop and join owned work; credentials remain startup settings.

A fresh High review found four concrete admission defects. The fixes retain loaded question metadata while explicit fields override its pointer, validate finite arrays before sending, enforce the whole-set size limit before composition, and install the existing file-size signal handler at startup. A fresh narrow High review accepted those corrections. Focused evidence covers 34 Rust tests, 16 installed groups, nine client checks and 25 public cases. Full integration tests, lint and executable documentation passed before the push.

## What the build taught us

The transport must reuse native admission, including explicit-field precedence and whole-set byte limits. A local server still needs the command’s file-size and cancellation behavior; successful small calls do not establish either. Complete results also need the original ordinal and selected whole-set candidate data; retaining them in native code lets each adapter expose the same answer without reconstructing it.

## Complete adoption

Slice B passes all 250 required cases through the installed command. Explicit typed input descriptors retain JSON, separate context, ordered candidates, images and physical source. Native complete results retain original ordinals, find candidates and ancillary images; failed incremental calls retain the actual completed prefix and final facts. Incoming frames remain bounded at 16 MiB; both equivalent output representations share a 192 MiB limit including the newline. One fresh High whole-change review accepted the native and MCP changes. Focused evidence includes 35 Rust tests, native schema checks, capture limits, Clippy and 16 installed behavior groups. Full tests, lint and specification checks passed on combined commit 4d9d6e794. Integration corrections updated the find fixture and extracted two test helpers without removing assertions.

## Ordered rank members and buffered packets

The MCP consumer reads each ordered rank member from the native complete result. A packet-local 8 KiB buffer reduces fragmented writes above the existing pipe writer. The 192 MiB preflight, mutex, newline, explicit flush, cancellation and error kinds remain unchanged. Explicit buffer disposal discards a failed tail without retrying output during drop. The regression covers escaped strings below and above buffer capacity, short writes, interruption, terminal broken-pipe and zero-write errors, and flush failure. It checks emitted bytes independently of serializer write counts.

Fresh High review accepted the buffer change on d1718f3d1. That source passed 36 focused MCP tests, policy and full lint. Its locally installed command passed all 251 required shared cases, all 16 installed behavior tests and nine client tests. Final full tests, lint and specification checks run on this recorded candidate before landing. The Rust ceiling is 159011 nonblank lines: six added production lines and ninety added MCP regression lines, with three removed fixture lines. Existing output-limit, protocol and cancellation tests were reused; one sink table and one assertion helper retain the new failure coverage.

The first full test run exposed an invalid ordering assumption in the existing single-signal image-input fixture. Signal termination can precede the polling thread's acknowledgment file. The fixture now observes bounded termination with the exact SIGINT or SIGTERM status after a completed write larger than the pipe buffer. It retains the open writer, empty output, secrecy and zero-send checks. Second-signal acknowledgment fixtures and production interrupt handling are unchanged. The unchanged candidate passed full lint and specification checks; the final gates run again after this test-only correction.

## Local stdio timing

The retained `target/0432-union-final-stress.log` records nine paired native/MCP calls for each case below. The adapter is Cargo-built via `CARGO_BIN_EXE_thinkthen` and uses local stdio against a loopback fixture backend. Cache is disabled. Startup and initialization took 6.759 ms. Atomic and whole-set pairs made 18 requests each; record pairs made 36. Each row reports nine native and nine MCP samples, in milliseconds.

| Case | Input JSON bytes | Native median | Native exploratory p95 / max | MCP median | MCP exploratory p95 / max |
| --- | --- | --- | --- | --- | --- |
| Atomic decide | 32 | 2.531 | 2.910 / 2.910 | 3.584 | 3.741 / 3.741 |
| Two-record decide | 59 | 2.721 | 3.050 / 3.050 | 3.840 | 4.200 / 4.200 |
| Whole-set find | 41 | 2.573 | 2.843 / 2.843 | 3.596 | 4.077 / 4.077 |

The bounded timing test passed. The wider stress log ends with `Terminated`; it does not establish a passing full stress command. These measurements cover the local Cargo-built adapter. Installed archive, remote-provider and proxy latency remain unmeasured. Nine samples give exploratory tail values only.

## Remaining

The full 29-consumer installed campaign remains required under 0432. Installed archive timing remains unmeasured. Final Windows behavior is measured on the completed 0.2 build. HTTP serving, proxy policy and unmeasured speed claims remain out.
