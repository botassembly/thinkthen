# 0107 build: the TypeScript surface over the public API

Date: 2026-09-25. Branch `ticket/0107-port-typescript-surface`, from `62bbc52f`. Author: Claude. Code review has not run yet.

## What landed

- `libraries/typescript` is its own Cargo workspace, crate `thinkthen-typescript`, a napi-rs cdylib over `crates/thinkthen` by path with default features off. The lock has 77 packages, and `cargo deny --offline ... advisories bans licenses sources` passes on it.
- `src/door.rs` holds the one JSON door, the one panic guard, the deadline check, and the engine settings. `src/node.rs` holds the `#[napi]` items: `engine`, `usage`, `call`, and `CallHandle::detach`. Each call runs on a named `std::thread` with the default stack.
- `index.js`, `index.mjs`, `index.d.ts`, and a nine-line `loader.js` are written by hand. The generated loader, `@napi-rs/cli`, the stand-in connector, `synthetic-partial`, `DIVERGENCES.md`, and `lto = true` are gone.
- `README.md`, a new `NOTES.md`, `LICENSE`, `.gitignore`, `check.sh`, `build-addon.sh`, `setup-toolchain.sh`, `node.sha256`, and four ratchet files.
- `sdlc/surfaces.txt` marks `libraries/typescript` landed. `javascript.md` gains the built call shape and "Differences from Python". ADR 0047 gains a TypeScript section.

## Toolchain

Node 22.22.3 from `~/.cache/thinkthen-toolchains/node-v22.22.3-linux-x64`. The archive sum `2e5d13569282d016861fae7c8f935e741693c269101a5bebcf761a5376d1f99f` was checked against `node.sha256`. `typescript` 7.0.2 came from npm's cache with `npm ci --offline`. Nothing was downloaded.

## Check result

`flock -o /run/user/1000/thinkthen-heavy.lock sh libraries/typescript/check.sh 1` exits 1.

- Toolchain, root builds, `cargo fmt --check`, Clippy with `-D warnings`, deny, the deny plant, and the three Rust unit tests pass.
- `node --test`: 23 tests, 22 pass, 1 fails. The failing test is "an abort settles a held batch at once, and the batch sends no more". The count reads 36 where 4 is expected. The cause is in the engine (finding 1).
- Conformance: 54 cases, 49 pass, 0 fail, 5 not run with reasons.
- A copy of `check.sh` with the node suite removed ran the later steps, and each passed: the corruption loop, `tsc --strict`, the pack list, the license, the home path, and the flag, pin, guard, `unsafe`, and sentence steps.
- `sh sdlc/scripts/surfaces --registry` passes. `python3 sdlc/scripts/policy.py` passes.

Not run: the five cases `18-annotate-two-groups`, `18-find-second`, `19-find-none`, `25-defect-fault`, and `30-local-question-file`. The runner prints each reason.

## Plants

Each plant was applied, its check was run, and the file was restored. Every row below turned red and turned green again once the plant was removed.

| Row | Plant | Red result |
|---|---|---|
| R1-25 | drop `removeEventListener` in `invoke` | listener test: the count after 2,000 calls is not 0 |
| R3-20 | drop the bare-score refusal | the usage table reads another sentence |
| R5-8 | drop the `typeof` check | `Error` in place of `ThinkThenError` |
| lone surrogate | drop `isWellFormed` | the usage table turns red |
| single abort, plant one | drop the abort listener | the call resolves after the release |
| single abort, plant two | forget the threadsafe function in `detach()` | the child does not exit while the reply is held |
| batch abort | `detach()` fires no token | count 37, not 4 |
| UTF-16 | skip the conversion | case 41 fails |
| R3-30 | skip one case silently | 53 of 54 |
| R3-26 | the runner drops its failure assertion | the corruption step fails |
| R4-13 | remove the score overload | `tsc` TS2345 |
| R4-13 | write "-1 is refused" in `README.md` | the sentence step fails |
| R2-31 | add a second `catch_unwind(` | the guard count fails |
| unsafe | add `unsafe {}` | the `unsafe` step fails |
| R1-29 | drop `--locked` from `build-addon.sh` | the flag step fails |
| R1-29 | write `^7.0.2` | the pin step fails |
| R1-34 | remove `"license"` | the pack check fails |
| R1-30 and R3-31 | add `tests/` to `files` | the pack list fails |
| R1-30 and R3-31 | drop the remap | `thinkthen.node` names `$HOME` |
| R1-28 | add `dbg!` in `src` | Clippy fails |
| R1-10 | remove the guard | the panic test fails |
| R1-31 | map `Deadline` to `backend` locally | the kind table fails |
| R1-11 | drop the range check | the deadline table fails |
| R2-10 | treat any negative as none | the deadline table fails |
| settings seed | `Engine::builder()` in place of `from_env()` | folder A stays empty. The plant's sends went to the loopback listener only |
| maxRequests | drop the mapping | the count turns red |

