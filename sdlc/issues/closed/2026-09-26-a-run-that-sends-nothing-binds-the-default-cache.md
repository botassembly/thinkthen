# A run that sends nothing binds the default cache to its address

Status: closed after fresh High review accepted `2b7ca868be955e479f610e8075a84d4a4e66603d`. Filed 2026-09-26 by the queue owner from local experiment 273, report 06, finding I-2. Does not block 0.1: ticket 0124 weighed the same trade and deferred it, and the bind order it kept needs Ian to change. Ian can overturn this placement.

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

## Current reconciliation

The original no-key reproduction is fixed by 0228; 0246 and 0250 fix two further deterministic pre-admission refusals. Register 10's universal no-touch and recoverable bound-address demands were explicitly declined after High review of 0327a13d. This issue remains open for its different Done when: a status binding indicator and a safe existing-binding recovery instruction. Current status has no marker comparison, and existing refusal advice names workarounds rather than clearing the marker. Fresh High source reconciliation confirmed this distinction. The work plan now gives this issue its own row; no status or unbind feature is claimed.

## Closure

Ticket0253 meets this issue’s distinct Done when. Read-only human and JSON status distinguish missing, unbound, legacy, matching, mismatched, unavailable and disabled caches. The default mismatch refusal tells the user to stop all folder users, preserve the entire folder by moving it aside, then retry at its former path or choose a fresh folder. It never deletes a marker from existing entries. Fresh High review accepted2b7ca868 after independently passing eight status, twelve recorder identity and fourteen cache identity cases. The writer’s binding gate remains authoritative. The source’s public documentation follow-up stays recorded with marketing; no broader register10 guarantee is inferred.
