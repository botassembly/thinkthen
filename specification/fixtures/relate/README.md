# Relate pair-question fixture

These three complete entity sets exercise one-to-many edges and the effect of adding unrelated entities. The files contain public names only. `key.jsonl` has one labeled edge set per input, with 12 band memberships and eight city-to-country edges in each city row. The 10 other countries are distractors. Madrid, Berlin, and Rome are added to the third input but not to its key.

The band memberships follow [The Beatles' own history](https://www.thebeatles.com/here-comes-sun-2019-mix), [Paul McCartney's Wings history](https://www.paulmccartney.com/wings), and [the Traveling Wilburys' official site](https://www.travelingwilburys.com/). The city-country pairs follow the country entries for [France](https://www.britannica.com/place/France), [Japan](https://www.britannica.com/place/Japan), and [Brazil](https://www.britannica.com/place/Brazil). The key contains identities, not model probabilities.

`member-of.json` and `located-in.json` pin the relation words. Each run reads one rule file and one complete set. All three plans share one state within their own request. No key or network is needed to inspect them.

```bash
set -euo pipefail
root=$(git rev-parse --show-toplevel)
fixture="$root/specification/fixtures/relate"
env -u THINKTHEN_API_KEY -u THINKTHEN_BASE_URL \
  thinkthen relate "@$fixture/member-of.json" --jsonl --dry-run < "$fixture/bands.jsonl" \
  | jq -c '{entity_count,logical_questions,request_count,rule:.relations[0].name}' \
  | mustmatch '{"entity_count":13,"logical_questions":30,"request_count":1,"rule":"member_of"}'
```

```bash
set -euo pipefail
root=$(git rev-parse --show-toplevel)
fixture="$root/specification/fixtures/relate"
for set in cities cities-plus; do
  env -u THINKTHEN_API_KEY -u THINKTHEN_BASE_URL \
    thinkthen relate "@$fixture/located-in.json" --jsonl --dry-run < "$fixture/$set.jsonl" \
    | jq -c '{entity_count,logical_questions,request_count,rule:.relations[0].name}'
done | mustmatch '{"entity_count":18,"logical_questions":80,"request_count":1,"rule":"located_in"}
{"entity_count":21,"logical_questions":110,"request_count":1,"rule":"located_in"}'
```

## Replayed runs

Ticket 0167 recorded the three pair requests at model `jev-1.13.0` on 2026-09-27. The answers found all 28 stated edges and three extra band edges: John Lennon, Paul McCartney, and Ringo Starr as members of Traveling Wilburys. The city results did not change when the three unrelated cities were added. This block replays only the public pair recordings, with no key or network. The whole-run audit reads ids 1 to 3 from the result line numbers. Each single-set audit renumbers its copied key to id 1 because its extracted result is one line; the original key stays unchanged.

```bash
set -euo pipefail
root=$(git rev-parse --show-toplevel)
fixture="$root/specification/fixtures/relate"
work=$(mktemp -d)
trap 'rm -rf "$work"' EXIT
for set in bands cities cities-plus; do
  if [ "$set" = bands ]; then rule=member-of; else rule=located-in; fi
  env -u THINKTHEN_API_KEY -u THINKTHEN_BASE_URL \
    thinkthen relate "@$fixture/$rule.json" --url https://api.typesafe.ai/v1 \
    --model jev-1.13.0 --no-cache --jsonl --details --replay "$fixture/recording" \
    < "$fixture/$set.jsonl" | jq -c '.' >> "$work/results.jsonl"
done
{
  thinkthen audit "$work/results.jsonl" "$fixture/key.jsonl" --match strict --table | sed -n '2p'
  for id in 1 2 3; do
    sed -n "${id}p" "$work/results.jsonl" > "$work/result.jsonl"
    jq -c --argjson id "$id" 'select(.id == $id) | .id = 1' "$fixture/key.jsonl" > "$work/key.jsonl"
    thinkthen audit "$work/result.jsonl" "$work/key.jsonl" --match strict --table | sed -n '2p'
  done
} | mustmatch '  matched 28, extra 3, missed 0: precision 0.903   recall 1.000   f1 0.949
  matched 12, extra 3, missed 0: precision 0.800   recall 1.000   f1 0.889
  matched 8, extra 0, missed 0: precision 1.000   recall 1.000   f1 1.000
  matched 8, extra 0, missed 0: precision 1.000   recall 1.000   f1 1.000'
```
