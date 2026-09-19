# cost.jq — the input tokens a run spent, and what they cost.
#
# Reads: the rows of one run, one JSON object per line. Run it with
#   `jq -n --argjson usd_per_million_input 0.042 -f cost.jq ROWS`.
# Arguments:
#   $usd_per_million_input  the price of a million input tokens. The hosted
#     decider charged 0.042 on 2026-09-19, which is the number in
#     sdlc/live-tokens. Another address has another price, and a price is a
#     fact about a contract rather than about a run, so the transform never
#     assumes one.
# Policies:
#   - A row whose `meta.replayed` is true came from a recording. It opened no
#     connection and paid nothing, so its tokens are counted apart under
#     `replayed` and never priced. Counting them would bill a gate.
#   - Only input tokens are priced, because the ledger this repository keeps
#     counts input tokens. Output tokens are reported beside them and are not
#     converted.
#   - A row whose `meta.usage` is absent is listed by id in `no_usage` and
#     adds nothing. The backend reports usage or it does not, and a missing
#     count is not a zero.
#   - Token counts are exact. The money is rounded to six decimals, because a
#     small run costs less than a cent.

reduce inputs as $row (
  {
    rows: 0,
    no_usage: [],
    charged: {rows: 0, input_tokens: 0, output_tokens: 0},
    replayed: {rows: 0, input_tokens: 0, output_tokens: 0}
  };
  .rows += 1
  | ($row.input.id // "with no id") as $id
  | (if $row.meta.replayed == true then "replayed" else "charged" end) as $side
  | .[$side].rows += 1
  | if $row.meta | has("usage") | not then .no_usage += [$id]
    else
      .[$side].input_tokens += ($row.meta.usage.input_tokens // 0)
      | .[$side].output_tokens += ($row.meta.usage.output_tokens // 0)
    end
)
| .usd = (
    .charged.input_tokens / 1000000 * $usd_per_million_input
    | . * 1000000 | round | . / 1000000
  )
| .usd_per_million_input = $usd_per_million_input
