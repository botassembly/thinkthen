# frozen_string_literal: true

# A plain public call hears Thread#raise with no tick and no token of the
# caller's: the shim's unblock function fires the call's own token from the
# interrupting thread, the engine stops at its next stop check, and the
# pending Interrupt re-raises when the call returns. Before the fix the VM
# lock was released with no way to wake the waiting thread, so the
# two-million-record null batch ran about 4.3 s deaf and the raise landed
# only after it finished.
#
# Offline; check.sh's null section runs it.
#
# Run with: ENGINE_NULL=1 ruby -I lib tests/test_interrupt_fast.rb

require "thinkthen"

RECORDS = 2_000_000
AFTER = 0.3
BOUND = 1.5

records = Array.new(RECORDS) { |i| "record #{i}" }
question = ThinkThen.question(decide: "Is this a complaint?")

started = Process.clock_gettime(Process::CLOCK_MONOTONIC)
raised = nil
wall = nil
caller_thread = Thread.current
watcher = Thread.new do
  sleep AFTER
  caller_thread.raise(Interrupt, "the test interrupts the batch")
end

begin
  ThinkThen.decide_many(question, records)
  warn "the batch ran deaf: no interrupt raised"
  exit 1
rescue Interrupt => e
  raised = e
  wall = Process.clock_gettime(Process::CLOCK_MONOTONIC) - started
ensure
  watcher.kill
  watcher.join
end

puts format("raised: %s after %.3f s (interrupt set at %.1f s)", raised.class, wall, AFTER)

if wall > BOUND
  warn format("the interrupt stayed deaf: %.3f s past the start; the deaf batch runs about 4.3 s", wall)
  exit 1
end

puts "a plain call hears Thread#raise"
