# PM checks

Each line is a check the PM runs on the build team. None holds status.

- Is the coordinator following the wave order in the current waves ruling, with fast checks only until the candidate?
- Does each landing go to main as one merge, with no status-only commit on main?
- Does outside-in coverage on the latest coverage record stay at or above the baseline, apart from deleted code?
- Does the library size report show any host library above three times the median, or any change that added far more handwritten lines in one language than the others?
- Does the surface manifest lint pass for every language library?
- Are landed worktrees, branches and proof folders gone?
