# 0626: Delete result/1, compatibility projections and legacy options

Status: OPEN.

Milestone: 0.2.1

Phase: 1

Reviews: revision 184b415, accept

## Outcome

- Result/2 is the only result shape. The result/1 definitions are gone from the generated schemas (`specification/result.schema.json`, `complete.schema.json`), from the serializers and from both spec pages.
- `legacy_annotation` and the paths behind it are deleted.
- Every remaining compatibility or legacy path in `crates/thinkthen/src` is either deleted or renamed to say what it does now.
- `specification/rust-api-0.2.md` and the frozen public declarations list only what remains.

### Proof

- On the M5 through pm job, the installed `thinkthen` binary replays each command's demo with `--details` (the `spec/result.md` replay). Every row checks against the result/2 schema, and no result/1 row is produced.
- The installed Rust library consumer builds and runs against the reduced public API.
- Every library gate on main still passes, including R, whose `tt_files` helper the conformance README describes as compatibility.

## Evidence

The 2026-10-12 ruling (`sdlc/decisions/2026-10-12-no-compatibility-before-1-0.md`) says no compatibility code before 1.0. The core still keeps old shapes:

- `crates/thinkthen/src/public/results/complete_schema.rs:4` keeps "Result/1 and the released generic interface ... compatibility definitions".
- `specification/result.md` still describes retained result/1 compatibility projections (23 mentions), and `spec/result.md` holds "retained result/1 compatibility examples to the legacy table".
- `crates/thinkthen/src/public/options.rs:120-124,181` keeps a `legacy_annotation` switch.
- `crates/thinkthen/src/public/plan.rs:97` and `crates/thinkthen/src/public/results/aggregate_reading.rs:57` describe compatibility diagnostics and a compatibility default.
- About 77 compatibility or legacy mentions remain in non-test core source. Some may be ordinary words; that needs checking.
