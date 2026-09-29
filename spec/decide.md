# Answer a yes/no question

`thinkthen decide QUESTION` answers one question about the evidence on standard input and sets the exit code. Nothing on this page reaches a network. Every example either asks the tool about itself or prints the plan and stops.

The short help shows the everyday options. The long help opens with two runnable examples before its explanation, adds the advanced options, and warns about `set -e`.

```bash
thinkthen decide -h | head -1 | mustmatch like "Answer one yes or no question about a text"
for option in --threshold --quiet --details --plan --url --profile; do
  thinkthen decide -h | grep -c -- "$option" | mustmatch not like "0"
done
for option in --model --record --replay --cache --jobs --timeout --max-retries; do
  thinkthen decide -h | grep -c -- "$option" | mustmatch "0"
  thinkthen decide --help | grep -c -- "$option" | mustmatch not like "0"
done
thinkthen decide --help | grep -c -- 'set -e' | mustmatch not like "0"
thinkthen decide --help | grep -c -- 'defaults to 0.5' | mustmatch not like "0"
thinkthen decide --help | grep -c -- '\[default: 4\]' | mustmatch not like "0"
thinkthen decide --help | sed -n '/^Examples:/,/^The answer is/p' | grep -c "printf 'Refund me please.' | thinkthen decide" | mustmatch "2"
thinkthen decide --help | sed -n '/^Examples:/,/^The answer is/p' | grep -Fxc "printf 'Refund me please.' | thinkthen decide 'Does this ask for a refund?' --threshold 0.1:0.9" | mustmatch "1"
```

`--plan` prints what would be sent, in the four fields the specification fixes, and opens no connection. The plan carries the evidence, because the evidence is what leaves the machine.

```bash
printf 'Refund me please.' | thinkthen decide 'asks for a refund' --plan | sed -n '1p' | mustmatch like '{"url":"https://api.typesafe.ai/v1/systemone","model":"jev-1.13.0","key_env":"THINKTHEN_API_KEY","request":{"state":"Refund me please.","model":"jev-1.13.0","questions":{"q1":{"type":"noul","instructions":"asks for a refund"}}}}'
printf 'Refund me please.' | thinkthen decide 'asks for a refund' --plan | sed -n '2p' | mustmatch '{"records":1,"requests":1,"estimated_bytes":120,"estimated_input_tokens":{"lower":61,"upper":109},"upper_bound":false}'
```

The plan needs no key. It names the variable a key would be read from and never a value.

```bash
env -u THINKTHEN_API_KEY sh -c "printf 'Refund me please.' | thinkthen decide 'asks for a refund' --plan" | grep -c '"key_env":"THINKTHEN_API_KEY"' | mustmatch "1"
```

Every source names a base, and the request is posted to `BASE/systemone`. `THINKTHEN_BASE_URL` names the base when `--url` does not, a trailing slash is accepted, and an empty variable counts as absent.

```bash
printf 'x' | env THINKTHEN_BASE_URL=http://127.0.0.1:1/v2/ thinkthen decide 'asks for a refund' --plan | grep -c '"url":"http://127.0.0.1:1/v2/systemone"' | mustmatch "1"
printf 'x' | env THINKTHEN_BASE_URL= thinkthen decide 'asks for a refund' --plan | grep -c '"url":"https://api.typesafe.ai/v1/systemone"' | mustmatch "1"
printf 'x' | env THINKTHEN_BASE_URL=http://127.0.0.1:1/v2 thinkthen decide 'asks for a refund' --plan --url http://127.0.0.1:1/v3 | grep -c '"url":"http://127.0.0.1:1/v3/systemone"' | mustmatch "1"
```

`--url` names a base and takes no companion option. The key still comes from `THINKTHEN_API_KEY`, because naming the address is the user's own act.

```bash
printf 'x' | thinkthen decide 'asks for a refund' --plan --url http://127.0.0.1:1/v1 | grep -c '"url":"http://127.0.0.1:1/v1/systemone","model":"jev-1.13.0","key_env":"THINKTHEN_API_KEY"' | mustmatch "1"
```

A base that is not an `http` or `https` address is a usage error, and so is a base carrying user information, because the address is printed in a plan and kept in a recording. Neither message shows the address it refused.

```bash
status=0
printf 'x' | env THINKTHEN_BASE_URL=ftp://127.0.0.1/v1 thinkthen decide 'asks for a refund' --plan >/dev/null 2>&1 || status=$?
echo "$status" | mustmatch "2"
printf 'x' | env THINKTHEN_BASE_URL=ftp://127.0.0.1/v1 thinkthen decide 'asks for a refund' --plan 2>&1 >/dev/null | mustmatch like "thinkthen: a base address begins with \`http://\` or \`https://\`"
status=0
printf 'x' | env THINKTHEN_BASE_URL=https://someone:secret@127.0.0.1/v1 thinkthen decide 'asks for a refund' --plan >/dev/null 2>&1 || status=$?
echo "$status" | mustmatch "2"
printf 'x' | env THINKTHEN_BASE_URL=https://someone:secret@127.0.0.1/v1 thinkthen decide 'asks for a refund' --plan 2>&1 >/dev/null | grep -c secret | mustmatch "0"
```

