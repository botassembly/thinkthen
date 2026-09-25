# frozen_string_literal: true

# Harmless wake-ups must not cancel a call, and a real interrupt must fire
# only the call's own token.
#
# Before the fix every crossing passed Ruby an unblock function, and Ruby
# calls one for every interrupt at all — a spurious Thread#wakeup, a
# trapped signal, a real raise alike. The callback fired the token the
# engine watched, which was the caller's own token when one was given, so
# a wake-up cancelled the batch and one call's interrupt cancelled every
# other call sharing that token. The poll hears interrupts with the VM
# lock taken now, where MRI can tell a real pending exception from a
# spurious wake-up, and the engine watches a token only the poll fires.
#
# Offline; check.sh's null section runs it.
#
# Run with: ENGINE_NULL=1 ruby -I lib tests/test_harmless_wakeups.rb

require "thinkthen"

QUESTION = ThinkThen.question(decide: "Is this a complaint?")
RECORDS = 1_500_000
failures = []

def batch
  Array.new(RECORDS) { |i| "record #{i}" }
end

# 1. Thread#wakeup on the calling thread does not cancel the batch.
records = batch
waker = Thread.new do
  sleep 0.3
  Thread.current.report_on_exception = false
  main = Thread.main
  main.wakeup
  sleep 0.3
  main.wakeup
end
answer = begin
  ThinkThen.decide_many(QUESTION, records)
rescue StandardError => e
  failures << "a wake-up cancelled the call: #{e.class}: #{e.message}"
  nil
ensure
  waker.join
end
if answer && answer.size != records.size
  failures << "the wake-up run answered #{answer.size} records, not #{records.size}"
end

# 2. A trapped signal does not cancel the batch: the handler runs, nothing
#    raises, and the batch finishes. The signal lands on the main thread,
#    which is the thread inside the call.
signalled = false
trap("USR1") { signalled = true }
sender = Thread.new do
  sleep 0.3
  Process.kill("USR1", Process.pid)
end
records = batch
answer = begin
  ThinkThen.decide_many(QUESTION, records)
rescue StandardError => e
  failures << "a trapped USR1 cancelled the call: #{e.class}: #{e.message}"
  nil
ensure
  sender.join
  trap("USR1", "DEFAULT")
end
if answer && answer.size != records.size
  failures << "the USR1 run answered #{answer.size} records, not #{records.size}"
end
failures << "the USR1 handler never ran" unless signalled

# 3. Two calls share one caller token; interrupting one must not cancel the
#    other, and must not fire the shared token. Both record lists are
#    built first, so the interrupt lands inside the calls, not the setup.
shared = ThinkThen::Cancel.new
records_a = batch
records_b = batch
interrupted = Thread.new { ThinkThen.decide_many(QUESTION, records_a, cancel: shared) }
other = Thread.new { ThinkThen.decide_many(QUESTION, records_b, cancel: shared) }
interrupted.report_on_exception = false
sleep 0.4
interrupted.raise(Interrupt, "the test interrupts one call")
raised = begin
  interrupted.value
  false
rescue Interrupt
  true
rescue StandardError => e
  failures << "the interrupted call raised #{e.class}: #{e.message}"
  true
end
other_answer = begin
  other.value
rescue StandardError => e
  failures << "the interrupt cancelled the call sharing the token: #{e.class}: #{e.message}"
  nil
end
failures << "the interrupted call never raised" unless raised
if other_answer && other_answer.size != RECORDS
  failures << "the sibling call answered #{other_answer.size} records, not #{RECORDS}"
end

if failures.empty?
  puts "wake-ups and trapped signals leave calls alone; an interrupt fires only the call's own token"
else
  failures.each { |one| warn one }
  exit 1
end
