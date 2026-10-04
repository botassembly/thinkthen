# The R package has no Linux install for most hosts and no proof before release

Status: open. Ticket 0398 slice B implements only the R-universe binary install check after publishing. The Linux install documentation and source-tarball proof before release remain open. Filed 2026-10-04 by the queue owner from the 0.1 releases. Owner: the queue owner.
Milestone: 0.2

R-universe built 0.1.2 in run 37142825335 in `r-universe/botassembly`. Its plain address serves Linux the source package, which needs Rust's `cargo` and `rustc`. R-universe's package listing shows built Linux packages only for Ubuntu 26.04 ("resolute"), for R-release 4.6 and R-devel. Quick fix `67d27b7d1` names the built package's address on the install page, but the page does not name Ubuntu 26.04. Other distributions and older Ubuntu releases must build from source.

The R package's published shape is built for real only by R-universe, after the release is public. `check.sh` covers it with a crates.io stand-in (ticket 0395). The 0.1.1 R-universe build failed that way, and the defect shipped. R-universe syncs on its own schedule, so the first build can come hours after the release.

To evaluate:

1. The install page and the R README name Ubuntu 26.04 for the built package, and say what other Linux hosts need.
2. Whether the release workflow builds the R source tarball as R-universe will, from the published crate, before the GitHub release goes public.
3. Whether a GitHub release can trigger R-universe's sync, or whether the release checklist waits for it.
