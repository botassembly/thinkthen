# The Python surface, lane notes

## 2026-09-21 — the lane lands

Tooling already present at the user level from 205: maturin 1.15.0 (`uv
tool`), uv. Folder-local venv `.venv` (CPython 3.13), made with
`uv venv .venv && uv pip install --python .venv/bin/python pytest pandas`;
removal is `rm -rf .venv`. pandas is transient, for the slide sample's
data frame only. Nothing was installed outside this folder; no key, no
paid call, nothing published, no sudo.

The shim binds `thinkthen-contract`, never the stand-in beneath it:
pyo3 0.29.2, `abi3-py310`, module `thinkthen._thinkthen` behind a
`thinkthen` package whose `__init__` owns the data-frame arm of
`annotate`. `panic = "unwind"` in the release profile, per the ADR. All
eight verbs plus `decide_many`, `question`, `details`, `usage`,
`reset_usage`; `deadline` in seconds on every entry point; the six kinds
map to exception classes that carry `kind` and `retryable`; `Cancelled`
subclasses `KeyboardInterrupt` (pyo3 gives an exception one base, per
the 211 shape); a bulk wait runs the signal poll and raises within a
tick plus one in-flight round.

## The check, by command

```
$ ./check.sh                      (stub on 8211 at 300 ms)
surface tests, null backend:   18 passed
conformance slice, offline:    18 passed, 0 failed, 1 skipped
slide sample:                  runs as drawn, below
cancel on the wire:            raised Cancelled, nothing served after (32)
```

Without the stub the script says `wire tests skipped` and everything
offline still runs. Conformance 17 (usage and cache) is the recorded
divergence: the stand-in holds no disk cache, so it counts 2 sends and 0
cache answers where the case expects 1 and 1. Conformance 18 (cancel)
runs as `tests/test_cancel.py` on the wire: SIGINT at t+1 s into a
40-record batch at width 8 raised `Cancelled` with the stub's count
frozen at 32 through a 2 s settle.

## The slide sample, as drawn

```
decide          -> True   (comment: True)
band decide     -> False   (comment: None)
filter          -> []   (comment: none drawn)
annotate        -> columns ['body', 'team', 'urgency', 'wants_refund']
  three new columns: ['team', 'urgency', 'wants_refund']
```

Two findings, both reported and not hidden:

1. **The band comment does not reproduce.** `tt.decide(refund, "I was
   charged twice. Can you fix this?")` answers `False` on the stand-in:
   the evidence carries none of the three-bucket keywords, so its
   probability falls below the band's low side. The real backend's
   recording put this evidence inside the band (`None`), which is where
   the slide's comment came from. The slide changes: either the comment
   or the evidence. This is the finding Phase A predicted.
2. **The column order follows the frame, not the set.** `wants_refund`
   lands last because `annotate` attaches in the set's name order but
   pandas places new columns at the end in insertion order
   (`team`, `urgency`, `wants_refund`); the set's own order is
   `wants_refund`, `team`, `urgency`. Three new columns either way. If
   the slide means to promise set order in the frame, the wrapper needs
   an explicit reorder; the slide's comment only promises the columns.

## The stub's wire shapes

The loopback stub answers one noul shape a request. The `choose`,
`score`, `tag`, and `annotate` wire shapes (a `choice` question with
criteria, a `score` keyed by level) cannot be served by it, so those
verbs run on the stand-in's offline backend and on recordings, the same
reason job 3 marked five exchanges `shaped-to-contract`. `decide`,
`decide_many`, `filter`, and `rank` are proven on the wire by the
conformance slice and the cancel proof.

## The digest equality

`tt.question(decide=..., threshold=(0.2, 0.8), true_=..., false_=...)`
and `tt.question(file="refund.json")` give the same digest, which the
test asserts. The yes and no arms are grammar keys like the threshold;
parts and files agree once the parts carry them.
