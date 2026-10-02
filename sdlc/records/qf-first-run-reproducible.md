# Quick Fix qf-first-run-reproducible: every target builds the same first-run archive

Status: built in lane claude-3; fresh review accepted. Ian can overturn the stored gzip blocks.

## Why

Release rehearsal run 36937157758 passed every build and all four smoke jobs for the first time. The `draft` job then stopped in `release-workflow collect` with "first-run differs between targets". `collect` requires each target's `thinkthen-first-run.tar.gz` and its `.sha256` to match byte for byte.

All four copies differed. Each held the same `report.txt` and `recording/thinkthen.jsonl` bytes. `release-pack` packed the archive with the host's `tar -czf`:

- Owners: `runner/runner` on Linux and `runner/staff` on macOS.
- Entry times: each runner's build time, 19:05 to 19:30.
- Entry order: GNU tar follows directory order, so the x86 Linux copy put `report.txt` before `recording/`. The ARM Linux copy and both Mac copies put it after.
- The gzip header: GNU tar's gzip wrote time 0, and macOS bsdtar wrote its build time.

## Change

`release-pack`'s first-run part writes the archive with Python's `tarfile`, in USTAR form. It holds `thinkthen-first-run/`, `recording/` and its files in sorted order, then `report.txt`. Every entry has owner 0, no owner names, time 0, and mode 755 for folders or executables and 644 for other files. The gzip layer has time 0, no file name, and stored deflate blocks. Compressed blocks would depend on each runner's zlib build. The archive grows from about 720 bytes to 10,263 bytes. The `collect` comparison is unchanged.

The extracted layout is the same as before. Each consumer extracts `thinkthen-first-run/` and reads the files inside it. The old leading `./` entry is gone.

## Checks

- Linux x86_64 (Python 3.12) and the M5 (Darwin arm64, Python 3.9.6) built the same bytes: SHA-256 `f0f187f0b45fd8b6f3728282d9e9618892576038f273efebeaaffecc20c233ce`. macOS bsdtar extracted the M5 copy.
- `release-archive-self-test.py` rebuilds the archive after new file times and umask 002. It requires the same bytes, a zero gzip time, and the pinned entry names, modes, owners and times. On main's `release-pack` it fails on the `./` entry, owner 1000 and the file times.
- Run 36937157758's artifacts with the new archive in each platform folder: `release-workflow collect` passes and gathers 82 files. Every `.sha256` checks.
- The run's x86_64 musl command answered `true` on the new archive's first run, offline, from a clean home.
- `release-workflow draft rehearse` ran against those 82 files with a stand-in `gh` that records its calls. It checked the SHA, looked for an existing release, proved the tag missing, created the draft, proved the tag still missing, and uploaded the 82 files. No GitHub call was made.
- `sdlc/scripts/lint` exit 0. `CARGO_NET_OFFLINE=true python3 sdlc/scripts/policy.py` exit 0. The release self-tests pass: archive, registry, managed pair, language tools, both smoke tests, and the workflow check.
- Not run: the release workflow.

## Read ahead

- The real `draft` makes four GitHub calls with the job's token: `gh release view`, `gh api --include` on the tag ref, `gh release create --draft --target SHA`, and `gh release upload` of 82 files, about 207 MB. The local run found nothing wrong. Only a real run proves that `gh api --include` puts its 404 status line on standard output for the tag check.
- A rehearsal leaves its draft release. A second rehearsal of the same commit stops at "release ... already exists" until someone deletes that draft.
- Every job after `draft` runs only in release mode, so a rehearsal ends at `draft`.
- No clear bug showed in the release-mode jobs. The four wheels, four gems, the npm package, and the four command archives that `tap` reads are present. Each command archive holds `thinkthen` at its root, as the formula's `bin.install` expects.

## Deferred gap

The other per-target archives still use the host's `tar -czf`. Nothing compares them across targets, so they need no change now.
