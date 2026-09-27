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

The exact request recordings and replayed `audit --match strict` rows will be added after the separately authorized paid run in ticket 0167. Until then this page proves the input and plan contract only. It makes no accuracy claim. The run keeps its three details lines together so `audit` reads ids 1 to 3 from their line numbers, matching `key.jsonl`.
