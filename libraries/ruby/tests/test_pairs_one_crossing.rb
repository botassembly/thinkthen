# frozen_string_literal: true

# Punch-list item 1, the acceptance: exposing each judgment's probability
# must cost no extra sends. One `decide_many_with_probabilities` call over
# 20 records is one crossing and exactly 20 counted requests on the stub.
# The old shape made a second details call a record, which the counted
# backend would show as 40. Skipped when no stub is up on 8214.
#
# Run with: ENGINE_BASE_URL=http://127.0.0.1:8214/v1 \
#          ruby -I lib tests/test_pairs_one_crossing.rb

require "json"
require "net/http"
require "uri"
require "thinkthen"

STUB = URI("http://127.0.0.1:8214")

def stats
  JSON.parse(Net::HTTP.get(STUB + "/v1/stats"))
end

held = stats
abort("no stub on 8214; run this with the stub up") if held.nil? || held.empty?

Net::HTTP.post(STUB + "/v1/reset", "")

records = Array.new(20) { |i| i.zero? ? "I want a refund for order 9" : "filler record #{i}" }
pairs = ThinkThen.decide_many_with_probabilities("Is this a complaint?", records)

abort "expected 20 pairs, got #{pairs.length}" unless pairs.length == 20
abort "the first pair carries no probability" unless pairs.first[:probability].is_a?(Float)
abort "the first answer is wrong" unless pairs.first[:answer] == true
abort "a filler answer is wrong" unless pairs[1][:answer] == false

at_return = stats["requests"]
if at_return != records.length
  abort "expected exactly #{records.length} requests, saw #{at_return} (a details " \
        "pass a record would make #{records.length * 2})"
end

sleep 2
settled = stats["requests"]
abort "requests moved after the return: #{at_return} -> #{settled}" unless settled == at_return

puts "one crossing: #{pairs.length} pairs beside their probabilities, " \
     "#{at_return} requests, none after the return"
