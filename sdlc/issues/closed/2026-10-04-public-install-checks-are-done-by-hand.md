# The public install checks are done by hand

Status: closed.
Resolution: 0425
Milestone: 0.2

After each release, the queue owner installed every channel by hand: Linux containers on one machine and macOS on another. Those checks found two defects that every release job had passed:

1. The macOS gems named darwin 24, so `gem install` on macOS 26 took the 0.0.1 placeholder (ticket 0394, fixed in 0.1.2).
2. R-universe served Linux the source package, which needs Rust. The site named no Linux route (quick fix `67d27b7d1`).

They also found that hosts below the version floors fall back to the 0.0.1 placeholders with no message: Ruby 3.3, and macOS's own Python 3.9. Issue `2026-10-03-rubygems-ruby-platform-gem-is-the-0-0-1-placeholder.md` owns the placeholders. This issue owns only checking for them.

The release smoke installs the built files, not the published packages. Nothing repeats these checks for the next release.

To evaluate:

1. A workflow, dispatched after `publish`, that installs each channel from its live registry on fresh runners: the newest macOS, Linux on x86-64 and ARM, and Windows once 0.2 ships it. Each install reads the version and replays the first-run sample with no key.
2. Whether it also installs on a host just below each version floor and fails when the placeholder is chosen.
3. Which channels need a wait for their index first: R-universe syncs on its own schedule, and the Go proxy caches a missing version for a while.

## Reconciliation, 2026-10-08

The dispatched install-check workflow and release guard landed at `dda236c21`; R index/archive contribution paths were corrected at `ddfcbc74c`. Existing offline channel fixtures validate mechanics. They do not establish a successful install from each public registry or on every required platform. [0398 record](../../records/0398-release-safety.md) and 0425 retain those checks after an authorized release. The observed 0.1 placeholder failures remain historical evidence; Ruby’s candidate diagnostic fallback now exists but its publication remains held.
