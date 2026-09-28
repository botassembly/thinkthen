# The site benchmark pin collides with its directory on macOS

Status: open. Owner: the marketing lead under `sdlc/planning/ownership.md`. Found during authorized native M5 package verification at main `10f85fa7`. This is a public 0.1 checkout portability gap.

Git tracks the file `site/examples/beatles/BENCH` and the directory `site/examples/beatles/bench/`. The M5's case-insensitive APFS checkout cannot hold both names and immediately reports the tracked pin file deleted. Setting skip-worktree would conceal the missing tracked input and is not a fix. The native verifier restored its index and records the checkout limitation explicitly; C and SQLite source inputs are unaffected.

Rename the pin to a noncolliding name such as `BENCH.pin`, keeping its exact commit text. Update its readers and copied-file exemptions in `site/scripts/{pull-bench,check-slides,export-slides,check-samples,smoke}.mjs`, plus current user-facing references. Historical records may retain their original path with a dated pointer. The queue owner holds no site source files and has not changed the pin or contacted another agent.

Proof: a fresh case-insensitive native checkout retains both the pin and directory with no tracked deletion; the existing script readers resolve the unchanged pin; focused Node syntax and applicable sample checks pass. Do not rerun model calls or regenerate benchmark data for this path correction. Keep this open until the site's owner lands the correction and native checkout proof confirms it.
