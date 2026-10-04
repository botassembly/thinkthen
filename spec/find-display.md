# Show the selected find candidate

Find keeps one aggregate candidate request while displaying a physical location, the winning probability or neighbors.

```bash
thinkthen find --help | grep -c -- '--line-number' | mustmatch '1'
thinkthen find --help | grep -c -- '--scores' | mustmatch '1'
thinkthen find --help | grep -c -- '--around' | mustmatch '1'
printf '' | thinkthen find 'Which unit answers?' -n --scores --around 0 --no-cache | mustmatch ''
```

Details and text display refuse before source admission.

```bash
for flag in -n --scores '--around 0'; do
  set +e
  refusal=$(printf 'first\nsecond\n' | thinkthen find 'Which unit answers?' $flag --details --no-cache 2>&1)
  status=$?
  set -e
  printf '%s\n' "$status" | mustmatch '2'
  printf '%s\n' "$refusal" | mustmatch like 'text display flags cannot accompany --details'
done
set +e
refusal=$(printf 'first\nsecond\n' | thinkthen find 'Which unit answers?' --around -1 --no-cache 2>&1)
status=$?
set -e
printf '%s\n' "$status" | mustmatch '2'
printf '%s\n' "$refusal" | mustmatch like '--around takes an ASCII whole number of at least 0'
```
