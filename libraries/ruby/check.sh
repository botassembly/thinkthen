#!/bin/sh
# The Ruby surface's check (ticket 0112). The surface rung passes its
# loopback port as $1. Each test starts its own 0092 backend from the binary
# the rung built, so the slide sample and the tests leave the rung's backend
# alone. Exit 77 means no host toolchain: the rung reports "not run".
set -eu
cd -- "$(dirname -- "$0")"
unset THINKTHEN_API_KEY
profile=${THINKTHEN_TEST_PROFILE:-routine}
case $profile in routine|full|stress|smoke) ;; *) echo "check ruby: unknown THINKTHEN_TEST_PROFILE: $profile" >&2; exit 2 ;; esac
[ "$profile" = routine ] || unset THINKTHEN_CONFORMANCE_IDS
repo=$(cd ../.. && pwd)
. "$repo/sdlc/scripts/scratch.sh"
# ADR 0113: this run's engines write a scratch usage folder, never the real one.
usage_home
# macOS has no `timeout` (ticket 0128).
LIMIT=$repo/sdlc/scripts/time-limit

fail() { echo "check ruby: $*" >&2; exit 1; }
not_run() { echo "check ruby: not run: $*"; exit 77; }
# Ticket 0394: a macOS gem names no macOS version, so a Ruby on any macOS
# version takes it. RubyGems matches arm64-darwin-24 only to darwin 24.
gem_platform() {
  "$RUBY" -rrubygems/package -e 'platform = Gem::Package.new(ARGV[0]).spec.platform
    exit if platform == Gem::Platform::RUBY
    exit unless platform.os == "darwin"
    abort "the macOS gem #{platform} names macOS version #{platform.version}" if platform.version
    %w[23 24 25].each do |darwin|
      host = Gem::Platform.new("#{platform.cpu}-darwin-#{darwin}")
      abort "the gem #{platform} does not match #{host}" unless platform === host
    end' "$1" || fail "the gem platform check failed on ${1##*/}"
}

# The file checks. The retired Docker build (R7-10, R4-8) and fixture build
# (R4-9, R5-13) stay gone, cargo never reaches the network (R7-3), the
# scripts fetch nothing (R4-19), library names follow the host (R3-32),
# and one guard catches panics (R2-31).
for file in Dockerfile rust-toolchain rust-toolchain.toml; do
  [ ! -e "$file" ] || fail "$file is back; the host toolchain builds this surface"
done
! grep -q '^\[features\]' Cargo.toml || fail "Cargo.toml has a [features] table; the port has one build"
# cargo fmt reads no lock, cargo deny takes --offline alone, and the deny
# plant fetches its local file:// git source into a scratch home.
if sed '/^[[:space:]]*#/d; s/[[:space:]]#.*$//' check.sh build.sh | grep -E '(^|[^[:alnum:]_])cargo [a-z]' |
   grep -vE -e '^[[:space:]]*cargo fmt --check$' -e '^[[:space:]]*(CARGO_HOME="\$plant/home" )?cargo deny ' \
     -e '^CARGO_NET_OFFLINE=false CARGO_HOME="\$plant/home" cargo fetch --quiet --manifest-path "\$plant/copy/Cargo.toml"$' \
     -e '--locked --offline' -e "grep -v?E " -e "^[[:space:]]+-e '" >&2; then
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
# Refuse an absent host toolchain before the offline deny plant or a build.
host=$(uname -s)
case $host in Linux|Darwin) ;; *) not_run "no Ruby host route for $host" ;; esac
. ./toolchain.env
if [ "$host" = Darwin ]; then
  prefix=$HOME/.cache/thinkthen-toolchains/ruby/$RUBY_VERSION-$(uname -m)-darwin
else
  prefix=$HOME/.cache/thinkthen-toolchains/ruby/$RUBY_VERSION
