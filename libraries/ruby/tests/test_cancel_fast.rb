# frozen_string_literal: true

# The fast-backend cancel proof for Ruby (lane B item 5, the poll-bug
# shape): on the null backend the wait's channel never idles, so the
# engine's poll must run on the busy arm too and a tick that raises one
# second in must surface within about a tick — not after the whole batch,
# which is what the bug did. Two million records run about 4.3 s deaf, so
# the 1.5 s bound separates the two behaviors.
#
# The documented interrupt path is `ThinkThen.with_tick(&tick)`: the block
# runs each wait interval with the VM lock taken, and a raise in it
# cancels the token, lets sent requests finish, and re-raises. Runs
# offline; check.sh calls it in the null section.
#
# Run with: ENGINE_NULL=1 ruby -I lib tests/test_cancel_fast.rb

require "thinkthen"

RECORDS = 2_000_000
AFTER = 1.0
BOUND = 1.5

records = Array.new(RECORDS) { |i| "record #{i}" }
question = ThinkThen.question(decide: "Is this a complaint?")
token = ThinkThen::Cancel.new
started = Process.clock_gettime(Process::CLOCK_MONOTONIC)
ticks = 0

tick = lambda do
  ticks += 1
  next unless Process.clock_gettime(Process::CLOCK_MONOTONIC) - started > AFTER

  # The tick cancels the engine's token first, so no new request starts
  # and the sent ones finish; the raise then rides out of the call.
  token.cancel
  raise Interrupt, "the tick interrupts the bulk call"
end

ThinkThen.with_tick(&tick)
raised = nil
wall = nil

begin
  ThinkThen.decide_many(question, records, cancel: token)
  warn "the batch ran deaf: no interrupt raised"
  exit 1
rescue Interrupt => e
  raised = e
  wall = Process.clock_gettime(Process::CLOCK_MONOTONIC) - started
end

puts format("raised: %s after %.3f s (interrupt set at %.1f s, %d ticks)",
            raised.class, wall, AFTER, ticks)

if wall > BOUND
  warn format("the interrupt stayed deaf: %.3f s past the start", wall)
  exit 1
end

puts "the fast-backend tick raise holds"