A base written as white space is blank, and the refusal says so rather than naming a scheme.

```bash
status=0
printf 'x' | thinkthen decide 'asks for a refund' --plan --url '' >/dev/null 2>&1 || status=$?
echo "$status" | mustmatch "2"
printf 'x' | thinkthen decide 'asks for a refund' --plan --url '   ' 2>&1 >/dev/null | mustmatch like "thinkthen: a URL is text, not white space"
```

The five `THINKTHEN_*` backend variables of ADR 0004 are gone. None of them changes the plan.

```bash
env THINKTHEN_BACKEND=nowhere THINKTHEN_URL=http://127.0.0.1:1/v1 THINKTHEN_ADAPTER=systemone THINKTHEN_MODEL=local-1 THINKTHEN_KEY_ENV=OTHER_KEY sh -c "printf 'x' | thinkthen decide 'asks for a refund' --plan" | grep -c '"url":"https://api.typesafe.ai/v1/systemone","model":"jev-1.13.0","key_env":"THINKTHEN_API_KEY"' | mustmatch "1"
```

`--model` replaces the default model, which is how a run is pinned to one version.

```bash
printf 'Refund me please.' | thinkthen decide 'asks for a refund' --plan --model jev-1.13.0 | grep -c '"model":"jev-1.13.0"' | mustmatch "1"
```

Options may sit before the question, and `--` ends option parsing, so a question that begins with a dash follows it.

```bash
printf 'x' | thinkthen decide --plan -- '--asks for a refund' | grep -c '"instructions":"--asks for a refund"' | mustmatch "1"
```

A question file carries the same settings under `@FILE`, and its question text and criteria may be objects or lists, which the request passes through as written.

```bash
cat > question.json <<'JSON'
{"decide":{"ask":"Does this message ask for a refund?","lang":"en"},"true":{"means":"Money back."},"false":null}
JSON
printf 'Refund me please.' | thinkthen decide @question.json --plan | grep -c '"instructions":{"ask":"Does this message ask for a refund?","lang":"en"},"criteria":{"true":{"means":"Money back."},"false":null}' | mustmatch "1"
rm question.json
```

A model given as white space is a usage error, and the exit code is 2.

```bash
status=0
printf 'x' | thinkthen decide 'asks for a refund' --plan --model '' >/dev/null 2>&1 || status=$?
echo "$status" | mustmatch "2"
status=0
printf 'x' | thinkthen decide 'asks for a refund' --plan --model '  ' >/dev/null 2>&1 || status=$?
echo "$status" | mustmatch "2"
```

`--plan` sends nothing, so it takes neither `--record` nor `--replay`. Naming two different folders is a usage error too, because one run keeps one folder.

```bash
status=0
printf 'x' | thinkthen decide 'asks for a refund' --plan --replay recording/ >/dev/null 2>&1 || status=$?
echo "$status" | mustmatch "2"
status=0
printf 'x' | thinkthen decide 'asks for a refund' --record here/ --replay there/ >/dev/null 2>&1 || status=$?
echo "$status" | mustmatch "2"
```

`--lines` and `--jsonl` turn the input into records, and the plan then shows the first batch with a fifth field naming the framing and the pointers. The batch quotes each record inside its own question, beside one fixed sentence of evidence.

```bash
printf '{"id":"T-1","body":"Payouts failed."}\n{"id":"T-2","body":"x"}\n' | thinkthen decide 'reports a payment failure' --jsonl --field /body --plan | sed -n '1p' | mustmatch like '{"url":"https://api.typesafe.ai/v1/systemone","model":"jev-1.13.0","key_env":"THINKTHEN_API_KEY","input":{"framing":"jsonl","field":["/body"]},"request":{"state":"Each question quotes the text it asks about.","model":"jev-1.13.0","questions":{"q1":{"type":"noul","instructions":"The text is \"Payouts failed.\". reports a payment failure"},"q2":{"type":"noul","instructions":"The text is \"x\". reports a payment failure"}}}}'
printf 'first line\nsecond line\n' | thinkthen decide 'reports a payment failure' --lines --plan | grep -c '"input":{"framing":"lines","field":\[\]},"request":{"state":"Each question quotes' | mustmatch "1"
```

`--field` given more than once sends an object of the named parts, keyed by the last part of each pointer. Two pointers that end in one name are a usage error, and so is `--field` beside `--lines`.

```bash
printf '{"id":"T-1","body":"Payouts failed."}\n' | thinkthen decide 'reports a payment failure' --jsonl --field /body --field /id --plan | grep -c '"state":{"body":"Payouts failed.","id":"T-1"}' | mustmatch "1"
for bad in "--field /a/text --field /b/text" "--lines --field /body"; do
  status=0
  printf '{"a":{"text":"x"},"b":{"text":"y"}}\n' | thinkthen decide 'reports a payment failure' --jsonl --plan $bad >/dev/null 2>&1 || status=$?
  echo "$status" | mustmatch "2"
done
```

