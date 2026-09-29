# Standalone relate boundary: corrected Quick Fix handoff

Source pin: `origin/main` `982bf442e`; the earlier parked preparation at `8769cda1` was read, then corrected before use. The [scale/shape issue, item 6](../issues/2026-09-25-recognize-and-relate-scale-and-shape.md) asks for count, pair arithmetic, remedy, and 254/255/256 evidence across every surface. This Quick Fix takes only the unheld command diagnostic and its small boundary proof. Item 6 and the umbrella stay open for the other hosts; SQL and DataFrame work remains held.

## Correction from fresh review

The parked note proposed carrying the complete actual input count in core `EntitySetError::TooMany`. That is wrong for `public::Engine::relate_with`: it collects only `MOST_ENTITIES + 1` (256) before `RelateSpec::admit`, so a longer caller input is deliberately unread. A core count would claim 256 even when more entities exist. Keep core `TooMany`, its unit/display, public `Error::Usage`, and the bounded public iterator unchanged.

`cli/relate/input.rs` instead builds a complete `pairs` vector for document, line, JSONL and table inputs before `admit`. It alone can capture `pairs.len()` when `admit` returns `TooMany`; other entity errors keep the existing mapping. `cli/failure/relate.rs` renders a CLI-only exit-2 message. The hypothetical all-kind unordered candidate count uses widened `u128` arithmetic, avoiding `usize` multiplication overflow; it never claims the selected rule's actual logical-question count or a blocking feature. The command page distinguishes this diagnostic from unchanged host errors.

## Reused proof and scope

The existing 256-line live refusal row in `tests/backend/refusals/relate.rs` is driven by `refusals.rs`, which checks exit 2, empty stdout, secrecy and zero loopback requests. The row can pin the complete new sentence but its shared harness checks containment; an exact-output assertion in the already claimed `tests/backend/relate/ceiling.rs` closes that remaining gap without a second listener. The same ceiling file already has a distinct 255 all-kind full-plan proof (64,770 directed logical questions); a small cross-kind 254/255 dry-plan pair can pin entity counts and one logical question using its existing entity helper. No public regression is needed because core/public paths are untouched.

Claimed files are the two CLI files, `specification/relate.md`, those two existing test files, this corrected preparation, the Quick Fix build record and measured shared ratchet. No shared refusal harness, core, public API, other host, site, SQL or DataFrame file is part of this slice. The root owns later host routing and issue disposition.
