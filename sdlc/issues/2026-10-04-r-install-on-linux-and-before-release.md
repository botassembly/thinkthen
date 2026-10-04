# The R package has no Linux install for most hosts and no proof before release

Status: open. Ticket 0398 slice B implements only the R-universe binary install check after publishing. The Linux install documentation and source-tarball proof before release remain open. Filed 2026-10-04 by the queue owner from the 0.1 releases. Owner: the queue owner.
Milestone: 0.2

R-universe built 0.1.2 in run 37142825335 in `r-universe/botassembly`. Its plain address serves Linux the source package, which needs Rust's `cargo` and `rustc`. R-universe's package listing shows built Linux packages only for Ubuntu 26.04 ("resolute"), for R-release 4.6 and R-devel. Quick fix `67d27b7d1` names the built package's address on the install page, but the page does not name Ubuntu 26.04. Other distributions and older Ubuntu releases must build from source.

The R package's published shape is built for real only by R-universe, after the release is public. `check.sh` covers it with a crates.io stand-in (ticket 0395). The 0.1.1 R-universe build failed that way, and the defect shipped. R-universe syncs on its own schedule, so the first build can come hours after the release.

## Read-only preflight, 2026-10-04

The coordinator's unauthenticated reads found the public source `PACKAGES` and package API available with HTTP 200 and version 0.1.2. The package API's `API_binaries` lists a successful resolute x86_64 R 4.6.1 build from run `37142825335`. At the exact installed-package base `https://botassembly.r-universe.dev/bin/linux/resolute-x86_64/4.6/`, however, `PACKAGES`, `PACKAGES.gz`, `PACKAGES.rds` and `thinkthen_0.1.2.tar.gz` each returned HTTP 404. Each binary read was limited to 64 response bytes. No package download or R install ran.

[R-universe's official binary-install documentation](https://docs.r-universe.dev/install/binaries.html) still specifies this repository base. A further coordinator read of that base plus `src/contrib/PACKAGES` returned HTTP 200, version 0.1.2 and a `Built` field naming R 4.6.1 on x86_64 Linux. `install.packages` normally adds `src/contrib` to a repository base. `install_check_channels.r_universe` instead appends plain `PACKAGES` and the archive filename directly to the base. Its offline fixture accepts those incorrect paths. The 404 therefore exposes an install-check defect and establishes neither a missing build nor source-only availability.

Ticket 0398 slice B owns the path and fixture correction before its hosted install check. Fresh build and review remain required. A direct archive read under the correct `src/contrib` path, using both the plain filename and indexed `Path`, returned HTTP 403 with Cloudflare code 1010 through `urllib`. A later `curl -q -I -L` request to `src/contrib/thinkthen_0.1.2.tar.gz` followed HTTP 302 to HTTP 200. Production fetch uses curl, so the urllib response reflects a client-policy difference and establishes no archive-unavailability finding. The confirmed consumer defect is the missing `src/contrib` suffix for the index and archive URLs. No R install or archive-body download ran; failed prefix reads stopped after at most 64 response bytes. Hosted proof remains pending. No install-check workflow dispatch is authorized by this preflight.

The lane retains these read-only receipts under ignored build output: `target/0398b-r-contrib-preflight/index-read.json` records the index's 404 and corrected 200 with version and Built metadata; `target/0398b-r-contrib-preflight/archive-head.log` records curl's archive HEAD redirect and final 200. The records above preserve their conclusion without publishing scratch artifacts.

To evaluate:

1. The install page and the R README name Ubuntu 26.04 for the built package, and say what other Linux hosts need.
2. Whether the release workflow builds the R source tarball as R-universe will, from the published crate, before the GitHub release goes public.
3. Whether a GitHub release can trigger R-universe's sync, or whether the release checklist waits for it.
