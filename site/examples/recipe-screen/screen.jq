[.answer.empirical, .answer.topic] as $a
| if any($a[]; .status != "answered") then "\(.id) review"
  elif all($a[]; .answer) then "\(.id) include"
  else "\(.id) exclude"
  end
