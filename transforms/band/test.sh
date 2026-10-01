#!/bin/sh
# A probability at a band's low edge is not sure, and one below it is no, in
# band.jq and score.jq, as specification/threshold.md says.
set -eu
cd -- "$(dirname -- "$0")"

. ../../sdlc/scripts/scratch.sh
scratch_dir work

jq -n -c '
  def row($id; $probability):
    {schema:"thinkthen.result/1",input:{id:$id,label:false},value:null,
     question:{verb:"decide",text:"Fixture question"},
     answer:{kind:"yes_no",probability:$probability},threshold:"0.2:0.8"};
  row("edge"; 0.2), row("below"; 0.19), row("high"; 0.8)
' > "$work/rows.jsonl"

jq -n -c --argjson band '[0.2,0.8]' -f band.jq "$work/rows.jsonl" \
  | jq -c '{resolved,unsure,refused:[.refused[].id]}' > "$work/band.json"
printf '%s\n' '{"resolved":2,"unsure":1,"refused":["edge"]}' | cmp - "$work/band.json"

jq -n -c --argjson cut '[0.2,0.8]' -f ../score/score.jq "$work/rows.jsonl" \
  | jq -c '{unsure,true_negative,false_positive}' > "$work/score.json"
printf '%s\n' '{"unsure":1,"true_negative":1,"false_positive":1}' | cmp - "$work/score.json"
