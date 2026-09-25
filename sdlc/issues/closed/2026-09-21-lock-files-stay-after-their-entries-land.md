# Lock files stay after their entries land

Status: Closed by ticket 0061. A valid final entry lets the owner unlink the digest lock while it still holds the original inode.

A completed `--cache` run leaves one lock file per entry. The cache holds two files for every answer, and the lock files never leave.

## Reproduction

A 200-record run through `filter --cache`, interrupted once and resumed to completion, 2026-09-21:

    $ python3 - <<'EOF'
    import os
    locks = set(os.listdir('cachecv/.locks'))
    entries = {f[:-5] for f in os.listdir('cachecv') if f.endswith('.json')}
    print('locks:', len(locks), 'entries:', len(entries))
    print('lock without entry:', len(locks - entries))
    EOF
    locks: 200 entries: 200
    lock without entry: 0

No process was running. The orchestrator reproduced the same shape on a clean three-record run with no interruption: 3 entries and 3 lock files.

## Expected

ADR 0017 (`sdlc/planning/adr/0017-libraries-over-one-bound-core.md`, line 89): "The lock files leave when the entry lands, or they shard with the entries, so a million entries is never two million files; the million-entry numbers counted no lock files." Two files per answer doubles the file count the ADR measured, and its million-entry arithmetic with it.

## How bad it is for a user

Minor today. The coalescing is correct, and no run was damaged. The cost is disk and directory size, and the broken promise to the ADR's own numbers.

Found by experiment 218, wave 1, area 8.
