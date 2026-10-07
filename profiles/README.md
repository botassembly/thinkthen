# Backend profiles

An explicit backend profile gives a compatible backend a stable name and local request limits. Pass its path as `--profile FILE`. The file never selects an address, model, key, adapter, cache, retry, timeout, or width.

The closed JSON shape is documented in [`specification/backends.md`](../specification/backends.md). The measured optional image declaration examples live in [`specification/fixtures/images/local/`](../specification/fixtures/images/local/README.md). They name exact setups and caller aliases; they neither configure nor attest a server. This folder carries no claimed vendor maxima. A token limit does not become a byte limit by guesswork. Add an example only after a measurement establishes the byte ceiling it states. The hosted backend needs no file here for relation plans: they split under a built-in ceiling of 96,000 request bytes, measured by ticket 0123 and described in `specification/backends.md`.
