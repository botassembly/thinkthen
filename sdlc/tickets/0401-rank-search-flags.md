# 0401: Shared search intake and record display, with multiquestion rank

Status: landed. All four slices landed through `b39774cce`. SQL question sets and foreign question sets remain later in tickets 0417 and 0418.

Landed: fb428165487488a9decbaf08c2269a64c2f57442

This identifies the accepted slice D candidate that the coordinator publishes through the landing merge. The merge carries the Ticket, Slice and Review trailers.

Milestone: 0.2

Lane: claude-0 for slice D, assigned after audit 0405 and landed C. Each slice lands on main and ships in 0.2. The documentation story and DuckDB version build follow.

Depends on: 0405

Implementation also waits for amended slice acceptance and main at 0.2.0.

Closes: 2026-10-03-search-features-positions-several-questions-and-grep-style-output
Closes: 2026-10-03-rank-details-prints-value-null

It answers item 3 of the help-gaps issue and leaves its other items open.

## Outcome

Seven record functions read named files and fixed text-line windows: decide, filter, rank, choose, score, tag and annotate. Filter, rank and find show locations, neighbors and scores through explicit output flags. Filter remains one question, one cut and passing input records in input order. No grep command is added. Rank alone gains the separately reviewed multiquestion turns merge and matching Rust API.

```sh
thinkthen rank @questions.json --window 10 --top 8 -n --around 5 --scores transcripts/*.txt
```

[The first audit answer](../records/0405-audit-report.md) explains source boundaries and cost. The following rules settle the previously open input/output details. The coordinator may overturn them after review.

## Intake contract

- Repeat existing `--input FILE` on all seven functions, in argument order. One occurrence behaves exactly as before. With none, use stdin. `-` is a literal filename. Support positional files on decide/filter/rank/annotate where no label list exists; mixing positional files with --input refuses. Choose/tag/score keep positional labels untouched and use repeated --input. Never infer label intent from filesystem existence. Find's existing one-file input is retained; shared intake does not expand its candidate contract.
- Validate and open every named source before any send. Missing paths, directories and read failures preserve the existing safe error classifications. Retain global pipeline labels separately from file-local positions. Each file restarts line numbering at one. A file never shares a window with the next file.
- Without --window, decide/choose/tag/score retain whole-document framing by default. Each named file is a separate item. An explicit multi-file document run prints one JSONL carrier per file: `{"input_file":FILE,"value":VALUE}`. A single file or stdin retains its existing scalar bytes. `--details` on several documents includes `input_file` beside the result's existing members. Reject --raw/--quiet for a multi-file document invocation before sending. Explicit record modes retain their existing carriers; --details supplies structured positions. An explicit multi-file document invocation is a run, not one scalar document call. A completed run exits 0 regardless of individual true, false or unresolved/null values; each carrier preserves its own value. Old single-file and stdin answer exits 0/1/3 stay unchanged. A later input or backend failure preserves already completed output and returns the actual error exit; it must not convert that failure into the last answer's exit. An empty document remains a refusal, including an empty second file, rather than an omitted item or a successful null answer. This requires a distinct run mode at completion: the existing single-document stop handling must not suppress a later document failure.
- Filter/rank keep default lines, or JSONL when the resolved pointer requires it. Explicit --lines/--jsonl/--csv/--tsv and saved on/--field retain existing evidence selection. Annotate retains its record carrier and saved group on. Multiple explicit table files each have their own required header; do not send a later header as data. Positions for CSV/TSV remain deferred.
- `--window N` accepts an ASCII whole number of at least one and explicitly chooses text-line windows for default-document value functions. It requires no pointer. Refuse JSONL/CSV/TSV, --field, or any resolved saved on before sending. Annotate admits it only when the set has no on pointers. The joined window is one text item, preserving internal physical line feeds. Strip only the last record-ending line feed according to existing line rules. A final short window is one item. Empty files make zero windows and zero sends. Preserve blank physical lines inside a nonblank window. An all-whitespace window is skipped exactly as an ordinary blank line; its physical lines still advance positions. Validate size before admitting the window. Existing MAX_RECORD_BYTES bounds each joined item.
- New intake remains at the CLI edge. It changes no question digest, request key or provider wire format when unused. Batching/splits use the same items and identities.
- Preserve original operating-system paths for file access. JSON `position.file` and `input_file` use lossy human display strings, replacing invalid UTF-8 bytes with the replacement character. A valid filename must not turn a completed answer into a serialization defect.

## Display contract

