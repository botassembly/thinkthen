# 0400 slice C: default throttle of 8

Status: ready for landing. Product source is reviewed and focused checks pass. Slice D waits for accepted provider measurements.

Unset CLI, library and SQL settings inherit the shared default of 8. Explicit settings retain precedence and the range 1 through 32. Mapping regressions use explicit 6 so the new default cannot hide a missing mapping. Cancellation, packed requests, ordered output, shared process limits and fork behavior retain their controls. Public defaults say 8; historical measurements retain the width at which they ran.

Whole-source review covered the product source retained at `7b50d4ca0`. The latest review of `1347baa56` found only verification-runner issues. Ian dropped those findings in the 2026-10-05 core-scope ruling. This candidate adds no runner feature and keeps no new runner-history records. Main's scope, proof rules, team note and deferred milestones prevail. Ticket 0404 owns later test-name and historical-record cleanup.

The recipe loopback fixture closes both response connections explicitly. This two-line prerequisite allows the maintained policy check to validate the fixture.

The maintained focused commands pass 81 Rust cases and the C omission/explicit-six packed-request case. Policy, formatting, tickets, settings and affected ratchets pass. Full tests and lint will run on the landing commit. No paid provider call, experiment, native qualification, release or publication ran for this finish.

The shared default remains one constant. Existing helpers carry the behavior checks; new bounded fixtures cover omitted settings, explicit mapping and preview/runtime agreement. The Rust ratchet is 118250, up 143 lines from main; C is 5546 and R is 2564. Growth belongs to bounded behavior fixtures and retained helper changes. The rejected runner work showed that proof tooling can delay a small product change without finding a product defect; the suite and one source review are sufficient for this slice.
