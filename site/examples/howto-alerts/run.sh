thinkthen choose --from lines --json \
  --choice storage --choice performance --choice security --choice other \
  'Which kind of problem does this alert report?' alerts.log |
  jq -r '[.position, .status, (.answer // "-")] | @tsv'
echo '--- counts'
thinkthen choose --from lines --json \
  --choice storage --choice performance --choice security --choice other \
  'Which kind of problem does this alert report?' alerts.log |
  jq -s -c 'group_by(.answer) | .[] | {kind: .[0].answer, alerts: length}'
