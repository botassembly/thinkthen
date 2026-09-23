---
flow: build
priority: 80
opens: crates/thinkthen/src/core crates/thinkthen/src/engine crates/thinkthen/src/cli crates/thinkthen/tests conformance specification spec demos sdlc/ratchet.json sdlc/planning
---

# 0080: Build `recognize` over the shared relation planner

Status: implementation complete; independent Sol review rejected; remediation complete; re-review pending

## Outcome and authority

Add the ninth command, `thinkthen recognize`, to the Rust core, private engine, and command. It finds every name in one text, assigns the caller's kind, computes the ruled `strength`, and optionally returns relation edges. It uses the same pure relation planner that ticket 0081 later exposes through `relate`. Ian ruled `recognize` in on 2026-09-21. The public shape comes from `sdlc/planning/recognize-design.md`; the latest planner rulings come from commits `1c29acf`, `f17931e`, and `c0d5651` in `sdlc/planning/relate-design.md`. This ticket records the boundary between those rulings and the older examples that they supersede. Ian can overturn the output and planner decisions below before implementation.

The command forms remain:

```text
thinkthen recognize [OPTIONS] [KIND]...
thinkthen recognize [OPTIONS] @names.json
```

With no kind, the kinds are `person`, `organization`, and `place`. Positional kinds and described `--kind KIND=DESCRIPTION` entries are the question, not recognition settings. A relation rule turns relations on. The only cuts are `--threshold` for names and `--relation-threshold` for edges, both defaulting to `0.5`.

## Result contract

One text returns one object. `entities` stays the top-level compatibility name. Each found name carries its own value rather than a vocabulary code:

```json
{
  "entities": [
    {"name":"Maria Chen","kind":"person","start":0,"end":10,"strength":0.98}
  ],
  "relations": [
    {
      "relation":"works_for",
      "source":{"name":"Maria Chen","kind":"person","start":0,"end":10,"strength":0.98},
      "target":{"name":"Northwind Freight","kind":"organization","start":18,"end":35,"strength":1.0},
      "probability":1.0
    }
  ]
}
```

`name`, `kind`, and `strength` replace the older `text`, kind-code, and `confidence` examples. `start` is inclusive and `end` is exclusive in Unicode scalar values on the command surface. Repeated equal names remain distinct through their offsets. A relation edge is self-contained: `source` and `target` repeat the complete recognized-name object, and `relation` names the rule. It never requires a reader to count input rows or chase an entity id. Its `probability` is the model's reported probability, unchanged. `relations` is absent when no rule was supplied and is an empty list when rules were supplied but no edge passed the cut.

No names is a successful answer with an empty `entities` list. A single text prints the object directly. Record modes use the existing `{"input":...,"value":...}` row contract and preserve input order. `--details` wraps the same value in `thinkthen.result/1`, carries every logical request digest in construction order, sums successful send counts under the existing rules, and exposes the inputs to each computed strength. A failed detection, kind, or relation question fails that input instead of silently returning a partial name or graph; completed earlier rows remain available under the existing ordered-stop contract.

## Recognition design

Port the measured baseline from `experiments/225-recognize-harvest-package/`, not a new natural-language implementation.

1. Tokenize, window, detect, assign kinds, assemble spans, and compute strength by the rules in `rules/rules.md`. Use the canonical lineage-B words in `words/` byte for byte. Ask detection and kind for every token as the recorded main pass does. Every member token counts. `strength` is the lowest detection probability times the mean probability of the winning kind, rounded to four decimals. Keep names at or above the cut.
2. Keep all recognition policy internal. Add no depth, word-threshold, repair, connector, formula, overlap, possessive, or policy option in the command, question file, environment, config, or public metadata. The connector list stays deleted. The fixed trailing-possessive rule remains. One maximal contiguous run of `IN` words is one candidate, so candidates are disjoint and no overlap resolver runs.
3. Preserve the exact source text through every request. Splitting divides questions, never the text or the assembly state. Offsets always index the original text. Names may cross a request boundary because assembly happens only after all required answers return.
4. Parse and canonicalize the `recognize` question-file form through the existing question-file machinery. The file carries ordered kinds, optional relation rules and `reads` phrases, the two cuts, model/profile, and ordinary evidence selection. It does not make `recognize` a fifth `annotate` entry type in this ticket.
5. A bad kind, rule, wildcard, duplicate, count, cut, source/target reference, file member, or impossible profile limit fails locally before key lookup, cache mutation, or a request. A kind count stays 1 through 20. Rules use `source` and `target`; `*` is explicit any-kind. A name is never related to itself.

