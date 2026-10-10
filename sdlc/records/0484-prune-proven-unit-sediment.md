# 0484: Separate release tests and remove proven duplicate assertions

The routine suite uses Nextest with its existing five-second limit. Large-input boundaries, nested package builds and complete installed parity belong to the release suite. Small invalid-input, secrecy, cancellation and caller-limit cases remain routine; release selection retains the distinct large boundary assertions.

The bounded cleanup removes source assertions only where retained public-library or compiled-CLI tests own the same inputs and stronger results. The final deletion removes four declarations: two transform catalog/name tests, ordered SystemOne choice criteria, and first-place leadership in an unresolved tie. Five existing CLI cases cover sorted duplicate-free names, exact shipped transforms and unknown-name refusals, criteria order, and the tied result with null value, threshold and exit three. No production code or helpers change.

Distinct threshold boundaries and defaults, parser failures, properties, serialization, digest compatibility, error cases, cancellation and memory boundaries remain. The audit did not establish that every source test is redundant, so those distinct tests stay.

## Evidence

Fresh review accepts 333b3150d362787540863499207ba9c53ae962a5. Both the developer and reviewer pass the five owning CLI cases using the existing executable. Formatting, strict workspace all-target Clippy, offline policy, exact ratchet and history/whitespace checks pass. Existing size warnings concern unchanged files.

The final deletion removes 39 physical lines, 34 nonblank lines and four literal test declarations. The exact native source ceiling decreases from 189310 to 189276. Earlier accepted deletions and release routing remain in Git. The completed routine run covers 1996 cases before these four deletions, one doctest, 21 external-consumer cases and the remaining script checks; all lint stages pass. Those unchanged results remain applicable. No full parity, large-input or load campaign was repeated during ticket closure.

## What the build taught us

Compare assertions and inputs before deleting a test. A public example does not replace a distinct parser, default, property or byte-ordering contract. Package-building smoke must be classified by its actual cost and placed in the release suite. Resume a failed gate at the affected stage instead of repeating unchanged passing work.
