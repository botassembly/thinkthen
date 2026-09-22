# The question file cannot carry TypeSafe's structured instructions, criteria, levels, or boundaries

Status: Open

TypeSafe's own documentation ("Advanced: structure", https://docs.typesafe.ai/primitives/advanced) says the four question fields — `instructions`, Choice `criteria` values, Score `criteria` entries, and Noul `criteria.true`/`criteria.false` — all accept `string`, `object`, `array`, or `null`. System One models are trained on that structure. The tool sends strings, and the question file refuses anything else, so three of the four structure surfaces go unused.

**Verified against the live API on 2026-09-21** through `sdlc/scripts/live` (probe: `experiments/218-thinkthen-release-qa/probe-typesafe-structure.sh`, ledger charge 4,000 tokens, actual use 1,116 input and 149 output over two requests). All five shapes from the documentation page were accepted and answered, and answered well: the structured rubric sent a phishing message to `account` rather than `billing`; the credential question with structured `true`/`false` definitions answered 0.99 on a message asking for a password; the taxonomy question with subtrees as option values answered `Sporting Goods` with both department probabilities returned. The capability is real on the wire today, not documentation only.

## What was verified, 2026-09-21 at main `db19349`

| Surface | TypeSafe accepts | The tool today |
| --- | --- | --- |
| `state` | string or object | **Structured, fully used.** Several `--field` pointers send an object of the named parts, and the pointer is the disclosure boundary. This one is right. |
| `instructions` | string, object, array, null | String only. The wire carries `"instructions": "Which team?"`. |
| Choice `criteria` values | string, object, array, null | String or null. An object description is refused: `` `options` in the question file is a list of labels, or a map from each label to its description ``, exit 5. |
| Score `criteria` entries | string, object, array | String only. An object level is refused: `` `levels` in the question file is a list of levels, lowest first ``, exit 5. |
| Noul `criteria.true`/`.false` | string, object, array, null | String only (`--true`/`--false` take one sentence; the file's `true`/`false` are strings). |

## What the tool cannot say, in TypeSafe's own patterns

1. **The field-verification cascade.** Their invoice example asks several questions over one state, each instruction an object naming a `field` and an `extracted_value`. That is exactly `annotate`'s architecture — many named questions, one state, one grouped request — and today no annotate entry can express it. This is the widest miss: the function built for the pattern cannot write the pattern.
2. **JSON rubrics on options.** `what` / `not_for` / `examples` on a Choice description sharpens boundaries between options. The description slot already exists on `choose` and `tag`; only the value's shape is missing.
3. **Taxonomy walking.** Criteria values carrying subtrees, one Choice per tree level. `choose --options` and a shell loop already do the walk; the options cannot carry the branch contents.
4. **Signal objects on Score levels.** `summary` plus `signals` per level, for scales whose steps need more than a name.
5. **Structured yes/no boundaries.** Noul `true`/`false` as definition-plus-examples, for subtle boundaries. `decide --true`/`--false` and the tag label description would take it.
6. **Array instructions.** Compare lists and focus notes. This one may matter soonest: the recognize and relate methods use windows, edge markers, and pair lists, and if the harvest package's exact words are best carried as structure, the engine needs this before those functions build. That check belongs to the build team reading the package against this page.

## What widening costs

The grammar is settled by ADR 0013 and the adapter table in `specification/backends.md`; widening is a public-surface change with the second-agent review that rule carries. It is backward compatible — strings stay valid, so no digest changes unless a user opts into structure. The adapter must pass structure through verbatim, never reserializing it, the same discipline the exact-words rule demands for recognize. The question-file page, the help for `--true`, `--false`, option and level descriptions, and the annotate set grammar all grow one paragraph.

## How bad it is for a user

Not a defect: nothing promised is missing. It is capability left on the table, paid for by every user with a schema, a taxonomy, or a subtle boundary — the exact users TypeSafe's page is written for. The decision is the product side's; the timing question is whether the recognize build forces it first.

Found by experiment 218, from Ian's question of 2026-09-21, verified against the binary and the vendor's documentation.
