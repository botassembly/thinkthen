# Build once, then execute the immutable prepared program without a compiler.
set -euo pipefail
[ "${SITE_EXECUTE_ONLY:-}" != 1 ] || { echo 'Use a prepared Go executable' >&2; exit 2; }
root=$(CDPATH= cd -- "$(dirname -- "${BASH_SOURCE[0]}")/../../.." && pwd)
packages=${SITE_PACKAGES:-$root/target/site-packages}
if [ "${1:-}" = --prepare ]; then work=$2; mkdir -p "$work"; else
  work=$(mktemp -d)
  trap 'status=$?; trap - EXIT; set +e; rm -rf "$work"; exit "$status"' EXIT
fi
cp show.go "$work/main.go"
version=$(awk '$1 == "go" { print $2; exit }' "$root/sdk/go/thinkthen/go.mod")
printf 'module example\n\ngo %s\n\nrequire github.com/botassembly/thinkthen/sdk/go/thinkthen v0.2.0\n\nreplace github.com/botassembly/thinkthen/sdk/go/thinkthen => %s\n' \
  "$version" "$root/sdk/go/thinkthen" > "$work/go.mod"
export PKG_CONFIG_PATH="$packages/c/lib/pkgconfig"
export GOFLAGS=-mod=mod
export CGO_LDFLAGS="-Wl,-rpath,$(pkg-config --variable=libdir thinkthen-c)"
(cd "$work" && go build -tags thinkthen_system -o example .)
cat > "$work/run.sh" <<'RUN'
exec "$(dirname "$0")/example"
RUN
[ "${1:-}" = --prepare ] || bash "$work/run.sh"
