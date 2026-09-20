# Lessons from BioMCP for release, install, CI, and the docs site

Status: Open. A checklist for the release tickets and the site ticket. Nothing here changes ticket 0015.

Ian asked on 2026-09-20 what ThinkThen should take from BioMCP, his public command-line tool with the same shape: a Rust binary, a `curl` installer, a PyPI package, a Homebrew tap, and a docs site on GitHub Pages at a custom domain. Two read-only surveys read its workflows, scripts, records, and commit history. BioMCP is MIT and Ian's own, so its scripts can be copied outright.

## What ThinkThen already does as well or better

Read by command on 2026-09-20. `gate.yml` pins every action to a commit and sets `permissions: contents: read`. `sdlc/scripts/spec` and `install` fail loudly when `mustmatch` or another tool is missing from `PATH`. BioMCP's build-host notes record the opposite trap: its ratchet tool exits 1 with no output when `mustmatch` is missing. ThinkThen's demos and spec pages already run as tests, and that is stronger than BioMCP's checks of its docs.

## Release: copy these

1. **One version, one file, one check.** `Cargo.toml` holds the version. BioMCP's `scripts/check-version-sync.sh` compares eight other files against it and runs as a CI job. ThinkThen will have more mirrors than BioMCP: five language packages, three database extensions, a header for C, and the site. Write the check with the first package.
2. **Pin every release job to the tagged commit.** BioMCP added this late, after release jobs could build a moving `main`.
3. **A checksum file beside every archive**, and `shasum` on macOS runners, where `sha256sum` does not exist. BioMCP shipped wrong checksums once.
4. **Prove the matrix before the day it matters.** BioMCP's first cross-compile matrix broke on macOS and on Linux arm64. Its release workflow was also "slimmed" during a release and had to be restored. Run a full dry release to a draft, and never change the release workflow in the cycle that ships.
5. **Smoke-test every published file.** BioMCP's 0.9.0 binary from PyPI overflowed its stack on one command while the GitHub release binary did not, because the two were built differently. Each archive, wheel, gem, and package runs the conformance cases under replay before it is published.
6. **Publish to PyPI with trusted publishing.** BioMCP uses GitHub's OIDC and a `pypi` environment, with no stored token. npm and RubyGems now offer the same, and crates.io does too. Check each before a token is ever stored. Unchecked for RubyGems and crates.io on 2026-09-20.
7. **One name everywhere.** `biomcp` was taken on PyPI, so the package is `biomcp-cli`, and its description has to warn people off the other one. The `thinkthen` name was free on every registry on 2026-09-20. Ian's todos to claim it matter for this reason.
8. **Add static Linux targets.** BioMCP ships glibc builds alone. ThinkThen's blocking client with `rustls` makes a musl build easy, and one static file runs on any Linux.

BioMCP wrote its matrix by hand. The library plan names `cargo-dist`. My recommendation: start from BioMCP's `release.yml`, because it is proven and Ian's, add the musl targets, and weigh `cargo-dist` after the launch. Lesson 4 argues against a new release tool in the cycle that ships.

## The installer: copy BioMCP's `install.sh` almost whole

It already does what a careful reviewer will look for.

- It refuses to install when the checksum cannot be fetched or does not match.
- It runs the staged binary's `version` and checks it against the requested version before it replaces the old binary.
- It resolves "latest" by skipping a release whose archive for this platform is not uploaded yet, and falls back to the redirect address when the API or `jq` is missing. That closes the window where a release exists and its binaries do not.
- It writes a receipt in two steps, pending then installed, so an interrupted upgrade is seen.
- It refuses a symlinked install folder or binary.
- It prints the `PATH` line and never edits a shell profile.
- `set -euo pipefail` and a cleanup trap mean a partial download leaves nothing behind.
- The root `install.sh` and the copy the site serves are compared byte for byte in CI.

Serve it from `thinkthen.dev/install.sh`. BioMCP has no uninstall. Add one line to the page that says which file to delete.

## The docs site: the one expensive lesson

BioMCP's docs deploy failed 119 times in a row and nobody saw it, because the site kept serving the last good build and answered 200. Record 1096 in that repository has the story. The fix was a step after every deploy that polls the live site until it serves the exact commit, and fails the job when it does not.

For Astro on GitHub Pages:

1. Use the official path: `actions/configure-pages`, `withastro/action`, and `actions/deploy-pages`. The custom domain is a `CNAME` file in `public/`.
2. Put the commit in a small `version.json`, and end the workflow with a poll of `thinkthen.dev/version.json` for that commit.
3. Fail the build on a broken link. Starlight has a links validator plugin. BioMCP gets this from `mkdocs build --strict`.
4. Ship `llms.txt` and `llms-full.txt`, and a raw Markdown twin of every page at the same address with `.md` added. BioMCP does this with a small build hook. Agents are one of ThinkThen's three audiences, so this is part of the product.
5. Build the how-to pages from `demos/`, so the page a visitor reads is the page the gate ran. Do not keep a second copy.
6. Search, a sitemap, and social cards come with Starlight and its integrations. BioMCP sets its cards by hand.

## The README and the first five minutes

- BioMCP's quick start is five commands that work with no key and no account. ThinkThen's first example should run under `--replay` from a recording the release ships, because a new user waits about a day for a key.
- BioMCP has no terminal recording. Its one demo is a long conference video. ThinkThen should script its recordings with VHS and run them under `--replay`. They then cost nothing, need no key, and come out the same every time.
- BioMCP has no badges. Add three: the gate, the version, and the license.

## Community files the repository lacks today

Read by command on 2026-09-20: `LICENSE` exists. `SECURITY.md`, `CONTRIBUTING.md`, `CODE_OF_CONDUCT.md`, `CHANGELOG.md`, `CITATION.cff`, and an issue template do not. BioMCP lacks the security page and the issue template too. Before the repository goes public, add all six. The changelog names breaking changes under their own heading. One issue template asks for the command, the output, and the version, as `launch.md` in the marketing repository already decided. Fill the repository's About box, its topics, and its social preview image on the same day.

## For agents, after the launch

BioMCP ships a skill, a plugin, and a registry entry, each checked against the version in CI. ThinkThen is not an MCP server and should not become one. A published skill file that teaches an agent the eight verbs, the exit codes, and `--dry-run` is the cheap equivalent, and it serves the audience of people who build agents.

## What Ian can overturn

All of it. The one judgment call is starting from BioMCP's release workflow in place of `cargo-dist`.
