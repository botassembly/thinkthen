# 0723: Command output tags and plans carry no version

Status: OPEN.

Milestone: 0.2.1

Phase: 2

## Outcome

- Changes: `conformance/cli_parity.py`, `crates/thinkthen/src/cli/annotate/error_row.rs`, `crates/thinkthen/src/cli/facts.rs`, `crates/thinkthen/src/cli/recognize/dry_run.rs`, `crates/thinkthen/src/cli/relate/dry_run.rs`, `crates/thinkthen/src/cli/status.rs`, `crates/thinkthen/tests/backend/annotate/record_failure.rs`, `crates/thinkthen/tests/backend/facts.rs`, `crates/thinkthen/tests/backend/facts/`, `crates/thinkthen/tests/backend/named_backends/command.rs`, `crates/thinkthen/tests/backend/recognize.rs`, `crates/thinkthen/tests/backend/relate/details.rs`, `crates/thinkthen/tests/backend/status.rs`, `crates/thinkthen/tests/windows/interrupt.rs`, `demos/16-triage-pipeline/review-jsonl`, `demos/16-triage-pipeline/self-test`, `demos/45-map-relationships/README.md`, `probes/annotate-batching/measure.py`, `probes/paid_evidence.py`, `probes/tag-score/measure.py`, `site/examples/reference/annotate/5-continue.out`, `site/examples/reference/annotate/5-continue.sh`, `site/examples/reference/status/1-json.out`, `site/examples/start/answer-cache/4-facts.out`, `site/src/components/AnnotateEdges.astro`, `site/src/data/flags.mjs`, `site/src/data/overhead.json`, `site/src/pages/start/configuration.astro`, `spec/recognize.md`, `spec/relate.md`, `specification/annotate.md`, `specification/channels.md`, `specification/fixtures/types/corpus.json`, `specification/question-file.md`, `specification/recognize.md`, `specification/recording.md`, `specification/records.md`, `specification/relate.md`, `specification/result.md`, `specification/settings.md`, `specification/types.md`.
- The command's other output tags keep their name and lose their number: `thinkthen.record-error`, `thinkthen.run` and `thinkthen.status`. Readers match the unnumbered tag.
- The `--plan` objects for recognize and relate drop their `schema` member, like the other verbs.
- The specification, site pages and examples, demos 16 and 45, and the probe measurement scripts change with them. `sdlc/scripts/spec` and `sdlc/scripts/demos` replay every changed page.

### Proof

- On the M5 through `pm job run --machine m5 0723 SHA --memory 8 -- cargo nextest run -p thinkthen facts status recognize relate record_failure`, the tests pin the unnumbered tags and plans without `schema`; they fail before the change on the numbered strings.
- On the M5 through pm job, `git grep -nE 'thinkthen\.(record-error|run|status|recognize-plan|relate-plan)/[0-9]' -- . ':!sdlc'` finds nothing, and `sdlc/scripts/spec` and `sdlc/scripts/demos` pass.

## Evidence

Ruling: [no version numbers before 1.0](../decisions/2026-10-19-no-version-numbers-before-1-0.md). Split from 0707, which keeps the result tag; these tags have their own writers and fixtures and land alone.

Starts from the writers: `crates/thinkthen/src/cli/annotate/error_row.rs` (`thinkthen.record-error/1`), `crates/thinkthen/src/cli/facts.rs` (`thinkthen.run/1`), `crates/thinkthen/src/cli/status.rs` (`thinkthen.status/2`), and the dry-run plans in `crates/thinkthen/src/cli/recognize/dry_run.rs` (`thinkthen.recognize-plan/2`) and `crates/thinkthen/src/cli/relate/dry_run.rs` (`thinkthen.relate-plan/1`). Readers: `conformance/cli_parity.py`, `probes/paid_evidence.py` and the probe `measure.py` scripts, and `demos/16-triage-pipeline/review-jsonl`.

Shares specification pages with 0707 and 0706; any order lands. Paths under `crates/thinkthen/src/cli/` move when 0639 lands; the claim follows the move.
