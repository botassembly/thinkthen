# Audit and diff cite a removed Beatles Bench commit

Status: Closed 2026-09-26 by Quick Fix qf-h1-h3-h6.

Filed 2026-09-26 by the marketing lead.

## Problem

Three pages name Beatles Bench commit `be7cea2e` as the source of `thinkthen audit` and `thinkthen diff`:

- `specification/audit.md` line 7
- `specification/diff.md` line 7
- `crates/thinkthen/tests/fixtures/measure/README.md` line 3

Ian replaced the Beatles Bench history on 2026-09-26 with one commit, `2cdb6445`. The old commits no longer exist on GitHub or in the bench checkout. A reader who follows the citation finds nothing.

## Evidence

- `git -C repos/beatles-bench log --oneline` prints one commit, `2cdb6445 Publish Beatles Bench`.
- `git -C repos/beatles-bench cat-file -t be7cea2e` fails.
- The fixture README gives each file's checksum, so the files themselves stay verifiable.

## Proposed fix

Say that the prototype script was the source, and cite the fixture checksums as the record. Drop the dead commit hash, or name the bench path where the script now lives at `2cdb6445` if it still exists there. The pages change together.
