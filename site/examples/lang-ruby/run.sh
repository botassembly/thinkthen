# Run the unchanged example against the staged Ruby package.
set -eu
root=$(CDPATH= cd -- "$(dirname -- "${BASH_SOURCE[0]}")/../../.." && pwd)
packages=${SITE_PACKAGES:-$root/target/site-packages}
work=$(mktemp -d)
trap 'status=$?; trap - EXIT; set +e; rm -rf "$work"; exit "$status"' EXIT
trap 'exit 1' HUP INT TERM
ruby -I "$packages/ruby/lib" show.rb > "$work/output.txt"
cat "$work/output.txt"
