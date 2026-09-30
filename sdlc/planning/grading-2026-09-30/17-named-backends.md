# Area 17: Named backends and addressing

Commit `58014776c`. Reviewer: Sonnet 5.5, fresh, read only. Rubric: `README.md` in this folder.

## Scope

Named backends pair an address, a key variable list and a model under a name. The tier walk picks one backend, the address rules vet the URL, and the key guard keeps a built-in key from another built-in's host.

All paths below are under `crates/thinkthen/src/` unless they start with `crates/`, `specification/` or `sdlc/`.

| Kind | Paths (nonblank lines) |
| --- | --- |
| Code | `core/backend.rs` (316, the address rules), `core/backend/named.rs` (287, tier walk and key guard), `cli/edge/backend.rs` (66), `cli/edge/key.rs` (67), `public/settings/backend.rs` (101), `config/backends.rs` (287, shared with area 18), the `Url` type in `core/text.rs`: 1,124 together without `Url` |
| Tests | About 45 tests. Unit: `core/backend/tests.rs` (9), `core/backend/named/tests.rs` (6), `config/backends.rs` (3). Integration: `crates/thinkthen/tests/backend/address.rs` (12), `named_backends/command.rs` (5), `builder.rs` (3), `rate.rs` (2), `precedence.rs` (1, a table over every pair of tiers), `crates/thinkthen/tests/library/key_address.rs` (4) |
| Contract | `specification/backends.md` "The key" (:34), "Named backends" (:45), "The address" (:89), "The model" (:109); `specification/settings.md` rows Address, Backend, Named backends, Backend key, Model; ADRs 0010, 0025, 0031, 0114, 0115; tickets 0334, 0339, 0343 |

## Complexity: 4 of 5

| Driver | Score | Measured fact |
| --- | ---: | --- |
| Code size | 3 | 1,124 nonblank lines across 6 files |
| States and concurrency | 2 | Sequential. A tier walk with four outcomes, and one shared key snapshot in a `OnceLock` (`cli/edge/key.rs:95-96`) read by worker threads |
| Rules and refusals | 4 | About 26. Address: scheme, user information, port, query or fragment, no host, clear text `http`, blank (`core/backend.rs:40-88`). Key: unset, control character, key in address. Name: invalid, unknown, built-in reuse. Guard, four tier rules, model order, keyless loopback, rate rules |
| Surfaces touched | 5 | Address and key rules sit in `core/backend.rs`, so all 22 surfaces pass them. Named selection reaches only the command and the Rust builder (`specification/settings.md:71-73`) |
| Settings | 4 | Seven rows: Address, Backend, Named backends, Backend key, Model, Requests a minute, Key |
| Contract weight | 4 | Four spec pages (`backends.md`, `settings.md`, `recording.md`, `channels.md`) plus six ADRs |
| Churn and debt | 4 | 7 commits on the named files in 7 days, all inside the ticket 0334 to 0343 window, with two follow-ups after landing (`e1d90f001`, `0136e1424`). One open issue names its drift: `sdlc/issues/2026-09-30-reference-page-exit-codes-and-key-rule-drift.md` |

Mean 3.7, rounded to 4.

## Grades

| Dimension | Grade | Evidence |
| --- | :---: | --- |
| Quality | C | The code follows ADR 0114 and 0115. One contract sentence is false: `backends.md:57` says `ollama` needs no key "at its default base" and "at any other base it reads `OLLAMA_API_KEY` ... a missing key exits 4". The code sends with no key at any loopback host (`engine/facade.rs:286`), and the test runs a keyless `ollama` against a loopback listener on a random port and expects exit 0 (`crates/thinkthen/tests/backend/named_backends/ollama.rs:279-293`). The published reference page still says an empty key is always exit 4 (open issue above). The address and guard match the contract: the host compares after lower case, percent decode and dropped trailing dots (`core/backend/named.rs:285-307`), and loopback is no other built-in's host (`:262-268`) |
| Reliability | B | Refusals print exact sentences and send nothing (`named_backends/command.rs:115`). A table pins the guard over trailing dots, escapes and case (`core/backend/named/tests.rs:155-250`). A second table walks every pair of tiers and counts which listener got which key (`named_backends/precedence.rs:209`, `builder.rs:254`). The code is under 3 days old and changed twice after landing, which caps the grade at B. The first review found two defects, the trailing-dot guard hole and a shared file naming any variable (`sdlc/tickets/0334-named-backends.md:33-34`), and both are fixed and tested |
| Maintainability | B | One owner for the choice (`named::choose`, `named.rs:154`) and one for the address (`core/backend.rs:246`). Files are far under the cap. No lint suppression. The CLI and the Rust builder each build the tier array and call the same function (`cli/edge/backend.rs:18-23`, `public/settings/backend.rs:76-84`). Three hand-written host readers differ in escape handling: `host_of` (`core/backend.rs:306`), `canonical_rest` (`:275`) and `comparable` (`named.rs:285`). `Choice::guard` resolves every built-in's base on each call (`named.rs:262-268`), which is cheap but repeated work |

