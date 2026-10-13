# PM checks

Each line is a check the PM runs on the build team. None holds status.

- Is the coordinator following the wave order in the current waves ruling, with fast checks only until the candidate?
- Does each landing go to main as one merge, with no status-only commit on main?
- Does outside-in coverage on the latest coverage record stay at or above the baseline, apart from deleted code?
- Does the library size report show any host library above three times the median, or any change that added far more handwritten lines in one language than the others?
- Does the surface manifest lint pass for every language library?
- Are landed worktrees, branches and proof folders gone?
- Does every outside pull request or issue have an owner checking it, and does any ticket marked complete still have an open outside PR or failing outside CI?
- Before a new ticket or amendment ships, has the PM checked it against the ADRs, the README contract, the specification and other open tickets it touches, and does each amendment name what it replaces?
- Does each measurement or size target in a ticket state its accepted variance, what gets committed and the comparison rule, or cite the standing rulings?
- Is the coordinator pane working, or has it stopped on a capacity error or an unanswered question for more than a few minutes?
