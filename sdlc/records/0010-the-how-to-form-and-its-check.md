# Record 0010: The how-to form and its check

- Ticket: `sdlc/tickets/0010-the-how-to-form-and-its-check.md`
- Branch: `ticket/0010-how-to-form`
- Written: 2026-09-19

## What landed

ADR 0011 rules that a green demo is the how-to and the test at once. This ticket makes that a check, puts demo 01 in the form, and adds two how-tos that need only `decide`.

### The check

`sdlc/scripts/demos` gained sixteen lines, run for every page whose status line reads `Status: green` and skipped for every red one. It refuses three faults.

| Fault | Message |
| --- | --- |
| The first `# ` heading does not start with "How to" | `is green, and its title does not start with "How to"` |
| No line reads `## What can go wrong` | `is green, and it has no "What can go wrong" section` |
| The index beside the folders does not hold the folder name | `is green, and README.md lists no NN-name` |

The three checks run before the recording-folder check and before any block, so a page out of form never runs. The check reads the index at the root of the folder it was pointed at, which is `demos/README.md` for the real run and a fixture index for each test.

### The pages

- `demos/01-refund-gate/README.md` is rewritten as "How to gate a script step on a yes/no answer". Every block is the block it held before, and the recording was not touched. The closing "What this demo decides" section left. One finding that asks for a change went to `demos/FINDINGS.md`, and the confirmations left with the section.
- `demos/19-no-or-could-not-ask/` is "How to tell "no" from "could not ask" in a script". It teaches `case $?` with `set -e` off, the `&& rc=0 || rc=$?` capture for a script with `set -e` on, what a bare `decide` does under `set -e`, a replay miss read through the `*` branch, the same miss read through an `if` with an `else`, and the wording where yes permits the action.
- `demos/27-test-with-no-network/` is "How to test a script with no network". It holds `triage.sh`, the script under test, and teaches `--record`, `--replay` with both variables unset, and a replay miss at exit 5 with the digest in the message.
- `demos/README.md` is the how-to index, in the groups and order of `sdlc/planning/documentation-plan.md`. It lists every folder that exists and no folder that does not.
- The root README names the three kinds of page, links the index, and names evals on its first screen. `AGENTS.md` gained one line.

## Each acceptance bullet and what proves it

| Bullet | Proof |
| --- | --- |
| The spec rung prints three green demos with the key unset and touches no network | `env -u THINKTHEN_API_KEY -u THINKTHEN_BASE_URL sdlc/scripts/spec`, exit 0, `demos: 3 green, 12 red`. The rung unsets both variables itself, and every `thinkthen` command on the three pages carries `--replay` |
| A test proves that the check refuses each of the three faults and passes a page in form | `crates/thinkthen/tests/demo_runner.rs`, three new tests over three new fixture roots. Each asserts exit 1, the message, and that no block ran. `a_green_page_runs_against_its_own_files_and_passes` is the page in form. A fourth fixture page, `demos/02-red-and-out-of-form`, is red and out of form, and the same test counts it as skipped |
| The recordings hold no key | A case-insensitive search of `demos/*/recording/*.json` for `apikey_`, `authorization`, and `bearer` returns 0, 0, and 0 |
| The live tokens spent are in `sdlc/live-tokens` | Two runs of `sdlc/scripts/live`, 643 input tokens for demo 19 and 318 for demo 27. The ledger reads 2772 of 476000000 |
| The whole ladder is green | `install`, `lint`, `test`, and `spec` each exit 0 with both variables unset |

## Red first

The three tests were written and run before the check existed. They failed with `left: Some(0), right: Some(1)` and the runner's own line, `demos: 1 green, 0 red`, which is the runner passing a page it should have refused. The check then made them pass, and the real demos folder failed next with `01-refund-gate/README.md is green, and its title does not start with "How to"`, which is the check catching the page this ticket was written to fix.

## Choices the ticket left open

- **The fixture pages take the form too.** Every green fixture page under `crates/thinkthen/tests/fixtures/` was retitled and given a traps section, and each fixture root gained an index. The alternative was to run the form check after the recording check so the older fixtures never reached it, and that would have made the order of the checks load-bearing.
- **The index check is a search for the folder name.** A green folder passes when `README.md` holds the string `NN-name/`, which every markdown link to the folder carries. A stricter reader of the table would be more code for the same answer.
- **Demo 19 needs no second recording folder for its miss.** It asks a question its own folder never recorded, so the miss is real and the page holds one folder.
- **Demo 27's `--record` step is shown and not run.** `mustmatch` runs a block only when the block pipes into `mustmatch`, so the recording command sits in an `sh` block that the gate skips. The blocks that follow assert on the file the command wrote.
- **`triage.sh` passes its extra arguments through.** The page points the script at a recording with `sh triage.sh report.txt --replay recording/`, so the script holds no replay logic and the page's `--replay` always names a folder that exists.
- **Several blocks write `set +e`.** `mustmatch` runs a block with `errexit` on, so a block that reads a non-zero code has to turn it off. That matches what the page teaches, because `case $?` needs `set -e` off.
- **`demos/FINDINGS.md` gained one row and lost nothing.** The row about the default cut was already there. The new row records that every page reads its evidence by redirect and that `--input FILE` arrives with the records slice.
