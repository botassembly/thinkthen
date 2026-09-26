# Settings some surfaces cannot reach

Status: open. Filed 2026-09-26 by ticket 0140 while it wrote `specification/settings.md`. No ticket owns it yet.

Filling the settings table showed three gaps. The table states each one as main has it.

1. **Timeout, retries and the backend profile.** No library or SQL surface can set them. `EngineBuilder::build` fixes a 30-second timeout, 2 retries and no profile (`crates/thinkthen/src/public/settings.rs:233-238`). ADR 0017 section 5 lists the engine settings a host can reach and names none of the three. No record says the omission is deliberate. A caller who needs a longer timeout on a slow backend has no lever outside the command.
2. **The model as an engine setting.** C and the three SQL surfaces have no engine-level model. ADR 0017 section 5 gives the model as "the question, then the engine value". A C or SQL caller can still put `model` in the question JSON, so each question must carry it.
3. **Wrong `cache_bytes` claims.** `databases/duckdb/README.md` line 39 says `SET thinkthen_cache_bytes = N` caps the cache's size. `databases/sqlite/README.md` line 50 calls `thinkthen_cache_bytes(n)` the cache's size cap, and line 56 tells the reader to raise it before a warm pass. `databases/postgresql/README.md` line 44 calls `thinkthen.cache_bytes` the cache cap. The setting has no effect on any library or SQL surface. Item 4 of `2026-09-25-public-library-api-gaps.md` holds its removal. These README lines should go with it, or say now that it has no effect.

One gap is deliberate and is not filed here. SQL cannot name an address or a key. Ticket 0109 decision 2, ticket 0110 decision 5 and `databases/postgresql/README.md` line 94 rule that the address and the key come from the environment on every SQL surface.
