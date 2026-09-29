# 0278 Rust API inventory build

Candidate branch: `ticket/0278-reconcile-the-rust-api-inventory`, merged from main `d71160cf9` after High design ACCEPT at `0a642671`. Scope is `sdlc/scripts/inventory`, the accepted [ticket](../tickets/0278-reconcile-the-rust-api-inventory.md), and this record. The [preparation record](0278-rust-api-inventory-preparation.md) retains the original source audit unchanged.

## Change and red-to-green proof

The selected integration checkpoint failed the unchanged inventory with 36 missing old eager signatures and 103 uncontracted additions, while refusing its four mutation plants. The accepted ticket independently freezes all 36 retired and 103 added canonical declarations. `contract()` now composes that static text delta after 0084/0095/0147, ADR 0017 and the `choices!` export. Its small reader rejects missing or padded text, duplicates within either block, an entry in both blocks, the wrong 36/103 counts, a retired entry absent from history, and an added entry already in history. `built()` and its `--no-default-features` extraction, implicit-trait allowances, `NEVER_CLONE`, mandatory public `Debug` check, and all four original plants are unchanged.

The checker runs a literal four-entry witness on every invocation. It pins a tuple variant, a struct variant field, a `const fn`, and a nested `Result<Call<Vec<u8>>, Error>` return through both the new text reader and existing built-listing parser. Four malformed versions of the actual delta then prove duplicate, overlap, missing-retirement and already-declared-addition refusals. This catches a parser silently dropping one of the reviewed declaration forms; the existing four plants only catch changes to a complete real extraction. There is no test-only public hook. No old test was deleted or consolidated: each existing plant retains a distinct export, removal, signature or trait boundary.

## Checks

| Check | Result |
| --- | --- |
| `THINKTHEN_HEAVY_LOCK=/run/user/1000/thinkthen-codex-2.lock RUSTC_WRAPPER=/usr/bin/env CARGO_BUILD_RUSTC_WRAPPER=/usr/bin/env CARGO_NET_OFFLINE=true CARGO_BUILD_JOBS=2 flock -o /run/user/1000/thinkthen-codex-2.lock sdlc/scripts/inventory` | Pass: `inventory: 442 declared items checked, 4 plants refused`; output saved at `target/codex-builds/0278-inventory/check.log`. Its internal literal and malformed-delta witnesses also passed. The pinned nightly public-api extraction reused the warm lane and completed in about 1.4 seconds; the output did not report new crate compilation. |
| `CARGO_NET_OFFLINE=true RUSTC_WRAPPER=/usr/bin/env CARGO_BUILD_RUSTC_WRAPPER=/usr/bin/env python3 sdlc/scripts/policy.py` | Pass: 189 resolved packages; accepted tables, bans and dependencies match. |
| `sdlc/scripts/pages`; `sdlc/scripts/tickets` | Pass: 1 coming/23 green pages; 0 ticket evidence failures from 0120 onward. |
| `python3 -m py_compile sdlc/scripts/inventory`; `git diff --check` | Pass. |

No full lint/test/spec/surfaces, package, all-port, provider or held SQL/DataFrame validation ran. The result proves this exact no-default-features API set and checker behavior; it does not qualify feature-enabled exports or close the issue before code review and integration.

## What the build taught us

The canonical ticket lines can be read as strict text without changing the older Rust-style declaration parser or normalizing away `const`, variant payload or nested return spellings. The arithmetic from preparation held: 375 historical entries minus 36 plus 103 equals 442, and the real extraction has no policy-filtered difference. The old four plants remained meaningful, so the only added witness targets the new reader's distinct failure mode. No accepted source/API mismatch surfaced; the remaining work is fresh code review and coordinator integration, not a broader port build.

## Review and integration

Fresh High code review accepted `05dc98da`. It independently checked unchanged normative delta blocks, separate expected and extracted declarations, all four mutation refusals, parser edge cases, prohibited Clone/Copy and required Debug, and policy. The coordinator integrated the reviewed checker unchanged. The no-default-feature inventory is green; a full lint or feature-enabled inventory is not claimed.
