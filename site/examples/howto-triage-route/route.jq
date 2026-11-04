.triage as $t
| if any($t.refund, $t.topic; . == null or . == "unsure")
  then "\(.id) person"
  elif $t.refund == true then "\(.id) refunds"
  else "\(.id) \($t.topic)"
  end
