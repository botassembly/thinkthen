# `tag`

Status: **Settled** by ADR 0029.

`tag` tests independent labels against one record and prints every label that reaches one cut.

```sh
thinkthen tag 'Which topics?' billing urgent < message.txt
```

The command takes 1 to 20 unique labels in order. A described label is `--label LABEL=DESCRIPTION`; repeated `--label` options replace positional labels and cannot stand beside them. A question file uses `tag` and `labels`, where `labels` is a list or an ordered map from label to description.

In record mode, `tag` fills each request with records up to `--batch N|max` and the request-size limit. `--batch 1` sends one record a request, in the same quoted form. A top-level question-file `batch` follows a typed `--batch` and `THINKTHEN_BATCH`; the default is `max`. `--context FILE` supplies shared evidence to each batch, and `--max-request-bytes N` sets its soft byte cut. A profile's limits still refuse a lone record that exceeds them. A typed batch or context on one document is refused. [records.md](records.md) describes grouping, halving, and ordered output.

All labels ride in one request. Each becomes a yes-or-no backend question whose instruction is `QUESTION`, a blank line, then `Determine whether the label JSON_STRING applies to this item.` A description becomes the true criterion. The backend answer for every label must be a yes-or-no probability.

When a question file gives the question text or any description as an object or a list, the sentence form would interpolate JSON, so every label of that question takes the array form instead: `instructions` is `[QUESTION, {"label": NAME, "description": DESCRIPTION}]`, with `description` absent where the label has none. One structured value switches every sibling label, so a request never mixes the two forms. Under ADR 0111, the record's quote opens the question text in element 0 of each array, as `The text is RECORD. QUESTION`, when that text is a string.

One cut applies independently to every probability. The default is 0.5, equality passes, and a band is refused. On one document the bare result is one compact JSON array in label order. In record mode that array sits under `value` beside the parsed `input` record. An empty array is a successful answer and exits 0. `tag` has no raw or quiet view.

```json
["billing"]
```

Under `--details`, the question lists the label names and the answer keeps every probability. This excerpt omits the required `schema`, `answer_id` and `meta` fields:

```json
{"value":["billing"],"question":{"verb":"tag","text":"Which topics?","labels":["billing","urgent"]},"answer":{"kind":"tag","probabilities":{"billing":0.91,"urgent":0.22}},"threshold":0.5}
```

Changing or reordering labels changes the resolved reading and result identity. Each label question retains its own cache key: unchanged questions can reuse their held observations, and added or changed questions are asked again. The new result follows the caller’s label order. Batch size remains outside those keys.