- `-n`/`--line-number` prints a selected record's first physical line with `:`. Print filename when several input files are named; one file/stdin omits it. `--scores` prints filter yes probability, rank's ordering value, or find's winning candidate probability before its record. These meanings are explicit; they are not comparable model calibration.
- `--around N` accepts ASCII whole number including zero. It shows N physical lines before and after each selected record. It never sends neighbor lines, adds candidates or changes selection. Each group starts with `--`; a score can appear on that group line. With line numbers, neighbors use `-` and selected records use `:`. Clamp at file edges. Groups never merge. Overlap repeats lines inside each independently ordered group. Rank groups follow ranking order; filter groups follow selected input order. None prints no fabricated group in find.
- Read and retain a bounded memory snapshot when --around is present. The combined original bytes across named sources are capped at MAX_RECORD_BYTES (16 MiB) for this new view; validate before sending. No implicit scratch/spool file is written. Filter still emits completed selected groups in input order and retains its backend-failure prefix; rank keeps stdout empty on failure. Find retains its existing aggregate count/byte bounds. New metadata follows the snapshot rather than rereading a mutable file later.
- Refuse textual -n/--scores/--around beside --details; structured JSON stays valid. --details carries optional `position: {file,first,last}` on line/document results, with file null for stdin. Do not prefix a JSON details object. Refuse text display flags for CSV/TSV; structured table position is deferred. Default output stays byte-for-byte unchanged except the additive optional details position member.
- Neighbor display context is distinct from provider `--context` and never enters question identity. Filter accepts no set, top or sorting option.

## Multiquestion rank contract

A saved set holds decide members with distinct names, no member cut or on. Ask each member independently over every record. Merge its stable ranked lists by turns, deduplicating each original input record at its first appearance. Preserve member order and input tie order. `--top` cuts the merged order. A one-member set has the same ordinary output and request/cache identity as the plain question. Its details intentionally differ: each set details row adds question_name, including a one-member set, while retaining the existing single-question digest and other matching answer fields. Plain-question details do not gain question_name merely to force equality. Never merge by comparing probabilities between different questions. Existing single saved score rank remains supported; score members inside a set are deferred.

Expose an additive Rust rank_set API with input index, probability and question name, using pure core/order.rs. Existing Engine::rank/Ranked and every binding/SQL shape stay unchanged. 0406 separately owns the already-shipped single-question score/criteria parity gap. [0417](0417-sql-rank-question-sets.md) owns SQL parity in 0.2 after D. [0418](0418-binding-rank-question-sets.md) owns C/language follow-ups later unless a reviewed slice is small.

## Evidence

- Starts from: the 0.1 `rank` command and library, three measurements, and one issue.
  - The library already returns positions. `Ranked<T>` in `crates/thinkthen/src/public/results/ranked.rs` carries `index()` and `probability()`, and `Engine::rank_with` in `crates/thinkthen/src/public/bulk.rs` fills them. The C door's `RankedRow` writes `{"index","record","probability"}`, and `libraries/BINDING-AUTHOR.md` makes bindings preserve semantic input positions and probabilities under their documented host mappings; R uses one-based place. The Python binding (`libraries/python/thinkthen/__init__.py`, `rank`) and the TypeScript binding (`libraries/typescript/index.js`, `rank`) both map `index` back to the caller's record. Semantic input-position mapping already exists on these bindings; field spelling and index base remain host-specific. The library takes a list, and an index is a position in it. A file and line exist only on the command line.
  - The command does not use the library's `rank`. `judge::rank` in `crates/thinkthen/src/cli/judge.rs` runs the shared record pipeline, and `Output::ordered` in `crates/thinkthen/src/cli/schedule.rs` holds each `Judged` row and sorts with `core::ranking` when the input ends. Each row's printed line is rendered in `Judging::row_of` (`crates/thinkthen/src/cli/asking/row.rs`) before the order is known.
  - The command already knows each record's line. `edge::numbered` in `crates/thinkthen/src/cli/edge.rs` numbers every physical line from 1, blank lines included, and skips blank records. `Held.at` in `crates/thinkthen/src/cli/asking/judged.rs` carries that number. Nothing carries it to the printed row. `Held.at` also supplies ordered pipeline labels and request spans; file-local positions must not replace those global identities.
  - A trailing argument is refused today. `RankArguments.extra` in `crates/thinkthen/src/cli/args.rs` catches it, and `Command::stray` makes `cli::run` refuse it with `hint::ONE_QUESTION`. `crates/thinkthen/tests/backend/hints.rs::a_second_argument_says_where_the_evidence_goes` pins that sentence for `rank`.
  - Several questions per record already pack. `Planner::plan` in `judged.rs` passes a list of questions to `quoted_plan_of`. `pack::asks` in `crates/thinkthen/src/core/pack.rs` makes one `Ask` per question, keyed by `QuestionKey::of(url, model, state, question)`. `annotate` uses this path.
  - Experiment 422 measured search approaches against 15 quoted lines in three talk transcripts, 5,829 caption lines, in 36 live runs for $0.134. One broad question on 10-line windows found 5 of 15 in 300 lines read. Four narrow questions merged by turns found 11 of 15 in 300 lines and 13 in 600. Merging by highest probability found 9 of 15, because one question's best window scored 0.93 and another's scored 0.31. 10-line windows beat 5, 20 and 40. Half-step overlapping windows cost twice as much and added at most one quote. Five neighbor lines on each side of the top 8 windows per question raised recall from 12 of 15 in 310 lines to 14 of 15 in 570. A cut at 0.3 on any question found 11 of 15 in 420 lines. Keeping caption times in every line cost 56% more tokens and found one fewer quote. One question that names all four ideas found 9 of 15 in 300 lines for a quarter of the cost.
  - Experiment 0010 in `botassembly/thinkthen-exp` (`experiments/0010-thinkthen-navigate`) reproduced experiment 422's search step for step: 14 of 15 at 600 lines. On 50 QASPER papers, flat ranking of every paragraph answered 25 of 50 questions within 40 paragraphs, and a walk that picked documents first answered 19. Its answer 5 says navigate is a how-to over these features and not a command. Its answer 6 says all six features sit under the prototype: several questions by turns is the biggest recall lever, then neighbors, then 10-line windows, and line numbers and file arguments make the manifest possible.
  - Experiment 434 ranked 10-line windows of 8 cancer treatment papers for 32 labeled questions. It reached every answer range for 19 questions at 50 lines read and 29 at 200, where grep on question words reached 22 and 26. Paragraph windows reached 14 and 29. Adding 5 neighbor lines on each side cut full recall to 9 to 12 at every budget. So neighbors stay off by default.
  - Experiment 433 ranked 50 article rows for each of 112 TREC Precision Medicine topics with one question. It scored below the search tool's own order at P@10 (0.314 against 0.379). The topics name a disease, a gene and a treatment, and its owner asked for several questions per run.
  - `crates/thinkthen/src/cli/asking/judged.rs:290–307` treats a stop after a completed single-document row as its answer exit. That retained scalar behavior cannot represent a multi-file document run: a later failure must remain an error. The new run mode needs its own completion/failure branch without weakening single-document behavior.
