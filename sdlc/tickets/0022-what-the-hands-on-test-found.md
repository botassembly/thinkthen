---
flow: build
priority: 46
opens: crates spec specification demos sdlc/ratchet.json
---

# 0022: What the hands-on test found

Status: ready

## Outcome

Every finding of the hands-on test of 2026-09-19 is closed: each refusal names the rule that fired, each page says what the binary does, and the papercuts are gone. No answer, exit code, request, or recording changes except where a line below says so.

## Current Facts

A second agent used the release binary as a careful stranger would, with no key and no live call, against committed recordings and a throwaway server on this machine. `sdlc/issues/2026-09-19-hands-on-test-pass-one.md` holds its full report, with the command that shows each finding. It found no wrong answer, no leak, no crash, and no hang. The threshold table, the three pinned question digests, the resume from `--cache` with zero requests, the stop on a closed pipe, the record size limit, the proxy rule, the terminal notice, and the secrecy of the key and the evidence over 21 runs all held. It found one wrong message, three places where a page disagrees with the binary, and six papercuts. `sdlc/issues/2026-09-19-small-leftovers-from-the-security-ticket.md` holds three more small items from the review of ticket 0019, and the first of them is finding 1 here.

## Scope

1. **An address that is refused names its rule.** A port past 65535, an empty port, a query, a fragment, and a bad scheme each get their own sentence. Today all five print the sentence about the scheme and user information. `backends.md` states the four rules and says the message names the rule.
2. **`recording.md` says what replay does with a stray file.** The binary reads only the entry it needs, so a file that is no entry beside a good entry is ignored. The page says exit 5. The binary is right, and the page changes.
3. **`recording.md` says what the foreign-schema refusal prints.** The sentence names the schema this version reads and never repeats the schema the file named. The page says it names no schema.
4. **The host is written in lower case, as the scheme is.** `http://LOCALHOST:1234` and `http://localhost:1234` then share one recording digest, which is the goal `backends.md` already states. No committed recording holds a host in mixed case, and a test proves that every committed recording still replays. This overturns one choice of ticket 0019, which kept the host's case for fear of changing digests.
5. **A reader of JSON names what it was reading.** A question file that is empty, that starts with a byte-order mark, or that holds a trailing comma says "the question file", and a whole document under `--field` says "the input". Today all say "the record". The message gives the line and the column, as a recording entry's message already does.
6. **`--threshold -0.1` is refused in the tool's own words.** Today the argument parser refuses it and advises `--`, which would turn the number into the question.
7. **An option on a command that does not take it is refused in the tool's own words,** such as `--quiet` on `score` and `--raw` on `decide`. Today the argument parser advises `--`, which would turn the option into a level. The message says which commands take the option.
8. **The pages say that `--quiet` is refused in record mode.** One clause in `channels.md` and one in `decide.md`. The binary's message is already right.
9. **"1 record finished".** The stop line uses the singular at one.
10. **A how-to that reads a transform says so under its inputs.** Pages 12, 13, 25, 28, and 41 reach `../../transforms/`, and a reader who copies one folder gets a missing file. One sentence in each page's input section names the transform folder. `sdlc/scripts/demos` checks that a page which reads `../../transforms/` says so.
11. **A pointer typed on the command line is checked for control characters** before any message repeats it, with the check ticket 0013 wrote for labels.
12. **A transport failure that can never succeed is tried once.** A header the HTTP library refuses is not retried.

Excluded: any new option, and the HTTP-date form of `Retry-After`.

## Acceptance

- One test per item pins the exact sentence or the exact page line.
- The key and the evidence never appear in any new message, and the shared secrecy test covers the new paths.
- The pinned `decide` digest holds, and every committed recording still replays.
- The ratchet equals the measured total, and the commit that raises it says what grew and why.
- The whole ladder is green, and a second agent reviews the public surface change.
