# 0620: Make release.yml one script per target driven by the manifest

Status: OPEN.

Milestone: 0.2.1

Phase: 1

Depends on: 0613, 0614, 0615, 0619, 0622

## Outcome

- Each release job runs one script entrypoint for its target and stage. The script reads the tool manifest to decide what to install and check.
- `release.yml` holds triggers, the job graph, runners, permissions, time limits and passing build outputs between jobs, with no per-language `if:` steps.
- The per-target install steps in the YAML and the `release-workflow` subcommands they called are deleted, and only the per-stage entrypoint is kept.
- Each tool-install step in the hosted Linux jobs that the 0619 image covers is deleted. Those jobs run in the image.
- The workflow takes the candidate name from the dispatched branch, not from a free-text input.
- The same entrypoint runs locally in the pinned Linux container and on the M5 through pm job.

### Proof

- Each stage's entrypoint passes locally through pm job for its matching target: macOS arm64 on the M5, Linux arm64 in the M5 container, Linux x86_64 in the Yellow container.
- The workflow tickets 0613, 0620 and 0622 land together. One dispatched nonpublishing rehearsal at the next checkpoint (0691) proves them: it publishes nothing, its archive list matches `package-inventory.py`, and the installed smokes pass.

## Evidence

`.github/workflows/release.yml` holds most of its steps behind `if: matrix.target == '...'` lines, each installing or checking one language's tools for one target. The release logic is split between this YAML and `sdlc/scripts/release-workflow` (1,062 lines), and each new tool or target adds branches to both. The hosted Linux jobs also install their own tools, which the 0619 image now covers.

The rehearsal takes the candidate name as free text. Run 39117319342 failed on a `candidate_tag` dispatch error.
