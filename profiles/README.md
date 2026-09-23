# Backend profiles

An explicit backend profile gives a compatible backend a stable name and local request limits. Pass its path as `--profile FILE`. The file never selects an address, model, key, adapter, cache, retry, timeout, or width.

The closed JSON shape is documented in [`specification/backends.md`](../specification/backends.md). This folder carries no claimed backend limits yet. A token limit does not become a byte limit by guesswork. Add an example only after a measurement establishes the byte ceiling it states.
