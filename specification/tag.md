# `tag`

Status: **Settled** by ADR 0029.

`tag` tests independent labels against one record and prints every label that reaches one cut.

```sh
thinkthen tag 'Which topics?' billing urgent < message.txt
```

The command takes 1 to 20 unique labels in order. A described label is `--label LABEL=DESCRIPTION`; repeated `--label` options replace positional labels and cannot stand beside them. A question file uses `tag` and `labels`, where `labels` is a list or an ordered map from label to description.

All labels ride in one request. Each becomes a yes-or-no backend question whose instruction is `QUESTION`, a blank line, then `Determine whether the label JSON_STRING applies to this item.` A description becomes the true criterion. The backend answer for every label must be a yes-or-no probability.

One cut applies independently to every probability. The default is 0.5, equality passes, and a band is refused. On one document the bare result is one compact JSON array in label order. In record mode that array sits under `value` beside the parsed `input` record. An empty array is a successful answer and exits 0. `tag` has no raw or quiet view.

```json
["billing"]
```

Under `--details`, the question lists the label names and the answer keeps every probability. This excerpt omits the required `schema` and `meta` fields:

```json
{"value":["billing"],"question":{"verb":"tag","text":"Which topics?","labels":["billing","urgent"]},"answer":{"kind":"tag","probabilities":{"billing":0.91,"urgent":0.22}},"threshold":0.5}
```

Changing or reordering one label changes the whole request and cache key. A rerun therefore asks every label again, and answers near the cut can move.
