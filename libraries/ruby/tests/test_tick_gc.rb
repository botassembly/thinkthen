# frozen_string_literal: true

# The with_tick block must survive the collector while the call still
# runs it. The block never crosses into Rust anymore - the fourth
# review's crash fix moved every tick to the Ruby-side watchdog - so its
# liveness is the registry row's hold: the row lives in the ThinkThen
# module for the whole call, and that hold is an ordinary GC root. The
# test drops the caller's thread-local, the only other reference, from
# inside the block's first run, defines a finalizer so the collection
# itself is observable, and runs GC.start from a second thread for the
# whole batch. The finalizer must not run while the call is in flight.
#
# The block runs on the watchdog thread. Thread.current inside it names
# the watchdog, so the drop names the caller's thread explicitly. The
# seventh review found the old drop cleared the watchdog's local and
# left the caller's hold in place, so a row that held the block weakly
# still passed. The test also checks that the drop took.
#
# Offline; check.sh's null section runs it.
#
# Run with: ENGINE_NULL=1 ruby -I lib tests/test_tick_gc.rb

require "thinkthen"

RECORDS = 2_000_000
question = ThinkThen.question(decide: "Is this a complaint?")
records = Array.new(RECORDS) { |i| "record #{i}" }

caller = Thread.current
collected = false
ticks = 0
dropped = nil

tick = proc do
  ticks += 1
  if ticks == 1
    # Drop the caller's reference, the only one outside the registry
    # row: the row's hold must keep the block alive for the rest of the
    # call.
    caller[:thinkthen_tick] = nil
    dropped = caller[:thinkthen_tick].nil?
  end
end
ObjectSpace.define_finalizer(tick, proc { collected = true })
ThinkThen.with_tick(&tick)
tick = nil

stop = false
pressure = Thread.new do
  until stop
    GC.start
    sleep 0.01
  end
end

answer = nil
failure = nil
begin
  answer = ThinkThen.decide_many(question, records)
rescue StandardError => e
  failure = e
ensure
  stop = true
  pressure.join
end

if failure
  warn "the batch failed: #{failure.class}: #{failure.message}"
  exit 1
end

failures = []
failures << "the drop did not clear the caller's reference" unless dropped
failures << "the collector took the tick while the call still ran it" if collected
failures << "the tick ran once and stopped: #{ticks} ticks" if ticks < 2
failures << "the batch answered #{answer.size} records, not #{RECORDS}" unless answer.size == RECORDS

if failures.empty?
  puts format("the tick survived the collector: %d ticks, %d answers, no collection",
              ticks, answer.size)
else
  failures.each { |one| warn one }
  exit 1
end
