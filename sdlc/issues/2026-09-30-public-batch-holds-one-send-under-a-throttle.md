Status: Open. Filed 2026-09-30 by ticket 0317.

# A public batch keeps one send in flight where its contract promises a throttle's worth

## What happens

A pulled public batch with `BatchSetting::Records(1)` and throttle 2 keeps one send in flight while that send is held. The loopback backend counts 1 request and nothing more until it releases the reply. The contract says the batch keeps a throttle's worth of sends in flight.

Two tests hid the gap. `public_batches::a_batch_reads_its_input_at_most_one_throttle_ahead_of_its_rows` waited for 2 held requests. `public_controls::a_stop_during_a_batch_or_a_cache_lock_wait_sends_nothing_new` waited for 4. Each wait saw 1 request and gave up at the backend's 5 s bound. Neither test asserted the count its wait returned, so both passed. Record 0305 measured both at 5.4 s on 2026-09-29, so the gap already existed then. Ticket 0317 changed both waits to 1, which removes 10 s of test time and keeps every assertion.

## What the contract says

- `sdlc/issues/closed/2026-09-26-batching-design.md`: "`--jobs N` means N batches in flight."
- `libraries/polars/README.md`: a column takes "the same batch path as a slice of strings, at the same throttle".
- `libraries/r/NOTES.md`: `batch = 2L` with enough records "fills eight held request slots under throttle 8".

## Proof it fails

`public_batches::a_batch_keeps_a_throttle_of_sends_in_flight_while_they_are_held` holds every reply and asserts the backend counts `THROTTLE` requests. It is ignored as known failing. Run it with `cargo test -p thinkthen --test public_batches a_batch_keeps_a_throttle -- --ignored`.

## Where to fix it

ADR 0111 slice 3 moves the public Rust API onto the one batching path, `ask_all`. That slice is the natural fix point. The one scheduler should keep a throttle's worth of sends in flight for a pulled batch.

## Done when

The ignored test passes and loses its `#[ignore]`.
