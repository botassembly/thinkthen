# Nothing says how the command gets installed

Status: Open. Found by the product side on 2026-09-21 while planning the deck's closing slide. For the build team.

## The gap

Every library and database surface has a planned install line (`pip install thinkthen`, `npm install thinkthen`, `gem install thinkthen`, `cargo add thinkthen`, `INSTALL thinkthen FROM community`, and so on). The shell command has none. No page in `README.md` or `sdlc/planning/` names a channel for the `thinkthen` binary.

## Why it matters

The command is the first thing most readers will try. The deck's shell slide and the site's first page both need one line a reader can paste. `cargo install` reaches only people who already have Rust, and the audience includes DevOps, data, and workflow people who do not.

## The product side's recommendation

For the first release: one Homebrew line and one download script that fetches a prebuilt binary, for Linux and macOS on both common chip types. `cargo install thinkthen` works as a side effect of publishing the crate and needs no promotion. Windows and Linux packages can wait for demand.

The release archive should carry the binary only. The C archive (header, both libraries, the `.pc` file) stays its own download.

## What the deck and site do until this is settled

The closing slide shows no install line and sends readers to the site. The site's install page gets one tab per surface. The library slides keep one canonical line per ecosystem and do not list alternates such as `uv`, `poetry`, `pnpm`, or `yarn`.

Ian can overturn the recommendation. The channel list commits build and release work, so it is the build team's to cost before anyone rules.
