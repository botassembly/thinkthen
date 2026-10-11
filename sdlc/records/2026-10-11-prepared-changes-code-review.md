# Prepared request and campaign code review

Two fresh read-only reviewers searched every Changes item in tickets 0549 through 0556 and all five 0558 amendments. The review used immutable prepared code: `f3ab2eb935b3fbcb8b24301c1f7db4bfdc7c4558` for request, binding, session and cleanup changes; `ad4b49b4aed14b78749a7d22620d64784008114f` for campaign preparation; and `391d562b25cfacb32cc9071db86e7d72cf263874` for parity tiers. Ticket criteria came from `60773e13d`. The reviewers ran no builds or tests and inspected no active campaign logs, output or processes. This evidence concerns prepared source, not a qualified release revision.

The review found a duplicated function-to-input-kind rule in `libraries/php/src/session/operation.php:30`: PHP chooses units for find, entities for relate and records otherwise. Existing installed cases cover behavior, but the handwritten mapping violates the shared-constructor requirement. The structural cause is truth restated outside the core.

The review also found serde diagnostic interpretation in `libraries/r/thinkthen/src/rust/src/ffi/request/inputs.rs:5-18`. Its generic decoder checks a string prefix to select the per-item context diagnostic; explicit input selectors reach that decoder. The installed unsupported-declaration case exercises a different path. The structural cause is fixing one decoder while leaving another copy.

The reviewers found source support for shared Engine admission, the eager Polars duplicate-dispatch repair, condition-based native waits, host scheduler handling, thread-local diagnostic lifetimes, generated envelope use, tier filtering and missing-binding-case rejection. The typed C routes use shared Request execution. An initial suspicion about direct C calls was withdrawn after tracing `current/execute.rs` to its test-only module guard.

The review confirms the safe-cut limitation: source measures 187,611 against the requested 187,000 target, with 2,082 test declarations retained. The public relation wrapper preserves typed entity admission, an unread excess-input tail and recoverable complete members that existing public Request carriers do not jointly preserve. The existing Mac script results establish standalone Foundation carrier execution, while missing installed packages prevented native session qualification.

A separate fresh read-only review accepted the corrective source range `0a4e34b61..fa83d9035cf0e6640567863a73a89aa0223ff263`. It found no runtime defect in shared typed decoding, named construction, counted-buffer ownership, eager materialized admission or independent immediate session diagnostics. PHP no longer selects an input kind or Call class from the function name. R no longer interprets serde diagnostic prefixes. Existing assertions remain, and the added PHP invalid-tail case uses the existing zero-send check. The reviewer ran no compiler or installed checks.

Independent ratchet commands on that corrective revision both exited 1: core source measured 187,637 against the unchanged 187,611 ceiling, and PHP source measured 1,916 against its unchanged 1,902 ceiling. The dependent R decoder accounts for 16 added core lines; named construction adds a net 10 after reusing the existing argument accessor. These measurements do not establish passing closure gates or authorize a ceiling increase.

The same product/test classifier used for the cleanup comparison measured 79,577 product lines and 103,127 test lines under `crates/` at the corrective revision. Against the identically classified baseline of 80,643 product lines, the reduction is 1,066. The retained classifier is `target/0558-count-comparison/split.sh` in the primary checkout; its product/test scope differs from the ratchet's combined `crates/` and `conformance/` scope.

## What the build taught us

A passing installed behavior case can coexist with duplicated construction or diagnostic rules. Searches need to follow every production selector and materialized-input path, then distinguish generated codecs from handwritten semantic choices. Module guards also matter: a direct Engine call retained only as a test projection does not prove a shipped routing defect.

Instruction chronology matters during integration. The earlier R/Python reader-rule deletion was followed by an explicit instruction-preservation change. Source review alone supplies no authority to discard the later retained rule.