A pointer that is not RFC 6901 is a usage error, and the message names RFC 6901 and the pointer that was typed.

```bash
for bad in '$.body' '#/id' '/*' '/list/-1' '/a~2b'; do
  status=0
  printf '{"body":"x"}\n' | thinkthen decide 'reports a payment failure' --jsonl --field "$bad" --plan >/dev/null 2>&1 || status=$?
  echo "$status" | mustmatch "2"
done
printf '{"body":"x"}\n' | thinkthen decide 'reports a payment failure' --jsonl --field '$.body' --plan 2>&1 >/dev/null | mustmatch like "thinkthen: --field \`\$.body\`: a pointer is RFC 6901, so it is empty or begins with \`/\`"
```

`--input FILE` names a path. An empty record stream succeeds with no output and no request, and `--quiet` cannot act over records, because no record's answer reaches the exit code.

```bash
no_records_output=$(printf '' | thinkthen decide 'reports a payment failure' --jsonl --plan)
test -z "$no_records_output"
status=0
printf 'a line\n' | thinkthen decide 'reports a payment failure' --lines --quiet --plan >/dev/null 2>&1 || status=$?
echo "$status" | mustmatch "2"
status=0
thinkthen decide 'reports a payment failure' --jsonl --input no-such-file.jsonl --plan >/dev/null 2>&1 || status=$?
echo "$status" | mustmatch "5"
```

A percent, a reversed band, an empty side, and a number that is not finite are usage errors before any request goes out.

```bash
for bad in 90 0 0.9:0.1 0.1: :0.9 inf NaN half; do
  status=0
  printf 'x' | thinkthen decide 'asks for a refund' --plan --threshold "$bad" >/dev/null 2>&1 || status=$?
  echo "$status" | mustmatch "2"
done
```

`--quiet` prints nothing and `--details` prints the whole object, so asking for both is a usage error.

```bash
status=0
printf 'x' | thinkthen decide 'asks for a refund' --quiet --details >/dev/null 2>&1 || status=$?
echo "$status" | mustmatch "2"
```

The remaining options from the earlier configuration surface stay gone. `--profile` has returned with the smaller limit-only meaning of ADR 0032.

```bash
for gone in --status "--min-prob 0.9" --plan "--backend jev" "--adapter systemone" "--key-env LOCAL_KEY" "--config site.json"; do
  status=0
  printf 'x' | thinkthen decide 'asks for a refund' --plan $gone >/dev/null 2>&1 || status=$?
  echo "$status" | mustmatch "2"
done
status=0
printf 'x' | thinkthen decide if 'asks for a refund' --plan >/dev/null 2>&1 || status=$?
echo "$status" | mustmatch "2"
```

An explicit profile refuses an oversized request before a key or connection. The limit counts the UTF-8 evidence bytes after selection.

```bash
cat > profile.json <<'JSON'
{"schema":"thinkthen.backend-profile/1","name":"four-byte-test","max_evidence_bytes":4}
JSON
printf 'four' | env -u THINKTHEN_API_KEY thinkthen decide 'asks for a refund' --plan --profile profile.json | grep -c '"state":"four"' | mustmatch "1"
status=0
printf 'five!' | env -u THINKTHEN_API_KEY thinkthen decide 'asks for a refund' --plan --profile profile.json >/dev/null 2>&1 || status=$?
echo "$status" | mustmatch "2"
printf 'five!' | env -u THINKTHEN_API_KEY thinkthen decide 'asks for a refund' --plan --profile profile.json 2>&1 >/dev/null | mustmatch like "thinkthen: profile four-byte-test allows at most 4 evidence bytes; this request has 5"
rm profile.json
```

Evidence that is empty or holds only white space is a usage error, because a judgment about nothing is a mistake in the pipeline. A question that is blank is refused the same way.

```bash
status=0
printf '' | thinkthen decide 'asks for a refund' --plan >/dev/null 2>&1 || status=$?
echo "$status" | mustmatch "2"
status=0
printf '   \n' | thinkthen decide 'asks for a refund' --plan >/dev/null 2>&1 || status=$?
echo "$status" | mustmatch "2"
status=0
printf 'x' | thinkthen decide '  ' --plan >/dev/null 2>&1 || status=$?
echo "$status" | mustmatch "2"
```

Standard input that is not valid UTF-8 is a local failure, and the exit code is 5.

```bash
status=0
printf '\377\376 not text' | thinkthen decide 'asks for a refund' --plan >/dev/null 2>&1 || status=$?
echo "$status" | mustmatch "5"
```

A diagnostic goes to standard error and never to standard output, so a script reading a result never reads an explanation.

```bash
refusal_code=0
refusal_stdout=$(printf 'x' | thinkthen decide 'asks for a refund' --plan --url ftp://127.0.0.1/v1 2>/dev/null) || refusal_code=$?
test -z "$refusal_stdout"
echo "$refusal_code" | mustmatch "2"
printf 'x' | thinkthen decide 'asks for a refund' --plan --url ftp://127.0.0.1/v1 2>&1 >/dev/null | mustmatch like "thinkthen: a base address begins with \`http://\` or \`https://\`"
```
