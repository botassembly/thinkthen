# The question file cannot carry TypeSafe's structured instructions, criteria, levels, or boundaries

Status: Closed by landed ticket 0069. Approved by the product side on 2026-09-22. Ian can overturn any ruling below.

## Product rulings, 2026-09-22

1. **Adopt the widening.** Every description slot the vendor takes as JSON takes JSON in the question file: `instructions`, option and label descriptions, score levels, and the `true` and `false` boundaries of `decide`. A string stays valid everywhere. The CLI flags stay sentences.
2. **One rule for every slot.** Every slot that reaches the vendor as `instructions` or `criteria` takes a string or JSON, in every question kind and in every `annotate` entry. That covers the question text, option and label descriptions, score levels, and the `true` and `false` boundaries. No new key such as `with` is added. The command-line flags stay sentences. (Amended 2026-09-22: the first wording let only `annotate` carry an object question text, and that was an inconsistency.)
3. **The adapter passes the JSON through unchanged.** The canonical form for the digest is compact JSON with keys in the order written. Two files that differ only in white space give one digest.
4. **Keep the name `annotate`.** The rename to `structure` is refused. The word "structure" names what a slot may hold, and the ruled line "fills out a form for every record" stays. The verb `fill` is noted as the one plain alternative and is not adopted.
5. **Order.** The ticket is not a 0.1 blocker. It follows ticket 0065. Exception: if the build finds that the recognize and relate methods need array instructions, the ticket moves ahead of them.
6. **A measurement before any public claim.** One probe runs a structured rubric against its string form on a labeled set. Marketing shows structured descriptions only after that probe.
7. **Size.** The ticket pins what the local size check counts when a description is JSON.
8. **Tooling, ruled 2026-09-22 after Ian asked whether structure needs better tools.** No new grammar and no new flag builds objects from the shell. The question file is the typed form, and the libraries' native types are the typed form in code. The ticket adds only: a published JSON Schema for the question file, checked by `--dry-run` and usable by editors and agents; a recommended shape for a description object (`what`, `not_for`, `examples`) stated as a recommendation the model does not require; and, when the public library shapes land, typed builders for a description in each language. The `--field` object over a record is already the structured state and stays as it is.
9. **Order, amended 2026-09-22.** The founder interview (`2026-09-22-what-the-vendors-founder-said-about-where-the-model-goes.md`) moves this ahead of recognize and relate: right after ticket 0065, or first if the recognize build needs array instructions.
10. **Structured score levels are named by a map, ruled 2026-09-22 on the build team's question.** Compact JSON is refused as a level name. `levels` takes the two forms `options` already takes: a list of strings, lowest first, or an ordered map from level name to description, lowest first, where the description is a string or JSON. The name keys the probability map and the detailed result. The description is what the model reads and goes through unchanged. A bare object or array inside the list form is refused with exit 5 and a message that names the map form. This reuses the options shape and adds no new grammar.
11. **Tag carries a structured question as an array, ruled 2026-09-22.** When the question and every label description are strings, the request bytes stay exactly as they are today. When the question or any label description is JSON, `instructions` becomes a two-element array: the question exactly as written, then an object with `label` and `description` exactly as written, the description omitted when null. No JSON is templated into a sentence anywhere.
12. **Accepted types, ruled 2026-09-22.** Every widened slot takes a string, an object, an array, or null at the top, with numbers and booleans allowed inside objects and arrays. A top-level number or boolean is refused with exit 5. A null option or label description still means no description. The question text itself is never null.

**Ruling 6 was done on 2026-09-22.** `experiments/232-structured-descriptions`: 40 support messages, three teams, 15 boundary cases, 80 requests straight to the vendor under the live guard (50,758 input and 3,040 output tokens). A sentence per option: 37 of 40. A `what`, `not_for`, `examples` object per option: 38 of 40. Mean probability on the right option 0.883 against 0.928. One answer changed, toward the label. Input tokens roughly doubled. The gain is mostly confidence. The set is small and agent-written, so it shows a direction only. The ticket should carry a larger measurement before any accuracy claim.


TypeSafe's own documentation ("Advanced: structure", https://docs.typesafe.ai/primitives/advanced) says the four question fields — `instructions`, Choice `criteria` values, Score `criteria` entries, and Noul `criteria.true`/`criteria.false` — all accept `string`, `object`, `array`, or `null`. System One models are trained on that structure. The tool sends strings, and the question file refuses anything else, so three of the four structure surfaces go unused.

**Verified against the live API on 2026-09-21** through `sdlc/scripts/live` (probe: `experiments/218-thinkthen-release-qa/probe-typesafe-structure.sh`, ledger charge 4,000 tokens, actual use 1,116 input and 149 output over two requests). All five shapes from the documentation page were accepted and answered, and answered well: the structured rubric sent a phishing message to `account` rather than `billing`; the credential question with structured `true`/`false` definitions answered 0.99 on a message asking for a password; the taxonomy question with subtrees as option values answered `Sporting Goods` with both department probabilities returned. The capability is real on the wire today, not documentation only.

## What was verified, 2026-09-21 at main `db19349`

| Surface | TypeSafe accepts | The tool today |
| --- | --- | --- |
| `state` | string or object | **Structured at the boundary, string on the wire.** Several `--field` pointers build an object of the named parts, and the pointer is the disclosure boundary. `request.rs` then sends that object as a string. Corrected 2026-09-22; the founder issue, ruling 7, sends the object as JSON. |
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
