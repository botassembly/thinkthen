# A run that sends nothing binds the default cache to its address

Status: Open. Filed 2026-09-26 by the queue owner from local experiment 273, report 06, finding I-2. Does not block 0.1: ticket 0124 weighed the same trade and deferred it, and the bind order it kept needs Ian to change. Ian can overturn this placement.

## What happens

A new user tries the tool with no key. `decide` at the default address exits 4 for the missing key, and it has already bound the default cache to that address by writing `.thinkthen-backend.json`. The user then points the tool at a local model. Every plain run now exits 5 with ``the default cache is bound to a backend address other than `URL`; go back to that address, use --no-cache, or set THINKTHEN_CACHE to another folder``. The message names only the new address, so "go back to that address" points at an address the user cannot see. `cache prune --max-size 1` does not clear the binding. No command does.

## Prior decisions

- `specification/recording.md` line 36 binds a folder on its first write-capable use, before the key is read. It says the message never names the bound address.
- Ticket 0124 decision 8 kept that order. Binding only at the first write would let two first users with different addresses both send, and ticket 0065 ruled the order to stop that.
- Ticket 0124's deferred gaps list a `status` line that says whether the default cache is bound to the current address, binding only at the first write, and a default cache per address. It says the last two need Ian.

## Checked on main

Verified: `recording.md:36` and ticket 0124 read as described, and the sentence comes from `crates/thinkthen/src/cli/failure/recording.rs:45`. The no-key run and the prune result come from the report.

## What would fix it

These need no new ruling:

1. Add the deferred `status` line that says whether the default cache is bound to the current address.
2. Make the refusal say how to clear the binding. Deleting the marker is the only way today. A `cache` subcommand that unbinds an empty folder would be another.

Binding at the first written entry, or one default cache per address, changes ruled behavior. Either goes to Ian with 0124's trade-offs.

## Done when

A user whose default cache was bound by a run that sent nothing can see the binding in `status` and learns from the refusal how to clear it.
