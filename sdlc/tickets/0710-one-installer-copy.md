# 0710: Serve the site's installers from the root copies

Status: OPEN.

Milestone: 0.2.1

Phase: 2

Reviews: revision bd47e44, accept

Reviews: revision 3209f5b, accept

## Outcome

- Changes: `site/public/install.sh`, `site/public/install.ps1`, `site/package.json`, `site/scripts/copy-installers.mjs`, `sdlc/scripts/versions`, `sdlc/scripts/windows-installer-test.py`, `sdlc/planning/release-process.md`, `sdlc/scripts/scratch.sh`.
- `site/public/install.sh` and `site/public/install.ps1` are deleted from the tree. The site build copies the root `install.sh` and `install.ps1` into its output, so the site serves the same bytes from one source.
- The copy comparison in `sdlc/scripts/versions` and the matching one in `sdlc/scripts/windows-installer-test.py` are deleted, with their self-test cases. The release process names one installer file to update.

### Proof

- On the M5 through `pm job run --machine m5 0710 SHA --memory 8 -- sh -c 'cd site && npm run build && cmp ../install.sh dist/install.sh && cmp ../install.ps1 dist/install.ps1'`, the built site serves byte-identical installers. Before the change the same command passes only because two tracked copies exist; with the site copies deleted and no build step it fails at `cmp`.
- On the M5 through pm job, `python3 sdlc/scripts/versions --self-test`, `python3 sdlc/scripts/versions` and `python3 sdlc/scripts/windows-installer-test.py --self-test` pass with no site copies. Before the change both scripts fail when the site copies are missing.

## Evidence

- Split from 0632 on 2026-10-19 so the copy cleanup lands without waiting for the musl toolchain.
- The repo tracks `install.sh` and `install.ps1` twice, at the root and under `site/public/`. `sdlc/scripts/versions` reads both copies of `install.sh` and `install.ps1` and fails when they differ. `sdlc/scripts/windows-installer-test.py` refuses in `main` when the root `install.ps1` and its site copy differ.
- The site build is the `build` script in `site/package.json`; `.github/workflows/pages.yml` runs `npm run build`, so a copy step there reaches the deployed site.
- `sdlc/planning/release-process.md` step 7 tells the releaser to update `install.sh` and its site copy.