The batch abort plant first stayed green. The child exited after the abort and took the batch with it. The test now keeps the child alive for 3 s with a timer, so a batch that keeps sending shows on the count. That change exposed finding 1.

Not planted:

- R4-14. Dropping `LICENSE` from `files` stays green, because npm always packs a `LICENSE` file. The pack check still pins `LICENSE` in the list.
- R2-27 shapes (the tag's hand-built details object). The `details` equality test against the command covers the row.
- R3-25 and R2-27 worker, the libuv plants (`AsyncTask`, and a synchronous door). Each needs a second door written only for the plant. The throttle and drift tests cover the rows.

## Findings

1. **Engine: a cancelled batch keeps sending.** The token fires, and the promise rejects within 100 ms. The held requests still sit at the throttle. Once they are released, the engine sends the remaining records. A probe on the held arm, with the child kept alive, read these counts one second after the release: throttle 2 over 200 records sent 38, throttle 4 over 20 records sent all 20, and throttle 8 over 200 records sent 138. ADR 0017's rule says no new request starts after a cancel. The binding passes the token through `CallOptions::cancel` and keeps pulling the batch. The engine's `Stream::pull` then fires the call's flag. The failing test is kept, because it pins the ticket's line.
2. **Engine versus ticket: `maxRequests`.** A three-record `decide_many` under `maxRequests: 2` sends two records, then refuses. `EngineBuilder::max_requests` documents that streaming order. The ticket expects zero sent. The test pins 2.
3. **API gap: `find` with `none`.** The public `find` takes no `none` candidate, so `18-find-second` and `19-find-none` cannot run. R2-27's "19-find-none resolves null" is not shown.
4. **The throttle sentence.** The engine's sentence has no `thinkthen: ` prefix. The ticket quotes the command's spelling. The test pins the engine's text.
5. **`usage()` is synchronous.** The ticket says it resolves. Each `Engine` counts its own calls.
6. **Gate changes not made.** The ticket adds Clippy, deny, and the deny plant to `lint`. The port brief forbids changes to ladder scripts. `check.sh` runs all three, and `surfaces --registry` already runs deny on the lock.
7. **The deny plant.** It uses a one-crate manifest with a `file://` git dependency under a scratch `CARGO_HOME`, in place of a copy of the binding workspace. The copy would change the lock and could reach the registry index. The scratch home leaves nothing under `~/.cargo`. cargo-deny exits 8 on a sources failure, and the step pins 8 and `source-not-allowed`.

## Budgets

Nonblank lines: production Rust 520 across three files, and `build.rs` 4. Package JavaScript and types 563: `index.js` 300, `index.d.ts` 233, `index.mjs` 22, and `loader.js` 8. Tests and helpers 758 across ten test files, and the runner `cases.mjs` 130. Scripts 116. Ratchets: Rust 524, `.js` 308, `.mjs` 740, `.ts` 272. Each equals its measured total.

## Left

- Code review by a fresh reviewer, including the new crates, the binding lock, deny's result, and the pack list.
- A decision on finding 1 before the surface rung can pass.
- The `lint` additions of finding 6, by the landing agent.
