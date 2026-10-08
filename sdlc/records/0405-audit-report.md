# 0405: Audit the ten functions before 0.2 hardens

Status: complete. Fresh independent review accepted `33dc12e9a2d812055f99aac423010724756add08`. Product source, specifications and existing tests remain unchanged. The first search answer received separate fresh ACCEPT at `e37b7150e278255adcd81ef25ba6985517b5cb22`. Resulting implementation tickets remain open.

## Verdict

The shared Rust engine supplies all ten functions. The public surfaces do not yet offer equivalent question tuning, context or full answer carriers. Most ordinary function behavior already has conformance coverage. That coverage does not establish every input/description/context/reading combination or a changed-rule no-send test on every host. The audit records those limits instead of calling every cell proved.

[The matrix](0405-surface-matrix.md) contains ten function rows and 28 explicit surface/variant columns. Its grouped dimensions name inputs, descriptions, context, reading rules, distributions, stores, backend selection, question files, per-record options/context, layers, rereading and available checks. Caller file and table parsing is an intentional edge adapter. Missing question capabilities and contradictory types are separate findings.

## Snapshot and evidence limits

Product pin: landed `ff047d8c50ce39ed9ab0acd22695c4960a963d38`. Product paths under crates, libraries, databases, conformance and specification match that pin. Windows work and search implementation are in flight and are excluded. The earlier `e760432c80dc22501fa51694964f2eeec5b8e63e` checkpoint passed offline test, specification, all 21 registry surfaces and 19 packed-file checks. The checkpoint then published 22 files plus their checksum and manifest. Its pushed checkpoint is `checkpoint/surfaces/2026-10-04-1`. The later landing changes release files; it does not invalidate unchanged engine/binding source evidence.

The canonical corpus has 55 cases. Routine surfaces select 32 IDs. Generic C JSON wrappers use a separate 29-case runtime/type corpus; they do not thereby execute all 55 canonical cases. JVM gate entries expand into Java, Kotlin and Scala; JavaScript runtime and TypeScript declarations are separate contracts. pandas, Python Polars, Rust Polars eager/lazy and Flutter are explicit variants. A cell's source-confirmed assertion is not a new execution receipt. Unsupported injections and unselected extended cases retain their exclusions. The C and SQL proof profiles explicitly separate corpus/selector membership from eligible executable IDs. A selected ID can still be excluded by its host runner.

| Runner | Omitted canonical cases | Source and reason |
| --- | --- | --- |
| C | 25-defect-fault; 18-annotate-two-groups | [SKIPPED list](../../libraries/c/tests/door/cases.rs): no public defect injection; legacy separate-group requests |
| SQLite | 25-defect-fault; 18-annotate-two-groups | [NOT_RUN and form](../../databases/sqlite/tests/conformance.py): no SQL invariant injection; legacy separate-group requests |
| DuckDB | 25-defect-fault; 18-annotate-two-groups | [NOT_RUN and reason](../../databases/duckdb/tools/conformance.py): separate private panic-boundary proof; legacy separate-group requests |
| PostgreSQL | 25-defect-fault; 18-annotate-two-groups; 23-cancelled-fault | [NOT_RUN list](../../databases/postgresql/tests/runner.py): separate panic probe, legacy separate-group requests and no SQL token for this cancellation injection |

These cases did not execute through those runners. A separate panic or host-cancellation test does not turn an omitted canonical case into a passing canonical case. The matrix’s function selectors are corpus membership; subtract these runner exclusions to obtain eligible cases.

No provider, credential, install, registry publication or experiment dispatch occurred. Probes used synthetic or committed public replies through counted loopback listeners. All new probes and logs remain in ignored owned `target/audit0405/`. Existing binary carriers were reused; no fresh full gate or independent rebuild is claimed.

## What the software does

ThinkThen turns a question and evidence into a typed answer. It preserves wire answers in recordings or cache, applies reading rules, and reports failures separately from valid unresolved answers. Host adapters supply input containers and rendering. A plain result can omit the alternatives that the underlying model returned; a full details/observation route is required to preserve them.

## Five layers