fi
setup="run libraries/ruby/setup-ruby.sh once on this machine, with the network"
[ -f "$prefix/thinkthen-toolchain.stamp" ] || not_run "no Ruby $RUBY_VERSION prefix; $setup"
stamp=$(head -n 3 "$prefix/thinkthen-toolchain.stamp")
[ "$stamp" = "$(printf 'ruby %s\nyaml %s\nyaml512 %s' "$RUBY_SHA256" "$YAML_SHA256" "$YAML_SHA512")" ] ||
  not_run "the prefix's stamp does not match toolchain.env; move the prefix aside and $setup"

# deny on the lock, then a planted file:// git source meets the sources rule
# (R3-28). This is the Ruby lint block: lint reaches this surface only
# through surfaces --registry, and the pinned-Ruby guard lives below (R5-35).
scratch_dir plant
# The replay smoke skips the deny plant (ticket 0335).
if [ "$profile" != smoke ]; then
cargo deny --version >/dev/null 2>&1 || not_run "no cargo-deny; install it once, with the network"
cargo deny --offline --manifest-path Cargo.toml check --config "$repo/deny.toml" advisories bans licenses sources
mkdir -p "$plant/dep/src" "$plant/copy/src"
printf '[package]\nname = "planted"\nversion = "0.0.1"\nedition = "2024"\nlicense = "MIT"\n' >"$plant/dep/Cargo.toml"
: >"$plant/dep/src/lib.rs"
: >"$plant/copy/src/lib.rs"
git -C "$plant/dep" init -q && git -C "$plant/dep" add -A
git -C "$plant/dep" -c user.name=plant -c user.email=plant@example.invalid commit -qm plant
printf '[package]\nname = "binding"\nversion = "0.0.1"\nedition = "2024"\nlicense = "MIT"\n[dependencies]\nplanted = { git = "file://%s/dep" }\n' "$plant" >"$plant/copy/Cargo.toml"
# A local file:// fetch under a scratch CARGO_HOME: no network and no residue.
# Cargo's offline mode also forbids the first checkout of this local fixture.
CARGO_NET_OFFLINE=false CARGO_HOME="$plant/home" cargo fetch --quiet --manifest-path "$plant/copy/Cargo.toml"
set +e
# cargo-deny exits with bit 8 set when the sources check fails.
CARGO_HOME="$plant/home" cargo deny --offline --manifest-path "$plant/copy/Cargo.toml" check --config "$repo/deny.toml" sources >"$plant/deny" 2>&1
code=$?
set -e
[ "$code" -eq 8 ] && grep -q source-not-allowed "$plant/deny" || fail "deny passed a git source (exit $code)"
fi

# Only the pinned prefix runs, never a ruby found on PATH.
if [ -n "${RUBY:-}" ] && [ "$RUBY" != "$prefix/bin/ruby" ]; then
  fail "RUBY names $RUBY; this check runs only $prefix/bin/ruby"
fi
RUBY=$prefix/bin/ruby
"$RUBY" -v | grep -q "^ruby $RUBY_VERSION " || fail "$RUBY is not Ruby $RUBY_VERSION behind a matching stamp"
if [ "$host" = Darwin ]; then
  command -v xcrun >/dev/null 2>&1 || not_run "xcrun is missing"
  openssl=${THINKTHEN_RUBY_OPENSSL:-$(brew --prefix openssl@3 2>/dev/null || true)}
  [ -x "$openssl/bin/openssl" ] || not_run "OpenSSL $RUBY_OPENSSL_SERIES host prefix is missing"
  case $("$openssl/bin/openssl" version) in "OpenSSL $RUBY_OPENSSL_SERIES."*) ;; *) not_run "OpenSSL host version differs from toolchain.env" ;; esac
  clang=$(xcrun --find clang 2>/dev/null)
  clang=${clang%/bin/clang}/lib
  [ -f "$clang/libclang.dylib" ] || not_run "no libclang.dylib beside the selected Xcode clang"
  [ "${MACOSX_DEPLOYMENT_TARGET:-$RUBY_MACOS_DEPLOYMENT_TARGET}" = "$RUBY_MACOS_DEPLOYMENT_TARGET" ] || fail "the macOS deployment target differs from $RUBY_MACOS_DEPLOYMENT_TARGET"
  MACOSX_DEPLOYMENT_TARGET=$RUBY_MACOS_DEPLOYMENT_TARGET
  SDKROOT=$(xcrun --sdk macosx --show-sdk-path)
  DYLD_LIBRARY_PATH=$prefix/lib:$openssl/lib${DYLD_LIBRARY_PATH:+:$DYLD_LIBRARY_PATH}
  export SDKROOT MACOSX_DEPLOYMENT_TARGET DYLD_LIBRARY_PATH
