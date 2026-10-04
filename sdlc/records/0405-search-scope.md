# 0405: Answer filter, rank and grep before search changes

Status: findings prepared for fresh review. Product implementation remains blocked on the audit's accepted scope.

Snapshot: landed `ff047d8c50ce39ed9ab0acd22695c4960a963d38`. The audit branch changes records only. Windows release safety and ticket 0401 are in flight and are not shipped behavior.

## Verdict

Keep filter as one question, one cut, passing records in input order. Keep rank as order by a yes probability or a saved score value. Add no grep command. Expand 0401 through separately reviewed intake and display slices before its builder writes product code. The PM's shared intake recommendation fits the code. Find needs its own display adapter because it judges one complete candidate set.

## Landed evidence

- [Common](../../crates/thinkthen/src/cli/args.rs) has one `input: Option<PathBuf>`, explicit framing and field pointers. Decide, filter and rank capture trailing arguments for a migration hint. Choose, tag and score consume trailing arguments as their label lists. Files cannot be inferred by looking for an existing path without changing valid labels.
- [asking::run](../../crates/thinkthen/src/cli/asking.rs) sends decide, choose, score, tag, filter and rank through one intake and question pipeline. [annotate::run](../../crates/thinkthen/src/cli/annotate.rs) has its own question-set runner but uses the same edge source, chunks and numbering. [edge](../../crates/thinkthen/src/cli/edge.rs) opens one named source or stdin, bounds records and numbers physical lines. Shared intake belongs here rather than in rank's ordering code.
- [judge::filter](../../crates/thinkthen/src/cli/judge.rs) refuses `--top`, resolves one decide question, streams output and uses one cut. [row_of](../../crates/thinkthen/src/cli/asking/row.rs) prints only passing records. [Output](../../crates/thinkthen/src/cli/schedule.rs) orders rank only after all judgments arrive and keeps input order for exact ties.
- [find](../../crates/thinkthen/src/cli/find.rs) reads the whole bounded set, asks once and maps a selected index back to the original unit. Its `Unit` has bytes, record and evidence but no location. It cannot share rank's per-record judgment pipeline. It admits 2–255 units, or 2–254 with none, and at most 16 MiB of original text.
- [hints tests](../../crates/thinkthen/tests/backend/hints.rs) refuse guessed `grep` and direct users to filter. [keeping tests](../../crates/thinkthen/tests/backend/keeping.rs) pin filter's input subsequence, original bytes, rank ties and partial-failure behavior. [graded rank](../../crates/thinkthen/tests/backend/keeping/graded_rank.rs) pins saved score ordering and original JSONL bytes.

## Scope proposal for 0401

The coordinator may overturn these details. Each slice needs fresh ticket review before implementation.

1. Shared file and window intake covers decide, filter, rank, choose, score, tag and annotate. Extend existing `--input` to repeat in argument order. An invocation with one `--input` keeps current behavior. Keep stdin when no input path is named. Positional files may be admitted for commands with no positional label list, but never infer file intent from existence or suffix. Repeated `--input` is the universal form. Do not add another spelling unless an actual parser limitation requires it.
2. Shared record display covers filter and rank with `-n`/`--line-number`, `--around` and `--scores`. Location remains separate from global record/request identity. Every file restarts its physical line at 1. One record carries its first and last physical line. Filter selects only the passing records. Neighbor lines belong to the display view and are never extra judgments or selected records. A details view carries structured positions; display prefixes do not turn a result document into invalid JSON.
3. Find gains the same display choices through its own adapter. Its score means the selected candidate probability, including a none candidate when present. Its surrounding lines never enlarge the candidate set or change request bytes. Its aggregate limits and one-request item stay intact. A none answer prints no fabricated record. Full candidate probabilities stay available in details.
4. Retain 0401's separately reviewed multiquestion rank and Rust merge slice. Filter accepts no question set. Multiquestion ordering remains in pure core. File intake and display never require that new ranking behavior.

## Framing and compatibility

