#!/usr/bin/env bash
# No release build carries the builder's home directory (surfaces-review-5:
# the PostgreSQL extension carried 163 home paths, and the DuckDB and R
# builds passed no remap).
#
# With no argument, the gate form: every build entry point that runs cargo
# on the host passes --remap-path-prefix for $HOME. The Ruby build runs in
# a container rooted at /src and needs none.
# With a directory, the artifact form: every release library under it is
# scanned, and a tree with no artifact fails rather than passing empty.
# Usage: bash scripts/check_artifact_paths.sh [BUILT-TREE]
set -euo pipefail
cd "$(dirname "$0")/.."
bad=0
if [ $# -eq 0 ]; then
  for entry in libraries/python/build-wheel.sh libraries/python/check.sh \
    libraries/typescript/build-addon.sh libraries/c/check.sh \
    libraries/r/thinkthen/src/Makevars.in \
    databases/duckdb/check.sh databases/duckdb/package.sh \
    databases/sqlite/check.sh databases/sqlite/package.sh \
    databases/postgresql/check.sh databases/postgresql/package.sh; do
    if ! grep -qE -- '--remap-path-prefix=\$\(?HOME\)?=/build' "$entry"; then
      echo "FAIL     $entry: the build passes no --remap-path-prefix for the builder's home"
      bad=1
    fi
  done
  [ "$bad" -eq 0 ] && echo "ok:      every host build entry point remaps the builder's home"
  exit "$bad"
fi
tree=$1
home=${HOME:?HOME is unset}
seen=0
while IFS= read -r artifact; do
  seen=$((seen + 1))
  hits=$(strings -a "$artifact" | grep -cF -- "$home" || true)
  if [ "$hits" -ne 0 ]; then
    echo "FAIL     ${artifact#"$tree"/}: $hits strings carry the builder's home"
    bad=1
  fi
done < <(find "$tree" \( -path '*/release/*' -o -path '*/dist/*' -o -path '*/lib/thinkthen/*' -o -name '*.node' -o -name '*.duckdb_extension' \) \
  \( -name '*.so' -o -name '*.node' -o -name '*.duckdb_extension' -o -name '*.dylib' -o -name '*.a' \) -type f 2>/dev/null \
  | grep -vE '/(deps|build|examples|incremental)/' | sort)
if [ "$seen" -eq 0 ]; then
  echo "FAIL     $tree: no built artifact to scan"
  exit 1
fi
[ "$bad" -eq 0 ] && echo "ok:      $seen built artifacts carry no home path"
exit "$bad"
