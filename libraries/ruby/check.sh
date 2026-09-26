#!/bin/sh
# The Ruby surface's check (ticket 0112). The surface rung passes its
# loopback port as $1. Each test starts its own 0092 backend from the binary
# the rung built, so the slide sample and the tests leave the rung's backend
# alone. Exit 77 means no host toolchain: the rung reports "not run".
set -eu
cd -- "$(dirname -- "$0")"
unset THINKTHEN_API_KEY
repo=$(cd ../.. && pwd)
. "$repo/sdlc/scripts/scratch.sh"
# macOS has no `timeout` (ticket 0128).
LIMIT=$repo/sdlc/scripts/time-limit

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
# cargo fmt reads no lock, cargo deny takes --offline alone, and the deny
# plant fetches its local file:// git source into a scratch home.
if sed '/^[[:space:]]*#/d; s/[[:space:]]#.*$//' check.sh build.sh | grep -E '(^|[^[:alnum:]_])cargo [a-z]' |
   grep -vE -e '^[[:space:]]*cargo fmt --check$' -e '^[[:space:]]*(CARGO_HOME="\$plant/home" )?cargo deny ' \
     -e '^CARGO_HOME="\$plant/home" cargo fetch --quiet --manifest-path "\$plant/copy/Cargo.toml"$' \
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
guards=$(cat src/*.rs | grep -o 'catch_unwind(' | wc -l)
[ "$guards" -eq 1 ] || fail "src holds $guards catch_unwind sites; the binding has one guard"
# R4-2 closed by construction: the wait, the unblock function, the handoff,
# and the worker never touch Ruby. lib.rs and call.rs hold the worker and
# the handoff, and they name no Ruby crate.
! grep -nE 'rb_sys|magnus' src/lib.rs src/call.rs >&2 || fail "the worker or the handoff above touches Ruby"
awk '/^(unsafe extern "C" )?fn (wait_for|wake)\(/ { held = 1 }
     held && /rb_sys|magnus/ { print FILENAME ": " $0; bad = 1 }
     held && /^}/ { held = 0 }
     END { exit bad }' src/ffi.rs >&2 || fail "the released wait above touches Ruby"
! grep -n 'rb_thread_call_with_gvl' src/*.rs >&2 || fail "src retakes the VM lock inside the released region"
# R2-8, R7-9, and R4-18 closed by design: Rust holds no Ruby object. A
# compile-time Send assertion in lib.rs covers every wrapped struct, and
# this catches the rooting calls that dodge it.
! grep -nE 'Opaque|BoxValue|rb_gc_register|magnus::gc|register_mark_object' src/*.rs >&2 || fail "src holds a Ruby object"

# deny on the lock, then a planted file:// git source meets the sources rule
# (R3-28). This is the Ruby lint block: lint reaches this surface only
# through surfaces --registry, and the pinned-Ruby guard lives below (R5-35).
cargo deny --version >/dev/null 2>&1 || not_run "no cargo-deny; install it once, with the network"
cargo deny --offline --manifest-path Cargo.toml check --config "$repo/deny.toml" advisories bans licenses sources
scratch_dir plant
mkdir -p "$plant/dep/src" "$plant/copy/src"
printf '[package]\nname = "planted"\nversion = "0.0.1"\nedition = "2024"\nlicense = "MIT"\n' >"$plant/dep/Cargo.toml"
: >"$plant/dep/src/lib.rs"
: >"$plant/copy/src/lib.rs"
git -C "$plant/dep" init -q && git -C "$plant/dep" add -A
git -C "$plant/dep" -c user.name=plant -c user.email=plant@example.invalid commit -qm plant
printf '[package]\nname = "binding"\nversion = "0.0.1"\nedition = "2024"\nlicense = "MIT"\n[dependencies]\nplanted = { git = "file://%s/dep" }\n' "$plant" >"$plant/copy/Cargo.toml"
# A local file:// fetch under a scratch CARGO_HOME: no network and no residue.
CARGO_HOME="$plant/home" cargo fetch --quiet --manifest-path "$plant/copy/Cargo.toml"
set +e
# cargo-deny exits with bit 8 set when the sources check fails.
CARGO_HOME="$plant/home" cargo deny --offline --manifest-path "$plant/copy/Cargo.toml" check --config "$repo/deny.toml" sources >"$plant/deny" 2>&1
code=$?
set -e
[ "$code" -eq 8 ] && grep -q source-not-allowed "$plant/deny" || fail "deny passed a git source (exit $code)"

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
"$RUBY" -v | grep -q "^ruby $RUBY_VERSION " || fail "$RUBY is not Ruby $RUBY_VERSION behind a matching stamp"
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

if [ -n "${THINKTHEN_ARTIFACT:-}" ]; then
  # The installed-file mode (ticket 0128): the gem in a fresh gem folder, and the shared cases
  # and examples from a copy of tests/, with no repository lib/ on the load path.
  # The copy sits inside the check's own plant folder, which its cleanup removes.
  . "$repo/sdlc/scripts/installed.sh"
  installed_tests "$repo" libraries/ruby "$plant"
  "$prefix/bin/gem" install --local --silent --no-document --install-dir "$scratch/gems" "$THINKTHEN_ARTIFACT"
  cd "$scratch/libraries/ruby"
  export GEM_PATH="$scratch/gems"
  unset RUBYLIB
  "$RUBY" -I lib -e 'require "thinkthen"; ours = $LOADED_FEATURES.grep(%r{/lib/thinkthen(\.rb|/)})
    abort "thinkthen loaded #{ours}" unless ours.any? && ours.all? { |path| path.start_with?(ARGV[0]) }' "$scratch/gems/gems/" ||
    fail "thinkthen loaded from outside the gem folder"
  for test in tests/conformance.rb tests/examples.rb; do
    sh "$LIMIT" 120 "$RUBY" -I lib "$test" || fail "$test failed, installed"
  done
  echo "check ruby: pass, installed"
  exit 0
fi

./build.sh
cargo fmt --check
cargo clippy --locked --offline --all-targets --quiet -- -D warnings
cargo test --locked --offline --quiet --lib
for test in tests/test_*.rb; do
  sh "$LIMIT" 120 "$RUBY" -I lib "$test" || fail "$test failed"
done
sh "$LIMIT" 120 "$RUBY" -I lib tests/conformance.rb || fail "the conformance runner failed"
sh "$LIMIT" 120 "$RUBY" -I lib tests/examples.rb || fail "an example failed"
"$RUBY" -rrubygems/package -I lib -rthinkthen/version -e '
  spec = Gem::Package.new(Dir["thinkthen-*.gem"].fetch(0)).spec
  version = File.read("../../crates/thinkthen/Cargo.toml")[/^version = "([^"]+)"/, 1]
  files = ["lib/thinkthen.rb", "lib/thinkthen/thinkthen.#{RbConfig::CONFIG["DLEXT"]}", "lib/thinkthen/version.rb"]
  abort "the gem is not MIT" unless spec.licenses == ["MIT"]
  abort "the gem names no platform" if spec.platform.to_s == "ruby"
  abort "the gem is #{spec.version}, the engine is #{version}" unless spec.version.to_s == version
  abort "ThinkThen::VERSION is #{ThinkThen::VERSION}, the engine is #{version}" unless ThinkThen::VERSION == version
  abort "the gem holds #{spec.files.sort}" unless spec.files.sort == files.sort
' || fail "the gem check failed"
sh "$LIMIT" 120 "$RUBY" -I lib tests/slide_sample.rb || fail "the slide sample failed"
echo "check ruby: pass"
