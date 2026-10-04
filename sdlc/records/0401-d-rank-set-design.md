# 0401D design: rank question sets and the Rust search API

Status: accepted design, adopted for implementation on landed C baseline `fea3a6fbc46b68ccc33a28e35eb0c4eb48329df9`.

Original frozen design SHA256: `ace1ddd3deec8ef962920c06804f1b66d55826cdb66d4e68e504241cfb014a87`. Fresh independent design acceptance settled the turns rule and additive API without blocking findings. The preparation baselines below are historical. The builder uses actual landed C main; SQL 0417 and foreign 0418 remain separate. Ian can overturn the accepted choices. No API deviation is adopted here.

## Scope and dependency order

0401D adds multiple named decide questions to existing `rank`, with matching Rust crate support. It adds no eleventh function, command, merge option, or general search framework.

The implementation follows accepted C landing. Original preparation used main `57de974e8` and accepted unlanded C source `a6953c6ff2a292d179bfd31bfde988588355e24d`. Current main is `4cf694673`; C was rebased without product changes, and its optional CLI result contract was corrected at `d97d912ea`. C remains unlanded pending full specification and canonical documentation proof. These later baseline updates change no proposed API choice.

Ticket 0399 landed through `4cf694673` after completed proof and fresh acceptance. D must preserve its backend-selection behavior and verify against the resulting main after C lands.

Release order:

1. Land accepted 0401C and finish its proof.
2. Review this D design independently.
3. Implement and review D against the resulting main.
4. Implement SQL question-set parity separately under 0417 for **0.2**.
5. Implement C and language follow-ups under 0418 later, unless separately reviewed as small.

0417 supersedes the older Rust-only release default. D does not implement SQL or foreign bindings.

## Available evidence

The design follows repository `AGENTS.md`, workspace instructions, the applicable `pm` skill, `README.md`, `specification/README.md`, `sdlc/planning/rust-standards.md`, the cleanup plan, and these owning records:

- `sdlc/tickets/0401-rank-search-flags.md`
- `sdlc/tickets/0405-audit-ten-functions.md`
- `sdlc/records/0405-search-scope.md`
- `sdlc/records/0405-audit-report.md`
- `sdlc/tickets/0417-sql-rank-question-sets.md`
- `sdlc/tickets/0418-binding-rank-question-sets.md`

Source establishes the following:

| Evidence | Consequence for D |
| --- | --- |
| `crates/thinkthen/src/cli/asked.rs` resolves rank as one saved decide or score question with `Cutting::NoRule` | Set dispatch must be additive and preserve the existing single-question branch |
| `crates/thinkthen/src/core/question_set.rs` preserves member order but supplies decide’s default cut and normalizes absent `on` to root | Ordinary annotate parsing cannot directly enforce rank’s absence-of-cut-and-`on` contract |
| `crates/thinkthen/src/cli/asking/judged.rs` builds quoted plans and runs `Engine::ask_all` | Set members should use the same planner, packer, transport, cancellation, and storage path |
| `crates/thinkthen/src/cli/asking/row.rs` retains an ordering value separately from printed output | Rank-set display should use the selecting member’s probability without recalculating it |
| `crates/thinkthen/src/cli/schedule.rs` retains bounded top rows, stable ties, and delayed output | Set rank needs member-local retention and delayed turns emission |
| `crates/thinkthen/src/core/order.rs` supplies pure stable ranking | Add the shared turns rule here |
| `crates/thinkthen/src/public/bulk.rs` prevalidates finite rank inputs and returns `Call<Vec<Ranked<T>>>` | Preserve this behavior on existing rank; mirror it on the additive route |
| `crates/thinkthen/src/public/results/ranked.rs` exposes zero-based index, original item, and yes probability | Extend through a separate result type |
| `crates/thinkthen/src/public/bulk/annotation.rs` reduces answers to named values and permits partial member failure | Do not implement rank sets by collecting public annotate results: that loses ordering probabilities and has different failure semantics |
| `crates/thinkthen/src/core/pack.rs` keys each wire question independently | Set names and turns order must remain outside request identity |

