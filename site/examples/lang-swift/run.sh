# Compile the unchanged example against the shared staged Swift package.
set -eu
[ "${SITE_EXECUTE_ONLY:-}" != 1 ] || { echo "Use a prepared executable" >&2; exit 2; }
root=$(cd ../../.. && pwd)
packages=${SITE_PACKAGES:-$root/target/site-packages}
build=$(cat "$packages/swift/module-path")
compiler=$(cat "$packages/swift/compiler-path")
if [ "${1:-}" = --prepare ]; then work=$2; mkdir -p "$work"; else
  work=$(mktemp -d)
  trap 'status=$?; trap - EXIT; set +e; rm -rf "$work"; exit "$status"' EXIT
fi
# SwiftPM's resource accessor looks beside the consumer executable.
for resources in "$build/ThinkThen_ThinkThen.resources" "$build/ThinkThen_ThinkThen.bundle"; do
  if [ -d "$resources" ]; then ln -s "$resources" "$work/"; fi
done
"$compiler" -swift-version 6 -I "$build/Modules" -I "$build/CThinkThen.build" \
  show.swift "$build"/ThinkThen.build/*.o "$build"/CThinkThen.build/*.o -o "$work/example"
cat > "$work/run.sh" <<'RUN'
exec "$(dirname "$0")/example"
RUN
[ "${1:-}" = --prepare ] || bash "$work/run.sh"
