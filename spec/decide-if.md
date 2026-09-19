# Ask whether a condition holds

`thinkthen decide if CONDITION` judges one condition against the evidence on standard input. Nothing on this page reaches a network. Every example either asks the tool about itself or prints the plan and stops.

The help names the condition and every option the verb carries.

```bash
thinkthen decide if --help | head -1 | mustmatch like "Ask whether a condition holds for the evidence on standard input"
for option in --min-prob --status --plan --backend --url --adapter --model --key-env --record --replay --timeout --max-retries; do
  thinkthen decide if --help | grep -c -- "$option" | mustmatch not like "0"
done
```

`--plan` prints what would be sent, in the six fields the specification fixes, and opens no connection. The plan carries the evidence, because the evidence is what leaves the machine.

```bash
printf 'Refund me please.' | thinkthen decide if 'asks for a refund' --plan | mustmatch like '{"backend":"jev","url":"https://api.typesafe.ai/v1/systemone","adapter":"systemone","model":"jev-latest","key_env":"TYPESAFE_API_KEY","request":{"state":"Refund me please.","model":"jev-latest","questions":{"q1":{"type":"noul","instructions":"asks for a refund"}}}}'
```

The plan needs no key. It names the variable a key would be read from and never a value.

```bash
env -u TYPESAFE_API_KEY sh -c "printf 'Refund me please.' | thinkthen decide if 'asks for a refund' --plan" | grep -c '"key_env":"TYPESAFE_API_KEY"' | mustmatch like "1"
```

An ad-hoc backend is a URL, an adapter, and a model together. It has no name and borrows no key variable, so both fields print as null.

```bash
printf 'Refund me please.' | thinkthen decide if 'asks for a refund' --plan \
  --url http://127.0.0.1:1/v1 --adapter systemone --model local-1 \
  | grep -c '"backend":null,"url":"http://127.0.0.1:1/v1","adapter":"systemone","model":"local-1","key_env":null' | mustmatch like "1"
```

An environment variable set to the empty string counts as unset, the way most Unix tools read one. One holding white space is a usage error.

```bash
env THINKTHEN_MODEL= sh -c "printf 'Refund me please.' | thinkthen decide if 'asks for a refund' --plan" | grep -c '"model":"jev-latest"' | mustmatch like "1"
status=0
env THINKTHEN_MODEL=' ' sh -c "printf 'Refund me please.' | thinkthen decide if 'asks for a refund' --plan" >/dev/null 2>&1 || status=$?
echo "$status" | mustmatch like "2"
```

A profile named only by `THINKTHEN_BACKEND` yields to an ad-hoc backend given by flags, because a flag beats an environment variable. The `--backend` flag beside a URL is a usage error, and so are a name and a URL that both come from the environment.

```bash
env THINKTHEN_BACKEND=jev sh -c "printf 'x' | thinkthen decide if 'asks for a refund' --plan --url http://127.0.0.1:1/v1 --adapter systemone --model local-1" | grep -c '"backend":null' | mustmatch like "1"
status=0
env THINKTHEN_BACKEND=jev THINKTHEN_URL=http://127.0.0.1:1/v1 THINKTHEN_ADAPTER=systemone THINKTHEN_MODEL=local-1 sh -c "printf 'x' | thinkthen decide if 'asks for a refund' --plan" >/dev/null 2>&1 || status=$?
echo "$status" | mustmatch like "2"
```

A model alone replaces the profile's model, which is how a run is pinned to one version.

```bash
printf 'Refund me please.' | thinkthen decide if 'asks for a refund' --plan --model jev-1.13.0 | grep -c '"model":"jev-1.13.0"' | mustmatch like "1"
```

Options that name no backend are usage errors, and the exit code is 2.

```bash
status=0
printf 'x' | thinkthen decide if 'asks for a refund' --plan --url http://127.0.0.1:1/v1 >/dev/null 2>&1 || status=$?
echo "$status" | mustmatch like "2"
status=0
printf 'x' | thinkthen decide if 'asks for a refund' --plan --adapter systemone >/dev/null 2>&1 || status=$?
echo "$status" | mustmatch like "2"
status=0
printf 'x' | thinkthen decide if 'asks for a refund' --plan --backend nowhere >/dev/null 2>&1 || status=$?
echo "$status" | mustmatch like "2"
```

`--plan` sends nothing, so it takes neither `--record` nor `--replay`. Naming two different folders is a usage error too, because one run keeps one folder.

```bash
status=0
printf 'x' | thinkthen decide if 'asks for a refund' --plan --replay recording/ >/dev/null 2>&1 || status=$?
echo "$status" | mustmatch like "2"
status=0
printf 'x' | thinkthen decide if 'asks for a refund' --record here/ --replay there/ >/dev/null 2>&1 || status=$?
echo "$status" | mustmatch like "2"
```

`--status` needs a pass mark, and a mark a coin would pass is refused.

```bash
status=0
printf 'x' | thinkthen decide if 'asks for a refund' --status >/dev/null 2>&1 || status=$?
echo "$status" | mustmatch like "2"
status=0
printf 'x' | thinkthen decide if 'asks for a refund' --plan --min-prob 0.5 >/dev/null 2>&1 || status=$?
echo "$status" | mustmatch like "2"
```

Evidence that is empty or holds only white space is a usage error, because a judgment about nothing is a mistake in the pipeline. A condition that is blank is refused the same way.

```bash
status=0
printf '' | thinkthen decide if 'asks for a refund' --plan >/dev/null 2>&1 || status=$?
echo "$status" | mustmatch like "2"
status=0
printf '   \n' | thinkthen decide if 'asks for a refund' --plan >/dev/null 2>&1 || status=$?
echo "$status" | mustmatch like "2"
status=0
printf 'x' | thinkthen decide if '  ' --plan >/dev/null 2>&1 || status=$?
echo "$status" | mustmatch like "2"
```

Standard input that is not valid UTF-8 is a local failure, and the exit code is 5.

```bash
status=0
printf '\377\376 not text' | thinkthen decide if 'asks for a refund' --plan >/dev/null 2>&1 || status=$?
echo "$status" | mustmatch like "5"
```

A diagnostic goes to standard error and never to standard output, so a script reading a result never reads an explanation.

```bash
printf 'x' | thinkthen decide if 'asks for a refund' --plan --backend nowhere 2>/dev/null | mustmatch like ""
printf 'x' | thinkthen decide if 'asks for a refund' --plan --backend nowhere 2>&1 >/dev/null | mustmatch like "thinkthen: no backend profile is named \`nowhere\`"
```
