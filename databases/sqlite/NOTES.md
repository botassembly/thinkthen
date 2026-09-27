# Builder notes: the SQLite port (ticket 0109)

The tag's `NOTES.md` stays at `surfaces-wave7-frozen-2026-09-24b` as history. These notes hold what the port found. Its build record is `sdlc/records/0109-build-port-sqlite-surface.md`.

## Where the port departs from the ticket's text

- **The loopback key.** The engine refuses to send with no key unless the base is a loopback address, and there it sends the request with no `Authorization` header (Quick Fix qf-command-edges-and-prune). Every test child therefore gets the fixed fake key `sk-sqlite-loopback` beside its own loopback address. The caller's `THINKTHEN_API_KEY` never reaches a child. Decision 16's text says the key stays unset in every test but the secrecy test. Its purpose holds: no child can reach a paid backend, and no real key crosses into a test.
- **`thinkthen_max_requests` on a warm.** The engine's `max_requests` sends the records inside the limit, then refuses. A limit of 2 over three rows counts 2 sends, not 0. The test pins 2.
- **Retries.** `EngineBuilder::build` sets `max_retries: 2` (`crates/thinkthen/src/public/settings.rs`). The 503 arm therefore counts three sends, and the test cites the setting.
- **`thinkthen_usage()` builds no engine.** Before the first call it reads 0 for every total, and a setting after it still applies. The settings test pins that order.
- **The seed plant.** The planted `Engine::builder()` copies the loopback address and the loopback key. It counts 1 send in the planted run too, so its second call still read a cached answer. Nothing lands in `THINKTHEN_CACHE`, and the test turns red there.
- **Case numbers.** The ticket's "case 68" is `41-offsets-past-an-accent-and-an-emoji` in `conformance/cases.json`. Its branch provenance is 68.
- **Not-run forms.** The conformance runner's one table names four forms with no SQL spelling: `rank`, `find`, a recognize spec with relations (decision 10), and the defect injection, which no outside boundary reaches. The ticket named `rank` and `find`. Find has no SQL function yet. A question set that reads a part with `on` runs since ticket 0150. Any other reason fails the run.
- **The cancelled case.** Case 23 cancels a Rust token before the call and counts 0 sends. SQL cancels through an interrupt during a call, so the runner interrupts a held call 300 ms in and counts 1 send.
- **Schema messages.** A CHECK constraint, generated column, or index in an attached file refuses at schema load, with `malformed database schema (NAME) - unsafe use of thinkthen_decide()`. A DEFAULT, view, or trigger refuses at use. The test pins each sentence.

## Findings for the landing agent

- **The registry's deny call uses the root `deny.toml`.** `sdlc/scripts/surfaces --registry` runs `cargo deny … --config deny.toml` for every landed binding. The root file rejects `foldhash` 0.2.0 (Zlib), which `rusqlite` brings. The binding's own `deny.toml` passes. Moving this surface to landed therefore fails `lint` until the registry passes `--config "$surface/deny.toml"` when that file exists. The ticket's comparison check of the two deny files also belongs in `lint`. The brief for this port forbade ladder-script changes, so neither is made here.
- **`thinkthen_max_requests` caps one engine call.** Each scalar row is its own call, so the limit caps nothing on a large statement. Ian's ruling of 2026-09-25 added `thinkthen_max_requests_total` (ticket decision 17). A probe showed a forked child's engine counts from zero: the parent sent 1 under a total of 2, and the child still sent 2.
- **The shared port.** Every test here counts sends, so each starts its own backend. `check.sh` takes the rung's port and leaves it unused.

## Measurements

- Production Rust: 1,448 nonblank lines in seven files. Unit tests: 158 lines.
- Python: 954 lines in eight test files and the helper. The conformance runner and its planted-failure test: 225 lines.
- Scripts: `check.sh`, `setup.sh`, and `tests/host_sqlite.sh` hold 99 nonblank lines.
- Conformance: 38 pass, 0 FAIL, 16 not run, 54 of 54.
