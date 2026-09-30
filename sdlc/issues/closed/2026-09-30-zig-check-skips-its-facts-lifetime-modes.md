# The Zig check skips its facts lifetime and allocation modes

Status: Closed by the quick fix landed as `Land quick fix: the Zig check runs its facts modes and the Flutter host test drops its Dart copy`. Found while building ticket 0314 slice 4d on branch `ticket/0314-s4-remaining-ports`. Resolution: `libraries/zig/check.sh` now runs `run_matrix.py facts` and `run_matrix.py facts-allocation` after the full matrix, so every Zig check runs both modes. The Zig check passed on the branch with both modes: facts exact=4 and post-native facts allocation faults exact=4. The Zig package holds no `test` blocks, so these two matrix modes are the facts lifetime proof.

Kind: debt

Severity: low

Paid: 2026-09-30

Pay when: the next change to how the Zig binding owns facts or results.

Keeping it means a leak or a use after free in Zig facts can land unseen, since no gate runs the two modes that test them.

## The problem

`libraries/zig/Tests/run_matrix.py:37-44` has two modes, `facts` (facts outlive a held call, a failure and engine teardown) and `facts-allocation` (each allocation failure after the native call is released with no leak). `libraries/zig/check.sh:64` runs `run_matrix.py` with no argument, so neither mode runs. Slice 4d ran both by hand after facts became `std.json.Parsed(std.json.Value)`, and both passed. Adding the two calls to `check.sh` costs two loopback runs of a few seconds each.