## Strengths

- The guard runs before the key is read and before any cache or connection (`cli/edge/backend.rs:40-53`, and in the builder `public/settings/backend.rs:94-99`). An explicit `api_key` skips it by design.
- The key snapshot is read once and the same value goes to every request (`cli/edge/key.rs:99-116`). The key in the address is refused by exact byte comparison (`core/backend.rs:187-190`).
- Clear text `http` reaches three loopback spellings and nothing else, and the rule reads text only (`core/backend.rs:90-96`, `:264-266`).
- `Debug` withholds the base on `Named` and the captured tiers (`core/backend/named.rs:27-39`, `public/settings/backend.rs:29-43`).
- A tier that names anything decides, and lower tiers are ignored, in one loop with the choice returned as a value (`named.rs:154-181`).

## Cleanup

1. **Correct the keyless Ollama sentence.** Where: `specification/backends.md:57`. Why: it says a missing `OLLAMA_API_KEY` exits 4 at any other base, but any loopback host sends with no `Authorization` header. A user on `http://127.0.0.1:9000/v1` expects a refusal and gets a keyless send. Say "at any loopback address" and keep exit 4 for non-loopback `https`. The intended meaning is unconfirmed, because "base" could be read as host. Size: S. Blocks 0.1: yes.
2. **Land the reference page fix.** Where: `sdlc/issues/2026-09-30-reference-page-exit-codes-and-key-rule-drift.md` (site file `site/src/pages/reference.astro`, owned by marketing). Why: the public page states the wrong key rule, names `thinkthen.status/1` and omits exit codes 7, 130 and 143. A published page that states a false key rule must not ship at 0.1. Size: S. Blocks 0.1: yes.
3. **Give the other surfaces the `backend` setting.** Where: `specification/settings.md:71-73` (eight cells read "not on this surface; ADR 0114 build slice 2"). Why: named backends, the key guard and the Ollama form are reachable only from the command and the Rust builder. Python, TypeScript, Ruby, R, C, DuckDB, PostgreSQL and SQLite cannot name Ollama. Size: L. Blocks 0.1: no, because ADR 0114 records the slice. Ian may rule otherwise, since 0.1 waits for every surface.
4. **Keep one host reader.** Where: `core/backend.rs:275`, `:306`, `core/backend/named.rs:285`. Why: escape, case and dot rules live in three functions, so the guard and the loopback rule could drift apart. One `Host` value in `core/text.rs` would carry both forms. Size: M. Blocks 0.1: no.
5. **Resolve each built-in's host once.** Where: `core/backend/named.rs:255-277`. Why: the guard re-resolves all built-in bases per call. A `const` table of resolved hosts removes that. Size: S. Blocks 0.1: no.
6. **Pin the library's explicit-key skip.** Where: `public/settings/backend.rs:92-101`, `crates/thinkthen/tests/library/key_address.rs:49`. Why: the skip is deliberate and documented, but a test naming the exact case (explicit key plus wrong-host built-in) keeps it from widening. Check whether `builder.rs:306` already covers it. Size: S. Blocks 0.1: no.

## Confidence: medium

What was read: `core/backend.rs` (all but 20 lines), `core/backend/named.rs`, `cli/edge/backend.rs`, `cli/edge/key.rs`, `public/settings/backend.rs`, `config/backends.rs`, all of `backends.md` and the matching settings rows, ticket 0334's review notes, the named tests by name, and the keyless test and the guard table in full.

Not checked: no test was run. The `Url` type in `core/text.rs` was located but not read, so its check of the port and host forms is taken from `core/backend.rs`. `tests/backend/address.rs` and `core/backend/tests.rs` were counted and not read, so the claim that each address refusal has a test is inferred. ADR 0025 and ADR 0115 were consulted through the spec and the code comments only.
