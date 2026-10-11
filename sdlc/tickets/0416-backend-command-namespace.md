# 0416: Move backend checking under the backend namespace

Status: COMPLETE.

Opened as: 2026-10-11. `thinkthen backends check` is canonical, `thinkthen check` remains a hidden alias, and the proxy nouns are reserved.

Milestone: 0.2

## Outcome

Canonical CLI spelling is thinkthen backends check. Reserve questions, items, answers, checks, datasets, runs, setups, findings, people and search as top-level command names. Keep thinkthen check as a hidden compatibility alias for published 0.1.2 calls unless fresh review identifies a material conflict.

## Evidence

- Starts from: landed `ff047d8c50ce39ed9ab0acd22695c4960a963d38`, [current check contract](../../specification/check.md), [command parser](../../crates/thinkthen/src/cli/args/command.rs) and published release evidence in ticket 0128. Ask 3 in `inbox/thinkthen/2026-10-04-docs-audit-the-ten-functions-before-0-2-hardens.md`, addendum asks for backend namespace and free nouns. Published 0.1.2 uses check; coordinator supplies compatibility alias default. This is independent planning, not an audit finding.
- Keeps: check’s four request bodies, eight reported rows, key/address resolution, no-send plan, secrecy and exit behavior on both spellings.
- Changes: move backend checking under the backend namespace. Product work starts only after the contract and ticket receive fresh review.
- Proof: Canonical help/examples/spec match new spelling; alias executes identical requests/results, stays hidden in help, and preserves refusal/secrecy tests. Command inventory and parser plants enforce reserved nouns.
- Defers: No provider call or namespace for another binary is implemented here. A new ADR records command naming and compatibility before product code.

## What Ian can overturn

The additive API and slice order within this outcome. No external action or new cost follows from this planning record.
