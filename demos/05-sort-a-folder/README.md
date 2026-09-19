# 05 Sort a folder

Status: red

Verbs: `choose`

A folder fills up with meeting notes and nobody files them. The job is to move each note into `defect`, `process`, or `other`, and to leave a note the tool could not place in `review` for a person. One file, one judgment, and the `mv` is written by the shell.

Every number in an expected output on this page is illustrative until a recording exists. No block asserts on a probability.

## Input

`inbox/` holds five short notes. The demo copies them into a scratch directory first, so the page can run twice and the repository stays clean.

## The loop

`find` produces NUL-delimited paths, because a filename may contain a newline. The paths are written to a file first, so `find` failing is a reason to stop rather than an empty loop that looks like success.

```bash
set -euo pipefail
work=$(mktemp -d)
trap 'rm -rf -- "$work"' EXIT

cp -R inbox "$work/inbox"
mkdir -p "$work/defect" "$work/process" "$work/other" "$work/review"

find "$work/inbox" -type f -name '*.txt' -print0 | LC_ALL=C sort -z > "$work/paths.nul"

while IFS= read -r -d '' path; do
  label=$(
    thinkthen choose 'What kind of work does this note record?' defect process other \
      --raw --threshold 0.8 --input "$path" --replay recording/
  ) && rc=0 || rc=$?
  case $rc in
    0) dest=$label ;;
    3) dest=review ;;
    *) printf 'choose failed on %s: %d\n' "$path" "$rc" >&2; exit "$rc" ;;
  esac
  case $dest in
    defect|process|other|review) ;;
    *) printf 'unknown label %s for %s\n' "$label" "$path" >&2; exit 2 ;;
  esac
  mv -- "$path" "$work/$dest/"
done < "$work/paths.nul"

for dir in defect process other review; do
  printf '%s=%d\n' "$dir" "$(find "$work/$dir" -type f | wc -l)"
done | mustmatch "defect=2
process=2
other=0
review=1"
```

The note with no example, no query, and nothing written down is the one in `review`. That is the answer a person would give too.

## The same job in parallel

`xargs -0 -P` runs several judgments at once and keeps the NUL boundary. The per-file work moves into a small script, because `xargs` runs a command and not a shell function.

```bash
set -euo pipefail
work=$(mktemp -d)
trap 'rm -rf -- "$work"' EXIT

cp -R inbox "$work/inbox"
mkdir -p "$work/defect" "$work/process" "$work/other" "$work/review"

cat > "$work/file-one" <<'SH'
#!/usr/bin/env bash
set -euo pipefail
root=$1
path=$2
label=$(
  thinkthen choose 'What kind of work does this note record?' defect process other \
    --raw --threshold 0.8 --input "$path" --replay recording/
) && rc=0 || rc=$?
case $rc in
  0) dest=$label ;;
  3) dest=review ;;
  *) exit "$rc" ;;
esac
mv -- "$path" "$root/$dest/"
SH
chmod +x "$work/file-one"

find "$work/inbox" -type f -name '*.txt' -print0 \
  | xargs -0 -n 1 -P 4 "$work/file-one" "$work"

find "$work/review" -type f -print0 | xargs -0 -n 1 basename | mustmatch "2026-03-06-note.txt"
```

Four hundred notes are four hundred runs. Nothing in ADR 0007 bounds that. `jobs` in the configuration file bounds the requests inside one run, and a per-file loop makes one request per run, so the setting never applies. A loop that must not spend more than a hundred requests counts them itself.

The recording under `recording/` does not exist yet.

## What this demo decides

- **The demo confirms `--raw` in a loop and the exit code beside it.** Demo 02 found the two-`case` shape and this page reuses it without change. The exit code is the only thing that separates an unresolved pick from an empty label, and in a per-file loop the exit code is right there.
- **A per-file loop sits outside every request budget in the surface.** No flag caps a job made of many runs, and `jobs` in the configuration file bounds only the inside of one run. The demo asks for no new flag. It asks that the help for the record flags say plainly that a per-file loop is outside the budget, and show `find -print0 | xargs -0 -n 1 -P 4` next to the `jobs` setting.
- **The demo could not read the margin between two neighbouring options.** `process` and `other` are neighbours, and a note about a budget sits between them. The old page used `--min-gap` to send such a note to `review`. With no per-option probabilities in the result, neither the tool nor the script can see the margin now, so the vague note lands in `review` only if the winner itself falls under 0.8. That is a weaker rule than the job wants.
- **Reading files stays out of the tool, and the demo confirms it.** `find` selects, `sort -z` orders, `xargs -0 -P` parallelises. Putting file reading inside the judge would buy one flag and cost a size policy, a binary-file policy, and a non-UTF-8 path policy.
- **`xargs` interleaves standard error with nothing tying a line to a file.** Nothing in the surface is wrong. A script that needs per-file diagnostics writes them itself, inside `file-one`.
