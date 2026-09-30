# The Zig check skips its facts lifetime and allocation modes

Status: open. Found while building ticket 0314 slice 4d on branch `ticket/0314-s4-remaining-ports`. Owner: none yet.

Kind: debt

Pay when: the next change to how the Zig binding owns facts or results.

Keeping it means a leak or a use after free in Zig facts can land unseen, since no gate runs the two modes that test them.

## The problem

`libraries/zig/Tests/run_matrix.py:37-44` has two modes, `facts` (facts outlive a held call, a failure and engine teardown) and `facts-allocation` (each allocation failure after the native call is released with no leak). `libraries/zig/check.sh:64` runs `run_matrix.py` with no argument, so neither mode runs. Slice 4d ran both by hand after facts became `std.json.Parsed(std.json.Value)`, and both passed. Adding the two calls to `check.sh` costs two loopback runs of a few seconds each.
