# Engine request repair

Engine calls use the shared Request intake and ordered execution path. Backend bounds admission happens inside the call's failure boundary, so a refusal before any send carries facts with zero requests. Source parsing remains outside that boundary and retains its existing location diagnostics.

The CLI checks its output reader and cancellation signal before dispatch. Observed aggregate calls at batch size one admit the next request after the observer returns. Unobserved aggregate calls retain duplicate coalescing. These changes preserve the existing closed-output, cancellation, cache-miss and ordered observer contracts.

Both recognize regressions in `crates/thinkthen/tests/backend/recognize/sized.rs` remain unchanged: exact encoded size versus one-byte excess, and contextual probe admission versus plan maxima. The complete official routine at `ea591efb433b62981f03ac372f022764ce7337cf` passed 2,005 Rust tests, with 73 skipped release cases, and its external consumer checks passed 21 cases with five platform skips. Official lint also passed. The optional private-name check did not run because its environment setting was absent.

The routine used isolated Cargo configuration and completed with both exits zero. Fresh code review accepted the repair and the explicit C++ fixture ceiling correction from 342 to 344 lines.

The unchanged Polars duplicate-input regression exposed another eager-dispatch defect. Singleton pacing retired the first pending question before admitting its identical neighbour, causing two sends and 98 reported input tokens instead of one send and 49. Cost arithmetic correctly reflected those extra sends. The one-line repair at `30979f0c4e2475d9a05c9ddb6fcf77edbc97304c` aggregates all eager atomic inputs while leaving observer pacing unchanged. The original cost, usage, original-position and send-count assertions stay unchanged. Its owning test and all 46 active Polars cases passed, with two existing ignored cases, followed by 128 selected library cases and 259 related integration cases. Independent before-and-after diagnostics confirmed the send and usage change. Strict production-library Polars Clippy passed; all-targets lint exposed duplicate fixture modules and unused lint expectations in the cleanup branch.

## What the build taught us

Focused Request, session and binding checks missed the two backend recognize regressions because they did not run the CLI backend recognize cases. The full routine exposed that gap, then exposed closed-output dispatch and observer pacing regressions in the same shared execution path. A focused success cannot stand in for the routine coverage named by the ticket.

The default routine also omitted the Polars feature's priced duplicate-input case. Installed parity did not exercise that exact combination of identical rows, singleton batching and explicit prices. The unchanged full Polars suite exposed the extra dispatch and passed after the eager-aggregation correction. A rounded-cost failure can reveal extra dispatches; changing its expected decimal would conceal the defect.

A detached background launcher inherited ignored interrupt signals and made a library signal case fail. Resetting the launcher signals and using the configured nextest routine made that exact unchanged case pass. The original failure and corrected-run logs remain separate. Build owners must preserve the signal environment required by the tests they launch.

The held-send fixture previously waited for a host callback while sends were held and used a ten-second fallback. The existing backend barriers now release those sends before invoking host checks. Every assertion and the later cache-miss case remains unchanged. The two recognize tests required no fixture changes.

A temporary repository Cargo configuration violated the existing policy. The successful routine used an owned, isolated Cargo home with existing cache symlinks and neutral compiler path remaps, then removed its configuration after both commands exited. It changed no shared configuration or credentials.