The contract and item remain the benchmark's fixed parts. Wording, context, model and reading can vary only where the current API exposes them.

| Function | Contract | Item | Wording | Context | Model and free reading |
| --- | --- | --- | --- | --- | --- |
| decide | yes/no | one evidence record | question and true/false criteria | shared text on eligible many calls; contextual singleton composition | model changes requests; cut/band rereads stored probability |
| choose | names or item-specific candidate rule | evidence plus candidate set when it varies | question, descriptions and option order | same eligible shared context | model changes requests; cut/ties read complete distribution |
| tag | label set | one evidence record | question, label descriptions/order | same eligible shared context | model changes requests; independent label cut rereads |
| score | level names and ordinal order | one evidence record | question and level descriptions | same eligible shared context | model changes requests; weighted value, external cuts only; level order is contract, not free wording |
| filter | yes/no | each record | question and criteria | same eligible shared context | model changes requests; one cut and input subsequence |
| rank | yes/no per record or CLI saved score | each independently judged record | CLI criteria/described score; other rank APIs plain text | one shared text | stable order/top can reread; saved score-rank gap outside CLI |
| find | member identity, including none when offered | complete candidate set | question | separate context refused | model changes requests; no free cut; adding none changes contract/plan |
| annotate | each named member's contract | one record with member/group projections | each member wording/descriptions | separate context refused | member models/rules stay separate; cuts reread; failed member distinct from null |
| recognize | kinds and span convention | immutable text | kind descriptions reach step two only; boundary wording fixed | separate context refused | strength/relation cuts reread available stages; lowering entity cut can require previously unasked relation stages |
| relate | relation names/endpoints, single/either semantics | complete entity set | textual reads independent of relation name | separate context refused | edge cut rereads; single/either alter questions and cannot be treated as free cut changes |

The PM note's typed-score and relation-wording premises are stale. [ScoreBuilder::level](../../crates/thinkthen/src/public/builders.rs) accepts `Option<Description>`, and [RelationRule::reads](../../crates/thinkthen/src/public/recognize.rs) supplies independent textual wording. [Relation file grammar](../../crates/thinkthen/src/core/recognize_file.rs) accepts reads. Rich relation description objects are absent; this is later design debt, not evidence that textual wording is impossible. Plain text relation reads has no rich-description semantics to preserve today.

Shared context differs from per-record context. [CallOptions](../../crates/thinkthen/src/public/options.rs) carries one string; [Decisions](../../crates/thinkthen/src/public/asking.rs) clones that one state for a batch. Eligible singletons can be composed as many calls with one item each. SQL scalar context expressions can vary by SQL row, as its adapter uses that singleton route. No integrated batch accepts record/context pairs. Combining retrieved passages into evidence changes item representation and is not a separate context channel.

## Stored answers and identities

[The metadata digest](../../crates/thinkthen/src/core/digest.rs) includes question verb, labels, wording, descriptions/order, cut and optional calibration profile. It is a run/question description. [QuestionKey](../../crates/thinkthen/src/core/pack.rs) hashes adapter, resolved address, model, state and wire question. The cut stays outside it. [Decided::judgment](../../crates/thinkthen/src/public/asking.rs) reads a stored wire answer with the current threshold. Changing a metadata digest does not imply a cache miss.

Separate benchmark contract/wording/setup fingerprints belong to the external store design described by the PM. No product fingerprint change is proposed for 0.2. Keep existing metadata and cache compatibility. Calibration profile and backend runtime limits are also distinct: a limit can refuse preparation before replay even when an answer exists.

CLI probes recorded answers and reread cuts for decide, choose, tag, filter, annotate and relate. Rank changed top without a send. Recognition changed strength cut and restored the entity without a send. Score/find replayed identically; neither has a free reading cut. Recognition with relations needs every newly eligible downstream question present in the store. [Recognition assembly](../../crates/thinkthen/src/engine/facade/recognize.rs) settles entities before building relation questions, so a lower strength cut can introduce cache misses. Strict replay then fails locally rather than making a paid call. General cache mode may send missing stages. Independent per-host changed-rule coverage remains unproved and has ticket 0411.