0405 accepted pure filter, shared edge intake/display, distinct metadata and cache identities, and explicit parity ownership. Its earlier execution receipts are historical evidence with their recorded scope. **No tests were executed for this design.** All witnesses below are proposed verification, not passing results.

## Accepted behavior to retain

Filter remains one decide question, one cut, and the passing input subsequence. It accepts no question set or sorting behavior.

Ordinary rank retains:

- Default lines, or JSONL when a resolved pointer requires it.
- Shared intake, repeated named sources, windows, file-local positions, and global pipeline identities.
- Stable descending yes probability, with exact ties in input order.
- Saved single score ordering by weighted level value.
- Top selection after every record is judged.
- Empty stdout on an input, member, transport, replay, or cancellation failure before successful completion.
- Existing ordinary output bytes and detailed result members.

Find remains C’s aggregate operation with its complete candidate set and none semantics. D changes no find behavior.

Existing `Engine::rank`, `rank_with`, `Ranked`, `RankedRow`, C carriers, SQL rows, and language shapes remain unchanged. Single-question criteria/score parity belongs to 0406.

## Question-set admission

Use the existing saved set shape:

```json
{
  "version": 1,
  "questions": {
    "billing": {
      "decide": "Does this explain the billing problem?",
      "true": "It explains the problem.",
      "false": "It does not."
    },
    "recovery": {
      "decide": "Does this explain how to recover?"
    }
  }
}
```

Preserve the existing ordered JSON parser, duplicate detection, name grammar, nested question grammar, structured wording, and true/false meanings. Names use lowercase letters, digits, and underscores and must be distinct.

Rank accepts only decide members. Refuse:

- Empty sets.
- Every non-decide member, including score.
- Top-level or member `threshold`, including an explicit default cut.
- Every explicitly authored member `on`, including `""` and `[""]`.
- Nested model, profile, or batch settings already prohibited by set grammar.
- Invalid version, names, duplicate keys, unknown keys, malformed JSON, and invalid member values.

Allow existing top-level `batch` and `profile`. Batch follows current eligible record-call precedence. Profile retains its existing calibration-name meaning; runtime profile limits remain separate.

### Smallest typed representation

Introduce a public `RankSet` wrapper over a private rank-resolved set. Provide only:

```rust
RankSet::from_json(text)
RankSet::load(path)
```

Do not add a builder, conversion from arbitrary `QuestionSet`, or another file format in D.

Reuse the existing parser through a narrow rank-specific entry point. Check forbidden authored keys before normalization, and resolve decide members under `Cutting::NoRule`. Keep ordinary `QuestionSet::parse`, annotate defaults, canonicalization, equality, and pointer behavior unchanged.

This avoids adding provenance fields throughout annotate merely to distinguish omitted settings from explicit defaults. `RankSet` represents the admitted subset of the same saved question-set grammar.

The existing capped loader remains authoritative: file loads admit at most 1 MiB. In-memory parsing retains existing JSON limits without inventing a file-size limit.

### CLI dispatch and overrides

`rank @FILE` reads the file once through the capped reader. Dispatch on its parsed top-level `questions` key; never retry invalid single-question parsing as set parsing. Plain text, including text that resembles JSON, remains literal question text unless introduced by `@`.

For an admitted set:

- `--model`, backend selection, shared `--context`, batching, framing, and display controls apply to the run.
- `--field` selects common evidence for every member. It does not introduce member-specific pointers.
- Refuse `--true` and `--false`: the set owns each member’s meanings.
- Preserve rank’s existing refusal of threshold, raw, quiet, and invalid top.
- Window and display admission follow A/B/C, including pointer and details conflicts.

Set grammar failures exit 5. New command-level conflicts exit 2. Rust in-memory grammar failures use `Error::Usage`; `load` follows existing public loader behavior and reports `Error::Local`.

