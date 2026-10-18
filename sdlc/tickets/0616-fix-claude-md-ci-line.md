# 0616: Correct the CI statement in the repo instructions

Status: OPEN.

Milestone: 0.2.1

Phase: 1

## Outcome

- The line says that everyday builds and tests run on the M5 through pm job, that Linux runs in the pinned container, and that hosted workflows run by hand only for releases.
- The statement about the Linux box is removed, or replaced with its real role.

### Proof

- The repo instructions match the workflow triggers: a grep finds no `push:` or `pull_request:` trigger in `.github/workflows/`, and no instruction claims one.

## Evidence

`CLAUDE.md:12` says "Linux CI runs on each push". Every workflow in `.github/workflows/` starts only by `workflow_dispatch` or `workflow_call`, which matches the ruling that GitHub Actions run only for public releases after local testing. The same line names a Linux box that has no commits in this repo's history. A reader following the instructions expects Linux coverage on each push that does not exist.
