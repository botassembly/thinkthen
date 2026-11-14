# Type-check and run the unchanged example against the staged npm package, offline.
set -eu
[ "${SITE_EXECUTE_ONLY:-}" != 1 ] || { echo "Use a prepared executable" >&2; exit 2; }
root=$(CDPATH= cd -- "$(dirname -- "${BASH_SOURCE[0]}")/../../.." && pwd)
packages=${SITE_PACKAGES:-$root/target/site-packages}
if [ "${1:-}" = --prepare ]; then work=$2; mkdir -p "$work"; else
  work=$(mktemp -d)
  trap 'status=$?; trap - EXIT; set +e; rm -rf "$work"; exit "$status"' EXIT
fi
trap 'exit 1' HUP INT TERM
mkdir "$work/node_modules"
ln -s "$packages/node/thinkthen" "$work/node_modules/thinkthen"
printf '%s' '{"type":"module"}' > "$work/package.json"
printf '%s' '{"compilerOptions": {"strict": true, "noEmitOnError": true, "target": "es2022",
  "lib": ["es2022", "esnext.disposable", "dom"], "types": [], "module": "nodenext",
  "moduleResolution": "nodenext", "outDir": "out"}, "files": ["show.ts"]}' > "$work/tsconfig.json"
cp show.ts "$work/"
"$root/site/node_modules/.bin/tsc" -p "$work"
cat > "$work/program.sh" <<'RUN'
exec node "$(dirname "$0")/out/show.js"
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
if [ "${1:-}" != --prepare ]; then bash "$work/run.sh" > "$work/output.txt"; cat "$work/output.txt"; fi
