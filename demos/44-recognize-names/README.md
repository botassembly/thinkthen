# How to find names in a text without a network

Status: green

Verbs: `recognize`

Use this when a test or local workflow needs names, kinds, and offsets from saved backend answers. The recording makes the result deterministic and needs no key.

```bash
tr -d '\n' < message.txt | env -u THINKTHEN_API_KEY -u THINKTHEN_BASE_URL thinkthen recognize --url https://api.typesafe.ai/v1 --replay recording/ --kind "PER=Part of a person's name." --kind 'ORG=Part of the name of an organization: a company, band, team, agency, government body, or media outlet.' --kind 'LOC=Part of the name of a place: a country, region, city, or geographic feature.' --kind 'MISC=Part of another named entity: a nationality, an event, a product, or the name of a creative work.' | jq -c '[.entities[] | .text]' | mustmatch '["Maria Chen","Northwind Freight","Chicago"]'
```

## Input

`message.txt` holds one sentence. `recording/` holds the two requests one live run sent on 2026-09-27 for ticket 0147. The first asks where each of the sentence's ten pieces stands in a name. The second asks each found name's kind, and it asks for no edge because no found name touches a mark.

## Step 1: read names and offsets

Each name comes back with its text, its Unicode scalar offsets into the original text, its length, and the caller's kind word.

```bash
tr -d '\n' < message.txt | env -u THINKTHEN_API_KEY -u THINKTHEN_BASE_URL thinkthen recognize --url https://api.typesafe.ai/v1 --replay recording/ --kind "PER=Part of a person's name." --kind 'ORG=Part of the name of an organization: a company, band, team, agency, government body, or media outlet.' --kind 'LOC=Part of the name of a place: a country, region, city, or geographic feature.' --kind 'MISC=Part of another named entity: a nationality, an event, a product, or the name of a creative work.' | jq -c '.entities[0]' | mustmatch '{"text":"Maria Chen","start":0,"end":10,"length":10,"kind":"PER","strength":0.9987}'
```

## Step 2: read strength correctly

`strength` is the kind's probability times the probability that the pieces form exactly this name. It ranks names, and it is not itself a probability. The default cut keeps names at 0.5 or above, and `--threshold` moves it.

```bash
tr -d '\n' < message.txt | env -u THINKTHEN_API_KEY -u THINKTHEN_BASE_URL thinkthen recognize --url https://api.typesafe.ai/v1 --replay recording/ --kind "PER=Part of a person's name." --kind 'ORG=Part of the name of an organization: a company, band, team, agency, government body, or media outlet.' --kind 'LOC=Part of the name of a place: a country, region, city, or geographic feature.' --kind 'MISC=Part of another named entity: a nationality, an event, a product, or the name of a creative work.' --threshold 0.999 | jq -c '[.entities[] | .text]' | mustmatch '["Chicago"]'
```

## What can go wrong

- **A replay miss exits 5.** Evidence, kinds, descriptions, model, and address all contribute to the recording name.
- **Each mark is its own piece.** A mark that belongs to a name, such as the `!` of `Help!`, comes back only when the edge question picks it.
- **The recording contains the input text.** Keep private evidence out of a committed recording.

## Related how-tos

- [How to test a script with no network](../27-test-with-no-network/) explains replay misses.
- [How to pick a threshold from labeled cases](../13-pick-a-threshold/) explains measured cuts.