## Findings and ownership

| Finding | Classification | Consequence and smallest correction | Owner / milestone |
| --- | --- | --- | --- |
| Rank question parity | design gap, medium | Rust/C/SQL lose CLI saved decide criteria and score ordering; add reviewed single-question route/carrier | [0406](../tickets/0406-rank-question-equivalence.md), 0.2; foreign slices later |
| Independent batch record contexts | design gap, medium | One shared string cannot carry distinct retrieved contexts; add typed record/context input | [0407](../tickets/0407-per-record-context.md), 0.2; foreign slices later |
| Generic C JSON/SQL complete details | design gap, medium | Bare aggregate results omit rejected alternatives/stages; add full carrier while retaining bare outputs | [0408](../tickets/0408-j1-full-probabilities.md), 0.2; foreign slices later |
| TypeScript rank/find types | inconsistency, medium | Declaration admits question keys runtime rejects; align declarations with reviewed API | [0409](../tickets/0409-typescript-rank-find-contract.md), 0.2 small fix |
| Dataframe function coverage | design gap, medium | Rust Polars lacks five named functions; Python column routes refuse four functions | [0410](../tickets/0410-column-function-equivalence.md), Rust 0.2, other new variants later; retain existing pandas issue |
| Complete host no-send rule proof | unproved, medium | Ordinary calls and replay tests do not prove their combined path on every host | [0411](../tickets/0411-binding-reading-replay-proof.md), later |
| R public index guidance | inconsistency, low | One-based place is intentional but contradicts blanket author guidance; document adapter | [0412](../tickets/0412-r-result-contract.md), later |
| Integrated per-record choice options | design gap, medium | CLI supports options pointer; fixed many APIs capture one candidate set | [0413](../tickets/0413-options-per-record-equivalence.md), 0.2; foreign slices later |
| Aggregate separate context | design gap, medium | annotate/find/recognize/relate explicitly refuse a separate channel; settle semantics before adding | [0414](../tickets/0414-separate-shared-context.md), 0.2; foreign slices later |
| Planned SQL question-set rank | design gap, medium | 0401D plans CLI/Rust only; use its shared merge through additive SQL route | [0417](../tickets/0417-sql-rank-question-sets.md), 0.2 after 0401D |
| Planned binding question-set rank | design gap, medium | Current host rank takes one question; add member identity and turns without breaking existing rows | [0418](../tickets/0418-binding-rank-question-sets.md), later |
| Named backend setting outside Rust | design gap, medium | Environment/config works, explicit named setting absent | Existing [0377](../tickets/0377-binding-backends.md), 0.2 |
| CLI score inline description | design gap, low | Saved maps and Rust builder work; CLI lacks --level | Existing [score-level idea](../issues/2026-10-01-score-levels-described-on-the-command-line.md), 0.2 |
| pandas Series and DuckDB kind descriptions | design gap, medium | Column convenience and kind-description parity missing | Existing [two-gap issue](../issues/closed/2026-10-03-pandas-series-and-duckdb-recognize-descriptions.md), 0.2 |
| Named SQL filter | confirmed host composition; naming idea | SQL WHERE already filters; naming alone adds no semantics | Existing [SQL naming idea](../issues/2026-10-01-sql-names-for-rank-and-filter.md), 0.2 decision retained |
| Rich relation description objects | design gap, low | Text reads works; object forms would need their own rendering contract | Debt 0405-D1, milestone later; proof requires name/read compatibility and structured wire rules |
| Whole saved-result standalone rereader | design gap, low | Host replay uses wire stores; bare result is insufficient for all stages | Debt 0405-D2, milestone later; prove offline complete distributions/identities before adding API |

The broader [run-facts remainder issue](../issues/2026-09-26-every-surface-should-give-back-run-facts.md) keeps its unresolved cross-host probability/identity proof. Ticket 0408 owns the confirmed missing C/SQL carrier and references that evidence instead of claiming it was already solved. The [annotation options issue](../issues/2026-09-23-annotate-options-from-a-file-or-a-record.md) keeps its separate dynamic tag/set scope.

