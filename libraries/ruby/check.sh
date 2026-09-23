#!/usr/bin/env bash
# The Ruby surface's check: build the extension and the gem inside the
# builder container, run the surface tests and the conformance slice
# offline, run the slide sample, and run the interrupt proof when the stub
# is up on this surface's port (8214).
# Experimental on macOS: the builder container needs Docker Desktop there,
# and --network host does not carry the loopback the way it does on Linux.
set -euo pipefail
cd "$(dirname "$0")"

root="$(cd ../.. && pwd)"
image="thinkthen-ruby-builder:local"
stub_url="http://127.0.0.1:8214/v1"

# The gate arms the stand-in's synthesized partial failure at compile
# time (`build.sh synthetic`), so conformance case 74 and the marker test
# run; the packaged build never carries the fixture.
# The fixture build, with the production restore armed on any exit - the
# fourth review's finding caught the restore running only on success,
# leaving the fixture build behind a failed gate.
restore_production() {
  echo "== ruby surface: restore the production build (no fixture)"
  if ./build.sh >/dev/null 2>&1; then
    echo "restored: $(ls -1 *.gem 2>/dev/null | tail -1)"
  else
    echo "RESTORE FAILED: the fixture build is still in place" >&2
    return 1
  fi
}
# A failed restore fails the check (surfaces-review-5: it only printed).
trap 'restore_production || exit 1' EXIT
./build.sh synthetic

wire=no
if curl -sf --max-time 1 "$stub_url/stats" >/dev/null 2>&1; then
  wire=yes
fi

docker_run() {
  docker run --rm --network host \
    -v "$root":/src \
      -w /src/libraries/ruby \
    -e ENGINE_NULL=1 \
    "$image" \
    bash -eu -c "export CARGO_HOME=/src/libraries/ruby/.runtimes/cargo; $1"
}

echo "== ruby surface: the defect kind maps to its error class (shim unit test)"
docker_run 'cargo test --locked --quiet --lib'

echo "== ruby surface: surface tests, null backend"
docker_run 'ruby -I lib -I tests tests/test_surface.rb'

echo "== ruby surface: the deadline's bounds, checked at the contract's door"
docker_run 'ruby -I lib tests/test_deadline_bounds.rb'

echo "== ruby surface: a plain call hears Thread#raise (no tick, no token)"
docker_run 'ruby -I lib tests/test_interrupt_fast.rb'

echo "== ruby surface: wake-ups and trapped signals leave calls alone"
docker_run 'ruby -I lib tests/test_harmless_wakeups.rb'

echo "== ruby surface: the tick survives the collector"
docker_run 'ruby -I lib tests/test_tick_gc.rb'

echo "== ruby surface: one error base, a refused bad deadline, records as JSON"
docker_run 'ruby -I lib tests/test_error_classes.rb'

echo "== ruby surface: fast-backend cancel, the poll-bug shape"
docker_run 'ruby -I lib tests/test_cancel_fast.rb'
docker_run 'ruby -I lib tests/test_fork.rb'

echo "== ruby surface: the flood shape, VM survives a raising trap flood"
docker_run 'ruby -I lib tests/test_flood.rb'

echo "== ruby surface: a signal never resends a paid request; Ctrl-C stops a call within one in-flight round"
docker_run 'ruby -I lib tests/test_signal_no_resend.rb'

echo "== ruby surface: the loaded module's public names are ruled or documented"
names="$(mktemp)"
docker_run 'ruby -I lib tests/public_names.rb' > "$names"
python3 ../../scripts/check_public_names.py --ruby-runtime "$names"
rm -f "$names"

echo "== ruby surface: annotate unions every record's keys"
docker_run 'ruby -I lib tests/test_annotate_union.rb'

echo "== ruby surface: the function examples"
docker_run 'ruby -I lib tests/examples.rb'

echo "== ruby surface: conformance slice, offline"
docker_run 'ruby -I lib tests/conformance.rb'

echo "== ruby surface: slide sample, as drawn"
docker_run 'ruby -I lib tests/slide_sample.rb'

if [ "$wire" = yes ]; then
  echo "== ruby surface: interrupt proof on the wire"
  docker run --rm --network host \
    -v "$root":/src \
      -w /src/libraries/ruby \
    -e ENGINE_BASE_URL="$stub_url" \
    -e ENGINE_WIDTH=8 \
    "$image" \
    bash -eu -c 'ruby -I lib tests/test_cancel.rb'
  echo "== ruby surface: interrupt proof on a delayed stub, no tick and no token"
  docker run --rm --network host \
    -v "$root":/src \
      -w /src/libraries/ruby \
    -e ENGINE_BASE_URL="$stub_url" \
    -e ENGINE_WIDTH=8 \
    "$image" \
    bash -eu -c 'ruby -I lib tests/test_interrupt_wire.rb'
  echo "== ruby surface: interrupts are bounded on the wire (bulk one wave, single one request)"
  docker run --rm --network host \
    -v "$root":/src \
      -w /src/libraries/ruby \
    -e ENGINE_BASE_URL="$stub_url" \
    -e ENGINE_WIDTH=8 \
    "$image" \
    bash -eu -c 'ruby -I lib tests/test_interrupt_bounded.rb'
  echo "== ruby surface: probabilities cost no extra sends (punch-list item 1)"
  docker run --rm --network host \
    -v "$root":/src \
      -w /src/libraries/ruby \
    -e ENGINE_BASE_URL="$stub_url" \
    "$image" \
    bash -eu -c 'ruby -I lib tests/test_pairs_one_crossing.rb'
else
  echo "== ruby surface: interrupt proof skipped, no stub on 8214"
fi

# The gate ran on the fixture build (synthetic-partial); the shape any
# package is made from never carries it. The restore runs here, before the
# proof, so the proof reads the production build (surfaces-review-5: the
# proof ran before the EXIT trap restored, and so read the fixture build).
trap - EXIT
restore_production
docker_run 'ruby -I lib -e '"'"'require "thinkthen"; q = ThinkThen.question(decide: "Is this a complaint?"); ans = ThinkThen.decide(q, "order 4471: charged twice, please refund"); puts("production build in place: the fixture text answers (#{ans.inspect[0, 20]})")'"'"''
