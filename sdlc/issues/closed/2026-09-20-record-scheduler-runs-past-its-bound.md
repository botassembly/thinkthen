# Record scheduler runs past its bound

Status: Closed by ticket 0024

Found 2026-09-20 in the full-project review at `2c32524`.

The record scheduler fills an open request slot before it prints a completed row. Under `--jobs 1`, a caller that writes one record and waits for its answer gets no answer until it writes another record. This contradicts the documented `coproc` loop.

The same order lets a slow first record hold every later completed row. A local listener delayed record 1 under `--jobs 4`; the tool accepted all 100 records before record 1 finished. The documented bound of at most `jobs` waiting rows does not hold, and paid work and memory can grow with the input.

The scheduler must print ready rows before reading more input and bound every dispatched row that has not reached its ordered output position. Deterministic tests need an incremental input pipe and a barrier on record 1.

Ticket 0024 closed both defects. It also added regression tests for a backend failure and a closed output pipe while standard input remains open.
