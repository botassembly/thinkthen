# Routine lint runs the full package validation

Status: Open. Confirmed on main `bd942f18` by source inspection.

`sdlc/scripts/lint` creates a stale source archive and invokes `sdlc/scripts/package`. That script runs every library-only test target, internal doctests, two private-surface builds, `cargo package`, library and command builds from a fresh unpacked source tree, and release panic-strategy builds. Its fresh temporary build directory discards the unpacked build cache each time.

The routine `test` rung now selects a bounded functional set, but ordinary lint still starts this separate broad campaign. This increases compilation and testing work for unrelated changes and makes the gate name misleading. No current timing or hardware-saturation claim has been measured for this finding.

## Outcome

Keep routine lint focused on formatting, static policy and bounded guard checks. Run complete package validation through an explicit, documented checkpoint used by the relevant release or packaging path. Preserve its stale-archive regression, library-only dependency graph, private export boundary, packaged transforms, doctests and panic-strategy guarantees. Move coverage deliberately; do not drop it or weaken the policy tables.

## Proof

Trace the selected commands with a bounded fixture or existing gate tests. Routine lint must not invoke the full package script or its hidden test/build campaign. The explicit package checkpoint must still own every retained guarantee and be reachable from the intended release process. Reuse prior package proof when inputs are unchanged; do not start a broad rebuild to prepare the ticket. SQL and DataFrame work remains held.

This follows Ian's instruction to keep ordinary checks small and reserve full validation for named batch checkpoints. The coordinator assigns design0275 before changing gate routing.
