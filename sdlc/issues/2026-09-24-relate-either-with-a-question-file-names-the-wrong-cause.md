# `relate --either` with a question file names the wrong cause

Status: Open

`relate --either @FILE` exits 2, which is right. Its message is the inline relation grammar sentence, which names the wrong cause. The refusal comes from `crates/thinkthen/src/cli/relate/config.rs` (the `arguments.either` check near line 45 at ticket 0088's landing).

## Reproduction

Found by the final review of ticket 0088 (`sdlc/records/0088-review-final.md`, finding F4). Write any valid relate question file to `q.json`, then run:

    $ printf 'Ada\n' | thinkthen relate --either @q.json --lines --dry-run --url http://127.0.0.1:9/v1 --model local-1 --no-cache

The command exits 2 before it reads the file and prints `thinkthen: a relate relation is NAME=SOURCE_KIND:TARGET_KIND, or a bare NAME`. The file holds its own rules, so the message should say that `--either` applies only to inline rules.

## Smallest fix

Give this case its own refusal sentence and add a row to the relate refusal table that pins it.
