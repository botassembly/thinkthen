# Build the unchanged source once, or execute its retained program.
set -eu
[ "${SITE_EXECUTE_ONLY:-}" != 1 ] || { echo 'Use a prepared Go executable' >&2; exit 2; }
export PKG_CONFIG_PATH="$(cd ../../../target/site-packages/c/lib/pkgconfig && pwd)"
export GOFLAGS=-mod=mod
export CGO_LDFLAGS="-Wl,-rpath,$(pkg-config --variable=libdir thinkthen-c)"
if [ "${1:-}" = --prepare ]; then work=$2; mkdir -p "$work"; else
  work=$(mktemp -d)
  trap 'status=$?; trap - EXIT; set +e; rm -rf "$work"; exit "$status"' EXIT
fi
go build -tags thinkthen_system -o "$work/example" show.go
cat > "$work/run.sh" <<'RUN'
exec "$(dirname "$0")/example"
RUN
[ "${1:-}" = --prepare ] || bash "$work/run.sh"
