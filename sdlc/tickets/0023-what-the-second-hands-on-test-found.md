---
flow: build
priority: 46
opens: crates spec specification demos README.md sdlc/ratchet.json
---

# 0023: What the second hands-on test found

Status: ready

## Outcome

Every finding of the second hands-on test of 2026-09-19 is closed. `filter` and `rank` name themselves in every message and in their help, the pages say what the binary does, and the README no longer tells a newcomer something that stopped being true. No answer, exit code, request, or recording changes.

## Current Facts

A second agent used the release binary as a careful stranger would, after tickets 0014, 0020, and 0021 landed, with no key and no live call. `sdlc/issues/2026-09-19-hands-on-test-pass-two.md` holds its full report, with the command that shows each finding. It found no wrong answer, no leak, no crash, and no hang. Byte-for-byte output, order under `--jobs` 1, 4, and 32, exact ties, the cut at its edge, `--top`, the stop at a failed record, `--cache` with zero requests on a second run, a `decide` recording answering `filter` and `rank` with zero requests, secrecy over 20 runs, 4,000 records in 9.5 MB, and every block of how-tos 03, 06, 43, 40, and 41 all held. It found two wrong messages, four places where a page disagrees with the binary, and four papercuts. The pass-one issue report holds the earlier findings; tickets 0022 and 0029 through 0033 closed six of them.

## Scope

1. **`filter` and `rank` name themselves when they refuse a question file of the wrong kind.** Today the message says "the command is `decide`", which the user never typed. The message names the command that was typed and says it reads a `decide` question.
2. **`--input` naming a directory says so.** Today the message blames standard input, which was never read, and adds a "stopped at record 1" line for a failure that framed no record. Every command shares the fix.
3. **The help of `rank` promises no input order.** The strings under `--lines`, `--jsonl`, and `--jobs` are shared with commands where they are true. `rank` gets its own: the order is by probability, and ties keep input order.
4. **`demos/README.md` and `README.md` say what is built.** The first still says only `decide`, `choose`, and `score` are built. The second still says the landed code speaks an earlier grammar. Both sentences go. `sdlc/scripts/pages` checks that the sentence naming the built commands matches the top-level help.
5. **The long help of `--field` on `filter` and `rank` describes record mode alone,** because neither command reads one whole document.
6. **`result.md` says what `value` is on a `filter` row and what `question.verb` means.** `value` is the boolean the cut produced, and `filter` keeps the record when it is `true`. `question.verb` names the kind of question and never the command, so a `filter` row and a `rank` row both read `decide`.
7. **The plan of `rank` names no source for `threshold`,** because `rank` takes none. `channels.md` says that the plan names the settings the command takes.
8. **A record that is not UTF-8 is called "the record".** Today the message says "the evidence", which under `--field` is a smaller thing than the line that holds the bad byte.
9. **`--top` is refused in the tool's own words on every bad value,** as ticket 0022 does for `--threshold`: a negative number, a word, an empty value, and `--top` on `filter`.
10. **Page 43 shows the question near the top.** The cold reader understood the page in thirty seconds and could not tell which way the question points, because the page never shows the text of the convention file. The README features this page, so the first block shows the question file's two or three lines before it runs `filter`.

Excluded: any new option.

## Acceptance

- One test per item pins the exact sentence or the exact page line.
- The key and the evidence never appear in any new message, and the shared secrecy test covers the new paths.
- The pinned `decide` digest holds, and every committed recording still replays.
- Page 43 stays inside the limits of ADR 0016.
- The ratchet equals the measured total, and the commit that raises it says what grew and why.
- The whole ladder is green, and a second agent reviews the public surface change.
