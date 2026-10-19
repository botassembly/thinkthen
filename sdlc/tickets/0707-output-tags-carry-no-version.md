# 0707 — Output tags carry no version

Status: OPEN.

Milestone: 0.2.1

## Outcome

- Changes: `crates/thinkthen/src/core/result/`, `crates/thinkthen/src/core/measure/`, `crates/thinkthen/src/cli/`, `crates/thinkthen/transforms/`, `specification/`, `site/src/`, `site/examples/`, `conformance/`, `databases/`, `libraries/`, `build-tools/`.
- Output type tags keep their name and lose their number: `thinkthen.result`, `thinkthen.record-error`, `thinkthen.run`, `thinkthen.status`, `thinkthen.trials`. Readers match the unnumbered tag. Generated wrapper `Version` enums become a plain tag.
- The `--plan` objects for recognize and relate drop their `schema` member, like the other verbs.
- The `thinkthen.result/1` refusals in `audit` and `measure` are deleted, and every `result/1` fixture is converted. The `trials` transform reads current output (today it refuses everything `--details` prints).
- Version-history prose in the specification (for example in `recording.md` and `types.md`) is deleted.

### Proof

- `git grep -nE 'thinkthen\.(result|record-error|run|status|trials|recognize-plan|relate-plan)/[0-9]'` outside `sdlc/` finds nothing.
- A transform test pipes real `--details` output into `trials` and gets rows; it fails before the change.
- `sdlc/scripts/spec` and routine conformance pass on the M5.

## Evidence

Ruling: [no version numbers before 1.0](../decisions/2026-10-19-no-version-numbers-before-1-0.md). Each tag has one value today; the number guards nothing, while readers compare exact strings (`core/measure/answer.rs`, `measure/verbs.rs`, `transforms/compare.jq`). The survey found `transforms/trials.jq` refuses any row that is not `result/1`, so it cannot read current output; its tests pass only because they build `result/1` rows.
