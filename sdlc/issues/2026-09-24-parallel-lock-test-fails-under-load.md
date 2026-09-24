# The parallel lock test fails under machine load

Status: open. Found while landing 0095.

`parallel::different_cache_digests_do_not_share_a_lock` in `crates/thinkthen/tests/backend/parallel.rs` asserts `listener.peak() == 2` for two records at `--jobs 2` with a 50 ms reply delay. On 2026-09-24, with the one-minute load at 10.27, `sdlc/scripts/test` at `62bdc623` failed it with a peak of 1. The same tree passed when the load was lower. The first worker can finish its 50 ms reply before a starved second worker sends, so the peak depends on scheduling.

A fix makes the proof independent of timing. For example, the listener can hold the first reply until a second request arrives, with a bounded wait. The ticket that fixes it plants a bug that shares one lock across digests and shows the test still turns red.
