---
flow: build
priority: 81
opens: crates/thinkthen/src/core crates/thinkthen/src/engine crates/thinkthen/src/cli crates/thinkthen/tests conformance specification spec demos sdlc/ratchet.json sdlc/planning
---

# 0081: Build `relate` over the shared relation planner

Status: ready

## Outcome and authority

Add the tenth command, `thinkthen relate`, to the Rust core, private engine, and command. It reads one bounded set of entities, applies the caller's relation rules, and prints self-contained edges. An entity is exactly its `name` plus its `kind`. The same name with two kinds names two entities.

Ian ruled `relate` in on 2026-09-21. The final planner comes from the 2026-09-23 direction and results recorded by commits `1c29acf`, `f17931e`, and `c0d5651` in `sdlc/planning/relate-design.md`. Those later commits supersede the older all-H and three-way-choice sections that remain on that page as history. Experiment 237 supplies method H and its shared wording. Experiment 239 supplies the cross-kind choice planner, the no-marker result, the ruled edge shape, and the known multi-target weakness. Ian can overturn the public grammar, result shape, and routing decisions below before implementation.

The command forms are:

```text
thinkthen relate [OPTIONS] RELATION...
thinkthen relate [OPTIONS] @links.json
```

A one-way rule is `NAME=SOURCE_KIND:TARGET_KIND`; bare `NAME` is shorthand for `NAME=*:*`. A both-ways rule uses `--either NAME=KIND:KIND`, with the same bare-name shorthand, or the equivalent question-file member. A question file may add a `reads` phrase. The command exposes no planner method, one/many marker, runner-up policy, or packing control. `--threshold` gates edges and defaults to `0.5`.

## Entity input

`relate` consumes the complete entity set before it asks anything. JSONL, CSV, and TSV records use `/name` and `/kind` by default. `--field` and `--kind-field` override those pointers through the existing record-field grammar. Each selected value must be a nonempty string.

`--lines` treats every nonempty line as an entity name of kind `*`. Therefore line input admits only a bare rule or an explicit `*:*` rule. All line entities form one same-kind candidate set: a one-way rule asks method H once for each ordered pair, and `--either` asks it once for each unordered pair. A concrete kind in a rule over line input is absent and fails locally.

```json
{"name":"Octopus's Garden","kind":"song"}
{"name":"Ringo Starr","kind":"person"}
{"name":"Help!","kind":"song"}
{"name":"Help!","kind":"album"}
```

Input order is stable planner order. Equal names with different kinds remain separate. An exact duplicate `name` plus `kind` is a usage error because the ruled entity identity and the public edge cannot distinguish two copies. More than 255 entities is a local usage error before cache, replay, key lookup, or a send.

Rules are ordered and uniquely named. Validate their syntax even when input is empty. After input parsing, empty input succeeds with no questions and no edges; this is the sole exception to the rule-kind presence check. With any entity present, every concrete source and target kind named by a rule must occur in the input or the command fails locally. The shared planner expands `*` against the admitted concrete kinds in first-seen order. Never pair an entity with itself.

## Result contract

Default output is one JSON edge per line. Every edge reads on its own:

```json
{"relation":"sung_by","source":{"name":"Octopus's Garden","kind":"song"},"target":{"name":"Ringo Starr","kind":"person"},"probability":0.93}
```

`relation` is the rule name. `source` and `target` are complete ruled entities, not row numbers or ids. `probability` is the backend's probability for that accepted option or yes answer, unchanged. Keep an edge when its probability is greater than or equal to the inclusive cut. Normalize every one-way edge to the rule's source-to-target direction even when the target side supplied the question. A both-ways edge prints once, with endpoints in input order.

Edges print deterministically by relation declaration, concrete kind expansion, asking entity, then candidate entity. No edge prints twice. A finished run with no accepted edge exits 0 and prints nothing. A failed logical question preserves edges from completed questions and uses the existing partial-failure exit and diagnostic contract; absence from a failed question never becomes a negative answer.

