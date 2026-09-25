# find --details keys probabilities by generated ids it never maps

Status: Open

Filed by the marketing session on 2026-09-25, from Beatles Bench ticket 0006. Build: thinkthen 0.0.1, release of 2026-09-24.

## What happened

```sh
jq -c '.records[]' examples/07-find/find-cold.jsonl \
  | thinkthen find 'These are songs by the Beatles. Which one did they release first?' --jsonl --field /input --details
```

The ten records carry their own ids, `u01` to `u10`. The answer keys every probability and its pick as `u001` to `u010`. The output never says which record each key means.

## Why it matters

`specification/find.md` settles the generated ids and input order. A reader still has to rebuild the map from the input order. The talk's find slide and the bench page each needed that step. The records' own ids `u01` and the generated `u001` look alike, which invites a wrong join.

## Asks

1. Under `--details`, list each generated id with its record, or its index into the input.
2. Or key the probabilities by the value at `--id` when the user names one.

Ian can overturn both.
