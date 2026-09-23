# frozen_string_literal: true

# The interrupt proof, brief item 7 for Ruby: an interrupt raised on the
# calling thread while a bulk call runs stops the call within one in-flight
# round, nothing is served after the return, and the interrupt itself
# re-raises. The crossing hears Thread#raise between its wait slices,
# fires the call's token, lets sent requests finish, and re-raises.
#
# Shape picked: a watcher thread fires ThinkThen::Cancel AND Thread#raise
# (Interrupt) one second in. Skipped when no stub is up on 8214.
#
# Run with: ENGINE_BASE_URL=http://127.0.0.1:8214/v1 ENGINE_WIDTH=8 \
#          ruby -I lib tests/test_cancel.rb

require "json"
require "net/http"
require "uri"
require "thinkthen"

STUB = URI("http://127.0.0.1:8214")

def stats
  JSON.parse(Net::HTTP.get(STUB + "/v1/stats"))
end

abort("no stub on 8214; run this with the stub up") if stats.nil? || stats.empty?

Net::HTTP.post(STUB + "/v1/reset", "")

records = Array.new(1000) { |i| i.zero? ? "I want a refund" : "filler record #{i}" }
token = ThinkThen::Cancel.new
question = ThinkThen.question(decide: "Is this a complaint?")

started = Process.clock_gettime(Process::CLOCK_MONOTONIC)
raised = nil
wall = nil

caller_thread = Thread.current
watcher = Thread.new do
  sleep 1
  token.cancel
  caller_thread.raise(Interrupt, "the test interrupts the bulk call")
end

begin
  ThinkThen.decide_many(question, records, cancel: token)
  warn "the call returned instead of raising"
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

puts format("raised: %s", raised.class)
puts format("wall:   %.3f s from start to raise (signal at 1.000 s)", wall)
puts format("stub:   requests at return %d, after 2 s settle %d", at_return, after_settle)

failures = []
failures << "the interrupt did not raise" unless raised.is_a?(Interrupt)
failures << format("the call stayed deaf: %.3f s past the signal", wall - 1.0) if wall > 1.45
failures << "the stub kept serving after the return (#{at_return} -> #{after_settle})" if at_return != after_settle

if failures.empty?
  puts "interrupt proof green"
else
  failures.each { |one| warn one }
  exit 1
end
