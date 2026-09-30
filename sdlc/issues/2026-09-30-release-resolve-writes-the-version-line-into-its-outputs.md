Status: open. Found by the first release rehearsal, run 36778953361, on 2026-09-30. Owner: ticket 0128 Phase 3b.

Kind: bug

Pay when: before the next rehearsal dispatch.

Keeping it stops every rehearsal and release at its first job, so no runner builds or smokes anything.

# The release `resolve` job writes the version check's line into its outputs

## The problem

The `resolve` job of `.github/workflows/release.yml` runs `sdlc/scripts/release-workflow resolve "$MODE" "$REF" >> "$GITHUB_OUTPUT"`. The `resolve` branch of `release-workflow` (line 283) calls `sdlc/scripts/versions` without a redirect. On success, `versions` prints `versions: 59 places read 0.0.1` to standard output (line 223 of `versions`). That line lands in `$GITHUB_OUTPUT`. GitHub reads each output line as `name=value` and refuses it.

The run's log shows:

```
##[error]Unable to process file command 'output' successfully.
##[error]Invalid format 'versions: 59 places read 0.0.1'
```

The `resolve` job failed in 8 seconds at commit `8bb32e59a`. All other jobs were skipped. The `sdlc/scripts/workflows` self-test reads the workflow's text and never runs `resolve` with an output file, so it missed this.

## A fix

Send the `versions` call's standard output to standard error inside the `resolve` branch, in both modes. Add one self-test case that runs `release-workflow resolve rehearse refs/heads/main` in a fixture checkout and checks that every output line has the form `name=value`.
