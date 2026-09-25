# diff's McNemar leaves out pairs that become right from a not-sure answer

Status: Open. Moved here on 2026-09-25 from Beatles Bench `sdlc/issues/2026-09-24-diff-mcnemar-leaves-out-pairs-that-become-right-from-not-sure.md`. That file is closed and points here.

## Why it moved

Beatles Bench ticket 0011 retires the prototype `scripts/tools/measure.py` and its golden files. The shipped `thinkthen diff` replaces them. The rule now lives only in this repository, so the ruling belongs here. `crates/thinkthen/tests/fixtures/measure/` already holds copies of the goldens.

## What happens

`specification/diff.md` says the test with a key runs "on the discordant pairs of right answers". The one function `discordant` in `core/measure/diff.rs` counts only pairs that go from wrong to right or from right to wrong. A pair that goes from tied or not sure to right is left out. So is the reverse. McNemar on right answers treats each case as right or not right. Under that reading those pairs are discordant too.

Example: in the `diff-choose` golden, right answers go from 2 to 4 and p is 1.0. Counting every pair that changed between right and not right gives p = 0.5.

Record 0114, "McNemar", kept the narrow rule until the prototype's owner ruled. The prototype is now retired, so no upstream golden holds the rule in place.

## Options

1. Count a pair as discordant when it is right in one run and not right in the other, whatever the other answer was. `specification/diff.md`, "Switching the McNemar rule", lists the steps: change `discordant`, its test, and the goldens in one commit.
2. Keep the narrow rule. Change the specification sentence to say "wrong to right against right to wrong", so the words match the code.

Recommendation: option 1. It matches the words "on right answers", and a user who reads the table sees a not-sure answer that turned right as a gain. Ian can overturn it.
