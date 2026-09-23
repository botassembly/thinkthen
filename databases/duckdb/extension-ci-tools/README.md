# The vendored DuckDB extension build tools

Upstream project: `https://github.com/duckdb/extension-ci-tools`, the build
tooling DuckDB publishes for C API extensions. License: MIT, the same
license as DuckDB's repositories.

Version: the exact upstream commit is not recoverable from the tree this was
copied from (recorded in `../NOTES.md`, "The ci-tools tree, recorded
truthfully"). The files were vendored for the DuckDB `v1.5.5` extension
build (`TARGET_DUCKDB_VERSION=v1.5.5` in `../Makefile`). To re-pin: clone
the upstream repository cleanly, record the commit, and copy the files
listed below.

Five files are kept, because they are the only ones this build path reads:

- `makefiles/c_api_extensions/base.Makefile` — included by `../Makefile`;
  it in turn uses `config/distribution_matrix.json` and
  `scripts/configure_helper.py` and `scripts/append_extension_metadata.py`.
- `makefiles/c_api_extensions/rust.Makefile` — included by `../Makefile`.
- `config/distribution_matrix.json` — read by `base.Makefile`.
- `scripts/configure_helper.py` — run by `base.Makefile` for the platform
  and version.
- `scripts/append_extension_metadata.py` — run by `../check.sh` and
  `../package.sh` to append the metadata footer.

The other twenty-five files the vendoring sweep carried (CI workflows,
Dockerfiles, vcpkg ports, other makefiles) were removed on 2026-09-22; the
build and the gate were rerun against the five-file set.

One local edit: `base.Makefile` pins `packaging==26.3` in the venv step,
the version the provisioned venv holds (surfaces-review-7 R3-29). A
re-vendor keeps the pin; `scripts/check_locked_calls.py` fails without
it.