Generic C JSON convenience differences do not remove a typed C route where one exists. The matrix names that distinction. Likewise SQL full scalar details can be composed even when its keyed many convenience table exposes only selected values. Complete aggregate carrier equivalence remains missing; the finding is not a claim that every probability is absent from every C/SQL API.

## Verification

Focused reused CLI carrier: 23 keeping cases, three hints cases, eight shared-context cases, one counted cross-function replay case and one counted find cache case all passed. Public carrier: ten contracts cases, three details cases and one changed-context replay case passed. Fresh first-answer reviewer independently reproduced all 36 CLI and 14 public cases and the counted probes. The full-audit reviewer additionally confirmed 280 unique valid cells, an empty product diff, three public details tests and one changed-context replay test. Review required explicit C/SQL corpus execution exclusions and removal of stale 0403 signing wording; both record corrections received fresh ACCEPT at `33dc12e9a2d812055f99aac423010724756add08`.

Disposable replay listener counts remained 1 (decide), 2 (choose), 3 (tag), 4 (filter), 5 (rank), 6 (score), 7 (find), 8 (annotate) and 9 (relate). Recognition stayed at two across cut increase/restoration. The outputs and exit codes appear in `target/audit0405/reapply-results.json` and `recognize-reapply-results.json`. Source traces establish the relation-stage limitation; no broader no-send inference is made.

Builder verification passed: 280 unique cells, 28 surfaces and ten functions; every referenced value and source/runner path exists. Ticket validation reports zero failures; changed-file link findings are zero; git diff --check passes. New-text private-name and home-path counts are zero. New text is checked for public naming without printing forbidden names. The fresh reviewer verified all 280 cells, all 40 corrected C/SQL execution profiles, source references and planning owners. Unproved per-host paths retain their unproved classification.

## Search and independent planning

The accepted first search answer below expands shared intake and output while retaining filter purity. Ticket 0401 now contains the concrete amended intake/display contracts. Its four slices subsequently landed; the audit snapshot remains historical. Bounded neighbor snapshots mean bounded memory. No implicit scratch/spool files are authorized.

Planning [0415](../tickets/0415-signed-duckdb-community-listing.md) owns the separately approved signed DuckDB listing after 0.2 core. [0416](../tickets/0416-backend-command-namespace.md) owns the independent backend namespace addendum. Neither is counted as an audit gap or authorizes external submission or spend. The coordinator answers the mailroom asks.

## What the build taught us

Source tracing avoided filing two stale PM premises as defects. A common engine supplies behavior but does not make every host question grammar, details carrier or type declaration equivalent. A green aggregate gate proves its selected cases and exclusions. Replay identity must be traced separately from question metadata. Recognition stage dependencies make a blanket changed-cut cache promise inaccurate. Keep these distinctions in future tickets and release claims.

## Historical search decision

Keep filter as one question, one cut, passing records in input order. Keep rank as order by a yes probability or a saved score value. Add no grep command. 0401 subsequently expanded shared intake and display. The PM's shared intake recommendation fits the code. Find needs its own display adapter because it judges one complete candidate set.

## Landed evidence

- [Common](../../crates/thinkthen/src/cli/args.rs) has one `input: Option<PathBuf>`, explicit framing and field pointers. Decide, filter and rank capture trailing arguments for a migration hint. Choose, tag and score consume trailing arguments as their label lists. Files cannot be inferred by looking for an existing path without changing valid labels.
- [asking::run](../../crates/thinkthen/src/cli/asking.rs) sends decide, choose, score, tag, filter and rank through one intake and question pipeline. [annotate::run](../../crates/thinkthen/src/cli/annotate.rs) has its own question-set runner but uses the same edge source, chunks and numbering. [edge](../../crates/thinkthen/src/cli/edge.rs) opens one named source or stdin, bounds records and numbers physical lines. Shared intake belongs here rather than in rank's ordering code.
- [judge::filter](../../crates/thinkthen/src/cli/judge.rs) refuses `--top`, resolves one decide question, streams output and uses one cut. [row_of](../../crates/thinkthen/src/cli/asking/row.rs) prints only passing records. [Output](../../crates/thinkthen/src/cli/schedule.rs) orders rank only after all judgments arrive and keeps input order for exact ties.
- [find](../../crates/thinkthen/src/cli/find.rs) reads the whole bounded set, asks once and maps a selected index back to the original unit. Its `Unit` has bytes, record and evidence but no location. It cannot share rank's per-record judgment pipeline. It admits 2–255 units, or 2–254 with none, and at most 16 MiB of original text.
- [hints tests](../../crates/thinkthen/tests/backend/hints.rs) refuse guessed `grep` and direct users to filter. [keeping tests](../../crates/thinkthen/tests/backend/keeping.rs) pin filter's input subsequence, original bytes, rank ties and partial-failure behavior. [graded rank](../../crates/thinkthen/tests/backend/keeping/graded_rank.rs) pins saved score ordering and original JSONL bytes.

