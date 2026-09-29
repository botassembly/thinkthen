# The new-user stumble register

Status: Open. This page stays open until launch. Add a row when a stumble is seen. Close a row when its fix lands, and name the commit.

Ian asked on 2026-09-20 for one list of every place a new user stumbles, so that the documents and the tool fix each one before the public push. A stumble is anything that stops a newcomer in the first hour with no clear next step.

Rows 1 to 8 were observed by command on 2026-09-20. An agent played a new user with an empty environment, no key, and the address pointed at a closed local port, so nothing left the machine. The binary was built at 01:26 that day, before the last code commit, so a message may have changed since. Rows 9 to 14 come from the two coverage passes filed the same day.

| No. | The stumble | What was seen | The fix | Kind |
| --- | --- | --- | --- | --- |
| 1 | The README never says how to install | No `cargo install`, no download, no build step in 83 lines | Source-checkout install commands already existed; Quick Fix qf-readme-first-run moves them above the first example. Closed at `90376c8f` after independent review | Page |
| 2 | The README never says where a key comes from or what a judgment costs | `THINKTHEN_API_KEY` is named and nothing else | Key source and backend guidance landed in `e63efd45`. Quick Fix qf-readme-first-run adds TypeSafe's publicly advertised input-token price with a current-source link; actual account terms may differ. Closed at `90376c8f` after independent review | Page |
| 3 | A guessed verb gets a generic refusal | `thinkthen grep`, `if`, `classify`, `tag`, `split`, and `summarize` all print `unrecognized subcommand` and the usage line | A hint table. `grep` names `filter`. `if` names `decide`. `classify` and `switch` name `choose`. `sort` names `rank`. `summarize` and `rewrite` say the tool writes no text and name a neighbor. The headline sells `if`, `grep`, and `sort`, so people will type them | Message. Closed on 2026-09-27: ticket 0153, code `3e8ad635` |
| 4 | Question and evidence swapped | `--dry-run` prints the swapped request and says nothing | Implemented candidate 0274: a one-document `decide`, `choose`, `tag` or `score` dry run with terminal stdout says which request field holds the evidence and which holds the question, on stderr before JSON appears. The coordinator approved this adjustment to “above the JSON” so stdout stays one parseable plan; redirected plans stay quiet. Focused PTY, piped JSON and zero-request proof are in the 0274 build record. Pending fresh code review and root closure | Message |
| 5 | A CSV file piped into `--jsonl` | `the record is not valid JSON`, stopped at record 1 | A hint that names `--lines` for plain text and the CSV recipe. The roadmap holds CSV reading. Ian's tagging demos both start from a CSV, and a real CSV with quoted commas cannot be converted by the base system alone, so the hold deserves a second look | Message, then flag. Closed on 2026-09-27: ticket 0153, code `3e8ad635` |
| 6 | A file path given as a second argument | `unexpected argument 'some-file.txt'` and the usage line | A hint: evidence comes on standard input or through `--input FILE` | Message. Closed on 2026-09-27: ticket 0153, code `3e8ad635` |
| 7 | The README never lists the exit codes | It says the exit code works in an `if`. The table lives in the specification and the help | Quick Fix qf-readme-first-run adds a table before the language list using the current command-specific codes, including partial failures and signals. The old four-outcome wording is stale. Closed at `90376c8f` after independent review | Page |
| 8 | `decide --help` runs 97 lines and opens with prose | A runnable example sat in the middle | Fixed by Quick Fix qf-command-help-current: two runnable examples now precede the long explanation, while short help keeps its summary. The compiled-help test and `spec/decide.md` pin the order. The register stays open for its other rows | Message |
| 9 | Matching against a list stops at 255 entries | `choose` and `find` refuse more | A recipe that makes a cheap first cut with `grep` or a database | Page |
| 10 | Text arrives as paragraphs, sentences, and functions, and the verbs read lines | No flag splits text | Closed by accepted 0241 (`db7e2418`), reconciled in `2026-09-29-first-hour-criterion-reconciliation.md`: the paragraph recipe uses `awk -v RS=`, `jq` and `filter`; its catalog names sentence and code splitters | Page |
| 11 | Many labels come back as true or false fields | A list takes one `jq` line | Closed by the shipped `tag` alternative: its contract returns a label array and demo39 pins the list. Reviewed reconciliation: `2026-09-29-first-hour-criterion-reconciliation.md` | Flag or page |
| 12 | Each agent host reads exit codes its own way | One host blocks a tool call on exit 2. This tool says no with exit 1 and uses exit 2 for a usage error | Closed by accepted 0241 (`db7e2418`): the replayed host guard maps allow, ask and deny and denies judge failures. Its build record checks the host-hook mapping | Page |
| 13 | A control loop cannot pay for a process on every step | The long-lived loop landed in ticket 0024. No how-to shows it | Closed by accepted 0241 (`db7e2418`): the replayed long-lived-loop recipe feeds three steps through one `choose` coprocess with `--batch 1` | Page |
| 14 | The tool reads text only | A screen or an image has to become text first | Closed by accepted 0241 and current refusals page: screens, images and audio become text first. Reviewed reconciliation: `2026-09-29-first-hour-criterion-reconciliation.md` | Page |
| 15 | The `cost` transform fails on a `--lines` run | Seen by command on 2026-09-20. A `--details` row from `--lines` has a string for `input`, and `transforms/cost/cost.jq` reads `$row.input.id`. jq stops with `Cannot index string with string "id"`. An object row works | Read the id only when `input` is an object, and add a text-record case to the transform's example | Defect. Closed on 2026-09-27: ticket 0044 fixed it in commit `ce96895b`, and how-to 28 runs a text record through the transform |
| 16 | A script written for `decide --details` breaks on `choose --details` | A builder hit it in experiment 206. `decide` gives `answer.probability`. `choose` gives `answer.pick`, `answer.probabilities`, and `answer.confidence`, as `specification/result.md` says. The shapes are right and nothing warns a reader that they differ | The result reference now opens its detailed view with all five current answer-kind shapes and tells scripts to read value for the judgment. Closed at reviewed `f6e22242` | Page |
| 17 | Nothing reports or holds a request rate | Experiment 206 measured it three times from this machine. A width of 4 sent about 1,270 to 1,320 requests a minute against a documented 1,200. A width of 3 sent about 980. The default width and the vendor's limit are both invisible to the user | Reference criterion closed: `specification/records.md` gives default width, vendor guidance and the named historical measurements. A measured-rate summary remains an optional later idea; register30 separately tracks pacing. Reviewed reconciliation: `2026-09-29-first-hour-criterion-reconciliation.md` | Page, maybe flag |
| 18 | A new user waits about a day for a key | Ian confirmed on 2026-09-20 that the vendor had a waitlist and his key took about a day | The README's shipped replay example needs no key and no network. TypeSafe's 2026-09-15 announcement described early access and a waitlist; its current public home page does not establish whether a new signup still waits. Keep current wait time and registry onboarding open | Page and packaging |

