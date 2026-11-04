[.screen.trial, .screen.lung] as $a
| if any($a[]; . == false) then "\(.id) exclude"
  elif all($a[]; . == true) then "\(.id) include"
  else "\(.id) person"
  end
