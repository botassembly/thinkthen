# 08 Release checklist

Status: red

Verbs: `decide run`

A team publishes release notes and has four things the notes must say. The checks live in a file the team owns and edits, not in a shell script, and the same file runs before every release. Each check asks about a fact that is printed in the notes or absent from them.

## Input

`release-notes.txt` is one set of release notes. `release-checks.md` is the saved question file: frontmatter for the defaults, one heading per question, and one small block under each heading holding the verb and the question.

## Run the checklist

All four questions travel in one request, because the backend answers several named questions over one evidence.

```bash
set -euo pipefail
work=$(mktemp -d)
trap 'rm -rf -- "$work"' EXIT

thinkthen decide run release-checks.md --replay recording/ \
  < release-notes.txt > "$work/checks.json"

jq -r '.answers | keys_unsorted | join(",")' "$work/checks.json" \
  | mustmatch "names_upgrade_command,states_upload_limit,lists_known_issues,session_lifetime"
jq -c '.answers.names_upgrade_command.assessment | {status, value}' "$work/checks.json" \
  | mustmatch '{"status":"accepted","value":true}'
jq -c '.answers.session_lifetime.assessment | {status, value}' "$work/checks.json" \
  | mustmatch '{"status":"accepted","value":"shorter"}'
```

The question names come back in file order, so the checklist reads top to bottom the way the file does.

## Read it as a checklist

One `jq` program turns the answers map into lines a person can read, and the `if` questions and the `which` question print the same way.

```bash
set -euo pipefail
work=$(mktemp -d)
trap 'rm -rf -- "$work"' EXIT

thinkthen decide run release-checks.md --replay recording/ \
  < release-notes.txt > "$work/checks.json"

jq -r '
  .answers
  | to_entries[]
  | [.key,
     (if .value.assessment.status != "accepted" then "review"
      elif .value.assessment.value == false then "fail"
      else "ok" end)]
  | @tsv
' "$work/checks.json" | mustmatch "names_upgrade_command	ok
states_upload_limit	ok
lists_known_issues	ok
session_lifetime	ok"
```

## Fail the build on a failed check

The gate is `jq -e`, so the decision to stop is made by code reading a saved result. Nothing runs because the model said so.

```bash
set -euo pipefail
work=$(mktemp -d)
trap 'rm -rf -- "$work"' EXIT

thinkthen decide run release-checks.md --replay recording/ \
  < release-notes.txt > "$work/checks.json"

jq -e '[.answers[] | .assessment.status == "accepted" and .assessment.value != false] | all' \
  "$work/checks.json" > /dev/null && rc=0 || rc=$?

case $rc in
  0) printf 'release notes pass\n' ;;
  1) printf 'release notes need a look\n' ;;
  *) printf 'check run failed: %d\n' "$rc" >&2; exit "$rc" ;;
esac | mustmatch "release notes pass"
```

The recording under `recording/` does not exist yet.

## What this demo decides

- **Markdown with frontmatter is the saved question file.** The team that owns the checks reads the file, and the prose between the headings is where the reason for a check lives. The exact grammar this demo used, which decide.md has to pin: a YAML frontmatter block for the defaults, a level-two heading per question whose text is the question name, and one fenced `yaml` block under that heading holding `verb`, `ask`, and the `options` or `levels` the verb needs. Prose anywhere else is ignored. One parser reads the frontmatter and the question blocks, so the grammar is YAML and the headings.
- **A question name is a key in JSON and a word in a `jq` path, so its characters have to be fixed.** This demo used lowercase letters, digits, and underscores. Without a rule, a heading with a space becomes `.answers["states upload limit"]` and every checklist reads worse.
- **The frontmatter carries a default pass mark and a question overrides it.** The draft puts a mark on each question and an override on the command line, with nothing in between, so a file of twelve `if` questions repeats one number twelve times. This file sets `min_prob: 0.9` once and the `which` question sets `0.8` for itself, because a four-way pick is not a two-sided decision. This bears on question 7 of the design study, the home of a pass mark.
- **`--min-prob` on the command line overriding every question is blunt, and it should stay.** In this file it would flatten a 0.9 and a 0.8 into one number, which is wrong for a release gate. It is still the right flag for one job: sweeping a file at several marks to see what moves. The help should say that is what it is for.
- **`run` prints one document and a checklist wants lines.** The `jq` program above is nine lines and every user of `run` will write it. Smallest fix: put that program in the `run` help as the worked example. No new output mode is needed, because the shape of a checklist line is a local preference.
- **No question in the file asks whether the notes are good.** Each one names something a reader can point at. That rule belongs in the `run` help, next to the file grammar, because a saved file is exactly where a bad question gets written once and run forever.