Without new flags, decide, choose, score and tag still read a whole document. Filter and rank still default to text lines, or JSONL when a field pointer requires it. Explicit `--lines`, `--jsonl`, `--csv` and `--tsv` keep their meaning. Annotate keeps its existing document, line and object-record behavior. A saved `on` pointer still selects the same evidence; the full source record remains the result's input. A window must not silently bypass that disclosure boundary.

`--window N` means text-line windows only. It explicitly selects a line-based input adapter for the four default-document value functions. It never joins JSONL or table rows into malformed records. Refuse it beside JSONL, CSV, TSV, a field pointer or saved `on`, rather than guessing a text field. Annotate can use line windows only for a question set without `on`; its line results retain `input` plus named `value`. A final short window is one record; a window never crosses a file boundary. Preserve blank physical lines inside windows. Specify empty-file and all-blank-window behavior before code, using current empty/blank refusal contracts as the baseline.

Several named inputs without a window retain each command's chosen framing. The four default-document value functions judge each named file as its own document; they do not concatenate documents and change the item. Their multi-file output needs an explicit documented carrier, because printing several scalar document results loses the association with the file. This is a design gap to resolve in the intake slice. Reject new ambiguous combinations until that carrier is settled. Shared intake does not authorize changing saved question digests, request keys, default output or old one-file behavior.

`--around` requires an explicit bound on retained display bytes. Filter needs a bounded before-buffer and delayed after-lines while retaining its streamed failure prefix. Rank already holds selected output; surrounding content must use bounded owned input snapshots, not reread a file that could change during the call. Find already owns its bounded set. Overlapping neighbor groups need deterministic separation and duplication rules. A rendered neighbor is display context, never provider shared context.

## Code and compatibility cost

| Slice | Code affected | Cost and material risk | Required proof |
| --- | --- | --- | --- |
| Shared intake | args/Common, edge source/chunks, asking reading/judged, annotate intake | Moderate adapter work; positional label ambiguity, document versus record semantics, pointers, byte bounds and file-local positions | Seven-command edge table; two files; final short window; blank lines; explicit-mode and pointer refusals; old one-file/default bytes unchanged |
| Filter/rank display | Held row metadata, row_of, Output, result JSON | Moderate output work; bounded neighbor retention, rank sorting, partial failures and JSON compatibility | Input subsequence and one cut; ties; locations through batching/splits; count-only no-send replay; record bytes with flags absent |
| Find display | read_units/Unit/rendered and result carrier | Small independent adapter; off-by-one locations and none handling | Duplicate candidate identity; candidate bounds; none; selected probability; neighbors leave requests unchanged |
| Multiquestion rank | existing 0401 core ordering and Rust public plan | Separate larger behavior slice; question identities and cross-surface parity | Independent accepted merge rule; all original single-question and typed score behavior retained |

No line estimate is claimed before implementation. Reuse the shared adapters rather than clone the seven runners. The public core and provider request format need no change for intake or display.

## Additional offline proof

Existing debug test carriers executed the unchanged snapshot's tests. `keeping::`: 23 passed; `hints::`: 3 passed; `batching::context::`: 8 passed; counted decide recording replay under filter and rank: 1 passed; counted find cache/replay: 1 passed. Each returned exit 0. The exact commands and complete logs live in owned build output `target/audit0405/focused-tests.json` and its sibling files. This is focused reused-carrier evidence, not a new full gate or independent rebuild.

A disposable loopback probe recorded synthetic replies, changed decide/choose/tag/filter cuts, rank top and annotate member cuts, then replayed. Listener counts stayed respectively 1, 2, 3, 4, 5 and 8. Changed values were false, null, empty labels, no passing records, one ranked record and false named answers. Score and find replayed unchanged at counts 6 and 7. A relation cut changed edges to no edges at count 9. No second send occurred. Score and find have no free reading rule to change, so those rows prove replay only. Complete probe inputs and results stay in owned build output `target/audit0405/reapply.py` and `reapply-results.json`.

## Separate request

The mailroom addendum asks to move backend checking under `backends check` and reserve proxy command nouns. The coordinator owns its separate ticket or ruling. It does not enlarge this audit or authorize product edits here.
