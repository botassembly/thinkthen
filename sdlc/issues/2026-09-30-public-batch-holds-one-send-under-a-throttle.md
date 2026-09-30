Status: Open. Filed 2026-09-30 by ticket 0317.

# A public batch holds one send where its tests expected a throttle's worth

Two tests waited for the held backend arm to count more than one request. `public_batches::a_batch_reads_its_input_at_most_one_throttle_ahead_of_its_rows` waited for 2 (its throttle). `public_controls::a_stop_during_a_batch_or_a_cache_lock_wait_sends_nothing_new` waited for 4. Each wait saw 1 request and gave up at the backend's 5 s bound. Both tests then passed, because neither asserts the count the wait returned. Record 0305 measured both at 5.4 s on 2026-09-29, so the wait already timed out then.

Ticket 0317 changed both waits to 1, which removes 10 s of test time and keeps every assertion. The question stays open: with `BatchSetting::Records(1)` and a throttle above 1, should a pulled public batch keep more than one send in flight while the first is held?

## Done when

ADR 0111's one batching path states how many sends a pulled batch keeps in flight, and a test through the public API asserts that count.
