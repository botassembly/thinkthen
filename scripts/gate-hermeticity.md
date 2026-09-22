# Gate hermeticity: what is pinned, and what cannot be

Date: 2026-09-22. Source: the second review's phase-4 item, "the gate still
runs npm install, an unpinned uv pip install, a PyPI fetch, and floating
Docker tags." This page records each pin and each honest exception. The
gate is `scripts/check_surfaces.sh`; every surface's `check.sh` runs under
it.

## Pinned

- **npm**: `libraries/typescript/check.sh` runs `npm ci`, which installs
  exactly `libraries/typescript/package-lock.json`. The first provision on
  a clean host needs the network or npm's cache; every later run is
  offline.
- **Python**: `libraries/python/check.sh` installs from
  `libraries/python/requirements-dev.txt`, pinned to the versions the
  checks were last green against (pytest 9.1.1, polars 1.44.2, pandas
  3.0.6, pyarrow 25.0.1). The venv is folder-local and reused; only the
  first provision needs the network or uv's cache.
- **postgres:16**: pinned to
  `postgres:16@sha256:a3b7f434b2dc57ce85a67e171163eb8ab1a1ebcb39d27484661f26b1dfbe30d6`
  (resolved on this host, 2026-09-22) in
  `databases/postgresql/package.sh` and `databases/postgresql/check.sh`.
- **The Rust toolchain in the Ruby builder**: the container's PATH is
  resolved from `/root/.rustup/toolchains/*/bin` instead of naming one
  toolchain directory, so a host toolchain that is not
  `stable-x86_64-unknown-linux-gnu` still works.

## Not pinnable here, with the reason

- **ubuntu:24.04** (`databases/sqlite/package.sh`) and
  **duckdb/duckdb:1.5.5** (`databases/duckdb/package.sh`): no local copy
  existed when this was written, so no digest was resolvable offline. Both
  scripts print the resolved digest on the pull path; record it in this
  page when a rehearsal next pulls.
- **thinkthen-ruby-builder:local** (`libraries/ruby/build.sh`): built
  locally from `libraries/ruby/Dockerfile` with `docker build`, so it has
  no registry digest. The Dockerfile's own base image is the thing to pin
  when the build next runs.
- **The loopback wire stub**: built from this repository
  (`tools/wire-stub`) and started by the gate itself; it needs no network.
  Its crates come from the local cargo registry cache; a cold cache needs
  the network once.
