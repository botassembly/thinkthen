# A live probe plan: many questions in one request, tagging, and status

Status: Open. Waits for Ian's direct go-ahead for the paid calls. Nothing here has been run.

Ian asked on 2026-09-20 for tests that show the tool gets the most out of the hosted service, and for a look at what status the service can report. He named a ceiling of 50 cents. At the listed price of 0.042 dollars per million input tokens, 50 cents buys about 11.9 million tokens. The shared ledger showed a limit of 476,000,000 tokens and 429,118 charged on that day, read from the local ledger file with no key and no network.

## The door

One command, as `probes/README.md` gives it:

    sdlc/scripts/live --max-tokens 11000000 probes/NAME/job.sh

A read of the launcher and of every registered worktree on 2026-09-20 found nothing that blocks a run from the main checkout. The standing caution holds: no registered worktree goes back to an older branch. Two earlier jobs already post straight to the vendor with `curl` for a request shape the tool cannot send, so a job that sends many questions in one request has a precedent.

## What to measure, in order of value

1. **How many yes or no questions one request holds.** Send 1, 5, 10, 20, and 40 questions over one piece of evidence, and the same questions one request each. Record whether any answer moves, the billed tokens, and the wall time. This repeats the vendor's claim of twelve times cheaper and ten times faster on our own cases. It sets the size of an `annotate` file and decides whether `tag` is real. `probes/token-budget/` was written for the request ceiling and never run. It costs well under one cent at the listed price. Run it first.
2. **Tagging.** Twenty made-up posts, ten topic labels, a trusted label set fixed before any call. Three arms: a bare label, a label with a one-sentence description, and the label set with two unrelated labels added. The third arm shows whether a tag flips when its neighbors change.
3. **Status.** Call the documented models listing. On one ordinary judgment, save the names of the response headers and no values. The documents show no usage endpoint and no rate headers. This confirms it, or finds what the documents leave out. The job guesses no undocumented address.
4. **The cost of one real file.** One public text file of a few thousand lines through `filter --lines`, with the charge read from the ledger before and after. The launch copy wants this number.

## Rules the job keeps

Every case is made-up text with its trusted answer committed before the first call. The key travels only in the environment of the job. No header value, no key, and nothing derived from a key is printed or saved. The job lands through a ticket and a worktree, as tickets 0011 and 0017 did.

## What Ian can overturn

The order, the ceiling, and whether item 3 may try addresses the documents do not list.
