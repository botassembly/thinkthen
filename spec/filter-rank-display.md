# Show filter and rank source locations

Filter and rank keep their ordinary record behavior and expose locations, scores and physical neighbors through text display flags. These examples validate local preflight and use no backend.

```bash
for verb in filter rank; do
  thinkthen "$verb" --help | grep -c -- '--line-number' | mustmatch '1'
  thinkthen "$verb" --help | grep -c -- '--scores' | mustmatch '1'
  thinkthen "$verb" --help | grep -c -- '--around' | mustmatch '1'
  printf '' | thinkthen "$verb" 'Clear?' -n --scores --around 0 --no-cache | mustmatch ''
done
```

The flags decorate text and refuse JSON details, CSV and TSV. Invalid neighbor counts refuse before a request can leave.

```bash
for verb in filter rank; do
  for flag in -n --scores '--around 0'; do
    set +e
    output=$(printf 'hit\n' | thinkthen "$verb" 'Clear?' $flag --details --no-cache 2>&1)
    status=$?
    set -e
    printf '%s\n' "$status" | mustmatch '2'
    printf '%s\n' "$output" | mustmatch like 'text display flags cannot accompany --details'
  done
  set +e
  output=$(printf 'hit\n' | thinkthen "$verb" 'Clear?' --around -1 --no-cache 2>&1)
  status=$?
  set -e
  printf '%s\n' "$status" | mustmatch '2'
  printf '%s\n' "$output" | mustmatch like '--around takes an ASCII whole number of at least 0'
done
```
