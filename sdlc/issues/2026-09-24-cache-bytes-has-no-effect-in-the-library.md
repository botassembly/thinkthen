# `EngineBuilder::cache_bytes` has no effect in the library

Status: Open. Found by ticket 0086's build and code review, 2026-09-24. Owner: ticket 0098 or the ticket that gives the library a cache prune.

## What happens

0084 freezes `EngineBuilder::cache_bytes(self, value: u64) -> Result<Self, Error>`. Ticket 0086 checks the value and refuses 0. Nothing else reads it: the engine's storage has no cap, and the library has no prune. The command's `thinkthen cache prune` reads its own configured cap, so the library value never reaches it.

## Why it is deferred

The frozen signature has to stay, and a cap with no prune changes nothing a caller can see. The doc on `cache_bytes` says the value is checked and has no effect in 0.1.

## Fix

Either give the library a prune that applies the cap, or drop the setter from the contract through an 0084 amendment before 0.1 ships. Then a test observes the cap through the cache folder's size.