Proposed new command sentence:

```text
`rank` with a question set takes no --true or --false; put meanings in each member
```

Rank-specific set refusals name the offending member/key using screened existing error conventions. Pin their complete sentences during implementation; do not echo member wording, evidence, or rejected values.

## Ordering and retained probabilities

Ask every member over every admitted input item. “Independently” means each member has its own logical question and answer; it does not require a separate HTTP request.

Retain each member’s validated yes probability. Do not reduce it to true/false, use confidence, or compare it with another member’s probability.

Each member list sorts by descending yes probability and preserves input order on exact ties. Deduplicate by original input item identity. Equal text at two input positions remains two items; repeated source arguments remain separate source occurrences.

### Precise turns rule proposed for review

The ticket fixes turns and first-appearance deduplication but does not specify whether a duplicate consumes a turn. Adopt this bounded rule:

1. Visit rank depth zero across members in saved order.
2. Visit depth one across members, then depth two, and so on.
3. Emit an item only at its first appearance.
4. A duplicate consumes that visit; do not advance that member again within the same turn.
5. Apply top to the emitted sequence.
6. Carry the name and probability from the member that first emitted the item.

For example:

| Member | Independently declared order |
| --- | --- |
| first | `[0, 1, 2]` |
| second | `[0, 2, 1]` |

The merged order is `[0, 1, 2]`; selecting members are `[first, first, second]`. A duplicate-refill interpretation would produce `[0, 2, 1]`, so this example must be a contract witness.

This interpretation is an **overturnable design choice**, not a claimed source-established rule. Settle it in fresh design review before implementation.

Implement the pure merge in `core/order.rs`, returning item and member indexes. The merge should consume already ranked lists and contain no file, environment, clock, transport, serialization, or probability calibration behavior.

### Top retention

Without top, retain all member lists until successful completion.

With top K, retain at most K candidates per member. A candidate below K in a member’s list already has K distinct items ahead of it in that list, so it cannot enter the first K distinct emitted items under this rule.

The retained bound is therefore at most M×K member candidates, plus existing bounded in-flight work. It is not a byte bound and does not reduce judgments or requests. Preserve this distinction in the specification. Do not allocate a full M×N probability matrix when top is present.

## Result contract

CLI ordinary output prints each selected original record once. Default set output adds no question label.

`--scores` prints the selecting member’s yes probability. These displayed numbers may rise across the merged order; that is expected because different questions have different probability scales.

Existing location and around rendering use the original item’s position and immutable snapshot. Groups follow merged order; overlap repeats, and groups never merge. Neighbors remain outside provider evidence.

Set details print the selecting member’s existing rank detail with one additive top-level field:

```json
"question_name": "billing"
```

Retain:

- `question.verb: decide`
- `answer.kind: yes_no`
- `threshold: null`
- `value: null`
- Selecting member’s yes probability under `answer`
- Selecting member’s single-question digest
- Whole original input and available position
- Existing request, usage, cache, profile, and attempt metadata semantics

Do not substitute a set digest for `meta.question_sha256`, attach other members’ answers to the winning detail, or sum their receipts into it.

A one-member set has identical ordinary output, wire question, request/cache identity, and matching answer/digest fields to the corresponding ordinary rank question. Its details deliberately add `question_name`. Plain rank details remain unchanged.

## Additive Rust API

Follow existing eager rank conventions:

```rust
impl Engine {
    pub fn rank_set<I>(
        &self,
        questions: &RankSet,
        records: I,
    ) -> Result<Call<Vec<SetRanked<I::Item>>>, Error>
    where
        I: IntoIterator,
        I::Item: Evidence;

    pub fn rank_set_with<I>(
        &self,
        questions: &RankSet,
        records: I,
        options: CallOptions<'_>,
    ) -> Result<Call<Vec<SetRanked<I::Item>>>, Error>
    where
        I: IntoIterator,
        I::Item: Evidence;
}
```

