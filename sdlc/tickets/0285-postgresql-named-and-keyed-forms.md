# 0285 — PostgreSQL named and keyed forms (T3)

Status: Done. The scoped PostgreSQL implementation passed fresh High code review at `576b5eaa1ede733b22c26c2aae75e0714660fd57` and lands with this record. The accepted preparation and independent installed correction proofs are recorded below. Broader package/release qualification and umbrella issue criteria remain open.

## Outcome

`settings json` directly after `input` (decide) or `members` (choose, score,
tag); defaulted named conveniences after it (pgrx 0.17 `define_string_guc_with_hooks`
for the warning; each argument's Rust identifier reaches the entity —
`probes/pgrx-argument-names`); keyed `_many` overloads replacing the array
overloads; `SET thinkthen.api_key` always succeeds with an interactive-only
WARNING; removals.

Use DEFAULT NULL for all optional named parameters. Bound the installed query with server SET statement_timeout='30s'; a shell timeout alone leaves server work running. With default logging the key warning is interactive-only; log_statement=all and pg_stat_statements can record the SET line.

Register `thinkthen_plan(question text, keyed_json jsonb[, settings json DEFAULT NULL])` returning **jsonb**. Convert the keyed input and optional settings through the same core parser and `call/settings.rs` merge rules as `_many`, then call 0283's shared summary through 0289's Engine wrapper. Keep the scalar named-parameter daily form and its `DEFAULT NULL` overload resolution. Invalid JSON/settings and a named/settings duplicate are usage refusals before send. P1 pins the exact no-send body/count/byte/token-band result through native `jsonb`; a small valid/invalid host conversion table proves PostgreSQL's own boundary.

## Prerequisites and proposed files

Prerequisite: T1; T7 before final cap/plan acceptance. Proposed file families: `databases/postgresql/src/{lib.rs,call.rs,call/settings.rs,context.rs,array.rs,warm.rs,find.rs,ffi.rs}; databases/postgresql/{README.md,check.sh,examples.json,ratchet.json} and exact GUC helper/tests after refresh`. Refresh exact nested helpers, package member inventories, nonblank source headroom and current Lanes claims before implementation. No source file is claimed by this preparation draft.

## Smallest meaningful proof

Installed PG P1 `jsonb` decodes to `records=1`, `requests=1`, 120 bytes, 61–109 token band and exact first body with zero listener accepts; invalid/duplicate input also sends zero. Retain named binding, duplicate/unknown refusal, E1 body, bounded server query and conditional warning/log proof. Record source and installed receipts separately; run affected functional cases and focused format/policy/ratchet/pages/tickets/diff. [Shared proof routes](../records/2026-09-29-sql-frame-redesign-proofs.md) name later qualification.

## Evidence

- Starts from: Main `869710193`, accepted ADR 0105, experiment 2038 HANDOFF and saved spikes, and the [preparation](../records/2026-09-29-sql-frame-redesign-preparation.md).
- Keeps: Existing successful values, owned facts, NULL/not-sure, cache identity, six error kinds, offline replay and privacy except the accepted changes.
- Changes: PostgreSQL DEFAULT NULL conveniences, keyed many, native jsonb plan over the shared core summary, interactive key warning and removals.
- Proof: P1 exact no-send jsonb summary and invalid/duplicate refusal, installed PG named binding, E1 body and bounded server/log proof.
- Defers: Unrelated package/release qualification, provider work, marketing site, token cap and per-record cache; named prerequisites remain.

## What the build taught us

The build used landed 0283/0289 APIs directly. PostgreSQL delivered SQL SET as `PGC_S_SESSION`; the hook now warns on a nonempty interactive/session assignment without echoing the value, and the next call refuses until reset as ADR0105 requires. A contextual scalar needed the public one-record many path, and a wrong-kind keyed question needed pre-send checking. The old array/warm internal tests gave way to installed keyed packing, cache, removal and cancellation receipts. P1, one exact E1 keyed body, 24 selected installed checks, six conformance IDs and six retained cancellation/failure checks passed on their source-pinned artifacts. Fresh review found that adding a native members array through `serde_json::Value` erased duplicate question fields before the core parser; a settings-parser proof did not cover that separate boundary. Raw question JSON now survives the native-array append, with an installed duplicate-name-before-deadline and zero-send witness plus a valid neighboring choice. Full host and release qualification remain distinct. See the [build record](../records/0285-postgresql-named-and-keyed-forms-build.md).
