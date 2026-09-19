# Answer a yes/no question

`thinkthen decide QUESTION` answers one question about the evidence on standard input and sets the exit code. Nothing on this page reaches a network. Every example either asks the tool about itself or prints the plan and stops.

The short help shows the everyday options. The long help adds the advanced ones and warns about `set -e`.

```bash
thinkthen decide -h | head -1 | mustmatch like "Answer a yes/no question about the evidence and set the exit code"
for option in --threshold --quiet --details --dry-run --profile; do
  thinkthen decide -h | grep -c -- "$option" | mustmatch not like "0"
done
for option in --url --adapter --model --key-env --record --replay --timeout --max-retries; do
  thinkthen decide -h | grep -c -- "$option" | mustmatch like "0"
  thinkthen decide --help | grep -c -- "$option" | mustmatch not like "0"
done
thinkthen decide --help | grep -c -- 'set -e' | mustmatch not like "0"
```

`--dry-run` prints what would be sent, in the six fields the specification fixes, and opens no connection. The plan carries the evidence, because the evidence is what leaves the machine.

```bash
printf 'Refund me please.' | thinkthen decide 'asks for a refund' --dry-run | mustmatch like '{"profile":"jev","url":"https://api.typesafe.ai/v1/systemone","adapter":"systemone","model":"jev-latest","key_env":"THINKTHEN_API_KEY","request":{"state":"Refund me please.","model":"jev-latest","questions":{"q1":{"type":"noul","instructions":"asks for a refund"}}}}'
```

The plan needs no key. It names the variable a key would be read from and never a value.

```bash
env -u THINKTHEN_API_KEY sh -c "printf 'Refund me please.' | thinkthen decide 'asks for a refund' --dry-run" | grep -c '"key_env":"THINKTHEN_API_KEY"' | mustmatch like "1"
```

An ad-hoc backend is a base, an adapter, and a model together. It has no name and borrows no key variable, so both fields print as null.

```bash
printf 'Refund me please.' | thinkthen decide 'asks for a refund' --dry-run \
  --url http://127.0.0.1:1/v1 --adapter systemone --model local-1 \
  | grep -c '"profile":null,"url":"http://127.0.0.1:1/v1/systemone","adapter":"systemone","model":"local-1","key_env":null' | mustmatch like "1"
```

Every source names a base, and the request is posted to `BASE/systemone`. `THINKTHEN_BASE_URL` names the base when `--url` does not, a trailing slash is accepted, and an empty variable counts as absent.

```bash
printf 'x' | env THINKTHEN_BASE_URL=http://127.0.0.1:1/v2/ thinkthen decide 'asks for a refund' --dry-run | grep -c '"url":"http://127.0.0.1:1/v2/systemone"' | mustmatch like "1"
printf 'x' | env THINKTHEN_BASE_URL= thinkthen decide 'asks for a refund' --dry-run | grep -c '"url":"https://api.typesafe.ai/v1/systemone"' | mustmatch like "1"
printf 'x' | env THINKTHEN_BASE_URL=http://127.0.0.1:1/v2 thinkthen decide 'asks for a refund' --dry-run --url http://127.0.0.1:1/v3 --adapter systemone --model local-1 | grep -c '"url":"http://127.0.0.1:1/v3/systemone"' | mustmatch like "1"
```

A base that is not an `http` or `https` address is a usage error, and so is a base carrying user information, because the address is printed in a plan and kept in a recording. Neither message shows the address it refused.

```bash
status=0
printf 'x' | env THINKTHEN_BASE_URL=ftp://127.0.0.1/v1 thinkthen decide 'asks for a refund' --dry-run >/dev/null 2>&1 || status=$?
echo "$status" | mustmatch like "2"
printf 'x' | env THINKTHEN_BASE_URL=ftp://127.0.0.1/v1 thinkthen decide 'asks for a refund' --dry-run 2>&1 >/dev/null | mustmatch like "thinkthen: a base address begins with \`http://\` or \`https://\` and carries no user information"
status=0
printf 'x' | env THINKTHEN_BASE_URL=https://someone:secret@127.0.0.1/v1 thinkthen decide 'asks for a refund' --dry-run >/dev/null 2>&1 || status=$?
echo "$status" | mustmatch like "2"
printf 'x' | env THINKTHEN_BASE_URL=https://someone:secret@127.0.0.1/v1 thinkthen decide 'asks for a refund' --dry-run 2>&1 >/dev/null | grep -c secret | mustmatch like "0"
```

The five `THINKTHEN_*` backend variables of ADR 0004 are gone. None of them changes the plan.

