# 0587: Experiment: the command inside the Python and npm packages

Status: COMPLETE.

Milestone: 0.3

Reviews: revision 0e46dc2, accept

## Outcome

Learn whether the Python wheel and the npm package can carry the `thinkthen` command, so `uvx thinkthen` and `npx thinkthen` work with nothing else installed while the libraries keep working, and how much each package grows.

## Evidence

- Starts from: `libraries/python/build-wheel.sh`, `libraries/python/pyproject.toml`, the Node package under `libraries/`, and today's package sizes.
- Keeps: the release workflow and published packages untouched; prototypes install only from local files. No product code changes, no paid model calls, no publishing, and no agent or machine time taken from 0.2 work.
- Changes: `sdlc/records/0587*` only. Owned numbered experiment folder with `OWNER` reading `thinkthen/0587`; clean installs run in `docker run --rm --label pm.repo=thinkthen --label pm.ticket=0587`.
- Proof: in a clean Linux x86-64 container, `uvx` and `npx` from the local prototype print the version and run `thinkthen mcp` far enough to answer an MCP initialize over standard input; `import thinkthen` and the Node library still pass a smoke call on a recording. Record compressed and installed sizes before and after, and the packaging approach for the other platforms.
- Stop rule: both work, or the record names what blocks each.
- Decision it changes: Ian's decision 16 and PRD 09's MCP Registry entry.
- Machine: the Beelink.
- Defers: other platforms and publishing go to the 0.3 publishing ticket.

## Progress

- 2026-10-14 started
