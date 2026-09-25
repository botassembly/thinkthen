# 0107 build: the TypeScript surface over the public API

Date: 2026-09-25. Branch `ticket/0107-port-typescript-surface`, from `62bbc52f`. Author: Claude. A fresh code review of `416652e0` returned findings, and the second commit answers them.

## What landed

- `libraries/typescript` is its own Cargo workspace, crate `thinkthen-typescript`, a napi-rs cdylib over `crates/thinkthen` by path with default features off. The lock has 77 packages, and `cargo deny --offline ... advisories bans licenses sources` passes on it.
- `src/door.rs` holds the one JSON door, the one panic guard, the deadline check, and the engine settings. `src/node.rs` holds the `#[napi]` items: `engine`, `usage`, `call`, and `CallHandle::detach`. Each call runs on a named `std::thread` with the default stack.
- `index.js`, `index.mjs`, `index.d.ts`, and a nine-line `loader.js` are written by hand. The generated loader, `@napi-rs/cli`, the stand-in connector, `synthetic-partial`, `DIVERGENCES.md`, and `lto = true` are gone.
- `README.md`, a new `NOTES.md`, `LICENSE`, `.gitignore`, `check.sh`, `build-addon.sh`, `setup-toolchain.sh`, `node.sha256`, and four ratchet files.
- `sdlc/surfaces.txt` marks `libraries/typescript` landed. `javascript.md` gains the built call shape and "Differences from Python". ADR 0047 gains a TypeScript section.

## Toolchain

Node 22.22.3 from `~/.cache/thinkthen-toolchains/node-v22.22.3-linux-x64`. The archive sum `2e5d13569282d016861fae7c8f935e741693c269101a5bebcf761a5376d1f99f` was checked against `node.sha256`. `typescript` 7.0.2 came from npm's cache with `npm ci --offline`. Nothing was downloaded.

## Check result

`env -u THINKTHEN_API_KEY flock -o /run/user/1000/thinkthen-heavy.lock sh libraries/typescript/check.sh 1` exits 0 and prints `typescript: pass`.

- `cargo fmt --check`, Clippy with `-D warnings`, and the three Rust unit tests run first, with no Node. Deny on the lock and the deny plant run next, then the toolchain check and the root builds. All pass.
- `node --test`: 23 tests, 23 pass.
- Conformance: 54 cases, 49 pass, 0 fail, 5 not run with reasons.
- The corruption loop, `tsc --strict`, the pack list, the license, the home path, and the flag, pin, guard, `unsafe`, and sentence steps pass.
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
| R3-25 and R2-27 worker | join the worker thread before `call` returns | the throttle test times out on the held arm, and the drift test reads 3,898 ms |
| R2-27 shapes | return a hand-built details object | the `details` equality with the command turns red |
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

The batch abort plant first stayed green. The child exited after the abort and took the batch with it. The test now keeps the child alive for 3 s with a timer, so a batch that keeps sending shows on the count. The drift test first stayed green under the join plant, because its clock started after the blocking call returned. The clock now starts before the call.

Not planted:

- R4-14. The pack check pins `LICENSE` in the list, and no plant can turn it red. npm packs a `LICENSE` file whether or not `files` names it.

## Findings

1. **Retracted: a cancelled batch does not keep sending.** The first build of this record reported that a cancelled batch sent the rest of its records after the release. That run tested a stale addon. The plant harness restored each file with an older modification time, so `build-addon.sh` kept the planted build in which `detach()` fires no token. On a fresh build, the probe reads 2, 4, and 8 sends before and after the release at throttles 2, 4, and 8, and the batch abort test passes three runs of three. The engine needs no fix for this. A plant harness must touch each restored file, or cargo keeps the planted build.
2. **Engine versus ticket: `maxRequests`.** A three-record `decide_many` under `maxRequests: 2` sends two records, then refuses. `EngineBuilder::max_requests` documents that streaming order. The ticket expects zero sent. The test pins 2.
3. **API gap: `find` with `none`.** The public `find` takes no `none` candidate, so `18-find-second` and `19-find-none` cannot run. R2-27's "19-find-none resolves null" is not shown.
4. **The throttle sentence.** The engine's sentence has no `thinkthen: ` prefix. The ticket quotes the command's spelling. The test pins the engine's text.
5. **`usage()` is synchronous.** The ticket says it resolves. Each `Engine` counts its own calls.
6. **Gate placement.** The ticket now follows the `libraries/rust` pattern. `check.sh` runs Clippy and deny, and `lint` reaches the binding through the policy check and `surfaces --registry`.
7. **The deny plant.** It uses a one-crate manifest with a `file://` git dependency under a scratch `CARGO_HOME`, in place of a copy of the binding workspace. The copy would change the lock and could reach the registry index. The scratch home leaves nothing under `~/.cargo`. cargo-deny exits 8 on a sources failure, and the step pins 8 and `source-not-allowed`.

## Budgets

Nonblank lines: production Rust 539 across three files, and `build.rs` 4. Package JavaScript and types 563: `index.js` 300, `index.d.ts` 233, `index.mjs` 22, and `loader.js` 8. Tests and helpers 760 across ten test files, and the runner `cases.mjs` 130. Scripts 125. Ratchets: Rust 543, `.js` 308, `.mjs` 743, `.ts` 272. Each equals its measured total.

## Left

- A fresh review of the second commit.

## Review answers (second commit)

- The `.mjs` and Rust ratchets equal their measured totals.
- `check.sh` runs fmt, Clippy, and the Rust unit tests before any Node or cargo-deny step. A missing cargo-deny skips only the deny steps, and the check then ends with exit 77.
- `engine()` runs through the one panic guard. `door::caught` holds the single `catch_unwind(`, and `guarded` uses it.
- A kind refusal names the kind with an explicit word. The usage table pins "choose does not take a score question".
- The ticket records the accepted departures: `maxRequests` sends 2, the throttle sentence has no prefix, `usage` is synchronous, R3-30 is a printed reason table, and `find` with `none` stays unproved under `sdlc/issues/2026-09-24-the-library-cannot-ask-find-none-or-per-question-parts.md`.
