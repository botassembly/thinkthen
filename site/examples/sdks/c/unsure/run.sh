#!/bin/sh
# Compile this cell's shown source once against the staged installed package.
set -eu
[ "${SITE_EXECUTE_ONLY:-}" != 1 ] || { echo "Use a prepared program" >&2; exit 2; }
root=$(CDPATH= cd "$(dirname "$0")/../../../../.." && pwd)
packages=${SITE_PACKAGES:-$root/target/site-packages}
if [ "${1:-}" = --prepare ]; then
  work=$2
  mkdir -p "$work"
else
  work=$(mktemp -d)
fi
export PKG_CONFIG_PATH="$packages/c/lib/pkgconfig"
${CC:-cc} -std=c11 -Wall -Wextra -Werror show.c \
  $(pkg-config --cflags --libs thinkthen-c) \
  -Wl,-rpath,"$packages/c/lib" -o "$work/example"
if [ -f normalize.sh ]; then
  cp normalize.sh "$work/normalize.sh"
elif [ -f ../c/normalize.sh ]; then
  cp ../c/normalize.sh "$work/normalize.sh"
fi
cat > "$work/run.sh" <<'RUN'
#!/bin/sh
set -eu
program=$(CDPATH= cd "$(dirname "$0")" && pwd)
if [ -f "$program/normalize.sh" ]; then
  output=$(mktemp)
  status=0
  "$program/example" > "$output" || status=$?
  cat "$output"
  [ "$status" -eq 0 ] || exit "$status"
  sh "$program/normalize.sh" "$output"
else
  exec "$program/example"
fi
RUN
[ "${1:-}" = --prepare ] || sh "$work/run.sh"