`--details` returns the same ordered edges as the `value` of one standard `thinkthen.result/1` object. It carries every logical request digest in construction order, successful send counts under the existing rules, cache metadata, and the probability distribution needed to audit each accepted or rejected candidate. It does not replace an entity with an input position. Replay returns the same value and metadata except for the already ruled cache/send fields.

## Shared planner contract

Ticket 0080 owns the pure relation rule types, planner, question-to-edge map, thresholding, direction normalization, and self-contained edge assembly. This ticket calls those owners. It must not fork, wrap, or copy them.

| Concrete rule shape | Plan |
| --- | --- |
| Different source and target kinds | Each entity on the larger side gets one choice over every legal entity on the smaller side plus `none`. On equal side counts, the rule's source side asks. Keep every non-`none` option at or above the cut and normalize direction. |
| Same kind, both ways | Lean method H: one yes/no for each unordered legal pair. |
| Same kind, one way | Lean method H: one yes/no for each legal ordered direction. |
| Cross-kind choice exceeds a ceiling | Fall back for that whole concrete relation to method H. |

Method H places the rule's shared wording in the request state once. Each question carries only its entity statement. One-way H keeps the two directions as separate questions and separate probabilities. Several rules over one pair remain separate judgments, so one edge never suppresses another.

A cross-kind relation falls back to H when one choice would exceed the fixed accepted ceiling of 255 total options, counting `none`, or when one exact encoded choice request with the complete state cannot fit the exact request-size budget supplied by ticket 0079. Therefore at most 254 candidate entities fit beside `none`. The fallback applies to the whole concrete relation so one rule never mixes probability meanings. A multi-question plan that merely exceeds one request does not fall back; ticket 0079 splits it into deterministic contiguous requests. If one H question with its complete state cannot fit, preflight refuses the run with zero sends.

The user supplies no one/many marker. Experiment 239 found no useful F1 gain from that marker. The planner keeps every choice option at or above the cut. Multiple true targets remain a known weakness: both methods found every singer for only 1 of 19 Beatles duets at their compared settings. Runner-up confirmation is unmeasured, changes the method, and does not block this ticket.

## Engine and command work

Add only the standalone `relate` orchestration around the shared planner:

1. Parse and validate entities, ordered rules, optional `reads` phrases, and the cut before any external effect.
2. Ask the shared planner for typed logical questions and their edge map.
3. Submit every many-question plan through ticket 0079's exact-size request path.
4. Merge typed answers in logical order and ask the shared edge assembler for the ordered edge stream.
5. Render the bare stream, standard detailed result, dry-run plan, diagnostics, and exit code through existing owners.

`--dry-run` sends nothing and reports the entity count, each relation's routed method, exact logical question count, fallback reason when present, exact split-request count, and exact encoded bytes per request. The public help describes the quadratic same-kind path and the 255-entity ceiling. It makes no token, price, or speed claim.

An under-budget request keeps the planner's established bytes and one logical request. Ticket 0079 preserves question order and request identity when it splits an oversized plan. Do not add relate-specific splitting or packing.

## Dependency and ownership boundary

Implementation starts from main only after ticket 0079 has landed. Ticket 0079 owns generic exact-size splitting, preflight of every chunk before the first send, request identity, ordered aggregation, replay/cache behavior, cancellation between chunks, and under-budget byte preservation. This ticket consumes that path and adds no splitter proof.

Implementation also starts after ticket 0080 has landed its shared relation planner and edge assembler. Ticket 0080 owns planner policy because `recognize` needs it first. Ticket 0081 owns the `relate` command, whole-set entity grammar, 255-entity guard, relate-specific dry-run/help/specification/how-to pages, and standalone edge stream. It does not rebuild recognition, relation planning, fallback, thresholding, or edge serialization.

The new-row proposal in `sdlc/issues/2026-09-23-relate-in-a-database-kinds-new-rows-and-the-cache.md` has no ruled command grammar or cache policy. This ticket records it as follow-up work and ships whole-set relate only. It does not invent `--new`, a second input set, or incremental graph mutation.

## Scope and exclusions

