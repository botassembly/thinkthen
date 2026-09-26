#!/bin/sh
set -eu
cd -- "$(dirname -- "$0")"

. ../../sdlc/scripts/scratch.sh
scratch_dir work

jq -n -c '
  def row($id;$provenance):
    {input:{id:$id},meta:({usage:{input_tokens:10,output_tokens:2}} + $provenance)};
  row("new-stored";{cached:true}),
  row("old-stored";{replayed:true}),
  row("new-live";{cached:false}),
  row("old-live";{replayed:false}),
  row("both-live-wins";{cached:false,replayed:true}),
  row("non-boolean";{cached:"yes",replayed:true})
' > "$work/rows.jsonl"

jq -n --argjson usd_per_million_input 1 -f cost.jq "$work/rows.jsonl" \
  | jq -c '{rows,charged,replayed,usd}' > "$work/report.json"
printf '%s\n' '{"rows":6,"charged":{"rows":4,"input_tokens":40,"output_tokens":8},"replayed":{"rows":2,"input_tokens":20,"output_tokens":4},"usd":0.00004}' \
  > "$work/expected.json"
cmp "$work/expected.json" "$work/report.json"
