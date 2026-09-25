# The fourth review's annotate probe, as a suite file: the collision
# check must union every record's keys. The old check read only the
# first record's, so a question landing on a later record's column
# overwrote it silently.
$LOAD_PATH.unshift File.expand_path("../lib", __dir__)
require "json"
require "thinkthen"

set = ThinkThen.set("tone" => { "decide" => "Is the tone angry?", "threshold" => 0.6 })

# A question landing on the SECOND record's key refuses before any
# request is paid.
records = [
  { id: 1, body: "please help" },
  { id: 2, body: "help me now", tone: "caller-supplied" },
]
begin
  ThinkThen.annotate(set, records, on: :body)
  raise "the collision was not refused"
rescue ThinkThen::UsageError => e
  raise "the refusal does not name the column" unless e.message.include?("tone")
end
puts "ok  annotate-union-1: a question on any record's key refuses"

# The first record's keys still refuse, as before.
begin
  ThinkThen.annotate(set, [{ tone: "x", body: "hello" }], on: :body)
  raise "the first-record collision was not refused"
rescue ThinkThen::UsageError
end
puts "ok  annotate-union-2: a question on the first record's key refuses"

# No collision: every record keeps its own keys and gains the answers.
clean = [
  { id: 1, body: "please help" },
  { id: 2, body: "help me now", priority: "high" },
]
out = ThinkThen.annotate(set, clean, on: :body)
raise "record 1 lost its id" unless out[0][:id] == 1 && out[0].key?(:tone)
raise "record 2 lost its priority" unless out[1][:priority] == "high" && out[1].key?(:tone)
puts "ok  annotate-union-3: distinct keys pass through with the answers added"
