# frozen_string_literal: true

# The with_tick block must survive the collector while the engine still
# holds it. The block is copied into Rust memory for the call's whole
# length, and a bare copy there is not a GC root: before the fix, a
# two-thread run that dropped the Ruby-side reference let GC free the
# block and the next tick call used freed memory.
#
# The test drops the last Ruby-side reference from inside the block's
# first run, defines a finalizer on the block so the collection itself is
# observable, and runs GC.start from a second thread for the whole batch.
# With the fix the block is registered for the call's length, so the
# finalizer never runs while the call is in flight.
#
# Offline; check.sh's null section runs it.
#
# Run with: ENGINE_NULL=1 ruby -I lib tests/test_tick_gc.rb

require "thinkthen"

RECORDS = 2_000_000
question = ThinkThen.question(decide: "Is this a complaint?")
records = Array.new(RECORDS) { |i| "record #{i}" }

collected = false
ticks = 0
engine = nil

tick = proc do
  ticks += 1
  if ticks == 1
    # Drop the only Ruby-side reference to this block: the engine's Rust
    # memory holds it for the rest of the call, and that hold must be a
    # GC root, not a bare copy.
    engine.instance_variable_set(:@tick, nil)
  end
end
ObjectSpace.define_finalizer(tick, proc { collected = true })
engine = ThinkThen.with_tick(&tick)
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
  answer = engine.decide_many(question, records, nil, nil, Thread.current[:thinkthen_tick])
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
failures << "the collector took the tick while the engine still called it" if collected
failures << "the tick ran once and stopped: #{ticks} ticks" if ticks < 2
failures << "the batch answered #{answer.size} records, not #{RECORDS}" unless answer.size == RECORDS

if failures.empty?
  puts format("the tick survived the collector: %d ticks, %d answers, no collection",
              ticks, answer.size)
else
  failures.each { |one| warn one }
  exit 1
end
