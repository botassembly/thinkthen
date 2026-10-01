# Four transforms score a probability at the band's low edge as no

Status: closed 2026-09-30 by ticket 0364 (`4a1cb5cf7`). Reported by the site team on 2026-09-30 and confirmed on main `d75a4bdf1`. Owner: the queue owner, as batch C3 in `../planning/issue-priorities-2026-09-30.md`. Resolution: `band`, `score`, `sweep` (both places) and `trials` treat p = LOW as not sure, as `specification/threshold.md` says. `transforms/band/test.sh` pins the edge.
Kind: bug

`specification/threshold.md:17-19` puts a band's low edge on the not-sure side, and `crates/thinkthen/src/core/threshold.rs:148` answers no only when `p < low`. These transform lines use `<=` and score the edge as no:

- `transforms/band/band.jq:31`, and its header at lines 6-7 ("at or below low is no").
- `transforms/score/score.jq:30`.
- `transforms/sweep/sweep.jq:65`.
- `transforms/trials/trials.jq:108`.
- `transforms/sweep/sweep.jq:419`, in `decision_value`, which rejects a real row whose probability equals the low edge with "mapped decision value must follow its probability and threshold".

Saved probabilities round to two places, so a band such as `0.2:0.8` meets the edge whenever p is exactly 0.2. `thinkthen transform show` ships the same lines.

Fix: change `<=` to `<` in the five places and the band header. Update `transforms/trials/test.sh:33` (row "band-low" expects `false` and becomes `null`), and rerun the band and score examples.
