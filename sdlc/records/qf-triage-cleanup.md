# Quick Fix: guarded review JSONL cleanup

Candidate branch: `ticket/qf-triage-cleanup`, from `origin/main` at
`ffe21b48c6b1b76e40827b37e03ce3987c0da0b7`. This record covers only
`review-jsonl`, its existing demo self-test, and the reported lint issue.

## Behavior and proof

`review-jsonl` used to reserve the caller's output with `mkdir`, then run
`rm -rf` on that output for any failure. The focused `scratch_lint` check
failed on that line before the fix. The script now checks the output name,
stages in a same-parent directory made by `scratch_dir`, and lets its guarded
trap remove only that scratch path after failure. After the saved replay
returns the expected annotate exit 7 and all six schemas and transforms pass,
`mv -Tn` publishes the completed directory without replacing a caller's
file, directory, or symlink. A raced symlink refusal leaves its target bytes
untouched. The script does not publish partial output after a failed replay.

Before the changed cleanup's first run, a lane-path plant called
`scratch_remove` on a folder not made by `scratch_dir`; the guard refused it,
and its `caller-data` file remained unchanged. The test then removed its own
plant. The actual replay uses the saved recording and makes no provider call.

| Focused check | Result |
| --- | --- |
| `scratch_lint demos/16-triage-pipeline/review-jsonl` before fix | Failed on the unguarded recursive `rm` |
| Lane-path guard plant before changed cleanup ran | Refused; caller bytes unchanged |
| `bash -n` on the script and existing self-test | Passed |
| `scratch_lint demos/16-triage-pipeline/review-jsonl` after fix | Passed |
| `demos/16-triage-pipeline/self-test` | Passed: saved replay has five decisions and one review row; existing paths, failed annotate, and a raced symlink preserve caller data and clean scratch |
| `mustmatch test README.md` in demo 16 with the warm local binary on `PATH` | 3 passed, 2 skipped |

The new self-test assertions use the real compiled replay for successful
output and a thin executable wrapper only to force annotate exit 67 or pause
before publication. They check the actual caller file bytes, directory
contents, and symlink after refusals. The prior self-test did not exercise
`review-jsonl` at all. Its triage flow, request disclosure, validation, move,
and signal cases remain; no test was retired. The page was unchanged.

The first page attempt used a demo-relative `PATH` and could not find the
binary. Repeating the same page check with the absolute warm build path
passed. This was a check setup error, not a source failure. No compilation,
SDK install, container, provider, or full test/spec/surfaces rung ran.

## What the build taught us

- The standalone review route inherited an output cleanup pattern from the
  original triage demo, but rule 11 requires the checked scratch helper. A
  stage in the output's parent permits one final directory publication while
  keeping failed work inside the guarded path.
- A raced symlink made GNU `mv -Tn` exit nonzero, so the first self-test run
  found an exit 1 rather than the intended existing-output refusal. The
  corrected script maps that conflict to exit 2 and preserves the target.
- The page can use the warm binary directly when its absolute build directory
  is placed on `PATH`; a relative `PATH` from the demo folder is wrong.

## Integration limit

The issue's `lint`-on-main criterion remains open. The coordinator owns the
single full lint checkpoint after focused checks, fresh review, and main
integration. This candidate claims only its focused checks and branch state.
