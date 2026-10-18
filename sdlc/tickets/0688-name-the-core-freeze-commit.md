# 0688: Name the core-freeze commit

Status: OPEN.

Milestone: 0.2.1

Phase: 2

Depends on: 0612, 0626, 0627, 0628, 0629, 0635, 0636, 0637, 0638, 0642, 0643, 0689, 0691, 0692

## Outcome

- One reviewed main commit is named as the 0.2.1 core freeze. It fixes the C header, the final packet schema, the Rust run function, the value tree for the direct Rust bridges, the settings parser and the packet oracle.
- The commit hash is recorded in a decision record, with the rule that any later change to the header, the packet schema, the run function or the value tree needs a ruling, and every language then moves in one batch.
- Findings after the freeze go to the next release unless they block shipping.
- Every phase 3 ticket depends on this ticket.

### Proof

- The decision record names the commit, and that commit passes the 0691 checkpoint run.
- The PM confirms in pm that no phase 3 ticket started before this ticket closed.

## Evidence

Rule 1 of the 0.2.1 ruling says a named commit fixes the C interface and the final result packet, and no language ticket starts before it. In 0.2, the binding design changed four times and the header changed 51 times during binding work, because no commit was ever named as the freeze. Rule 7 closes scope at the same point.
