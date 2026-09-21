# How to lint a change by meaning and fail the build

Status: green

Verbs: `filter`

A house rule that no linter can check is checked by asking one yes/no question of every changed hunk. `filter` keeps the hunks that break it, the kept hunks are the report, and the build step fails when any came back.

The question is “Does the changed code break the house rule?” True means the change holds money in a floating-point number. False means it holds no money, or holds money only as whole pence in an integer.

```bash
set -euo pipefail

thinkthen filter @convention.json --jsonl --replay recording/ < hunks.jsonl \
  | jq -r '.file' \
  | mustmatch "billing/invoice.rs
checkout/cart.ts"
```

Two of the five hunks break the rule. The other three add an import, add an import, and count late orders, and none of them touches money.

## Input

`hunks.jsonl` holds five made-up changed hunks, one JSON record each, with `file` and `hunk`. `clean.jsonl` holds the three that pass. `convention.json` is the house rule as a question file, and `recording/` holds the five live exchanges the page replays, made by `record.sh` through `sdlc/scripts/live`. `--replay` answers from those files, so every command here reads no key and costs nothing.

The made-up house rule: money is held as a whole number of pence in an integer, never in a floating-point number.

One line cuts the fixture out of a real branch, one record per hunk:

```sh
git diff -U0 origin/main -- '*.rs' '*.ts' |
  awk '/^\+\+\+ /{f=substr($2,3)} /^@@/{print "\f" f "\t" $0; next} /^[+-][^+-]/{print}' |
  jq -Rsc 'split("\f")[1:][] | split("\t") as [$f, $rest] | {file: $f, hunk: ($rest | rtrimstr("\n"))}'
```

## Step 1: put the rule in a question file

The rule is the question, the two sentences that say what a yes and a no mean, the cut it was tuned at, and `on`, which sends the hunk and nothing else. The command names the file and repeats none of it.

```bash
set -euo pipefail

jq -S -c 'keys' convention.json \
  | mustmatch '["decide","false","on","threshold","true"]'
jq -c '{threshold, on}' convention.json \
  | mustmatch '{"threshold":0.7,"on":"/hunk"}'
```

A rule worth trusting is tuned against hunks a person already judged, the way [how-to 41](../41-tune-a-question-file/) tunes one, and the cut is picked the way [how-to 13](../13-pick-a-threshold/) picks one.

## Step 2: read the report

A kept hunk comes back byte for byte as it went in, so the report is the change itself and never a summary of it.

```bash
set -euo pipefail

thinkthen filter @convention.json --jsonl --replay recording/ --input hunks.jsonl \
  | head -1 \
  | mustmatch '{"file":"billing/invoice.rs","hunk":"@@ -41,6 +41,9 @@ impl Invoice {\n+    pub fn total(&self) -> f64 {\n+        self.lines.iter().map(|line| line.price * line.quantity as f64).sum()\n+    }"}'
```

An author reads that as the diff. A coding agent reads it as the change to make.

## Step 3: fail the build

`lint.sh` is the build step. It runs the command above, prints the kept hunks, and exits 1 when there are any.

```bash
set -euo pipefail

sh lint.sh hunks.jsonl >/dev/null && dirty=0 || dirty=$?
sh lint.sh clean.jsonl && clean=0 || clean=$?
printf '%s %s\n' "$dirty" "$clean" | mustmatch "1 0"
```

A clean change prints nothing and exits 0, which is what a build step is for.

## What can go wrong

- **A paid request for every hunk.** A branch with three hundred hunks is three hundred requests. Cut the diff to the files the rule is about first, as the line above does with `-- '*.rs' '*.ts'`.
- **A cut nobody measured.** `0.7` here was read off five made-up hunks. Tune it on hunks a person judged before a build fails on it.
- **A rule the model cannot see.** The hunk is all that leaves the machine, so a rule about the file it sits in, or about a function two hundred lines above, cannot be answered from it.
- **A run that stops.** A failed hunk ends the run at exit 4 or 5 and prints a prefix, so a build that reads an empty report as a clean change would pass a branch nobody judged. `lint.sh` exits on the first failure, because `set -e` is on.
- **A band.** `filter` takes a single cut. [How to pick a threshold from labeled cases](../13-pick-a-threshold/) measures the coverage trade, and [the triage pipeline](../16-triage-pipeline/) shows three policy outputs.

## Related how-tos

- [How to keep only the records that match a meaning](../03-grep-for-meaning/) is the plain introduction to `filter`.
- [How to tune a question file and use the same file in the gate](../41-tune-a-question-file/) earns the wording in `convention.json`.
- [How to pick a threshold from labeled cases](../13-pick-a-threshold/) earns the cut.
- [How to test a script with no network](../27-test-with-no-network/) is how this page runs in a gate.