else
  clang=$(for lib in /usr/lib/llvm-*/lib; do [ -e "$lib/libclang.so.1" ] && echo "$lib"; done | sort -V | tail -n 1)
  [ -n "$clang" ] || not_run "no libclang under /usr/lib/llvm-*/lib; install the host's libclang"
  LD_LIBRARY_PATH=$prefix/lib${LD_LIBRARY_PATH:+:$LD_LIBRARY_PATH}
  export LD_LIBRARY_PATH
fi
cargo fetch --locked --offline --quiet 2>/dev/null ||
  not_run "a crate is missing from Cargo's cache; fetch libraries/ruby's locked crates once, with the network"
backend=$repo/${CARGO_TARGET_DIR:-target}/debug/conformance-backend
case $backend in /*) ;; *) backend=$repo/$backend ;; esac
[ "$profile" = smoke ] || [ -x "$backend" ] || fail "no loopback backend at $backend; the surfaces rung builds it"
PATH=$prefix/bin:$PATH
LIBCLANG_PATH=$clang
THINKTHEN_TEST_BACKEND=$backend
export RUBY PATH LIBCLANG_PATH THINKTHEN_TEST_BACKEND

if [ -z "${THINKTHEN_ARTIFACT:-}" ]; then
  python3 "$repo/sdlc/generators/results/generate.py" --target ruby --check
  ./build.sh
  cargo fmt --check
  cargo clippy --locked --offline --all-targets --quiet -- -D warnings
  THINKTHEN_ARTIFACT=$(find "$PWD" -maxdepth 1 -name 'thinkthen-*-*.gem' | head -n 1)
fi
THINKTHEN_ARTIFACT=$(realpath "$THINKTHEN_ARTIFACT")
. "$repo/sdlc/scripts/installed.sh"
installed_tests "$repo" libraries/ruby "$plant"
if [ "${THINKTHEN_ARTIFACT##*/}" = "thinkthen-$(sed -n 's/^version = "\(.*\)"$/\1/p' ../../crates/thinkthen/Cargo.toml | head -n 1).gem" ]; then
  "$RUBY" tests/test_package.rb -n test_fallback_install_explains_support_and_import_refuses_to_work
  exit 0
fi
gem_platform "$THINKTHEN_ARTIFACT"
"$prefix/bin/gem" install --local --silent --no-document --install-dir "$scratch/gems" "$THINKTHEN_ARTIFACT"
GEM_PATH=$scratch/gems
export GEM_PATH
unset RUBYLIB
"$RUBY" -e 'require "thinkthen"; ours=$LOADED_FEATURES.grep(%r{/lib/thinkthen(\.rb|/)})
  abort "loaded outside gem" unless ours.any? && ours.all? { |p| p.start_with?(ARGV[0]) }' "$scratch/gems/gems/"
THINKTHEN_TEST_LIBRARY=$scratch/gems/gems/$(basename "$THINKTHEN_ARTIFACT" .gem)/lib
export THINKTHEN_TEST_LIBRARY
if [ "$profile" = smoke ]; then
  smoke_guard
  "$RUBY" -e 'require "thinkthen"
    value = ThinkThen::Client.open { |client| client.decide(ENV.fetch("THINKTHEN_TEST_SMOKE_QUESTION"), ENV.fetch("THINKTHEN_TEST_SMOKE_TEXT")).value }
    puts "smoke: #{value.nil? ? "null" : value}"'
  exit 0
fi
"$RUBY" "$scratch/libraries/ruby/tests/test_owned_session.rb"
python3 "$repo/libraries/ruby/tests/native_cases.py"
echo "check ruby: pass, installed"