## Messages that already work

These were checked the same way and need nothing: no key set, the backend unreachable, no question, `filter` with no framing, `choose` with one option, `--field` with no leading slash, `--threshold 90`, and the `set -e` warning in the help.

## No demo splits one file into piles by a label

Ian asked whether splitting a file into two or three files by a choice is a demo. The capability is there. It was verified offline by replaying the recording of how-to 21:

    thinkthen choose 'Which of these actions should be taken next?' --jsonl --field /state --options /actions --details --input steps.jsonl --replay recording/ \
      | jq -r '[.value, (.input|tostring)] | @tsv' \
      | awk -F'\t' '{print $2 > ($1".out.jsonl")}'

It wrote one file per label. How-to 04 splits one file into three piles by yes, no, and unsure. How-to 02 moves files in a folder by label. No how-to splits one file by a `choose` label. Ian wants this shown. It needs a page and no code.

## Marketing notes for 0.1, 2026-09-29

- Rows 1, 2 and 7 are closed by reviewed Quick Fix `90376c8f`: the source-checkout install section now precedes usage, a dated link cites TypeSafe's public price, and the exit-code table follows the current channel contract. These do not promise registry installation. R-universe and Windows qualification remain future channels under `mktg/sdlc/planning/2026-09-29-thinkthen-0-1-punch-list.md`.
- Row 18's replay half was already done. TypeSafe's [2026-09-15 announcement](https://typesafe.ai/blog/introducing-system-one-models-and-jev) described a waitlist, but the [public home page](https://typesafe.ai/) checked 2026-09-29 gives no current wait time. Do not publish the old one-day observation as present guidance.
- After 0.1 ships, marketing runs a clean-machine install of each package from its registry. New stumbles from that run go in this register.
