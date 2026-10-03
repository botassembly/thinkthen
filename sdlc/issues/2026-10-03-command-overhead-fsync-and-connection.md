# The command's own time: the usage fsync and a new connection per run

Status: open. Filed 2026-10-03 from the docs message "The command spends 15 to 18 ms of its own time a run" of 2026-10-01 (site ticket 0050). Owner: the queue owner.
Kind: idea
When: a user or a page needs the command faster
Milestone: later

Site ticket 0050 measured what the command adds to a call, on checkpoint `surfaces/2026-10-01-2` (`bfc180a10`), against the conformance backend on a loaded machine. A whole `decide` run took a median of 22 ms in a debug build and 18 ms in a release build. The site's overhead page holds the figures.

Settled: the published command is a release build. `sdlc/scripts/release-pack` builds the command, the C library and the SQL extensions with `cargo build --release`, so the debug figures describe only local builds.

Open:

1. The usage write. `strace -T` put 3.6 to 11 ms of a release run inside fsync and fdatasync, one or two calls a run. The question is whether count-only usage totals need fsync on every run, or whether a cheaper write, such as a rename without fsync, keeps the same guarantee. `crates/thinkthen/src/engine/usage/storage.rs` and `usage.rs` call `sync_all` on the totals file, its lock and its folder.
2. The remainder. About 5 ms of a release run is unexplained, inside the noise of a loaded machine. Profile `thinkthen decide` against the conformance backend on a quiet machine before chasing it.
3. The connection. A live command run took a median of 219 ms, and a library call took 138 ms. Each command run opens a new connection, and a library keeps its connection. Time the connection with a test build before changing anything. A live timing run needs Ian's authorization.
