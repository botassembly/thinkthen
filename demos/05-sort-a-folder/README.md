# How to sort files into folders by label

Status: green

Verbs: `choose`

Use this when a folder fills up with short documents and nobody files them. The job is to move each one into a folder named after its label, and to leave the ones the tool could not place where a person will see them. One file, one judgment, and the `mv` is written by the shell.

The recording under `recording/` holds the five live exchanges this page replays, one per note. No block asserts on a probability.

## Input

`inbox/` holds five short meeting notes. The page copies them into a scratch directory first, so it can run twice and the repository stays clean.

## The quick loop

`find` produces NUL-delimited paths, because a filename may contain a newline. The paths go to a file first, so `find` failing is a reason to stop rather than an empty loop that looks like success.

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
      --raw --replay recording/ < "$path"
  ) && rc=0 || rc=$?
  case $rc in
    0) dest=$label ;;
    3) dest=review ;;
    *) printf 'choose failed on %s: %d\n' "$path" "$rc" >&2; exit "$rc" ;;
  esac
  mv -- "$path" "$work/$dest/"
done < "$work/paths.nul"

for dir in defect process other review; do
  printf '%s=%d\n' "$dir" "$(find "$work/$dir" -type f | wc -l)"
done | mustmatch "defect=2
process=2
other=1
review=0"
```

With no threshold every note takes the label that led, and `review` stays empty. Nothing in that output says which notes were close.

## The mark decides how much a person sees

The same loop with `--threshold 0.8` sends every note whose winner fell under the mark to `review`.

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
      --raw --threshold 0.8 --replay recording/ < "$path"
  ) && rc=0 || rc=$?
  case $rc in
    0) dest=$label ;;
    3) dest=review ;;
    *) printf 'choose failed on %s: %d\n' "$path" "$rc" >&2; exit "$rc" ;;
  esac
  mv -- "$path" "$work/$dest/"
done < "$work/paths.nul"

for dir in defect process other review; do
  printf '%s=%d\n' "$dir" "$(find "$work/$dir" -type f | wc -l)"
done | mustmatch "defect=2
process=0
other=0
review=3"
```

Two notes name a broken thing plainly, and the model put every point of probability on `defect` for both. The other three are a warehouse call, a budget review, and a hallway chat, and none of them reached 0.8. A strict mark on a three-option list sends most of a mixed folder to a person, and that is the cost the desk is choosing.

## The careful version, in parallel

`process` and `other` are neighbours, and a note about a budget can sit between them. `--details` carries a probability for every option, so a note whose winner barely beat the runner-up goes to `review` even when it won. `xargs -0 -P` runs several judgments at once and keeps the NUL boundary. The per-file work moves into a small script, because `xargs` runs a command and not a shell function.

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
judgment=$(
  thinkthen choose 'What kind of work does this note record?' defect process other \
    --details --replay recording/ < "$path"
) && rc=0 || rc=$?
case $rc in
  0|3) ;;
  *) exit "$rc" ;;
esac
dest=$(printf '%s' "$judgment" | jq -r '
  (.answer.probabilities | to_entries | sort_by(-.value)) as $p
  | if .value == null or ($p[0].value - $p[1].value) < 0.2 then "review" else .value end
')
mv -- "$path" "$root/$dest/"
SH
chmod +x "$work/file-one"

find "$work/inbox" -type f -name '*.txt' -print0 \
  | xargs -0 -n 1 -P 4 "$work/file-one" "$work"

for dir in defect process other review; do
  printf '%s=%d\n' "$dir" "$(find "$work/$dir" -type f | wc -l)"
done | mustmatch "defect=2
process=1
other=0
review=2"

find "$work/review" -type f -print0 | xargs -0 -n 1 basename | LC_ALL=C sort | mustmatch "2026-03-04-note.txt
2026-03-06-note.txt"
```

The margin rule and the mark disagree, and both are honest. The warehouse note won `process` by a margin of 0.5 and still fell under 0.8, so the margin rule files it and the mark does not. The budget note and the hallway chat land in `review` either way. A desk picks one rule, writes it down, and measures it against labelled notes.

Four hundred notes are four hundred runs. A per-file loop makes one request per run, so nothing inside the tool bounds the whole job. A loop with a budget counts its own calls.

Every `thinkthen` line carries `--replay recording/`, so the page touches no network and reads no key. `record.sh` made the five exchanges once, through `sdlc/scripts/live`.

## What can go wrong

| Exit code | What happened | What to do |
| --- | --- | --- |
| 0 | A label was returned | Move the file |
| 2 | A usage error, or evidence that is empty | Fix the command line or skip the empty file |
| 3 | The winner fell under the mark, or the top two tied exactly | Move the file to `review` |
| 4 | The backend failed, or the adapter refused the reply | Stop the loop. The file has not moved |
| 5 | A local failure: the recording folder, the file | Stop the loop |

- An empty file is a usage error, not a label. `find` in a real inbox will meet one, so the loop needs a branch for exit 2 or a `find -size +0` filter.
- `--raw` prints nothing for an unresolved pick, so `label` is empty and `mv -- "$path" "$work//"` would fail. Read `$rc` before reading `$label`, as both loops do.
- `xargs` interleaves standard error and ties no line to a file. A loop that needs per-file diagnostics writes them inside `file-one`.
- `mv` over an existing name overwrites it. Two notes with the same basename in different subfolders will collide, and `mv -n` or a per-run folder is the answer.
- Under `set -e` a single unresolved answer ends the loop. Capture the code with `&& rc=0 || rc=$?`.

## Related how-tos

- [How to branch on a label with `choose` and `case`](../02-route-a-ticket/) is one file and the shape this loop repeats.
- [How to rate on a scale, sort by it, and test it with `jq -e`](../17-rate-and-sort/) orders a folder instead of filing it.
- [Refund gate](../01-refund-gate/) is the two-sided form.
