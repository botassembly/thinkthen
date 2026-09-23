# How to find names in a text without a network

Status: green

Verbs: `recognize`

Use this when a test or local workflow needs names, kinds, and offsets from saved backend answers. The recording makes the result deterministic and needs no key.

```bash
tr -d '\n' < message.txt | env -u THINKTHEN_API_KEY -u THINKTHEN_BASE_URL thinkthen recognize --url https://api.typesafe.ai/v1 --replay recording/ --kind "PER=Part of a person's name." --kind 'ORG=Part of the name of an organization: a company, band, team, agency, government body, or media outlet.' --kind 'LOC=Part of the name of a place: a country, region, city, or geographic feature.' --kind 'MISC=Part of another named entity: a nationality, an event, a product, or the name of a creative work.' | jq -c '[.entities[] | .name]' | mustmatch '["Maria Chen","Northwind Freight","Chicago"]'
```

## Input

`message.txt` holds one sentence. `recording/` is mechanically adapted from experiment 225. The response keeps every answer, probability, confidence, model, usage count, and map order from the source experiment. The recording adaptation normalized JSON number spellings such as `1.0` to `1`, so it does not claim byte identity.

## Step 1: read names and offsets

The command returns the caller's kind words and Unicode scalar offsets into the original text. Long text keeps its context while only questions split across requests.

```bash
tr -d '\n' < message.txt | env -u THINKTHEN_API_KEY -u THINKTHEN_BASE_URL thinkthen recognize --url https://api.typesafe.ai/v1 --replay recording/ --kind "PER=Part of a person's name." --kind 'ORG=Part of the name of an organization: a company, band, team, agency, government body, or media outlet.' --kind 'LOC=Part of the name of a place: a country, region, city, or geographic feature.' --kind 'MISC=Part of another named entity: a nationality, an event, a product, or the name of a creative work.' | jq -c '.entities[0] | {name,kind,start,end}' | mustmatch '{"name":"Maria Chen","kind":"PER","start":0,"end":10}'
```

## Step 2: read strength correctly

`strength` combines several model probabilities. It is computed and is not itself a probability. No public price is claimed.

```bash
tr -d '\n' < message.txt | env -u THINKTHEN_API_KEY -u THINKTHEN_BASE_URL thinkthen recognize --url https://api.typesafe.ai/v1 --replay recording/ --kind "PER=Part of a person's name." --kind 'ORG=Part of the name of an organization: a company, band, team, agency, government body, or media outlet.' --kind 'LOC=Part of the name of a place: a country, region, city, or geographic feature.' --kind 'MISC=Part of another named entity: a nationality, an event, a product, or the name of a creative work.' | jq -r '.entities[0].strength >= 0.5' | mustmatch 'true'
```

## What can go wrong

- **A replay miss exits 5.** Evidence, kinds, descriptions, model, and address all contribute to the recording name.
- **A lowercase connector may split a name.** Every token receives its own detection decision.
- **Relations are beta.** Add a relation rule only when a beta graph is acceptable.
- **The recording contains the input text.** Keep private evidence out of a committed recording.

## Related how-tos

- [How to test a script with no network](../27-test-with-no-network/) explains replay misses.
- [How to pick a threshold from labeled cases](../13-pick-a-threshold/) explains measured cuts.