## Shared relation planner

Build one pure planner over recognized names and relation rules. `recognize` calls it after name assembly. Ticket 0081 must call this planner and edge assembler rather than copy either one.

- A rule between different kinds uses a choice question. The side with more entities supplies the questions and the smaller side supplies the options plus `none`; the planner normalizes every accepted answer back to the rule's source-to-target direction. Every option at or above the relation cut becomes an edge. The user supplies no one/many or method marker.
- A rule within one kind uses lean method H. A both-ways rule asks each unordered pair once. A one-way rule asks each legal ordered direction. Shared wording rides once in the state, and each question carries only its statement.
- If one choice would exceed the 255-option product ceiling or cannot fit by itself under the resolved backend profile's exact encoded request-byte limit, that whole relation uses method H. Do not infer a token limit from bytes and do not revive the stale 100-option claim.
- The default relation cut remains `0.5`. Experiment 239's tuned `0.4` stays evidence for a later threshold choice, not this ticket's default. The duet runner-up confirmation remains unbuilt.
- All generated many-question plans pass through ticket 0079's splitter. An under-budget plan keeps its established bytes and one logical request. Splitting exists for correctness. Do not combine records, regroup unrelated work, or change request bytes solely to save money.

`--dry-run` reports the exact token, detection/kind question, and split-request counts it can know before asking. It labels relation pairs and requests as an upper bound until names and kinds exist. It must not print an exact relation count it cannot know.

## Dependency boundary

Implementation starts after ticket 0079 lands on main. Current main `6195737` records the launch-first queue: 0079 is the only engine prerequisite for 0080, while 0076 through 0078 follow 0087. Ticket 0079 owns generic exact-byte request splitting, request identity across chunks, replay/cache behavior, cancellation between chunks, and under-budget byte preservation. Ticket 0080 consumes those interfaces and adds only recognition/planner integration tests. It does not add a second splitter.

Ticket 0080 owns the pure relation rule types, planner, question-to-edge map, and self-contained edge assembler because `recognize` needs them now. Ticket 0081 owns the `relate` command, its records/entity input grammar, `--kind-field`, whole-set and incremental/new-row behavior, the 255-record guard, relate-specific dry-run/help/specification/how-to pages, and any standalone edge stream. Ticket 0081 must not rebuild recognition, planning, fallback, thresholding, or edge serialization.

The latest planner has no recorded fixture that proves it inside an in-text `recognize` relation run. The 225 relation recordings use the withdrawn pick-one-per-pair method. Experiment 239 proves only its recorded cross-kind planner cases over supplied named, kinded entities. Pure planner tests prove same-kind H and composition with recognized names. Ticket 0079's integration fixtures prove the option-ceiling and exact-size fallback paths through the real splitter. This ticket authorizes no live replacement fixture and no fallback to the old method.

## Scope and exclusions

Allowed: the `recognize` specification and executable page; command arguments/help; question-file schema and canonical digest support; pure tokenizer, span, strength, rule, planner, and result types; staged private-engine orchestration through the 0079 splitter; ordinary and record-mode rendering; focused unit, property, integration, secrecy, cache, replay, cancellation, and dry-run tests; mechanically adapted recorded fixtures from experiments 225 and 239; one replay-only how-to; exact ratchet, ticket, queue, and record updates.

Excluded: the `relate` command or its input surface; public Rust, C, language, Polars, or database APIs; the `surfaces` branch; `annotate` option sources or a recognize question type; new-row graph updates; two-set syntax; graph traversal; threshold comparison; prompt optimization; a `0.4` default; runner-up duet confirmation; word lists, dictionaries, templates, user-selectable recognition policy, packed records, cost claims, dependencies, workflows, publication, credentials, live calls, and paid calls. Relations remain labeled beta everywhere they appear.

Production changes may touch at most eighteen Rust files. Total additions may not exceed 2,400 nonblank Rust lines including tests. Add no dependency. Keep every source and test file under the repository's 500-line limit. Search for reusable question-file, scheduler, result, and record-rendering paths before raising the exact ratchet. The implementation and review record must name each increase and why it earns its lines.

