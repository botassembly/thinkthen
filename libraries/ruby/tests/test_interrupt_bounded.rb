# The fourth review's latency probe, as a wire suite file: a bulk call
# must stop soon after a real interrupt. The watchdog fires the call's
# token; sent requests finish and no new one starts, so a 500-record
# batch on an 8-wide engine ends one wave after the raise instead of
# running the whole batch. Needs the stub on 8214 with a delay
# (check.sh starts it with STUB_DELAY_MS=1000).
$LOAD_PATH.unshift File.expand_path("../lib", __dir__)
require "json"
require "net/http"
require "uri"
require "thinkthen"

STUB = ENV.fetch("STUB_URL", "http://127.0.0.1:8214/v1")

def stats
  JSON.parse(Net::HTTP.get(URI("#{STUB}/stats")))
end

Net::HTTP.post(URI("#{STUB}/reset"), "")

question = ThinkThen.question(decide: "Is this a complaint?")
records = Array.new(500) { |i| "record #{i} wants a refund" }

started = Process.clock_gettime(Process::CLOCK_MONOTONIC)
sniper = Thread.new { sleep 0.5; Thread.main.raise Interrupt, "sniped" }
outcome = begin
  ThinkThen.decide_many(question, records)
  :ok
rescue Exception => e
  e.class
ensure
  begin; sniper.join; rescue Exception; end
end
elapsed = Process.clock_gettime(Process::CLOCK_MONOTONIC) - started
requests = stats.fetch("requests")

raise "the raise did not surface (#{outcome})" unless outcome == Interrupt
raise "the batch ran past one wave: #{format('%.1f', elapsed)} s" if elapsed >= 2.0
raise "more than one width-8 wave was sent: #{requests} requests" if requests > 16
puts "ok  bulk-interrupt-1: raise at 0.5 s surfaced at #{format('%.2f', elapsed)} s with #{requests} requests (one wave)"

# A single call's interrupt honesty: one request is in flight, the
# contract lets it finish, and the raise surfaces the moment the
# crossing returns - never swallowed, never resent.
Net::HTTP.post(URI("#{STUB}/reset"), "")
# Measure one round trip first, so the assertions below are relative to
# whatever delay the stub is running with, not a constant.
control_started = Process.clock_gettime(Process::CLOCK_MONOTONIC)
ThinkThen.decide(question, "measure the round trip")
rtt = Process.clock_gettime(Process::CLOCK_MONOTONIC) - control_started
Net::HTTP.post(URI("#{STUB}/reset"), "")
started = Process.clock_gettime(Process::CLOCK_MONOTONIC)
sniper = Thread.new { sleep 0.15; Thread.main.raise Interrupt, "sniped" }
outcome = begin
  ThinkThen.decide(question, "I want a refund for order 9")
  :ok
rescue Exception => e
  e.class
ensure
  begin; sniper.join; rescue Exception; end
end
elapsed = Process.clock_gettime(Process::CLOCK_MONOTONIC) - started
requests = stats.fetch("requests")
raise "the raise did not surface" unless outcome == Interrupt
raise "the single call resent: #{requests} requests" unless requests == 1
raise "the raise surfaced before the in-flight request finished: #{format('%.2f', elapsed)} vs rtt #{format('%.2f', rtt)}" if elapsed < rtt * 0.8
puts "ok  single-interrupt-1: one request, no resend, raise at the crossing's exit (#{format('%.2f', elapsed)} s)"
