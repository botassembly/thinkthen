# Find names in text

`recognize` asks detection and kind questions for every word. A trailing `.`, `!`, `?`, `,`, `:`, or `;` counts as its own word, so `Ada met Acme.` is four words. Its dry run sends nothing. It reports the exact counts and every request it would send, with its digest, size in bytes, and body.

```bash
printf 'Ada met Acme.' | env -u THINKTHEN_API_KEY thinkthen recognize --dry-run | jq -c '{schema, words, detection_questions, kind_questions, request_count, sent: (.requests | length)}' | mustmatch like '{"schema":"thinkthen.recognize-plan/1","words":4,"detection_questions":4,"kind_questions":4,"request_count":1,"sent":1}'
printf 'Ada met Acme.' | env -u THINKTHEN_API_KEY thinkthen recognize --dry-run | jq -r '.requests[0].body_utf8 | fromjson | .state' | mustmatch like 'Ada met Acme.'
```

The forty-case harvest fixture includes this recorded sentence. The replay needs no key or network, and caller kinds come back unchanged.

```bash
printf 'Maria Chen joined Northwind Freight in Chicago last spring.' | env -u THINKTHEN_API_KEY -u THINKTHEN_BASE_URL thinkthen recognize --url https://api.typesafe.ai/v1 --replay "$(git rev-parse --show-toplevel)/crates/thinkthen/tests/fixtures/recognize-225/C01" --kind "PER=Part of a person's name." --kind 'ORG=Part of the name of an organization: a company, band, team, agency, government body, or media outlet.' --kind 'LOC=Part of the name of a place: a country, region, city, or geographic feature.' --kind 'MISC=Part of another named entity: a nationality, an event, a product, or the name of a creative work.' | jq -c '[.entities[] | {name,kind}]' | mustmatch '[{"name":"Maria Chen","kind":"PER"},{"name":"Northwind Freight","kind":"ORG"},{"name":"Chicago","kind":"LOC"}]'
```

An empty or blank text refuses the dry run as it refuses a live run. It prints no plan.

```bash
for text in '' ' '; do
  printf '%s' "$text" | { env -u THINKTHEN_API_KEY thinkthen recognize --dry-run 2>&1 && echo 'exit 0' || echo "exit $?"; } | mustmatch 'thinkthen: the evidence is empty or blank
exit 2'
done
```

Relations are beta. A malformed rule is a usage error before any key is read.

```bash
status=0
printf 'Ada met Acme.' | env -u THINKTHEN_API_KEY thinkthen recognize person --relation works_for=person --dry-run >/dev/null 2>&1 || status=$?
echo "$status" | mustmatch like '2'
```
