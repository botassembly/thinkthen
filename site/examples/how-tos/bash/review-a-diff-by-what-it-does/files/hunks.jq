reduce inputs as $line ({file: null, hunks: []};
  if ($line | startswith("+++ b/")) then
    .file = $line[6:]
  elif ($line | startswith("@@")) then
    .hunks += [{file, at: $line, hunk: ""}]
  elif (.hunks | length) > 0
    and ($line | test("^(diff |--- )") | not) then
    .hunks[-1].hunk += $line + "\n"
  else
    .
  end)
| .hunks[]