- Keeps: default whole-text/record framing, inline labels, single --input bytes, pointer disclosure, batching, cache identities, request bytes, cuts and errors. Filter preserves one question/one cut and input subsequence. Rank preserves stable ties/top/empty-on-failure, value null on ordinary details and saved score ordering. Find preserves the whole candidate set and none semantics. Existing host result shapes stay unchanged.
- Changes: four separately reviewed slices. A implements shared intake/framing/positions for the seven functions. B implements filter/rank display and bounded memory snapshots. C implements find display through its aggregate adapter. D implements pure turns and the additive Rust rank_set API. B and C require A's shared position type, without forcing find into the per-record pipeline. D follows A and uses the same positions; it is independently useful without neighbor display.
- Proof: seven-command edge tables for two files, repeated --input, whole documents versus lines, carriers, labels that look like existing files, blank/final-short windows, per-file headers, resolved on and invalid framing; count zero sends for all preflight refusals. Retain original keeping/graded_rank/default_framing/permutation tests. Pin locations through batches/splits, complete selected bytes, exact scores/group overlap, failure prefixes, memory byte-limit boundary and no unapproved writes. Find covers duplicate candidates, bounds, none and unchanged request bytes under display flags. Turns uses independently declared lists and cross-question probability counterexamples, cache replay from individual member runs and zero-send refusals. For multi-file default-document runs, pin two-file no and not-sure/null answer carriers with completed exit 0 and count the expected replies; retain single-file no/unsure exits 1/3. Pin an empty second document and a failed second backend reply: keep the first completed carrier, emit no fabricated second answer, and return the actual existing error exit. Distinguish open-time preflight failures, which send nothing, from later document-content/backend failures. For a one-member set, compare exact ordinary output, wire request bytes and recording/cache keys with the plain question, then separately assert the details difference is additive question_name and that its digest and answer fields match. Replay the individual member recording under the set with zero counted sends. Run focused checks, policy/lint and coordinator-named spec checkpoint, with a fresh reviewer for each slice.
- Defers: paragraph/overlap windows, strip expressions, asymmetric neighbors, merged display groups, score members in question sets, CSV/TSV positions, rank threshold, foreign multiquestion APIs under 0418 and new provider semantics. SQL question-set parity is not deferred beyond 0.2; 0417 owns it. 0402 owns recipes; existing help issue retains its other items. No disk spooling or implicit scratch files.

## What Ian can overturn

Repeated --input and unambiguous positional routes; multi-file JSON carrier; all-whitespace-window skip; 16 MiB bound for around snapshots; independent overlap groups; score meaning and text/details conflicts; turns and its deduplication rule; additive position member. Changes need contract review before code.

## Public declarations

### Added public declarations

```text
struct RankSet
fn RankSet::from_json(&str) -> Result<RankSet, Error>
fn RankSet::load(impl AsRef<Path>) -> Result<RankSet, Error>
struct SetRanked<T>
fn SetRanked::index(&self) -> usize
fn SetRanked::input(&self) -> &T
fn SetRanked::into_input(self) -> T
fn SetRanked::probability(&self) -> f64
fn SetRanked::question_name(&self) -> &str
fn Engine::rank_set<I>(&self, &RankSet, I) -> Result<Call<Vec<SetRanked<I::Item>>>, Error> where I: IntoIterator, I::Item: Evidence
fn Engine::rank_set_with<I>(&self, &RankSet, I, CallOptions<'_>) -> Result<Call<Vec<SetRanked<I::Item>>>, Error> where I: IntoIterator, I::Item: Evidence
```
