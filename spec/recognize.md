# Find names in text

`recognize` asks detection and kind questions for every token. Its dry run sends nothing and reports exact recognition counts.

```bash
printf 'Ada met Acme.' | env -u THINKTHEN_API_KEY thinkthen recognize --dry-run | mustmatch like '{"tokens":4,"detection_questions":4,"kind_questions":4,"requests":1}'
```

The forty-case harvest fixture includes this recorded sentence. The replay needs no key or network, and caller kinds come back unchanged.

```bash
printf 'Maria Chen joined Northwind Freight in Chicago last spring.' | env -u THINKTHEN_API_KEY -u THINKTHEN_BASE_URL thinkthen recognize --url https://api.typesafe.ai/v1 --replay "$(git rev-parse --show-toplevel)/crates/thinkthen/tests/fixtures/recognize-225/C01" --kind "PER=Part of a person's name." --kind 'ORG=Part of the name of an organization: a company, band, team, agency, government body, or media outlet.' --kind 'LOC=Part of the name of a place: a country, region, city, or geographic feature.' --kind 'MISC=Part of another named entity: a nationality, an event, a product, or the name of a creative work.' | jq -c '[.entities[] | {name,kind}]' | mustmatch '[{"name":"Maria Chen","kind":"PER"},{"name":"Northwind Freight","kind":"ORG"},{"name":"Chicago","kind":"LOC"}]'
```

Relations are beta. A malformed rule is a usage error before any key is read.

```bash
status=0
printf 'Ada met Acme.' | env -u THINKTHEN_API_KEY thinkthen recognize person --relation works_for=person --dry-run >/dev/null 2>&1 || status=$?
echo "$status" | mustmatch like '2'
```
