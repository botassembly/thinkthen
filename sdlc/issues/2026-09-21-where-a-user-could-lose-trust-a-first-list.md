# Where a user could lose trust: a first list

Status: Open

Ian asked on 2026-09-21 what else could erode a user's trust through quality or unexpected behavior, after the cache question in `2026-09-21-the-disk-cache-is-never-on-unless-the-user-names-a-folder.md`. This is the marketing side's list, ranked by how badly each one would hurt. It authorizes nothing. Each item names what was observed and what would answer it.

## The bill

1. **Nothing caps a user's spend.** `sdlc/scripts/live` caps this project's own runs, and a user has no such guard. `filter` over the wrong file sends a request per line at up to 32 wide. The options today are `--jobs`, `--timeout`, and `--max-retries`, read from `filter --help` on 2026-09-21. A `--max-requests N` that refuses before the first request, and a `--dry-run` that prints the request count and a token estimate, would answer it. This is the first thing a stranger's blog post would complain about.
2. **Ctrl-C does not stop the bill at once.** The command has no interrupt handling, and the database experiment saw 16.6 s of paid work after Ctrl-C. The engine plan's cancel token answers it. The pages promise exactly this: no new request starts.
3. **`rank --top 5` judges every record.** The specification says so. A user reads `--top` as "cheaper". The help line for `--top` says it saves nothing, and the how-to repeats it.

## The answer changes

4. **`jev-latest` moves.** The same command gives another answer after the vendor changes the model, and a threshold earned on one model does not carry. `--details` names the model that answered. The how-to on picking a threshold teaches pinning `--model` wherever a threshold matters.
5. **The same request can answer differently twice.** Nothing here has measured how much a probability moves between two identical live requests. Near a threshold, that flips an answer. One small capped probe measures it: one hundred requests, sent twice. The pages then state the number, and the band is the answer to it.
6. **Neighbors move an answer inside `annotate`.** Measured: one of six borderline answers moved when questions joined the set, by at most 0.04. `specification/annotate.md` says it. The how-to says to keep a set fixed between runs that get compared.
7. **Wording moves an answer.** Two criteria sentences moved spam accuracy from 96.8 to 98.0 percent. A question is code, and a changed question needs its labeled cases rerun. The transforms exist for this, and the first-week pages teach it.

## The stream

8. **One bad record ends a run.** The how-to on resuming shows it, and `--cache` is the resume. A nightly job that dies at record 40,000 on one malformed line still surprises people. A user will ask for a way to skip a bad record, write it to a side file, and go on. The builder decides whether that enters. The pages say plainly what happens today.
9. **The vendor's two-decimal rounding.** Ticket 0038 fixed the refusal. It leaves a fact worth stating: a probability is one of a hundred values, a threshold of 0.995 can never be passed, and many records tie in `rank`. The threshold page says it, and `rank` names its tie rule.
10. **Exit code 1 means no.** Under `set -e` a plain `thinkthen decide` that answers no ends the script. It is the right design, the same as `grep`, and the first how-to shows `if` around it.

## What leaves the machine

11. **The text goes to a vendor.** `--dry-run` and `--field` are the honest answer, and both exist. No page yet says what the vendor keeps. The site's privacy paragraph links the vendor's own policy and claims nothing on its behalf.
12. **A planted fact moves an answer.** Measured on twenty made-up messages: planted commands moved the probability by 0.04 or less, and two planted false facts moved it by 0.57 and 0.24. The tool cannot tell a planted claim from a true one. No page ever says hostile input cannot steer it.

## A user's first minutes

13. **A new user waits about a day for a key.** The first example must run under `--replay` from a recording the release ships.
14. **A person reads a probability as the chance of being right.** Calibration was measured on one set, in one bucket: 96 percent right at a stated 95 percent, on 97 messages. No page says "calibrated" without that number and its size.

## What Ian can overturn

All of it. Items 1, 5, and 8 ask for something new: an option, a probe, and a decision. The rest ask for a sentence on a page.