D needs no new top control in `CallOptions`. Rust returns the full merged order, as existing Rust rank does; callers can take its prefix.

`SetRanked<T>` owns the original item and exposes `index()`, `input()`, `into_input()`, `probability()`, and `question_name()`. The index remains zero-based input position, not merged rank. Probability belongs to the selecting member.

Do not require `T: Clone`, `Send`, or `Serialize`. Retain original items once and move each into its final result. Debug withholds original input and member names.

Use one pipeline call and one `Stop` for the entire set. `CallOptions` cancellation, deadline, interrupt, observers, shared context, batching, and send budgets apply across all members. Do not call ordinary rank once per member: that would restart controls and fragment facts.

Successful facts count N completed input records, not N×M member answers. Attempts, cache answers, tokens, and elapsed time reflect the entire call through worker join. Failed started calls retain final facts through the existing error path.

Question observations retain every member’s full probability and receipt, tagged with member name and original input index. Preserve existing row-observation conventions without widening public exhaustive enums solely for turns. Compact final rows retain only the selecting probability; observations and storage preserve all admitted member answers.

No top-level convenience free function is required in D.

## Failure, recording, and cache behavior

Rust prevalidates every finite record before sending, matching existing rank’s blank-evidence and record-limit refusals. CLI retains lazy intake unless an existing snapshot or plan path requires whole-input preparation.

Any failed member makes set rank fail. Do not inherit annotate’s partial-success behavior, invent zero probability, omit a failed member, or return an incomplete order. Valid low probabilities remain valid results.

Retain model consistency checks, cancellation, worker joining, retry limits, secrecy, and broken-pipe handling.

Reuse `quoted_plan_of`, `pack::asks`, and the existing pipeline. Names, set order, turns, top, positions, and display flags do not enter `QuestionKey`. Its existing adapter/address/model/state/wire-question identity stays unchanged.

Consequences:

- Individual member recordings replay under a set when every required key exists.
- Renaming or reordering members can change attribution/order without sending.
- Top and display changes reread stored answers.
- Changed wording, meanings, model, address, context, or selected evidence can require new answers.
- Strict replay misses fail locally with zero sends.
- Normal cache mode may send missing questions.
- Failed answers remain unstored.
- No recording schema, cache migration, new scratch file, or fingerprint system is introduced.

One-member request-body equality must hold at equal framing and settings. Multi-member request grouping may differ from separate member runs; logical question keys must still match.

`--plan` validates the full input and admitted set using the production planner/packer, then prints the existing first-request and whole-input summary form. Count records once and all logical questions through existing summary machinery. Plan is never the witness for a runtime no-send claim.

## Source changes

Keep changes local:

| Path | Intended change |
| --- | --- |
| `crates/thinkthen/src/core/question_set.rs` and a small sibling rank module | Reuse set grammar through rank-specific admission and no-rule resolution |
| `crates/thinkthen/src/core/order.rs` | Pure turns merge |
| `crates/thinkthen/src/cli/asked.rs` | Single-read single/set dispatch |
| `crates/thinkthen/src/cli/judge.rs` | Route admitted sets through existing rank preparation |
| `crates/thinkthen/src/cli/asking.rs`, `asking/judged.rs`, and a dedicated set adapter | Reuse intake, quoted planning, engine, and completion handling; decode every member |
| `crates/thinkthen/src/cli/schedule.rs` and a small set-retention sibling | Member-local top retention and final turns emission through shared display |
| `crates/thinkthen/src/public/rank_set.rs` | `RankSet`, eager engine methods, pipeline adapter |
| `crates/thinkthen/src/public/results/set_ranked.rs` | Additive result type |
| `crates/thinkthen/src/public/mod.rs`, `public/results.rs` | Register and export types |
| `crates/thinkthen/src/cli/args.rs` | Explain saved rank sets in existing help |

Avoid a second scheduler, repeated source reading per member, collecting annotate values, or generalizing every judgment into a configurable framework.

