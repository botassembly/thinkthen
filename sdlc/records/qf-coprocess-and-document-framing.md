# Quick Fix: name coprocess limits and annotate document framing

Status: accepted by fresh independent review; ready for coordinator landing. Branch: `ticket/qf-coprocess-and-document-framing`. Based on `origin/main` at `5de14a00`. Scope: experiment 284 register 55 and 70. No runtime source changed.

## Findings and source evidence

- Register 55 remains a README defect. `README.md` under “What it is not for” advised record mode through a `coproc` without naming a suitable verb. `crates/thinkthen/src/cli/asking.rs::Judging::row_of` returns `printed: None` for a `filter` result that does not pass, including under `--details`. `crates/thinkthen/src/cli/schedule.rs::Output::take` prints only `Some(line)` and holds `rank` rows until `ended`. `crates/thinkthen/src/cli/edge.rs::write_line` flushes each written line. `crates/thinkthen/src/cli/asking/batched.rs::Former::next` sends an open batch after a 50 ms input pause, while `--batch 1` closes one record per request. The loopback observation is in local experiment 284's `55-filter-cannot-serve-coprocess.md`. Output ordering does not cause the missing `filter` reply.
- Register 70 remains an annotate-page defect. `crates/thinkthen/src/cli/annotate.rs::Judging::record` calls `Reading::annotation_record` for document bytes. In `crates/thinkthen/src/core/records.rs::Reading::annotation_record`, document mode with no selected field parses valid JSON into a JSON record, falls back to text only on JSON syntax failure, and refuses other JSON errors. The test `crates/thinkthen/tests/backend/record_values.rs::one_document_annotation_keeps_every_established_shape` checks text, a number, an array, and an object. The existing `specification/annotate.md` “What it reads” paragraph omitted that rule. The shared `specification/records.md` already said framing never comes from a filename; it needed the annotate-only content exception beside that sentence.

## Change and retained behavior

The README now recommends `decide --lines --batch 1` for a line-at-a-time request-and-reply coprocess loop and states why `filter` and `rank` do not serve it. The annotate page states its exact one-document JSON rule, names the explicit record flags, and says that `--lines` splits a multi-line document and no flag forces a JSON-looking whole document to text. Its examples now name which framing rule they use. The shared records page links to the annotate exception. The commands, output, record flags, batch planner, and existing tests retain their behavior. This Quick Fix adds no runtime test for prose that describes established behavior.

## Checks and closure recommendation

The builder checked the changed pages, the cited source and existing regression test, `sdlc/scripts/pages`, `sdlc/scripts/tickets`, and `git diff --check`. The changed files are not executable demo pages with a 900-word limit. No provider, broad gate, or paid call ran.

A fresh independent reviewer returned ACCEPT for exact candidate `4ac6f43755febf60c5bed2160d7db3e076b8e46d`, with no blocking findings. The review traced the flush, dropped filter output, held rank order, annotate JSON detection, and available framing choices in source. Its abbreviated-command note was optional; product prose stays as reviewed. A later fetch found `origin/main` still at the candidate's base `5de14a00`, so merging main added no commit and changed no reviewed prose. After this record update, `sdlc/scripts/pages`, `sdlc/scripts/tickets`, and `git diff --check` passed again.

Recommend closing register 55 and 70 when the coordinator lands the accepted candidate. Preserve the shared records page's independent 0213 edits when merging second. The work plan and register remain for the coordinator to update.

## What the build taught us

A line-at-a-time coprocess needs both a request boundary and an observable reply. The 50 ms pause can close an open batch, but it cannot make a dropped `filter` row print or make `rank` release an incomplete order. Document framing also has two separate choices: record flags select stream structure, while only default `annotate` document mode detects valid JSON content. A one-line text flag cannot stand in for an exact multi-line document mode.
