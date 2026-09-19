# 08 Release checklist

Status: red

Verbs: `annotate`

A team publishes release notes and has four things the notes must say. The checks live in a file the team owns and edits, not in a shell script, and the same file runs before every release. Each check asks about a fact that is printed in the notes or absent from them.

Every number in an expected output on this page is illustrative until a recording exists. No block asserts on a probability.

## Input

`release-notes.txt` is one set of release notes. `checklist.json` is the saved question file: four named questions, three `decide` and one `choose`, each with its own threshold.

## Check the file before the release

`annotate --dry-run` reads the question file, validates it, and sends nothing. It runs in a lint job with no key in the environment.

```bash
set -euo pipefail

env -u TYPESAFE_API_KEY thinkthen annotate checklist.json --dry-run \
  --input release-notes.txt > /dev/null && printf 'checklist ok\n' | mustmatch "checklist ok"
```

## Run the checklist

The evidence is one text document, so the output is the object of named answers alone. There is no record to add fields to.

```bash
set -euo pipefail
work=$(mktemp -d)
trap 'rm -rf -- "$work"' EXIT

thinkthen annotate checklist.json --input release-notes.txt --replay recording/ \
  > "$work/checks.json"

jq -r 'keys_unsorted | join(",")' "$work/checks.json" \
  | mustmatch "names_upgrade_command,states_upload_limit,lists_known_issues,session_lifetime"
jq -c '.' "$work/checks.json" \
  | mustmatch '{"names_upgrade_command":true,"states_upload_limit":true,"lists_known_issues":true,"session_lifetime":"shorter"}'
```

The names come back in file order, so the checklist reads top to bottom the way the file does. A `decide` answer is a boolean, a `choose` answer is a string, and an unresolved answer of either kind is `null`.

## Read it as a checklist

One `jq` program turns the object into lines a person can read. The `decide` answers and the `choose` answer print the same way.

```bash
set -euo pipefail
work=$(mktemp -d)
trap 'rm -rf -- "$work"' EXIT

thinkthen annotate checklist.json --input release-notes.txt --replay recording/ \
  > "$work/checks.json"

jq -r '
  to_entries[]
  | [.key,
     (if .value == null then "review"
      elif .value == false then "fail"
      else "ok" end)]
  | @tsv
' "$work/checks.json" | mustmatch "names_upgrade_command	ok
states_upload_limit	ok
lists_known_issues	ok
session_lifetime	ok"
```

`session_lifetime` prints `ok` because its answer is a label and not `false`. The checklist cannot tell `shorter` from `longer` without naming the labels it wants, and the release gate below has to do that itself.

## Fail the build on a failed check

The gate is `jq -e`, so the decision to stop is made by code reading a saved result. Nothing runs because the model said so.

```bash
set -euo pipefail
work=$(mktemp -d)
trap 'rm -rf -- "$work"' EXIT

thinkthen annotate checklist.json --input release-notes.txt --replay recording/ \
  > "$work/checks.json"

jq -e '
  .names_upgrade_command == true
  and .states_upload_limit == true
  and .lists_known_issues == true
  and (.session_lifetime == "shorter" or .session_lifetime == "unchanged")
' "$work/checks.json" > /dev/null && rc=0 || rc=$?

case $rc in
  0) printf 'release notes pass\n' ;;
  1) printf 'release notes need a look\n' ;;
  *) printf 'check run failed: %d\n' "$rc" >&2; exit "$rc" ;;
esac | mustmatch "release notes pass"
```

The recording under `recording/` does not exist yet.

## What this demo decides

- **The demo confirms JSON for the saved file.** One format for the configuration, the questions, and the results means `jq` edits all three and no second parser enters the build. The old Markdown proposal needed a grammar of frontmatter, headings, and fenced blocks, and this file needs none.
- **The demo could not write a default threshold.** Three `decide` questions repeat `"0.1:0.9"` three times, and a real checklist of twenty repeats it twenty times. ADR 0007 gives the file `version` and `questions` and makes any other key an error, so a user cannot add a default and the command line has no `--threshold` for `annotate` either. The demo asks for one optional top-level `threshold` that a question overrides.
- **The demo could not tell a failed check from an unresolved one in one pass.** A bare answer is the value, so `false` and `null` are two `jq` tests and a `choose` answer needs a third. The nine-line checklist program above is what every user of `annotate` will write. The demo asks that it sit in the `annotate` help as the worked example.
- **The exit code says nothing about the answers, and that is right.** `annotate` finishes at 0 whatever the checks said. The gate is `jq -e`. That is code, and the surface never lets the model set the build's exit code.
- **No question in the file asks whether the notes are good.** Each one names something a reader can point at. That rule belongs in the `annotate` help next to the file grammar, because a saved file is exactly where a bad question gets written once and run forever.
