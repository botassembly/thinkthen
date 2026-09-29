# Find names in text

`recognize` splits the text into pieces and asks where each piece stands in a name. White space separates pieces, and each punctuation mark or symbol is its own piece, so `Ada met Acme.` is four pieces. Its plan sends nothing. It reports the pieces, the step-one requests it would send with each digest, size in bytes, and body, and an upper bound on the name requests that follow.

```bash
printf 'Ada met Acme.' | env -u THINKTHEN_API_KEY thinkthen recognize --plan | sed -n '1p' | jq -c '{schema, pieces, request_count, name_requests_upper_bound, sent: (.requests | length)}' | mustmatch like '{"schema":"thinkthen.recognize-plan/2","pieces":4,"request_count":1,"name_requests_upper_bound":1,"sent":1}'
printf 'Ada met Acme.' | env -u THINKTHEN_API_KEY thinkthen recognize --plan | sed -n '1p' | jq -c '.requests[0].body_utf8 | fromjson | {state, questions: (.questions | length)}' | mustmatch '{"state":"Ada met Acme.","questions":4}'
```

Demo 44's recording holds one live run of this sentence. The replay needs no key or network, and caller kinds come back unchanged.

```bash
printf 'Maria Chen joined Northwind Freight in Chicago last spring.' | env -u THINKTHEN_API_KEY -u THINKTHEN_BASE_URL thinkthen recognize --url https://api.typesafe.ai/v1 --replay "$(git rev-parse --show-toplevel)/demos/44-recognize-names/recording" --kind "PER=Part of a person's name." --kind 'ORG=Part of the name of an organization: a company, band, team, agency, government body, or media outlet.' --kind 'LOC=Part of the name of a place: a country, region, city, or geographic feature.' --kind 'MISC=Part of another named entity: a nationality, an event, a product, or the name of a creative work.' | jq -c '[.entities[] | {text,kind}]' | mustmatch '[{"text":"Maria Chen","kind":"PER"},{"text":"Northwind Freight","kind":"ORG"},{"text":"Chicago","kind":"LOC"}]'
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
