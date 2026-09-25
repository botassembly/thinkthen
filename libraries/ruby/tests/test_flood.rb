# The fourth review's crash probe, as a suite file: a trap handler that
# raises under a 0.2 ms signal flood during a 200k-record decide_many.
# The old crossing re-took the VM lock inside the engine's wait and the
# raise's jump crossed the with-gvl machinery - a core dump in
# coroutine_transfer, four of four. The restructured crossing never
# re-takes the lock beneath the engine call, so every round ends in a
# caught raise and the VM survives.
#
# Offline: the null backend answers the batch; the flood supplies the
# interrupts.
$LOAD_PATH.unshift File.expand_path("../lib", __dir__)
require "json"
require "thinkthen"

FLOOD_US = 200
RECORDS = 200_000
ROUNDS = 4

Signal.trap("USR1") { raise "trap raise" }
question = ThinkThen.question(decide: "Is this a complaint?")

def run_round(question, records)
  outcome = nil
  begin
    begin
      flood = Thread.new do
        loop { Process.kill("USR1", Process.pid); sleep(FLOOD_US.to_f / 1_000_000) }
      end
      ThinkThen.decide_many(question, Array.new(records) { |i| "record #{i}" })
      outcome = :completed
    ensure
      flood.kill rescue nil
      flood.join rescue nil
    end
  rescue Exception => e
    outcome = e
  end
  outcome
end

Signal.trap("USR1") { } # the flood must not kill the harness after the rounds

results = ROUNDS.times.map { run_round(question, RECORDS) }
raise "a round left no outcome" if results.any?(&:nil?)

caught = results.count { |one| one.is_a?(StandardError) || one == :completed }
raise "only #{caught}/#{ROUNDS} rounds ended cleanly: #{results.inspect[0, 200]}" unless caught == ROUNDS
puts "ok  flood-1: #{ROUNDS} rounds under a raising trap flood, no VM crash"
