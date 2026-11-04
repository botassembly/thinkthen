#!/bin/sh
# Compile the shown source against the site's staged installed package.
set -eu
[ "${SITE_EXECUTE_ONLY:-}" != 1 ] || { echo "Use a prepared program" >&2; exit 2; }
root=$(cd ../../../../.. && pwd)
packages=${SITE_PACKAGES:-$root/target/site-packages}
if [ "${1:-}" = --prepare ]; then
  work=$2
  mkdir -p "$work"
else
  work=$(mktemp -d)
  trap 'status=$?; trap - EXIT; set +e; rm -rf "$work"; exit "$status"' EXIT
fi
trap 'exit 1' HUP INT TERM
export PKG_CONFIG_PATH="$packages/c/lib/pkgconfig"
c++ -std=c++17 -Wall -Wextra -Werror show.cpp \
  $(pkg-config --cflags --libs thinkthen-c) \
  -Wl,-rpath,"$packages/c/lib" -o "$work/example"
cat > "$work/run.sh" <<'RUN'
#!/bin/sh
exec "$(dirname "$0")/example"
RUN
[ "${1:-}" = --prepare ] || sh "$work/run.sh"
