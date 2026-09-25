# recognize --dry-run help promises the request and shows no size

Status: Open

Filed by the marketing session on 2026-09-25, from Beatles Bench ticket 0006. Build: thinkthen 0.0.1, release of 2026-09-24.

## What happened

```sh
printf '%s' "Ringo Starr wrote Octopus's Garden on a boat off Sardinia, and the band recorded it at Abbey Road Studios for the album Abbey Road." \
  | thinkthen recognize person song album place --threshold 0.01 --dry-run
```

It printed `{"tokens":26,"detection_questions":26,"kind_questions":26,"requests":1}`. The live request for the same text cost 8,092 input tokens.

## The mismatch

- `recognize --help` says `--dry-run` will "Print what would be sent and stop."
- `specification/recognize.md` ("Dry run and cost") says it reports counts only. The output matches the spec.
- "tokens" counts words of the text. It is not the request's input tokens. A reader takes it as cost.

## Asks

1. Make the help agree with the spec.
2. Name the field for what it counts, such as `words`.
3. Add an input-token estimate, so a user sees the 8,000-token request before sending it.

Ian can overturn all three.
