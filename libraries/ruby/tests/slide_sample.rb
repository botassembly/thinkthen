# frozen_string_literal: true

# The Ruby slide example in a scrubbed child against its own loopback
# backend on 0092's generic arm. The site deck needs its separately owned
# migration to the public Call shape. The fixtures around this example
# only pick the data. After the drawn block, the harness checks what the
# generic arm makes checkable: every text answers yes at 0.9, and every
# score lands nearest the first level.
require_relative "backend"

lines, count = TestBackend.run(<<~'RUBY')
  reviews = [
    "I want a refund for order 9",
    "The refund never arrived",
    "maybe I was charged twice, not sure",
    "Thanks, the product is great",
    "Where is my order?",
    "Love it, no problems",
    "Invoice question, all fine otherwise",
    "Just checking in"
  ]
  inbox = [
    "Where is my order?",
    "I want a refund for order 9",
    "Hello team",
    "maybe escalate this one",
    "The refund never arrived",
    "Weekly summary attached"
  ]
  outage = "I want a refund because the outage broke my orders"
  require "stringio"
  $stdout = StringIO.new

  # === the Ruby example ===
  # keep the records where the answer is yes
  upset = ThinkThen.filter("Is this a complaint?", reviews, batch: 1).value
  puts "#{upset.size} of #{reviews.size} are complaints"

  # any Enumerable works, and it crosses to the engine once
  urgent = ThinkThen.rank("Is this urgent?", inbox, top: 5, batch: 1).value
  urgent.each { |mail| puts mail }

  # place a text on your own scale: 2.0 is "Immediate."
  levels = ["Routine.", "Soon.", "Immediate."]
  ThinkThen.score("How urgent is this?", outage, levels:).value
  # === end of the Ruby example ===

  printed = $stdout.string
  $stdout = STDOUT
  say [printed, upset == reviews, urgent.map(&:record) == inbox.first(5),
       ThinkThen.score_with_level("How urgent is this?", outage, levels:).value]
RUBY

printed, kept, ranked, (scored, nearest) = lines.fetch(0)
failures = []
drawn = ["8 of 8 are complaints", "Where is my order?", "I want a refund for order 9", "Hello team",
         "maybe escalate this one", "The refund never arrived"]
failures << "the slide printed #{printed.inspect}" unless printed == drawn.map { |line| "#{line}\n" }.join
failures << "filter did not keep every yes" unless kept
failures << "rank's tied top five left input order" unless ranked
failures << "score gave #{scored} nearest #{nearest.inspect}" unless scored.between?(0.0, 2.0) && nearest == "Routine."
# 8 filter sends, 6 rank sends, and 1 score send; the repeat comes from the cache.
failures << "the backend counted #{count} sends, not 15" unless count == 15
if failures.empty?
  puts "slide sample green"
else
  failures.each { |one| warn one }
  exit 1
end
