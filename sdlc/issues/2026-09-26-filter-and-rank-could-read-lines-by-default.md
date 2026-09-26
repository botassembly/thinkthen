# filter and rank could read lines by default

Status: Open. Filed 2026-09-26 by the marketing session after Ian asked why `--lines` appears in almost every example.

## What happens

`specification/records.md` says `filter` and `rank` need one record flag. With no flag they stop with a usage error, because one document is not a stream. So nearly every `filter` and `rank` example on the site and in Beatles Bench carries `--lines`. Most of them read plain text, one record per line.

`decide`, `choose`, `tag`, `score`, and `annotate` read the whole input as one document by default. That default is right for them. A paragraph is one piece of evidence.

## Options

1. `filter` and `rank` read `--lines` by default. `--jsonl`, `--csv`, and `--tsv` still pick the other framings. Today the no-flag case is an error, so no working script changes behaviour.
2. Keep the rule and document why the flag is required.
3. Guess the framing from the input. The specification rules this out: the tool never guesses.

## Recommendation

Option 1. It removes a flag from the most common case and breaks no working script. The explicit flags stay for scripts that want to say what they read. `--help`, the specification, the site examples, and the Beatles Bench scripts follow. Ian can overturn this.
