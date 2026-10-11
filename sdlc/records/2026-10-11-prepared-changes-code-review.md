# Prepared request and campaign code review

Two fresh read-only reviewers searched every Changes item in tickets 0549 through 0556 and all five 0558 amendments. The review used immutable prepared code: `f3ab2eb935b3fbcb8b24301c1f7db4bfdc7c4558` for request, binding, session and cleanup changes; `ad4b49b4aed14b78749a7d22620d64784008114f` for campaign preparation; and `391d562b25cfacb32cc9071db86e7d72cf263874` for parity tiers. Ticket criteria came from `60773e13d`. The reviewers ran no builds or tests and inspected no active campaign logs, output or processes. This evidence concerns prepared source, not a qualified release revision.

The review found a duplicated function-to-input-kind rule in `libraries/php/src/session/operation.php:30`: PHP chooses units for find, entities for relate and records otherwise. Existing installed cases cover behavior, but the handwritten mapping violates the shared-constructor requirement. The structural cause is truth restated outside the core.

The review also found serde diagnostic interpretation in `libraries/r/thinkthen/src/rust/src/ffi/request/inputs.rs:5-18`. Its generic decoder checks a string prefix to select the per-item context diagnostic; explicit input selectors reach that decoder. The installed unsupported-declaration case exercises a different path. The structural cause is fixing one decoder while leaving another copy.

The reviewers found source support for shared Engine admission, the eager Polars duplicate-dispatch repair, condition-based native waits, host scheduler handling, thread-local diagnostic lifetimes, generated envelope use, tier filtering and missing-binding-case rejection. The typed C routes use shared Request execution. An initial suspicion about direct C calls was withdrawn after tracing `current/execute.rs` to its test-only module guard.

The review confirms the safe-cut limitation: source measures 187,611 against the requested 187,000 target, with 2,082 test declarations retained. The public relation wrapper preserves typed entity admission, an unread excess-input tail and recoverable complete members that existing public Request carriers do not jointly preserve. The existing Mac script results establish standalone Foundation carrier execution, while missing installed packages prevented native session qualification.

## What the build taught us

A passing installed behavior case can coexist with duplicated construction or diagnostic rules. Searches need to follow every production selector and materialized-input path, then distinguish generated codecs from handwritten semantic choices. Module guards also matter: a direct Engine call retained only as a test projection does not prove a shipped routing defect.

Instruction chronology matters during integration. The earlier R/Python reader-rule deletion was followed by an explicit instruction-preservation change. Source review alone supplies no authority to discard the later retained rule.
