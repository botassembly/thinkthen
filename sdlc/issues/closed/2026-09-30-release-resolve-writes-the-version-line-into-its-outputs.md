Status: Closed by the quick fix landed as `Land quick fix: release resolve writes only outputs to its output file`. Found by the first release rehearsal, run 36778953361, on 2026-09-30. Owner: ticket 0128 Phase 3b. Resolution: the `resolve` branch of `sdlc/scripts/release-workflow` sends the `versions` check's output to standard error in both modes, so standard output holds only the `sha`, `version`, and `name` lines. `sdlc/scripts/release-archive-self-test.py` runs `resolve` in both modes on every host and requires exactly those three `name=value` lines on standard output; the case failed on the old script. No other `release-workflow` step sends its standard output to a GitHub output, environment, path, or summary file.

Kind: bug

Pay when: before the next rehearsal dispatch.

Keeping it stops every rehearsal and release at its first job, so no runner builds or smokes anything.

# The release `resolve` job writes the version check's line into its outputs

## The problem

The `resolve` job of `.github/workflows/release.yml` runs `sdlc/scripts/release-workflow resolve "$MODE" "$REF" >> "$GITHUB_OUTPUT"`. The `resolve` branch of `release-workflow` (line 283) calls `sdlc/scripts/versions` without a redirect. On success, `versions` prints `versions: 59 places read 0.0.1` to standard output (built at line 223 of `versions` and printed at line 331). That line lands in `$GITHUB_OUTPUT`. GitHub reads each output line as `name=value` and refuses it.

The run's log shows:

```
##[error]Unable to process file command 'output' successfully.
##[error]Invalid format 'versions: 59 places read 0.0.1'
```

The `resolve` job failed in 8 seconds at commit `8bb32e59a`. All other jobs were skipped. The `sdlc/scripts/workflows` self-test reads the workflow's text and never runs `resolve` with an output file, so it missed this.

## A fix

Send the `versions` call's standard output to standard error inside the `resolve` branch, in both modes. Add one self-test case that runs `release-workflow resolve rehearse refs/heads/main` in a fixture checkout and checks that every output line has the form `name=value`.
