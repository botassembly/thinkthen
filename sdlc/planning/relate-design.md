# `relate`: design authority for the command and future surfaces

Status: amended 2026-09-23 after the independent Sol rejection of ticket 0081 and Ian's Option A ruling. This page authorizes no live or paid run.

Read `recognize-design.md` first. `recognize` and `relate` use one shared relation planner and one shared edge assembler. Recognition keeps its complete name fields. Standalone relate returns name-and-kind endpoints.

## The one line

**Find records that clash, repeat, or rely on each other.**

## Current command contract

```text
thinkthen relate [OPTIONS] RELATION...
thinkthen relate [OPTIONS] @entities.json
```

One-way rules use `NAME=SOURCE_KIND:TARGET_KIND`. A bare `NAME` means `NAME=*:*`. `--either` makes one unordered relation. A question file carries the same rule and its `reads` phrase under `relate`.

The command reads JSONL, CSV, or TSV entities. `/name` and `/kind` select fields, and command options can override those fields. Without a kind field, records receive synthetic kind `*`, and a rule naming a concrete kind fails locally. Both values must be nonempty. Input order stays stable. The same name with two kinds is two entities. An exact duplicate name and kind is a usage error. More than 255 entities is a usage error. Empty input succeeds without questions or edges.

`--lines` assigns synthetic kind `*` to each nonempty line. It accepts a bare relation or `*:*` only. The line set uses the same-kind planner below and does not expand. `--dry-run` sends nothing and reports planned counts and fallback facts without token or price claims. No planner method, one-or-many marker, runner-up question, or packing control is public. The threshold defaults to `0.5` and accepts the cut. Existing exit codes remain in force, including 0 for a completed run with no edges.

## Current output contract

Bare output emits one JSON edge per line:

```json
{"relation":"sung_by","source":{"name":"Octopus's Garden","kind":"song"},"target":{"name":"Ringo Starr","kind":"person"},"probability":0.93}
```

`source` and `target` are complete input entities. One-way edges use the declared direction. `--either` normalizes them to input order. The command emits no duplicate edge. A failed later question preserves completed edges, and a failed question never creates an edge.

The detailed result audits mixed choice and H questions through ordered entries under `answer.questions`. Each entry is self-contained and carries its request digest. It includes accepted candidates, rejected candidates, and failed questions. `value` contains accepted edges only. This Option A ruling matches the ordered `answer.tokens` shape from `recognize --details` without making public question ids permanent.

## Current final hybrid planner — authoritative

This is the only current method ruling. It applies to `relate` and to the relation step inside `recognize`.

### Entity and wildcard preparation

The shared core owns one validated `RelationEntity { name, kind }`, one `RelationEntityView` over that value and `RecognizedName`, and one generic `RelationEdge<E>`. The planner and question map use entity indexes. The assembler is generic over the endpoint value. Relate serializes only `name` and `kind`. Recognize continues to serialize offsets and strength.

The planner scans the input once and records concrete kinds in first-seen order. A rule's concrete kind expands to itself. A rule's `*` expands to all admitted concrete kinds in that order. The synthetic line kind is one special same-kind set and does not expand.

The planner plans each expanded concrete kind pair separately. It keeps rule order, kind order, and entity order. It excludes self-pairs. `--either` removes reverse duplicates by first-seen order. One-way `*:*` keeps both directions. A same-kind one-way rule keeps ordered pairs.

### Method selection

- A same-kind relation uses H. Both ways asks one unordered yes/no per pair. One way asks one yes/no for each ordered direction.
- A different-kind relation asks from the side with more entities. Each question offers the smaller side's entities and `none`. Equal sides ask from the declared source side. Every accepted non-`none` option at or above the cut becomes an edge.
- If a choice would exceed 255 wire options, including `none`, or an explicit backend profile's request-byte limit, that concrete relation uses H. Other concrete pairs from the same wildcard rule keep their own method.

The 255 and profile checks happen before any request. A plan that cannot fit does not send. Ticket 0079 supplies request splitting, identity, ordering, replay, and cancellation. It does not choose a relation method.

### H request wording

