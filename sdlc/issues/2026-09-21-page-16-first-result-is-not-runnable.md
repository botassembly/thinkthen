# Make page 16's first result runnable

Status: Open

Found 2026-09-21 during a cold marketing read of `demos/16-triage-pipeline/README.md` at commit `1caae5a`.

## Finding

The first command on the page tells a new reader to run `./triage "$work/triage" --replay recording/` and immediately assert `draft=2`, `block=1`, and `review=3` (README lines 9-20). The page's `recording/` directory is absent. With the checked-out binary and the command run from the demo directory, replay stops before the first record with `the replay folder holds no entry named ...` and exits 5. The reader therefore cannot see the advertised first result. The page also says the reviewed recording is pending (line 7), so this is known page state rather than a transient setup problem.

## Fix

Commit the reviewed recording before presenting this page as a runnable how-to, or change the first block to a fixture-backed command that exists in the page folder. Keep the expected three-file counts and the `reviewed_action` comparison visible when the page turns green.

## Evidence

- `demos/16-triage-pipeline/README.md:7-20` promises six rows and the three counts, then replays `recording/`.
- `demos/16-triage-pipeline/` contains no `recording/` directory.
- Local command: `PATH=target/debug:$PATH ./demos/16-triage-pipeline/triage <temp-output> --replay demos/16-triage-pipeline/recording/` exits 5 before processing record 1.
