# 05 Sort a folder

Status: red

Verbs: `decide which`

A folder fills up with meeting notes and nobody files them. The job is to move each note into `defect`, `process`, or `other`, and to leave a note the tool could not place in `review` for a person. One file, one judgment, and the `mv` is written by the shell.

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
    thinkthen decide which defect process other \
      --by 'the kind of work this note records' \
      --min-prob 0.8 --min-gap 0.2 --replay recording/ \
      < "$path" | jq -r '.assessment.value // "unsure"'
  )
  case $label in
    defect|process|other) dest=$label ;;
    unsure)               dest=review ;;
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
  thinkthen decide which defect process other \
    --by 'the kind of work this note records' \
    --min-prob 0.8 --min-gap 0.2 --replay recording/ \
    < "$path" | jq -r '.assessment.value // "unsure"'
)
case $label in
  defect|process|other) dest=$label ;;
  unsure)               dest=review ;;
  *) printf 'unknown label %s\n' "$label" >&2; exit 2 ;;
esac
mv -- "$path" "$root/$dest/"
SH
chmod +x "$work/file-one"

find "$work/inbox" -type f -name '*.txt' -print0 \
  | xargs -0 -n 1 -P 4 "$work/file-one" "$work"

find "$work/review" -type f -print0 | xargs -0 -n 1 basename | mustmatch "2026-03-06-note.txt"
```

The recording under `recording/` does not exist yet.

## What this demo decides

- **The dropped `--files0-from` is not missed, and dropping it was right.** `find` already selects files, `sort -z` already orders them, and `xargs -0 -P` already runs them four at a time. Putting file reading inside the judge would have bought one flag and turned a judging tool into a reading tool, with a size policy, a binary-file policy, and a non-UTF-8 path policy to write. The loop is shorter than that list.
- **What is missed is a budget, and the loop has no lever for one.** `--max-requests` bounds one run of one stream command. A folder of four hundred notes is four hundred runs, and no flag caps the whole job. The loop can count its own calls, which is honest but means every user writes the same counter. Smallest fix: no new flag in the tool, and the help for the stream verbs says plainly that a per-file loop is outside every limit in records.md. This is the one place where a stream verb would have paid for itself.
- **`--jobs` is unreachable from a loop, and `xargs -0 -P` replaces it.** The parallel block gets the same effect with a tool people already have. Smallest fix: the help shows the `find -print0 | xargs -0 -n 1 -P 4` line next to `--jobs`, so a user who has files rather than records knows the shape.
- **`--min-gap` mattered more than `--min-prob` here.** `process` and `other` are neighbours, and a note about a budget can sit between them. The gap is what sent the vague note to `review`.
- **`xargs` interleaves standard error.** Four parallel judgments write diagnostics into one stream with nothing to tie a line to a file. Nothing in the specification is wrong; the demo notes that a script that needs per-file diagnostics writes them itself, inside `file-one`.
