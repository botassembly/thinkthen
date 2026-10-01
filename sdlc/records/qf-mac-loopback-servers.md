# Quick Fix qf-mac-loopback-servers: the Mac release smoke starts its loopback servers at once

Status: built in lane claude-3. Ian can overturn the shared server class.

## Why

Release rehearsal run 36926183166 passed every surface and `installer-test` (50/50) on both Mac smoke jobs. Both then failed in `release-workflow installer-smoke` with "loopback install server did not start", 10.7 s after `installer-test` finished. Linux passed the same command.

Ticket 0386 found the cause. CPython's `HTTPServer.server_bind` calls `socket.getfqdn`, and that reverse lookup stalls about 35 s on a macOS GitHub runner. The install server wrote its port only after the stall, and `installer-smoke` waits 10 s for the port. Ticket 0386 fixed only the shared SQL test proxy and listed the other Python servers as left alone.

The same run shows the stall elsewhere. `installer-test` took 43 s on the ARM Mac against 3 s on x86 Linux. It has no start wait, so it passed slowly.

## Change

- `sdlc/scripts/loopback_server.py` holds the one `LoopbackServer` class. It is a `ThreadingHTTPServer` that names itself from its bound address and skips `socket.getfqdn`. Run as a script, it serves a folder and writes its port file whole, through a rename.
- `release-workflow installer-smoke` starts that script. The wait stops early when the server ends. A server still running after 10 s fails with "loopback install server wrote no port in 10 s". A server that ended fails with "loopback install server exited N before it wrote its port", after Python's own error output.
- `databases/sqlite/tests/conditional_backend.py` imports the class from the new file in place of its own copy from ticket 0386.
- `sdlc/scripts/installer-test` and DuckDB's `PackedReplies` (`databases/duckdb/tools/verbs_budget.py`) use the class. The DuckDB installed step runs `PackedReplies` on the Macs under a 900 s limit.
- The Python ratchets follow: DuckDB tools 4177 to 4182, SQLite tests 2543 to 2539.

## The sweep

These are the Python HTTP and TCP servers that the Mac smoke step starts, and the release steps after it. Every job after `smoke` runs on Linux.

- `release-smoke` on a Mac runs the command, C, SQLite, DuckDB, PostgreSQL, Python, TypeScript and Ruby surfaces. The conformance backend is Rust. The SQL proxy already had the fix, and DuckDB's `PackedReplies` gets it here. The Python surface's installed tests use the Rust backend. Ruby's `TCPServer` and Node do no reverse lookup.
- `installer-test` and `installer-smoke` get the fix here.
- `crate-smoke` starts no server.
- The Go, C++, Swift, Zig, PHP, Dart, Ada, Objective-C, COBOL, C# and JVM fixture servers run in the release smoke only on x86 Linux. Python's full-check tests and the `probes/` measures do not run in the release. These keep their own `ThreadingHTTPServer` binds.

## crate-smoke on a Mac

`release-workflow crate-smoke crate-dist smoke-files` had never run on a Mac runner. On the M5 (Darwin arm64, official Rust 1.95.0, `MACOSX_DEPLOYMENT_TARGET=15.0`), it built run 36926183166's packed crate offline in 19 s and the first run answered `true`. The crate's `Cargo.lock` names 252 registry packages, and the workspace lock that `host-setup` fetches holds every one, so `--offline` finds them. No Mac-specific change was needed. The M5 has no x86_64 Rust standard library, so the Intel build was not run. The Intel runner builds natively.

## Checks

- A `sitecustomize.py` shim made `socket.getfqdn` sleep 35 s, as ticket 0386 did.
- M5, main, run 36926183166's ARM archive: `installer-smoke` passes without the shim and fails under it with "loopback install server did not start" after 11 s.
- Linux x86_64, main, run 36926183166's musl archive: the same failure under the shim after 10 s.
- M5, branch, under the shim: `installer-smoke` passes in 2 s with the ARM archive and in 3 s under Rosetta with the Intel archive. `installer-test` holds 50/50 in 11 s. Ticket 0386's SQLite regression passes, and `PackedReplies` starts and stops.
- Linux, branch: `installer-smoke` passes with and without the shim. Under the shim, `installer-test` holds 50/50 in 2 s, against 38 s on main. The PostgreSQL proxy run as a script prints its port at once.
- Plants on Linux: a Python start that exits prints "loopback install server exited 1 before it wrote its port" after the planted error. A Python start that sleeps 15 s prints "loopback install server wrote no port in 10 s".
- `sdlc/scripts/lint` exit 0. Its one `Killed` line comes from the planted time-limit test. `CARGO_NET_OFFLINE=true python3 sdlc/scripts/policy.py` exit 0.
- Not run: the release workflow. The next rehearsal shows the Mac smoke jobs.

## Deferred gap

No check stops a new Python server from binding `ThreadingHTTPServer` directly. The library fixture servers listed above still do, and a future Mac job that runs them would pay the stall again.