## Search slice scope at the snapshot

The coordinator may overturn these details. These were proposed slice boundaries at the audited snapshot; 0401 subsequently implemented them.

1. Shared file and window intake covers decide, filter, rank, choose, score, tag and annotate. Extend existing `--input` to repeat in argument order. An invocation with one `--input` keeps current behavior. Keep stdin when no input path is named. Positional files may be admitted for commands with no positional label list, but never infer file intent from existence or suffix. Repeated `--input` is the universal form. Do not add another spelling unless an actual parser limitation requires it.
2. Shared record display covers filter and rank with `-n`/`--line-number`, `--around` and `--scores`. Location remains separate from global record/request identity. Every file restarts its physical line at 1. One record carries its first and last physical line. Filter selects only the passing records. Neighbor lines belong to the display view and are never extra judgments or selected records. A details view carries structured positions; display prefixes do not turn a result document into invalid JSON.
3. Find gains the same display choices through its own adapter. Its score means the selected candidate probability, including a none candidate when present. Its surrounding lines never enlarge the candidate set or change request bytes. Its aggregate limits and one-request item stay intact. A none answer prints no fabricated record. Full candidate probabilities stay available in details.
4. Retain 0401's separately reviewed multiquestion rank and Rust merge slice. Filter accepts no question set. Multiquestion ordering remains in pure core. File intake and display never require that new ranking behavior.

## Framing and compatibility

Without new flags, decide, choose, score and tag still read a whole document. Filter and rank still default to text lines, or JSONL when a field pointer requires it. Explicit `--lines`, `--jsonl`, `--csv` and `--tsv` keep their meaning. Annotate keeps its existing document, line and object-record behavior. A saved `on` pointer still selects the same evidence; the full source record remains the result's input. A window must not silently bypass that disclosure boundary.

`--window N` means text-line windows only. It explicitly selects a line-based input adapter for the four default-document value functions. It never joins JSONL or table rows into malformed records. Refuse it beside JSONL, CSV, TSV, a field pointer or saved `on`, rather than guessing a text field. Annotate can use line windows only for a question set without `on`; its line results retain `input` plus named `value`. A final short window is one record; a window never crosses a file boundary. Preserve blank physical lines inside windows. Specify empty-file and all-blank-window behavior before code, using current empty/blank refusal contracts as the baseline.

Several named inputs without a window retain each command's chosen framing. The four default-document value functions judge each named file as its own document; they do not concatenate documents and change the item. Their multi-file output needs an explicit documented carrier, because printing several scalar document results loses the association with the file. This is a design gap to resolve in the intake slice. Reject new ambiguous combinations until that carrier is settled. Shared intake does not authorize changing saved question digests, request keys, default output or old one-file behavior.

`--around` requires an explicit bound on retained display bytes. Filter needs a bounded before-buffer and delayed after-lines while retaining its streamed failure prefix. Rank already holds selected output; surrounding content must use bounded owned input snapshots, not reread a file that could change during the call. Bounded snapshots mean bounded memory. No implicit scratch or spool writes follow from this proposal. Find already owns its bounded set. Overlapping neighbor groups need deterministic separation and duplication rules. A rendered neighbor is display context, never provider shared context.

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
