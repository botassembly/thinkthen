# 0596 — even-result-shapes

Status: OPEN.

Milestone: 0.3

Draft: Ian filed this on 2026-10-15 for the 0.3 PM to finish. Each item needs a decision before a developer starts; the 0.3 PM may split it into one ticket per item.

## Outcome

The ten functions return `value` and `probability` in one consistent pattern across every library, the command line and SQL, and each option type accepts only the options its function uses.

## Evidence

- Ian, 2026-10-15, asked for the full shape of options and values from the ten functions, and on hearing the uneven parts: "the uneven things certainly should be fixed as tickets."
- Shapes on `ticket/0543-final`, read from `libraries/python/thinkthen/__init__.pyi`, `_calls.py`, `_native_results.py`, `_named_generated.py`, `libraries/python/README.md` and `specification/`:

| Function | `value` | Top-level `probability` |
| --- | --- | --- |
| `decide` | `True`, `False`, or `None` under a band | chance of yes |
| `choose` | picked label or `None` | chance of the pick |
| `score` | probability-weighted position from 0 to levels minus one | none |
| `tag` | labels that pass | none |
| `filter` | kept records | one per kept record |
| `rank` | original indices, best first | one per record |
| `find` | best record or `None` | chance of the pick |
| `annotate` | one row per record, an answer per name | none |
| `recognize` | names with kind and offsets | one per name |
| `relate` | edges | one per edge |

- These shapes follow ticket 0581's PM decisions of 2026-10-14, which set rank's value to original indexes and probability to null for score, tag and annotate. Items 1 and 3 would overturn them.

## Items

1. Score, tag and annotate carry no top-level probability. Python's `Result.probability` returns `None` for them (`probability_mode` `null` in `_native_results.py`); the per-level, per-label and per-question probabilities live only in `results`. Decide the top-level form, such as a label-to-probability map.
2. Score returns a probability-weighted position, not the caller's scale. `specification/score.md`: five levels on a "Rate this from 1 to 5" question return 0 to 4. The answer already holds the most probable `level` name, which can differ from the weighted value. Decide whether `value` stays the position and the level name gains a plain accessor.
3. Rank returns original indices while `filter` and `find` return records. Decide whether rank returns the records in order, with indices kept in `results`.
4. Option types are shared. In `_named_generated.py`, every function's options type inherits `RequestOptions`, which lists recognize-only keys (`examples`, `mode`, `seed_spans`, `stage_context`, `snippet_pieces`, `relation_threshold`) and others. A type checker accepts `tt.decide(q, text, mode="whole")`, and the generated `decide` signature takes every one of these keys. The runtime already refuses it before any send: native admission in `crates/thinkthen/src/public/request/admission.rs` raises a usage error, "mode belongs to recognize", and does the same for the other recognize-only keys. Give each function only its own keys in the types and signatures.
5. `decide`'s value type is the wide `_ReadingValue` union in `__init__.pyi`, though the value is `bool | None` for one input and `list[bool | None]` for a list. Narrow it.

## Scope

- Changes: the generated result and option contracts in `crates/thinkthen/src/` that every library reads, `libraries/*/` result projections and their READMEs, `specification/result.md`, `specification/score.md`, `specification/tag.md`, `specification/rank.md`. The coordinator narrows this when the decisions land.
- Keeps: request bodies byte for byte; cache and recording identities; "not sure" as the host's null; a failure never reads as an answer.

## Proof

One routine-suite table test per library, under replay, asserts each function's `value` and `probability` shape against one shared expected table, and a type check rejects a call with another function's option, which the runtime already refuses before any send.

## Defers

Truthiness, tools and examples are ticket 0595. A change to the command line's printed output follows only if an item's decision says so.
