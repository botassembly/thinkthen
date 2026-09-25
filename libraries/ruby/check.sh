#!/bin/sh
# The Ruby surface's check (ticket 0112). The surface rung passes its
# loopback port as $1. Each test starts its own 0092 backend from the binary
# the rung built, so the slide sample and the tests leave the rung's backend
# alone. Exit 77 means no host toolchain: the rung reports "not run".
set -eu
cd -- "$(dirname -- "$0")"
unset THINKTHEN_API_KEY
repo=$(cd ../.. && pwd)

fail() { echo "check ruby: $*" >&2; exit 1; }
not_run() { echo "check ruby: not run: $*"; exit 77; }

# The file checks. The retired Docker build (R7-10, R4-8) and fixture build
# (R4-9, R5-13) stay gone, cargo never reaches the network (R7-3), the
# scripts fetch nothing (R4-19), library names follow the host (R3-32),
# and one guard catches panics (R2-31).
for file in Dockerfile rust-toolchain rust-toolchain.toml; do
  [ ! -e "$file" ] || fail "$file is back; the host toolchain builds this surface"
done
! grep -q '^\[features\]' Cargo.toml || fail "Cargo.toml has a [features] table; the port has one build"
# cargo fmt reads no lock and takes neither flag.
if sed '/^[[:space:]]*#/d' check.sh build.sh | grep -E '(^|[^[:alnum:]_])cargo [a-z]' |
   grep -v -e 'cargo fmt' -e '--locked --offline' -e "grep -E" >&2; then
  fail "a cargo call above lacks --locked --offline"
fi
if sed '/^[[:space:]]*#/d' build.sh | grep -nE '(^|[^-[:alnum:]_])(apt-get|curl|wget|docker)([^-[:alnum:]_]|$)' >&2 ||
   sed '/^[[:space:]]*#/d; /grep -nE/d' check.sh | grep -nE '(^|[^-[:alnum:]_])(apt-get|curl|wget|docker)([^-[:alnum:]_]|$)' >&2; then
  fail "a script above fetches; setup-ruby.sh alone fetches, once, by hand"
fi
if sed '/^[[:space:]]*#/d' build.sh | grep -nE '\.(so|bundle|dylib)([^[:alnum:]]|$)|sed -i|(^|[^[:alnum:]_])ss ' >&2 ||
   sed '/^[[:space:]]*#/d; /grep -nE/d; /libclang/d' check.sh | grep -nE '\.(so|bundle|dylib)([^[:alnum:]]|$)|sed -i|(^|[^[:alnum:]_])ss ' >&2; then
  fail "a script above names a host library file, edits in place, or calls ss"
fi
guards=$(cat src/*.rs | grep -o 'catch_unwind(' | wc -l)
[ "$guards" -eq 1 ] || fail "src holds $guards catch_unwind sites; the binding has one guard"

# The host and toolchain probe. Only the pinned prefix runs, never a ruby
# found on PATH.
[ "$(uname -s)" = Linux ] || not_run "Linux only at 0.1"
. ./toolchain.env
prefix=$HOME/.cache/thinkthen-toolchains/ruby/$RUBY_VERSION
setup="run libraries/ruby/setup-ruby.sh once on this machine, with the network"
[ -f "$prefix/thinkthen-toolchain.stamp" ] || not_run "no Ruby $RUBY_VERSION prefix; $setup"
stamp=$(head -n 3 "$prefix/thinkthen-toolchain.stamp")
[ "$stamp" = "$(printf 'ruby %s\nyaml %s\nyaml512 %s' "$RUBY_SHA256" "$YAML_SHA256" "$YAML_SHA512")" ] ||
  not_run "the prefix's stamp does not match toolchain.env; move the prefix aside and $setup"
if [ -n "${RUBY:-}" ] && [ "$RUBY" != "$prefix/bin/ruby" ]; then
  fail "RUBY names $RUBY; this check runs only $prefix/bin/ruby"
fi
RUBY=$prefix/bin/ruby
"$RUBY" -v | grep -q "^ruby $RUBY_VERSION " || not_run "$RUBY is not Ruby $RUBY_VERSION; $setup"
clang=$(for lib in /usr/lib/llvm-*/lib; do [ -e "$lib/libclang.so.1" ] && echo "$lib"; done | sort -V | tail -n 1)
[ -n "$clang" ] || not_run "no libclang under /usr/lib/llvm-*/lib; install the host's libclang"
cargo fetch --locked --offline --quiet 2>/dev/null ||
  not_run "a crate is missing from Cargo's cache; fetch libraries/ruby's locked crates once, with the network"
backend=$repo/${CARGO_TARGET_DIR:-target}/debug/conformance-backend
case $backend in /*) ;; *) backend=$repo/$backend ;; esac
[ -x "$backend" ] || fail "no loopback backend at $backend; the surfaces rung builds it"
PATH=$prefix/bin:$PATH
LIBCLANG_PATH=$clang
LD_LIBRARY_PATH=$prefix/lib${LD_LIBRARY_PATH:+:$LD_LIBRARY_PATH}
THINKTHEN_TEST_BACKEND=$backend
export RUBY PATH LIBCLANG_PATH LD_LIBRARY_PATH THINKTHEN_TEST_BACKEND

./build.sh
cargo fmt --check
cargo clippy --locked --offline --all-targets --quiet -- -D warnings
cargo test --locked --offline --quiet --lib
for test in tests/test_*.rb; do
  timeout 120 "$RUBY" -I lib "$test" || fail "$test failed"
done
timeout 120 "$RUBY" -I lib tests/conformance.rb || fail "the conformance runner failed"
timeout 120 "$RUBY" -I lib tests/examples.rb || fail "an example failed"
"$RUBY" -rrubygems/package -e '
  spec = Gem::Package.new(Dir["thinkthen-*.gem"].fetch(0)).spec
  version = File.read("../../crates/thinkthen/Cargo.toml")[/^version = "([^"]+)"/, 1]
  abort "the gem is not MIT" unless spec.licenses == ["MIT"]
  abort "the gem names no platform" if spec.platform.to_s == "ruby"
  abort "the gem is #{spec.version}, the engine is #{version}" unless spec.version.to_s == version
' || fail "the gem check failed"
timeout 120 "$RUBY" -I lib tests/slide_sample.rb || fail "the slide sample failed"
echo "check ruby: pass"
