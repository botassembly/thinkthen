#!/bin/sh
# Build this revision's engine once and exercise an isolated installed Ruby gem.
set -eu
. scripts/native-tools-env
dev_gems=${THINKTHEN_RUBY_DEV_GEMS:-$HOME/.local/share/thinkthen-build-tools/ruby/dev}
case "${1:-}" in --smoke|--release-only) ;; *) sdk/ruby/tests/type-gate --dev-gems "$dev_gems" ;; esac
if [ "${1:-}" = --types-only ]; then exit 0; fi
export CARGO_TARGET_DIR="$HOME/.local/share/thinkthen-build-tools/ruby-development-target"
cargo build --offline --locked -p thinkthen-ffi -p loopback --lib --bin loopback
root=$PWD
dev_gems=${THINKTHEN_RUBY_DEV_GEMS:-$HOME/.local/share/thinkthen-build-tools/ruby/dev}
scratch=$(mktemp -d)
trap 'gate_status=$?; trap - EXIT; set +e; rm -rf "$scratch"; exit "$gate_status"' EXIT
trap 'exit 1' HUP INT TERM
for source in sdk/ruby/lib/thinkthen.rb sdk/ruby/lib/thinkthen/*.rb sdk/ruby/tests/*.rb; do ruby -wc "$source"; done
THINKTHEN_RUBY_SOURCE_NATIVE=1 scripts/ruby-package "$scratch/packages"
case "$(uname -s):$(uname -m)" in
    Darwin:arm64) platform=arm64-darwin ;;
    Linux:x86_64) platform=x86_64-linux ;;
    *) echo 'Ruby supports macOS arm64 and glibc Linux x86-64' >&2; exit 1 ;;
esac
dependency=$(find "$HOME/.local/share/thinkthen-build-tools/ruby/packages" -name "ffi-*-$platform*.gem" -print)
test -f "$dependency"
mkdir -p "$scratch/home" "$scratch/tmp" "$scratch/gems"
export HOME="$scratch/home" TMPDIR="$scratch/tmp" GEM_HOME="$scratch/gems" GEM_PATH="$scratch/gems:$dev_gems"
unset THINKTHEN_API_KEY OPENAI_API_KEY THINKTHEN_BASE_URL THINKTHEN_BATCH THINKTHEN_LIBRARY RUBYLIB RUBYOPT BUNDLE_GEMFILE
export THINKTHEN_LOOPBACK="$CARGO_TARGET_DIR/debug/loopback" THINKTHEN_CASES="$root/conformance" THINKTHEN_STUB_CASES="$root/construction/stub.json"
gem install --local --no-document "$dependency" "$scratch/packages"/*"-$platform.gem"
cd "$scratch"
if [ "${1:-}" = --release-only ]; then
    ruby -w "$root/scripts/release/ruby.rb" "$root/construction/stub.json"
elif [ "${1:-}" = --ideal-only ]; then
    python3 "$root/scripts/ruby_guide_examples.py"
elif [ "${1:-}" = --smoke ]; then
    ruby -w "$root/sdk/ruby/tests/ideal.rb"
else
    python3 "$root/scripts/ruby_guide_examples.py"
    python3 "$root/scripts/ruby_guide_examples.py" --controls
    ruby -w "$root/sdk/ruby/tests/run.rb" --surface-only
fi
