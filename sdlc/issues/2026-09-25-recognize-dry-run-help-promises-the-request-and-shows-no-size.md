# recognize --dry-run help promises the request and shows no size

Status: Open

Reported from a user's benchmark on 2026-09-25 and checked against main at `535cb3e7` with the debug build of that morning. Verdict: confirmed. One correction to the report: the specification does not agree with itself, so the help text is not the only thing wrong.

## What the user sees

The user piped one 24-word sentence to `recognize` with four kinds, `--threshold 0.01`, and `--dry-run`, with no key set. It prints `{"tokens":26,"detection_questions":26,"kind_questions":26,"requests":1}` and exits 0. The user reports that the live request for the same text billed 8,092 input tokens. That number was not re-measured here. Nothing in the dry run warns that one short sentence becomes a request of that size.

## What the code and specification do

- `recognize --help` shows the shared line "Print what would be sent and stop. No key is read and no connection opens" (`crates/thinkthen/src/cli/args.rs:85`).
- `specification/channels.md:87-97` makes the same promise for every command. The plan carries `url`, `model`, `key_env`, and the exact `request` body.
- `specification/recognize.md:49-51` overrides that for recognize. The dry run reports only the token count, the two question counts, and the request count. Ticket 0080 line 68 ordered the same counts.
- `crates/thinkthen/src/cli/recognize/dry_run.rs:16-28` and `:69-80` print exactly those counts. `tokens` is `tokenize(&text).len()` (`dry_run.rs:52`, `core/recognize.rs:70`). It counts whitespace-split words with trailing punctuation peeled off. It is not a model token count.
- `relate --dry-run` already prints each split request with its `bytes` and `body_utf8` (`crates/thinkthen/src/cli/relate/dry_run.rs:13-50`, `specification/channels.md:113`).

So the output matches `specification/recognize.md`. The help text and the general rule in `specification/channels.md` both promise more than recognize delivers.

## Why it matters

Recognize is the most expensive command per word of input. It asks two questions of every word and repeats the instructions in each. A user checks the dry run before paying. The dry run shows small numbers under a field named `tokens`, and a reader takes that as cost. The request body, which would show the size, never appears.

## Fix options

1. Print the split requests as relate does, with `bytes` and `body_utf8` for each, beside the current counts. Rename `tokens` to `words`. Update `specification/recognize.md` and the `spec/recognize.md` page in the same commit. This keeps the channels promise and needs no token estimate. Ticket 0080 line 64 already bans inferring tokens from bytes, and exact bytes give the user an honest size.
2. Keep the counts only. Rename `tokens` to `words`, give recognize its own `--dry-run` help line that says it prints counts, and add a sentence to `specification/channels.md` naming recognize as an exception.

Recommendation: option 1. It keeps one meaning for `--dry-run` across commands and shows the real size before the user pays. Changing the dry-run output widens a public surface, so it needs a ticket and a second reviewer. Ian can overturn this choice.
