# 0676: Delete Objective-C value mapping and use the run call

Status: OPEN.

Milestone: 0.2.1

Phase: 3

Depends on: 0688

## Outcome

- `ThinkThen.m` calls the single run-call entry point.
- `TTResult` exposes the packet's final answer, probability, pick and resolution through generated accessors.
- Hand-written number conversion and answer accessors are deleted.
- The Objective-C session loop and its private closed, reader and producer error text are deleted. Async calls run the call on one dispatch queue worker and cancel through the handle.
- Its files leave the 0637 answer-rule check's list of exceptions.

### Proof

On the M5 through pm job, the installed Objective-C consumer runs all ten named calls against a recorded backend and its packet dump matches the 0642 oracle, including absence, null, false and failure.

## Evidence

Objective-C maps numbers and answers by hand, and that mapping thrashed whenever the core result shape changed:

- five commits on decimal-range number conversion (6e7b37620, 4a22a18ea, 86b7857dc, 2d512ee33, a44dc2295);
- about eight on the filter answer accessor (e0c9359d3 through 0d16adfe8);
- repeated reshaping of call value and probability (d8e403036, c32c2ad3f, 1d5c7285c).