H request state carries one numbered entity table and one copy of the relation's `reads` wording. Each H question carries only its pair statement, such as `Does this hold: Item 1 and Item 2?`. The question does not repeat the table, names, kinds, or `reads` wording.

Recognition keeps its complete source context in the state. Its original source text remains byte-for-byte in the relation state. Relate uses the entity table as its source context. The state is private request data and is not a new public result type.

0081 owns the wildcard and H-state correction because the landed planner does not yet provide either promise. Recognition regression tests must prove the source text, one-copy state wording, pair-only H questions, request order, offsets, and strength. The existing non-relation evidence path stays unchanged.

## Shared ownership and evidence

`core/relation` owns the entity view, generic edge, wildcard expansion, planner, question mappings, threshold assembly, and direction rules. `recognize` owns `RecognizedName` and recognition-only fields. `relate` owns entity input and the bare edge renderer. No command owns a second planner or edge serializer.

Migration adds the standalone entity and view, generalizes the existing edge and assembler, adapts recognition, then calls the same generic path from relate. Tests prove that the same mappings produce name-and-kind-only relate endpoints and full recognize endpoints. They also prove equal names with different kinds remain distinct.

Experiment 239 supports choice for different kinds. Its measured graph used one choice per larger-side entity and retained options above the cut. Same-kind H and the fallback boundary come from the 2026-09-23 planner direction and bounded probes. Duets remain a known model limit. The product does not add runner-up questions in this ticket.

The relation rule continues to use `source` and `target` everywhere. The command rule spelling keeps `NAME=FROM:TO` because it names kinds, not JSON fields. Future library and database surfaces use the same source and target entity meaning.

## Future surfaces

Library and database surfaces remain future work. They must consume the same edge meaning and must not create a second planner. Ticket 0081 does not change the `surfaces` tree.

| Surface | Future call | Edge result |
| --- | --- | --- |
| Python | `tt.relate(records, relations=["caused_by"])` | Edges with `source`, `target`, and `probability` |
| TypeScript | `await tt.relate(records, { relations: ["caused_by"] })` | `Edge[]` |
| Ruby | `ThinkThen.relate(records, relations: %w[caused_by])` | An array of edges |
| R | `tt_relate(records, relations = "caused_by")` | A data frame of edges |
| Rust | `tt.relate(&spec, &records)?` | `Vec<Edge>` |
| C | `thinkthen_relate(...)` | Edges as a JSON string |
| DuckDB, SQLite, PostgreSQL | Table or query input | Rows with source, target, and probability |

The table does not settle future detailed-result or host-language types.

## Superseded history: 2026-09-21 pick-one proposal

The first proposal asked one pick-one question per unordered pair, with every legal relation and `no relation` as options. It used record numbers in output and packed pairs under backend limits. It predates the entity input, hybrid method, and name-and-kind edge rulings. It is retained for history only.

## Superseded history: 2026-09-23 all-H proposal

The all-H proposal said every relation, including `recognize` relation steps, should ask one yes/no per pair. It withdrew three-way choice and placed shared wording in request state. Ian's later final direction combines cross-kind choice with same-kind H. The all-H method is superseded and is not a current authority.

## Superseded history: 2026-09-23 three-way proposal

The three-way proposal asked a one-way pair whether the direction was source to target, target to source, or neither. It is superseded. One-way relations now use the current hybrid rule: cross-kind choice from the larger side and same-kind ordered H. The three-way method must not return in ticket 0081.

## Ian ruling and current rule

Ian chose Option A on 2026-09-23. Ordered self-contained entries represent choice, H, rejected candidates, and failures without a second join step. Each entry's request digest supports audit and run comparison without permanent public question ids. The estimated cost is about 200 production and 300 test lines.

Current rule: `relate` uses cross-kind choice over expanded concrete kinds and same-kind lean H, with shared H wording in request state once, an inclusive `0.5` default cut, 255/profile fallback, one shared generic planner and edge assembler, and name-and-kind-only standalone endpoints. `recognize` keeps offsets and strength. Detailed output uses Option A under `answer.questions`; normal `value` contains accepted edges only.