```bash
env THINKTHEN_BACKEND=nowhere THINKTHEN_URL=http://127.0.0.1:1/v1 THINKTHEN_ADAPTER=systemone THINKTHEN_MODEL=local-1 THINKTHEN_KEY_ENV=OTHER_KEY sh -c "printf 'x' | thinkthen decide 'asks for a refund' --dry-run" | grep -c '"profile":"jev","url":"https://api.typesafe.ai/v1/systemone","adapter":"systemone","model":"jev-latest","key_env":"THINKTHEN_API_KEY"' | mustmatch like "1"
```

A model alone replaces the profile's model, which is how a run is pinned to one version.

```bash
printf 'Refund me please.' | thinkthen decide 'asks for a refund' --dry-run --model jev-1.13.0 | grep -c '"model":"jev-1.13.0"' | mustmatch like "1"
```

Options may sit before the question, and `--` ends option parsing, so a question that begins with a dash follows it.

```bash
printf 'x' | thinkthen decide --dry-run -- '--asks for a refund' | grep -c '"instructions":"--asks for a refund"' | mustmatch like "1"
```

Options that name no backend are usage errors, and the exit code is 2.

```bash
status=0
printf 'x' | thinkthen decide 'asks for a refund' --dry-run --url http://127.0.0.1:1/v1 >/dev/null 2>&1 || status=$?
echo "$status" | mustmatch like "2"
status=0
printf 'x' | thinkthen decide 'asks for a refund' --dry-run --adapter systemone >/dev/null 2>&1 || status=$?
echo "$status" | mustmatch like "2"
status=0
printf 'x' | thinkthen decide 'asks for a refund' --dry-run --profile nowhere >/dev/null 2>&1 || status=$?
echo "$status" | mustmatch like "2"
```

`--dry-run` sends nothing, so it takes neither `--record` nor `--replay`. Naming two different folders is a usage error too, because one run keeps one folder.

```bash
status=0
printf 'x' | thinkthen decide 'asks for a refund' --dry-run --replay recording/ >/dev/null 2>&1 || status=$?
echo "$status" | mustmatch like "2"
status=0
printf 'x' | thinkthen decide 'asks for a refund' --record here/ --replay there/ >/dev/null 2>&1 || status=$?
echo "$status" | mustmatch like "2"
```

A percent, a reversed band, an empty side, and a number that is not finite are usage errors before any request goes out.

```bash
for bad in 90 0 0.9:0.1 0.1: :0.9 inf NaN half; do
  status=0
  printf 'x' | thinkthen decide 'asks for a refund' --dry-run --threshold "$bad" >/dev/null 2>&1 || status=$?
  echo "$status" | mustmatch like "2"
done
```

`--quiet` prints nothing and `--details` prints the whole object, so asking for both is a usage error.

```bash
status=0
printf 'x' | thinkthen decide 'asks for a refund' --quiet --details >/dev/null 2>&1 || status=$?
echo "$status" | mustmatch like "2"
```

Every option the earlier surface carried is gone, and each one is a usage error now.

```bash
for gone in --status "--min-prob 0.9" --plan "--backend jev"; do
  status=0
  printf 'x' | thinkthen decide 'asks for a refund' --dry-run $gone >/dev/null 2>&1 || status=$?
  echo "$status" | mustmatch like "2"
done
status=0
printf 'x' | thinkthen decide if 'asks for a refund' --dry-run >/dev/null 2>&1 || status=$?
echo "$status" | mustmatch like "2"
```

Evidence that is empty or holds only white space is a usage error, because a judgment about nothing is a mistake in the pipeline. A question that is blank is refused the same way.

```bash
status=0
printf '' | thinkthen decide 'asks for a refund' --dry-run >/dev/null 2>&1 || status=$?
echo "$status" | mustmatch like "2"
status=0
printf '   \n' | thinkthen decide 'asks for a refund' --dry-run >/dev/null 2>&1 || status=$?
echo "$status" | mustmatch like "2"
status=0
printf 'x' | thinkthen decide '  ' --dry-run >/dev/null 2>&1 || status=$?
echo "$status" | mustmatch like "2"
```

Standard input that is not valid UTF-8 is a local failure, and the exit code is 5.

```bash
status=0
printf '\377\376 not text' | thinkthen decide 'asks for a refund' --dry-run >/dev/null 2>&1 || status=$?
echo "$status" | mustmatch like "5"
```

A diagnostic goes to standard error and never to standard output, so a script reading a result never reads an explanation.

```bash
printf 'x' | thinkthen decide 'asks for a refund' --dry-run --profile nowhere 2>/dev/null | mustmatch like ""
printf 'x' | thinkthen decide 'asks for a refund' --dry-run --profile nowhere 2>&1 >/dev/null | mustmatch like "thinkthen: no backend profile is named \`nowhere\`"
```
