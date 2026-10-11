# 0405: Audit the ten functions across every surface before 0.2 hardens

Status: COMPLETE.

Opened as: 2026-10-11. Complete audit accepted for landing at `33dc12e9a2d812055f99aac423010724756add08`. Read-only validation; no product behavior changes. Ian requested this audit on 2026-10-04. It precedes ticket 0401 implementation. Run it in physical lane claude-0 after the baseline package checkpoint.

Milestone: 0.2

## Outcome

Answer filter, rank and grep first against the landed code. Preserve filter as one question, one cut, passing records in input order. Assess named files and fixed windows on decide, filter, rank, choose, score, tag and annotate, and line numbers, neighbors and scores on filter, rank and find. Record the shared edge boundaries, retained behavior, affected code and proof, and the cost in code and compatibility risk. Amend 0401 or propose separately reviewed slices before any search implementation.

Produce one table with ten function rows and the command, Rust crate, C library, eighteen language libraries and three SQL extensions as columns. Every cell names supported input forms, label descriptions, shared context, reading rules, full option probabilities, record/replay/cache and backend choice. Attach source and existing conformance evidence to each assertion. Treat intentional edge adapters separately from missing equivalent capability. Include Polars and pandas variants in the evidence without counting them as separate languages.

Trace the five layers for every function: contract, item, wording, context, and model/reading. Verify independent changes and re-reading stored answers under changed reading rules without sending. Examine relation descriptions, typed score descriptions and question identities without assuming that metadata digests and backend cache keys are the same mechanism. Assess separate per-record context on every surface.

Each confirmed gap becomes a separate ticket or a named debt record. CLI, Rust and SQL equivalence gaps default to 0.2; other binding gaps default to later unless small. Retain existing tickets for the same outcome instead of filing duplicates. Record proposed fingerprint changes separately from the current compatibility contract; the PM note currently places that store work in the proxy.

## Evidence

- Starts from: Ian's 2026-10-04 steering and the mailroom request inbox/thinkthen/2026-10-04-docs-audit-the-ten-functions-before-0-2-hardens.md. Read the PM's fixed-and-tunable note and admitted contracts, implementations, schemas, bindings and conformance tests. Pin the audit to a named landed commit and distinguish in-flight 0380, 0398 and 0401 designs. The baseline checkpoint on e760432c80dc22501fa51694964f2eeec5b8e63e passed offline test, spec, all 21 canonical surfaces and release smoke; use it as prior evidence with its precise scope.

- Keeps: every request byte, cache identity, digest, result shape and refusal. Product source, specification and existing tests remain unchanged by this audit. It writes audit records and runs existing tests or disposable probes that create only owned build output.

- Changes: audit records and gap tickets only. No product code, specification or existing test changes.
- Proof: fresh ticket review accepts the scope before the audit. Preserve source links and exact command outcomes. Count loopback sends for no-send claims; use saved replies for every test. Mark each assertion as confirmed behavior, confirmed defect, design gap, inconsistency or unproved, and identify which existing conformance cases prove it. No broad green suite stands in for cell-specific evidence. Check every cell is filled and every gap has an owning ticket or debt line with milestone and proof. A fresh read-only reviewer accepts the complete findings and first search answer. Use the normal record, merge trailers and push process.

- Defers: implementation belongs to the resulting tickets. No live provider call, registry install, publication, external change or credential read is needed. Clinical and biological conclusions are outside this software audit. Full test, spec and surfaces require a named coordinator checkpoint; reuse prior receipts where their source and scope still apply.
