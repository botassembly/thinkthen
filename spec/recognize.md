# Find names in text

`recognize` splits the text into pieces and asks where each piece stands in a name. White space separates pieces, and each punctuation mark or symbol is its own piece, so `Ada met Acme.` is four pieces. Its plan sends nothing. It reports the pieces, the step-one requests it would send with each digest, size in bytes, and body, and an upper bound on the name requests that follow.

```bash
printf 'Ada met Acme.' | env -u THINKTHEN_API_KEY thinkthen recognize --plan | sed -n '1p' | jq -c '{schema, pieces, request_count, name_requests_upper_bound, sent: (.requests | length)}' | mustmatch like '{"schema":"thinkthen.recognize-plan/2","pieces":4,"request_count":1,"name_requests_upper_bound":4,"sent":1}'
printf 'Ada met Acme.' | env -u THINKTHEN_API_KEY thinkthen recognize --plan | sed -n '1p' | jq -c '.requests[0].body_utf8 | fromjson | {state, questions: (.questions | length)}' | mustmatch '{"state":"Ada met Acme.","questions":4}'
printf 'Ada met Acme.' | env -u THINKTHEN_API_KEY thinkthen recognize --plan | sed -n '2p' | jq -c '{records, requests, upper_bound}' | mustmatch '{"records":1,"requests":5,"upper_bound":true}'
```

A JSON record can supply context separately from its selected text. Recognition sends that context to each stage and retains the original record in its row.

```bash
printf '%s\n' '{"body":"Ada met Acme.","policy":"First context"}' '{"body":"Bob met Corp.","policy":"Second context"}' | env -u THINKTHEN_API_KEY thinkthen recognize --jsonl --field /body --context-field /policy --plan | sed -n '1p' | jq -c '.requests[0].body_utf8 | fromjson | .state' | mustmatch '{"context":"First context","evidence":"Ada met Acme."}'
printf '%s\n' '{"body":"Ada met Acme.","policy":""}' | env -u THINKTHEN_API_KEY thinkthen recognize --jsonl --field /body --context-field /policy --plan | sed -n '1p' | jq -c '.requests[0].body_utf8 | fromjson | .state' | mustmatch '"Ada met Acme."'
```

Caller wording and label descriptions define literal entities. This controlled fixture proves the saved declaration and exact offsets. It measures no model accuracy.

```bash
fixture="$(git rev-parse --show-toplevel)/specification/fixtures/recognize/caller-defined"
cat "$fixture/text.txt" | env -u THINKTHEN_API_KEY -u THINKTHEN_BASE_URL thinkthen recognize @"$fixture/question.json" --url "$(cat "$fixture/url.txt")" --replay "$fixture/recording" --no-cache | jq -c '.entities' | mustmatch '[{"text":"42.75","start":8,"end":13,"length":5,"kind":"amount","strength":1.0}]'
```

An empty or blank text refuses the plan as it refuses a live run. It prints no plan.

```bash
for text in '' ' '; do
  printf '%s' "$text" | { env -u THINKTHEN_API_KEY thinkthen recognize --plan 2>&1 && echo 'exit 0' || echo "exit $?"; } | mustmatch 'thinkthen: the evidence is empty or blank
exit 2'
done
```

Relations are beta. A malformed rule is a usage error before any key is read.

```bash
status=0
printf 'Ada met Acme.' | env -u THINKTHEN_API_KEY thinkthen recognize person --relation works_for=person --plan >/dev/null 2>&1 || status=$?
echo "$status" | mustmatch like '2'
```
