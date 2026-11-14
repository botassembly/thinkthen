# Compile the unchanged example against the shared staged Rust package.
set -eu
[ "${SITE_EXECUTE_ONLY:-}" != 1 ] || { echo "Use a prepared executable" >&2; exit 2; }
root=$(CDPATH= cd -- "$(dirname -- "${BASH_SOURCE[0]}")/../../.." && pwd)
packages=${SITE_PACKAGES:-$root/target/site-packages}
if [ "${1:-}" = --prepare ]; then work=$2; mkdir -p "$work"; else
  work=$(mktemp -d)
  trap 'status=$?; trap - EXIT; set +e; rm -rf "$work"; exit "$status"' EXIT
fi
rustc --edition=2024 --extern thinkthen="$packages/rust/libthinkthen.rlib" \
  -L "dependency=$packages/rust/deps" show.rs -o "$work/example"
cat > "$work/program.sh" <<'RUN'
exec "$(dirname "$0")/example"
RUN
if [ -n "${SITE_FUNCTION_NORMALIZER:-}" ]; then
  cp "$SITE_FUNCTION_NORMALIZER" "$work/normalize.sh"
  cat > "$work/run.sh" <<'NORMALIZED'
#!/bin/sh
set -eu
entry=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
work=$(mktemp -d)
trap 'status=$?; trap - EXIT; set +e; rm -rf "$work"; exit "$status"' EXIT
bash "$entry/program.sh" > "$work/output.txt"
cat "$work/output.txt"
bash "$entry/normalize.sh" "$work/output.txt"
NORMALIZED
else
  cp "$work/program.sh" "$work/run.sh"
fi
[ "${1:-}" = --prepare ] || bash "$work/run.sh"
