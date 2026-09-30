# Six contract sentences drift from the code

Status: open. Filed 2026-09-30 from the system grading (`sdlc/planning/grading-2026-09-30/`), checked against main `ebe9bb7a2`. Owner: queue owner.
Kind: bug
Pay when: before 0.1.

Each sentence below is published contract, and each tells a user something the code does not do. They are small text fixes, grouped here so one Quick Fix pays them.

1. **Request counts and entries in `specification/records.md`** (grading area 1, item 1). Line 106 gives `annotate` on one document one request "for each distinct `on`", and line 108 one request per `(on group, question slice)` batch. Since ADR 0111 every annotate group shares one state and packs together. Line 113 says "equal request digests share one backend call", and line 139 says batching leaves "one entry for each batch". The store keeps one entry per question under its question key, and a rerun asks only the questions it lacks.
2. **The retryable statuses in `specification/result.md:157`** (area 5, item 2). The run facts text says only 429, 500, 502, 503, 504 and 529 have `retryable:true`. The code retries eleven statuses (`crates/thinkthen/src/engine/error.rs`, `RETRIED`: adds 520 to 524), and `specification/backends.md` lists eleven. A script written from `result.md` expects `retryable:false` on a 520.
3. **The model-refusal lines in `specification/audit.md:202-204`** (area 13, item 1). The page prints `kept the model for TARGET; ...`. The command prints `kept the model for the question; ...` (`crates/thinkthen/src/cli/audit/write.rs:119`), and `tests/backend/audit_model.rs` pins it.
4. **The re-bill on a grown entity set in `specification/relate.md`** (area 15, item 1). Line 97 says a rerun under `--cache` sends only the questions no earlier run answered, and names the entity state as part of each question key. Line 54 says every request carries the state of all entities. So adding one entity changes every key, and the whole run is billed again. The page never says so. Item 4 of `2026-09-25-recognize-and-relate-scale-and-shape.md` asks for the same sentence.
5. **Keyless `ollama` in `specification/backends.md:57`** (area 17, item 1). The page says that at any base other than its default, `ollama` reads `OLLAMA_API_KEY` and a missing key exits 4. The code sends with no key and no `Authorization` header to any loopback host, and `crates/thinkthen/tests/backend/named_backends/ollama.rs` runs a keyless `ollama` against a loopback listener on a random port and expects exit 0. Say "at any loopback address", and keep exit 4 for other addresses.
6. **The surfaces in `CHANGELOG.md:5`** (area 10, item 5). The 0.1.0 entry names the command, six libraries and three extensions. It names none of the eleven C-door language packages or Polars, while ruling 10 says 0.1 ships every surface.

## Done when

Each sentence matches the code, and any sentence a test can pin is pinned by one.
