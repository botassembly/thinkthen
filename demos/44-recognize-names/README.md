# Find literal entities with a saved task

Status: green

Verbs: `recognize`

A caller can define a receipt amount as an entity. This controlled offline fixture returns the amount instead of the TOTAL label. It measures no model accuracy.

```bash
fixture="$(git rev-parse --show-toplevel)/specification/fixtures/recognize/caller-defined"
cat "$fixture/text.txt" | env -u THINKTHEN_API_KEY -u THINKTHEN_BASE_URL thinkthen recognize @"$fixture/question.json" --url "$(cat "$fixture/url.txt")" --replay ../../specification/fixtures/recognize/caller-defined/recording --no-cache | jq -c '.entities' | mustmatch '[{"text":"42.75","start":8,"end":13,"length":5,"kind":"amount","strength":1.0}]'
```

The saved question supplies instructions, an entity definition and the `amount` label description. Each recognition stage receives that declaration. Offsets count Unicode scalar values, so the prefix `é` occupies one position. The span retains its decimal point and excludes the trailing sentence period.

The original `message.txt` and `recording/` retain the live 2026-09-27 name-recognition result. Its described labels now select caller-defined wording under ADR 0124. Its historical answers do not predict results for the new questions.
