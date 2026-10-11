# Core usage, readers and rank

The public usage finalizer returns `UsagePersistence`, so a failed write remains observable without invalidating an answer. The three SQL exit hooks explicitly consume the result and retain their existing shutdown behavior. The silent internal wrapper and the unfrozen C JSON persistence requests were removed. The C README and input schema name the existing typed exports.

A poisoned reader lock now reports a safe defect instead of end-of-input. Cleanup wakes a waiting reader. The retained CLI readiness paths have one table covering emission while input remains open and cancellation for decide, annotate and recognize. Plain set-rank coverage checks retained originals after excluded members are released.

Fresh code review accepted these changes and the root ceiling increase from 189337 to 189472 for distinct behavior regressions. The C ceiling fell from 31818 to 31742 after deleting obsolete routes and repeated assertions. The reviewer found one inaccurate commit explanation: the added rank regression takes its existing file past the size-warning boundary. The explanation was corrected without changing the accepted implementation.

Focused Rust usage, poisoned-reader, rank, CLI readiness and C AddressSanitizer checks passed. Routine coverage includes all 1995 workspace cases, the public consumer, doctests and supporting checks. The original workspace run timed out on two aggregate hostile-entry cases. Both passed unchanged under the five-second limit on focused retry. Remaining cases passed with the suite's owned configuration and usage setup; 108 cases initially resumed without that outer setup were checked again with it. No timeout or expectation was relaxed.

Closure lint exposed stale ticket formatting and the old usage-finalizer declaration; the inline summaries and reviewed API delta were corrected. Policy, workflow, installer, ratchet, dependency, formatting, Clippy and documentation checks passed, and the corrected public inventory passed separately. The normalized ticket status also exposed a speed-check parser that recognized only historical `landed`. It now recognizes the first canonical `COMPLETE` token too. Fresh review accepted that small correction, and the existing stale-exception regression passed.

The retained logs and test selections are in `target/0539-core-6a0f32dc6/` in the main checkout. They include the original failures, focused retries, isolated remaining cases, public inventory and supporting-suite results. These local checks use fake responses and establish no platform qualification.

## What the build taught us

An ignored persistence result and a reader failure disguised as exhaustion both hide a failure behind successful work. Return the existing status and error rather than creating another control path.

Ticket readers must recognize the canonical status token. Reusing a completed run requires preserving its configuration and usage isolation as well as its selected cases.