Allowed: `relate` specification and executable page; command arguments and help; question-file schema and canonical digest support; whole-set entity parsing; calls into the shared planner and edge assembler; private-engine submission through the 0079 request path; bare and detailed rendering; focused command, secrecy, replay, dry-run, and loopback tests; one mechanically adapted replay case from experiment 237 and one from experiment 239; one replay-only how-to; exact ratchet, ticket, queue, and record updates.

Excluded: recognition; a second planner or edge type; public Rust, C, language, Polars, or database APIs; the `surfaces` branch; annotate option sources; new-row or two-set syntax; graph traversal; graph storage or mutation; ids in place of entities; a one/many or method marker; runner-up confirmation; candidate blocking; threshold comparison; prompt optimization; a `0.4` default; packed records; token or price claims; dependencies; workflows; publication; credentials; live calls; and paid calls. Relations remain labeled beta everywhere they appear.

Production changes may touch at most seven Rust files. Total additions may not exceed 450 nonblank production Rust lines or 900 nonblank Rust lines including tests. Add no dependency. Keep every source and test file under the repository's 500-line limit. Search the 0080 planner, question-file, request, result, and record-format owners before raising the exact ratchet. The implementation and review record must name each increase and why it earns its lines.

## Acceptance

- Observe focused red tests before implementation. Pin the relate command's default and overridden entity pointers, Unicode names, same-name/different-kind identity, exact-duplicate refusal, empty-input exception, missing kinds on nonempty input, line-input `*:*` behavior, 255 entities accepted, 256 refused, and local no-send failures.
- At the command boundary, use typed planner doubles or already-owned planner fixtures to prove that parsed entities and rules reach the 0080 planner unchanged, its routed method and fallback reason reach dry-run, and its typed edges reach rendering unchanged. Do not repeat 0080's routing, threshold, wording, option-ceiling, size-fallback, or edge-assembly tests.
- Replay one recorded cross-kind case from experiment 239 and one same-kind H case from experiment 237 with no key or network. Prove the standalone command reaches the existing request path and prints the ruled self-contained edges at the default `0.5`. Keep experiment 239's tuned `0.4` and duet result as evidence only. Never edit a recorded response to make it agree.
- Pin the exact bare edge line, empty output, standard detailed result, deterministic edge order, inclusive `0.5` default, both-ways endpoint order, no duplicate edge, partial-failure exit, dry-run report, and all ordinary exit codes.
- Count loopback requests to prove malformed entities, malformed rules, impossible profile limits, 256 entities, and dry-run send nothing. Search stdout, stderr, Debug output, recordings, cache entries, and fixture files for credentials and unredacted failure evidence on every new success and failure path.
- Publish `specification/relate.md`; update the specification index plus question-file, result, channels, backend-profile, and recording pages where the command changes them; and add one replay-only how-to in the ADR 0011 form. State that entities are name plus kind, edges are self-contained, relations are beta, same-kind cost grows with pairs, cross-kind multi-target recall is weak, and no public price is claimed.
- Run focused tests, policy, exact ratchet, formatting, Clippy, and `git diff --check` before code review. An independent reviewer checks grammar and output, the 0080 and 0079 ownership seams, deterministic replay, local no-send paths, and budgets. The coordinator then runs all four local gates sequentially with the key and base-address variables unset. No live or paid call runs.

## Dependencies and blockers

- Ticket 0079 must land on main with its reviewed splitter contract.
- Ticket 0080 must land on main with the one pure planner and self-contained edge assembler.
- No experiment blocks the ruled planner. The multi-target weakness is accepted and nonblocking. A later ticket may test runner-up confirmation without changing this ticket's public shape.

## Complexity

Contract 3; state/timing 3; reach 3; proof 4; cost of error 3; total 16. Final level: 3. Route implementation to `sol-implementer` with medium reasoning and route design and code review to separate `sol-reviewer` sessions. Stop and re-score if the work needs a new public setting, a second planner or splitter, changed request bytes under budget, a live fixture, a public library API, more than seven production Rust files, or more than the stated line budget.
