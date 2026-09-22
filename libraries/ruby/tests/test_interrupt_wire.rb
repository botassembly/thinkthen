# frozen_string_literal: true

# The interrupt proof on a delayed stub, with no tick and no token of the
# caller's: Thread#raise lands while the call waits without the VM lock,
# the shim's unblock function fires the call's own token, the in-flight
# round finishes, no further round starts, and the pending Interrupt
# re-raises — bounded by a round or two, not by the whole batch. The deaf
# batch would take (records / width) rounds; the bound is measured against
# the stub's own round time, so any stub delay works. A stub with no delay
# skips, because the timing proof needs one.
#
# Skipped when no stub is up on 8214.
#
# Run with: ENGINE_BASE_URL=http://127.0.0.1:8214/v1 ENGINE_WIDTH=8 \
#          ruby -I lib tests/test_interrupt_wire.rb

require "json"
require "net/http"
require "uri"
require "thinkthen"

STUB = URI("http://127.0.0.1:8214")
RECORDS = 400
WIDTH = (ENV["ENGINE_WIDTH"] || "8").to_i
AFTER = 0.5

def stats
  JSON.parse(Net::HTTP.get(STUB + "/v1/stats"))
rescue StandardError
  nil
end

if stats.nil?
  puts "interrupt wire proof skipped: no stub on 8214"
  exit 0
end

Net::HTTP.post(STUB + "/v1/reset", "")
question = ThinkThen.question(decide: "Is this a complaint?")
records = Array.new(RECORDS) { |i| i.zero? ? "I want a refund" : "filler record #{i}" }

# The stub's own round time: one call's wall, which the delay sets.
round_started = Process.clock_gettime(Process::CLOCK_MONOTONIC)
ThinkThen.decide(question, "I want a refund")
round = Process.clock_gettime(Process::CLOCK_MONOTONIC) - round_started
if round < 0.05
  puts format("interrupt wire proof skipped: the stub round is %.0f ms; the timing proof needs a delay",
              round * 1000)
  exit 0
end

Net::HTTP.post(STUB + "/v1/reset", "")
rounds = (RECORDS.to_f / WIDTH).ceil
deaf = rounds * round
bound = AFTER + (2 * round) + 1.0

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

at_return = stats["requests"]
sleep 2
after_settle = stats["requests"]

puts format("raised: %s after %.3f s (signal at %.1f s, one round %.3f s)",
            raised.class, wall, AFTER, round)
puts format("stub:   requests at return %d, after 2 s settle %d", at_return, after_settle)
puts format("deaf would be about %.1f s over %d rounds; the bound is %.1f s", deaf, rounds, bound)

failures = []
failures << "the interrupt did not raise" unless raised.is_a?(Interrupt)
failures << format("the call stayed deaf: %.3f s past the signal", wall - AFTER) if wall > bound
if at_return != after_settle
  failures << "the stub kept serving after the return (#{at_return} -> #{after_settle})"
end

if failures.empty?
  puts "the interrupt stops a delayed batch"
else
  failures.each { |one| warn one }
  exit 1
end
