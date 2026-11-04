# Compile the unchanged example against the shared staged Rust package.
set -eu
[ "${SITE_EXECUTE_ONLY:-}" != 1 ] || { echo "Use a prepared executable" >&2; exit 2; }
root=$(cd ../../.. && pwd)
packages=${SITE_PACKAGES:-$root/target/site-packages}
if [ "${1:-}" = --prepare ]; then work=$2; mkdir -p "$work"; else
  work=$(mktemp -d)
  trap 'status=$?; trap - EXIT; set +e; rm -rf "$work"; exit "$status"' EXIT
fi
rustc --edition=2024 --extern thinkthen="$packages/rust/libthinkthen.rlib" \
  -L "dependency=$packages/rust/deps" show.rs -o "$work/example"
cat > "$work/run.sh" <<'RUN'
exec "$(dirname "$0")/example"
RUN
[ "${1:-}" = --prepare ] || bash "$work/run.sh"