## Ruled 2026-09-23

Ship the measured baseline. One maximal run of `IN` words is one candidate, and runs are disjoint. No overlap promise or resolver belongs in ticket 0080. This ruling explicitly overturns the earlier settled overlap line in `sdlc/planning/recognize-design.md` and supersedes earlier 0080 planning that requested overlap selection.

Later Beatles Bench branch-function research found boundary errors, not overlap errors: `album` enters `the album Abbey Road`; `Don't` drops from `Don't Pass Me By`; long titles lose their middles; exact-match songs measured precision 0.23 and recall 0.15; overlap matching measured precision 1.00 and recall 0.54. This evidence is recorded for later research only and creates or queues no ticket. See `sdlc/issues/2026-09-23-harvest-the-beatles-and-relate-runs-for-efficiency-thresholds-and-tuning.md`.

## Acceptance

- Observe focused red tests before implementation. Pin tokenization, punctuation, windows, kind voting, strength rounding and inclusive cut, no names, repeated names, fixed possessive handling, disjoint maximal `IN` runs, deleted connectors, and exact Unicode offset round trips over an accent, emoji, and combining mark.
- Pin the command and question-file grammar, defaults, both cuts at `0.5`, rule direction and wildcards, exact canonical digest behavior, local no-send refusals, and the complete absence of recognition policy settings. Help says relations are beta and says which quantity each cut gates.
- Replay the forty 225 cases with no key and no network. Preserve their ten labeled divergences. Mechanically migrate expected public names to `name`/`kind`/`strength`, offsets to command indexing, and relation output to self-contained edges; never edit a recorded backend response to make it agree.
- Replay experiment 239's planner requests with no key and prove only the cross-kind cases those recordings contain: choice planning, the larger-side/smaller-options rule, every option at or above the cut, direction normalization, and the ruled `0.5` default independently of the experiment's tuned result. Do not cite experiment 239 as proof of same-kind H, the option ceiling, or exact-size fallback.
- Use pure planner tests to prove same-kind H for both-ways and one-way rules and to feed typed planner answers into recognition without inventing a second wire fixture. Reuse ticket 0079's integration fixtures to prove the 255-option boundary, exact encoded-size fallback of the whole relation to H, split H batches, complete source state, logical order, request digests, cancellation between chunks, replay, and under-budget byte preservation. Do not retest the splitter's internals here.
- Pin the exact bare object, standard detailed object, self-contained edge, absent-versus-empty `relations`, `{"input","value"}` record rows, input order under concurrency, request/send metadata, cache answers, and exit codes. A failed logical question produces no partial entity or edge object for that input.
- Count loopback requests to prove malformed input, impossible profile limits, and dry-run send nothing. Search stdout, stderr, Debug output, recordings, cache entries, and fixture files for credentials and unredacted failure evidence under every new success and failure path.
- Publish `specification/recognize.md`, update the specification index and question-file/result/channel/profile pages that the command changes, and add one replay-only how-to in the ADR 0011 form. State that long text keeps its context while questions split, strength is computed rather than a probability, relations are beta, lowercase connector words may split a name, and no public price is claimed.
- Run focused tests, policy, exact ratchet, formatting, Clippy, and `git diff --check` before code review. An independent reviewer checks the public grammar and output, fixture provenance, planner sharing with 0081, 0079 reuse, Unicode offsets, profile boundaries, no-send paths, cancellation, and budgets. The coordinator then runs all four local gates sequentially with the key and base-address variables unset. No live or paid call runs.

## Dependency

Ticket 0079 is the only required engine prerequisite and must be landed before implementation starts. Tickets 0076 through 0078 are later work under the launch-first queue on main `6195737`; they do not block 0080. The recorded and pure proof boundary above closes the in-text composition gap without a live call.

## Complexity

Contract 4; State/timing 3; Reach 4; Proof 4; Cost of error 3; Total 18. Minimum floor: level 3 for staged multi-request work, cancellation, request identity, and a new public command. Final level: 3. Selected implementation: `sol-implementer` (`gpt-5.6-sol`, medium). Independent design and code review use separate `sol-reviewer` sessions. Stop and re-score if implementation needs a new public setting, a second splitter, a changed planner, a live fixture, a public library API, or more than the stated file or line budget.
