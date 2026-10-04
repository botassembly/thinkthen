# 0398 slice A: Check publish inputs during rehearsal

Date: 2026-10-04
Status: candidate for fresh code review
Starting revision: `e760432c80dc22501fa51694964f2eeec5b8e63e`
Ticket: `sdlc/tickets/0398-release-safety.md`
Lane: `claude-2`

## Result

The crate job dry-runs `cargo publish` after source packaging. The wheel job installs pinned Twine 6.2.0 and checks four wheels strictly. The draft job renders the Homebrew formula from four checked Unix archives and checks its Ruby syntax before collection. The release tap operation uses that renderer before any clone, then keeps its previous commit and push behavior. The formula bytes match the former renderer. The release job graph retains eighteen jobs and its existing outward guards.

Twine 6.2.0 exists on [PyPI](https://pypi.org/project/twine/6.2.0/) and supports Python 3.9 and newer. This is a release-run tool pin. No Twine package was installed locally for this change.

## Review

Fresh ticket review `review_0398_ticket` returned ACCEPT for slice A. Its conclusion requires preserving the fifth Windows target when merging later Windows work. This candidate keeps the existing four Unix archives and four wheels. A fresh code review remains pending. The builder will fix findings before landing.

## Proof

- `python3 sdlc/scripts/workflows --self-test`: 81/81 cases hold. New plants remove or misplace the Cargo dry run, ignore or condition it, remove the strict wheel check, unpin Twine, ignore a wheel failure, remove or misplace formula rendering, and ignore its syntax failure. Each pins its refusal.
- `python3 sdlc/scripts/release-archive-self-test.py`: passes. Four synthetic archives produce the exact prior formula bytes. Ruby reports `Syntax OK`. Missing archive, mismatched checksum and mismatched version fail with exit 1 and their pinned refusal. The renderer runs with an explicit environment containing no key variables.
- `python3 sdlc/scripts/workflows`: passes on the real workflow files.
- `CARGO_NET_OFFLINE=true python3 sdlc/scripts/policy.py`: passes; 254 resolved packages checked.
- `python3 sdlc/scripts/tickets`: zero failures.
- `sh -n sdlc/scripts/release-workflow` and `git diff --check`: pass.
- Full `sdlc/scripts/lint`: passes with exit 0 under a user systemd scope with MemoryHigh 6 GiB, MemoryMax 8 GiB, one compiler worker, offline Cargo and the lane 2 heavy lock. Clippy, docs and the inventory pass; the inventory checked 569 declared items and refused four plants.

## What the build taught us

The formula renderer needs no deploy key and no remote access. Running it before cloning also preserves the existing early archive refusal. Workflow substring checks alone would accept `ruby -c ... || true`; the guard therefore requires the exact command line, and a plant proves the refusal. The formula test uses the prior template as an independent byte oracle.

## Deferred proof

No workflow was dispatched, no registry upload ran, and no deploy key or provider credential was read. The authorized local implementation supplies offline proof only. The next rehearsal needs Ian's dispatch approval and will prove Cargo's registry dry run, actual wheel metadata and the runner's Ruby syntax check. Trusted publishers still require release mode. Slices B and C remain open. The coordinator owns the main baseline gates and landing.
