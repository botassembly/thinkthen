# Explicit source examples retain physical locations

All ten published command examples use the shared files corpus and controlled saved replies. This executes the same scripts the files page renders, compares every original record and source coordinate to its published output, and keeps repeated-path windows as distinct occurrences. Counted loopback regressions separately prove missing inputs, unrepresentable filenames, replay and excess source rank send nothing.

```bash
set -euo pipefail
root=$(git rev-parse --show-toplevel)
work=$(mktemp -d)
trap 'rm -rf -- "$work"' EXIT
export HOME="$work/home"
unset XDG_CONFIG_HOME XDG_CACHE_HOME THINKTHEN_API_KEY THINKTHEN_BASE_URL
binary=$(command -v thinkthen)
thinkthen() { "$binary" "$@" --replay "$root/site/recordings"; }
cd "$root/specification/fixtures/files"
for script in "$root"/site/examples/learn/read-files/*.sh; do
  source "$script" > "$work/printed"
  cmp "$work/printed" "${script%.sh}.out"
done
policy=$(thinkthen find 'Which line gives the refund policy?' --input documents --unit line --model local-1)
printf '%s\n' "$policy" | jq -c '[.input,.file,.first_line,.last_line]' | mustmatch '["Customers may request a refund within 30 days.","documents/01-policy.txt",2,2]'
```