Update `specification/rank.md`, `question-file.md`, `result.md`, `settings.md`, and `libraries/rust/README.md`. Add executable rank-set contract examples under `spec/`. Keep SQL/language documentation explicit that their set routes await their owning tickets.

## Meaningful witnesses

Add separate 0401 test modules under existing CLI/backend and public test binaries. Register exact nonblank counts in `crates/thinkthen/tests/test-file-caps.json`.

| Witness | Required assertion |
| --- | --- |
| Distinct probability scales | Independently declared member lists produce literal turns order and selecting probabilities; global maximum sorting would fail |
| Duplicate consumes visit | Pin the `[0,1,2]` / `[0,2,1]` example above |
| Item identity | Equal text at different indexes emits twice; repeated sources preserve separate positions |
| Stable ties and top | Exact ties keep input order; every tested top equals a literal merged prefix; all records are judged |
| Probability retention | Ordering uses yes probability despite conflicting provider fields or confidence |
| One-member equivalence | Exact ordinary bytes and request bodies; equal keys/digest/answer fields; only additive name in details |
| Individual-member replay | Populate member recordings separately; set replay returns literal order with counted zero sends |
| Cache rereading | Rename/reorder members and change top/display without sends; a missing member fails strict replay |
| Invalid set table | Wrong kind, explicit cuts, root/nonroot `on`, bad version/name/keys/batch, and empty sets pin exit, complete diagnostic, empty stdout, and zero sends |
| Rust preflight | Blank later record and exceeded record limit refuse before any counted request |
| Late member failure | Valid early answers followed by a failed member return an error and no ranked output |
| Shared controls and facts | Cancellation/budget spans members; workers join; records count N; requests/tokens cover the combined call |
| Intake/display composition | Named windows, blank physical lines, JSONL common field, positions, scores, and around groups retain exact bytes and admitted evidence |
| Rust ownership | Non-Clone, non-Send caller items return once with correct original indexes |
| Secrecy | New types, diagnostics, failures, observers’ Debug, and recordings preserve existing withholding rules |

CLI set acceptance fails the inspected source because rank currently parses a single question. The additive Rust contract is absent today. Assertions must use independently declared expected orders and bodies rather than deriving their oracle from the new merge implementation.

Retain existing `keeping.rs`, `keeping/rank_top.rs`, `keeping/graded_rank.rs`, `keeping/default_framing.rs`, `keeping/permutation.rs`, `public_batches/ranks.rs`, and distinct parser, secrecy, cancellation, cache-miss, conflict, intake, and display regressions.

## Growth, risks, and completion

Estimate net growth of **650–900 nonblank Rust source lines** and **500–750 test lines**, including documentation comments and registration. This is an estimate, not a measured implementation ceiling.

Inspected files already near the 500-line cap include:

- `core/question_set.rs`: 483 nonblank lines.
- `public/bulk.rs`: 479.
- `public/mod.rs`: 462.

Use focused sibling modules rather than extending these past their caps. Update `sdlc/ratchet.json` to the actual measured total and explain growth by admission, retained probabilities, merge, and public result/control behavior.

Main risks are ambiguous duplicate handling, accidental annotate default cuts, insufficient probability retention, M×K memory growth, fragmented call facts, and receipt attribution after packing/splitting. The witnesses above target each risk. Backend reconciliation after 0399 and preservation of C’s shared emitter require fresh source review.

Implementation completion requires focused offline witnesses, policy before Rust code review, lint, and coordinator-named full checkpoints. Public API changes also require the applicable crate, C-door, Polars, and surface compatibility checks. No such checks have been run here.

Deferred work remains named: 0406 single-question parity; 0417 SQL sets in 0.2; 0418 C/language sets later; score members, filter sets, alternate merges, paragraph/overlap windows, asymmetric neighbors, table positions, per-record context, new providers, and store fingerprints.

Ian can overturn the proposed duplicate-consumes-visit rule, the `RankSet`/`SetRanked` names, and the bounded additive API choices. Fresh independent design review must settle those choices before code.