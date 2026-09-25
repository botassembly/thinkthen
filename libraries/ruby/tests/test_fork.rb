# frozen_string_literal: true

# Ian's item 6 for Ruby: a fork after the first call answers its own call.
#
# The parent answers once (whatever the engine holds is held before the
# fork), forks, and the child answers the same question. The parent's read
# is under a 10 s Timeout, so a hang fails the test instead of hanging it.
#
# Run with: ENGINE_NULL=1 ruby -I lib tests/test_fork.rb

require "thinkthen"
require "timeout"

BOUND_SECONDS = 10

question = ThinkThen.question(decide: "Does the customer ask for a refund?")
parent = ThinkThen.decide(question, "I want a refund for order 9")
raise "the parent's first call did not answer" unless parent == true

reader, writer = IO.pipe
pid = Process.fork do
  reader.close
  answered = ThinkThen.decide(question, "I want a refund for order 9") == true
  writer.write(answered ? "ok" : "wrong")
  writer.close
  exit!(0)
end
writer.close

outcome =
  begin
    Timeout.timeout(BOUND_SECONDS) { reader.read }
  rescue Timeout::Error
    Process.kill("KILL", pid)
    raise "the child hung past #{BOUND_SECONDS} s — a fork that does not answer"
  end
Process.wait(pid)
raise "the child did not answer its own call" unless outcome == "ok"

puts "fork after the first call answers in the child"
