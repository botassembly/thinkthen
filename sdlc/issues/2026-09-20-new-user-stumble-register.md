# The new-user stumble register

Status: Open. This page stays open until launch. Add a row when a stumble is seen. Close a row when its fix lands, and name the commit.

Ian asked on 2026-09-20 for one list of every place a new user stumbles, so that the documents and the tool fix each one before the public push. A stumble is anything that stops a newcomer in the first hour with no clear next step.

Rows 1 to 8 were observed by command on 2026-09-20. An agent played a new user with an empty environment, no key, and the address pointed at a closed local port, so nothing left the machine. The binary was built at 01:26 that day, before the last code commit, so a message may have changed since. Rows 9 to 14 come from the two coverage passes filed the same day.

| No. | The stumble | What was seen | The fix | Kind |
| --- | --- | --- | --- | --- |
| 1 | The README never says how to install | No `cargo install`, no download, no build step in 83 lines | An install section above the first example | Page |
| 2 | The README never says where a key comes from or what a judgment costs | `THINKTHEN_API_KEY` is named and nothing else | One paragraph: who issues the key, the listed price, and the one address that swaps the backend | Page |
| 3 | A guessed verb gets a generic refusal | `thinkthen grep`, `if`, `classify`, `tag`, `split`, and `summarize` all print `unrecognized subcommand` and the usage line | A hint table. `grep` names `filter`. `if` names `decide`. `classify` and `switch` name `choose`. `sort` names `rank`. `summarize` and `rewrite` say the tool writes no text and name a neighbor. The headline sells `if`, `grep`, and `sort`, so people will type them | Message. Closed on 2026-09-27: ticket 0153, code `3e8ad635` |
| 4 | Question and evidence swapped | `--dry-run` prints the swapped request and says nothing | The plan says in plain words which text is the question and which is the evidence, above the JSON | Message |
| 5 | A CSV file piped into `--jsonl` | `the record is not valid JSON`, stopped at record 1 | A hint that names `--lines` for plain text and the CSV recipe. The roadmap holds CSV reading. Ian's tagging demos both start from a CSV, and a real CSV with quoted commas cannot be converted by the base system alone, so the hold deserves a second look | Message, then flag. Closed on 2026-09-27: ticket 0153, code `3e8ad635` |
| 6 | A file path given as a second argument | `unexpected argument 'some-file.txt'` and the usage line | A hint: evidence comes on standard input or through `--input FILE` | Message. Closed on 2026-09-27: ticket 0153, code `3e8ad635` |
| 7 | The README never lists the exit codes | It says the exit code works in an `if`. The table lives in the specification and the help | The four outcomes and their codes, as a table near the top | Page |
| 8 | `decide --help` runs 97 lines and opens with prose | A runnable example sits in the middle | Open the help with two examples | Message |
| 9 | Matching against a list stops at 255 entries | `choose` and `find` refuse more | A recipe that makes a cheap first cut with `grep` or a database | Page |
| 10 | Text arrives as paragraphs, sentences, and functions, and the verbs read lines | No flag splits text | A recipe page: one `awk` line for paragraphs, a named neighbor tool for sentences and for functions. The likeliest first flag request after launch | Page |
| 11 | Many labels come back as true or false fields | A list takes one `jq` line | The proposed `tag` question type, or the `jq` line at the end of the page | Flag or page |
| 12 | Each agent host reads exit codes its own way | One host blocks a tool call on exit 2. This tool says no with exit 1 and uses exit 2 for a usage error | The host hook how-to shows the three-line `case` that maps them | Page |
| 13 | A control loop cannot pay for a process on every step | The long-lived loop landed in ticket 0024. No how-to shows it | A how-to | Page |
| 14 | The tool reads text only | A screen or an image has to become text first | One line on the refusals page | Page |
| 15 | The `cost` transform fails on a `--lines` run | Seen by command on 2026-09-20. A `--details` row from `--lines` has a string for `input`, and `transforms/cost/cost.jq` reads `$row.input.id`. jq stops with `Cannot index string with string "id"`. An object row works | Read the id only when `input` is an object, and add a text-record case to the transform's example | Defect. Closed on 2026-09-27: ticket 0044 fixed it in commit `ce96895b`, and how-to 28 runs a text record through the transform |
| 16 | A script written for `decide --details` breaks on `choose --details` | A builder hit it in experiment 206. `decide` gives `answer.probability`. `choose` gives `answer.pick`, `answer.probabilities`, and `answer.confidence`, as `specification/result.md` says. The shapes are right and nothing warns a reader that they differ | The `--details` reference page opens with one table of the three answer shapes side by side | Page |
| 17 | Nothing reports or holds a request rate | Experiment 206 measured it three times from this machine. A width of 4 sent about 1,270 to 1,320 requests a minute against a documented 1,200. A width of 3 sent about 980. The default width and the vendor's limit are both invisible to the user | The reference page for `--jobs` gives the measured widths and the arithmetic. A summary line on standard error at the end of a run could name the measured rate | Page, maybe flag |
| 18 | A new user waits about a day for a key | Ian confirmed on 2026-09-20 that the vendor has a waitlist and his key took about a day | The install page says to ask for the key first. The first example runs from a shipped recording under `--replay`, with no key and no network | Page and packaging |

## Messages that already work

These were checked the same way and need nothing: no key set, the backend unreachable, no question, `filter` with no framing, `choose` with one option, `--field` with no leading slash, `--threshold 90`, and the `set -e` warning in the help.

## No demo splits one file into piles by a label

Ian asked whether splitting a file into two or three files by a choice is a demo. The capability is there. It was verified offline by replaying the recording of how-to 21:

    thinkthen choose 'Which of these actions should be taken next?' --jsonl --field /state --options /actions --details --input steps.jsonl --replay recording/ \
      | jq -r '[.value, (.input|tostring)] | @tsv' \
      | awk -F'\t' '{print $2 > ($1".out.jsonl")}'

It wrote one file per label. How-to 04 splits one file into three piles by yes, no, and unsure. How-to 02 moves files in a folder by label. No how-to splits one file by a `choose` label. Ian wants this shown. It needs a page and no code.

## Marketing notes for 0.1, 2026-09-29

- Row 2 is partly done. Quick Fix `e63efd45` gave the README the key source and the backend lines. The listed price is still missing. Ian ruled that the README links to TypeSafe's site for keys, with no launch coordination.
- Rows 1 and 7 (install section, exit-code table) are still owed in the README. The install lines must match the channels Ian ruled on 2026-09-29: R publishes through R-universe, and Windows waits on a first GitHub Actions check. The punch list is `mktg/sdlc/planning/2026-09-29-thinkthen-0-1-punch-list.md`.
- Row 18 may be stale. Ask whether TypeSafe still has a waitlist before the install page tells users to wait a day.
- After 0.1 ships, marketing runs a clean-machine install of each package from its registry. New stumbles from that run go in this register.
