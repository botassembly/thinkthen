# 0284 — SQLite settings and keyed many (T2)

Status: Complete candidate for fresh High code review. The public 0289 API is merged and SQLite plan/cap integration has selected source-matched and unpacked-archive proof; root owns landing and issue closure.

## Outcome

Settings text third on every judgment call; the keyed `_many` table functions
with the **connection-scoped LRU slot map** (eight slots by argument digest,
`key =` value excluded from the key, per-slot memory stated, freed at
disconnect, length-then-bytes comparison before any copy, one parse per
distinct argument — measured ~16 ms per 100,000-pair object);
`thinkthen_relations` rename; `relate` on the shared `(query, rules)` form;
`thinkthen_configure(json)` replacing the twelve setters (same before-build
rule); `thinkthen_plan(question, keyed_json[, settings])` after 0289's planner;
removals with plain messages. Keep probability on decide/choose rows only and
return `(key, value)` for score/tag.

The eight-slot map Arc-shares rows and frees them at disconnect. Correlated/EXISTS proof is bounded to 1,000 rows; the 100,000-row join aggregates once. The distinct SQLite .locks filter package criterion remains owed.

Register `thinkthen_plan(question, keyed_json[, settings])` as **JSON text** using the same SQLite text/settings converter and core parser as `_many`, then call 0283's shared plan summary (and 0289's public Engine wrapper where the host reaches it). Preserve the keyed JSON argument order and SQLite's scalar-per-row daily call; this plan does not run a judgment. Invalid JSON, an invalid settings object, and question/settings duplicates are usage refusals before any send. P1 in the shared corpus pins one exact no-send count/body/byte/token-band result through SQLite's JSON-text return; a separate valid options or threshold conversion and one invalid object cover the host's converter without copying all V/I rows.

## Prerequisites and proposed files

Prerequisite: T1; T7 before final cap/plan acceptance. Proposed file families: `databases/sqlite/src/{ffi.rs,settings.rs,scalars.rs,tables.rs,worker.rs,recognize_document.rs} and narrow subordinate modules; databases/sqlite/tests/{test_settings.py,conformance.py}; databases/sqlite/{README.md,check.sh,ratchet.json,ratchet.py.json,ratchet.sql.json}`. Refresh exact nested helpers, package member inventories, nonblank source headroom and current Lanes claims before implementation. No source file is claimed by this preparation draft.

## Smallest meaningful proof

On the installed pinned SQLite host, P1 returns native JSON text decoding to `records=1`, `requests=1`, 120 bytes, the 61–109 token band and the independent exact first body while the listener accepts zero requests; invalid/duplicate input also sends zero. E1 sends one exact body. An aggregate-once 100,000-row packed join and two alternating packed calls prove reuse; correlated scalar and EXISTS shapes stop at 1,000 rows. Pin engine call count, EXPLAIN QUERY PLAN and byte-comparison path, plus a spent positive cap and exact removed-name refusal. Reconcile the separate `.locks` filter criterion without claiming the full package gate. [Shared proof routes](../records/2026-09-29-sql-frame-redesign-proofs.md) keep host qualification separate.

## Evidence

- Starts from: Main `869710193`, accepted ADR 0105, experiment 2038 HANDOFF and saved spikes, and the [preparation](../records/2026-09-29-sql-frame-redesign-preparation.md).
- Keeps: Existing successful values, owned facts, NULL/not-sure, cache identity, six error kinds, offline replay and privacy except the accepted changes.
- Changes: SQLite connection LRU keyed many calls, native JSON-text plan over the shared core summary, settings, configure and removals.
- Proof: P1 exact no-send JSON-text summary and invalid/duplicate refusal, E1 body, alternating-slot join and bounded repeated shapes against captured wire bodies.
- Defers: Unrelated package/release qualification, provider work, marketing site, token cap and per-record cache; named prerequisites remain.

## What the build taught us

The existing SQLite virtual-table callback copies every hidden argument into owned `Value`s before it calls the table implementation. A connection-owned row store therefore needs a borrowed `xFilter` path, not only a new table body. The pinned host proved that a visible `key =` equality can be passed into that path while the packed answer rows stay shared. The portable five-record fixture also remained a three-request call after replacing the warm aggregate with a keyed table.

The old settings corpus and package tests invoked setters, warm and positional deadlines; migrated selected consumers retain exact removed-name zero-send refusals. Landed 0289 provided the public `Engine::plan_with` and active process reservation, so P1 now returns native JSON text with the exact 120-byte body, 61–109 band and zero sends, and a positive total refuses a later packed send after one arrival. The host uses core's reservation rather than its previous parallel counter.

Fresh High early review caught three false passes in the first keyed implementation. The visible-key index omitted SQLite's residual comparison, breaking NULL, NOCASE and numeric TEXT-affinity equality; an ordinary-table oracle now proves all three while BINARY text keeps the fast candidate lookup. The keyed `serde_json::Value` reader flattened repeated and escaped-equivalent keys; a narrow duplicate-aware visitor refuses both with zero sends and keeps source order. The row slot bypassed a changed `@file`; its metadata stamp now invalidates reuse and a captured model change proves the second send. Preparation accounted for pointer/byte identity but missed SQL comparison and external-file freshness. The bounded 1,000-row correlated/EXISTS proof still passes; the aggregate-once 100,000-row result is retained from its one prior run without another performance run. A selected source-matched SQLite archive consumer passed; Actions and release-runner qualification remain separate. Details and measured growth are in the build record.

The final High review found that `thinkthen_configure` accepted zero for `max_requests_total` despite SQLite's retained 1+ rule. A host-only validation now refuses it before replacing held settings; a focused pinned-host case proves the exact sentence, `null` reset, surviving prior model and one later loopback send. Core's shared parser and other hosts retain their own rules. The 0299 integration check then found that its future core-valid `max_estimated_input_tokens_total` would reach SQLite's default Defect arm. A named SQLite Usage arm now refuses that unsupported key. Today's core rejects it before the arm; the bounded transactional fixture pins the current exact refusal, surviving configuration and one later send, while root's combined-source checkpoint must exercise the arm after 0299 lands.
