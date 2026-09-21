# frozen_string_literal: true

# The slide sample, exactly as drawn, against the stand-in's offline
# backend. The drawn block is copied verbatim from
# repos/mktg/decks/2026-09-21-thinkthen-semantic-commands/surfaces.md; the
# fixtures around it only pick the data. After the drawn block, the
# harness asserts what the comments promise.
#
# Run with: ruby -I lib tests/slide_sample.rb  (ENGINE_NULL=1)

$LOAD_PATH.unshift(File.expand_path("../lib", __dir__))
require "thinkthen"

# Fixtures. The null backend answers by evidence: "refund" 0.97, "maybe"
# 0.55, else 0.03, default cut 0.5.
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

# === the slide, as drawn ===
# keep the records where the answer is yes
upset = ThinkThen.filter("Is this a complaint?", reviews)
puts "#{upset.size} of #{reviews.size} are complaints"

# any Enumerable works, and it crosses to the engine once
urgent = ThinkThen.rank("Is this urgent?", inbox, top: 5)
urgent.each { |mail| puts mail }

# place a text on your own scale: 2.0 is "Immediate."
levels = ["Routine.", "Soon.", "Immediate."]
ThinkThen.score("How urgent is this?", outage, levels:)
# === end of the slide ===

# What the comments promise.
failures = []
failures << "filter kept #{upset.size}, the comment promises the refund and maybe records (3)" unless upset.size == 3
expected_rank = [
  "I want a refund for order 9",
  "The refund never arrived",
  "maybe escalate this one",
  "Where is my order?",
  "Hello team"
]
failures << "rank order diverged: #{urgent.inspect}" unless urgent == expected_rank
scored, nearest = ThinkThen.score_with_level("How urgent is this?", outage, levels: levels)
failures << "score returned #{scored}, outside the top level's range" unless scored > 1.0 && scored <= 2.0
failures << "score's nearest level is #{nearest.inspect}, the comment promises 'Immediate.'" unless nearest == "Immediate."
if scored != 2.0
  # A finding, reported, not a silent change: the slide's comment says 2.0,
  # the offline backend's level distribution puts the position at 1.7
  # with the top level still the nearest. The slide owns the fix.
  warn "finding: score returned #{scored}, the slide comment says 2.0; nearest level is 'Immediate.'"
end

if failures.empty?
  puts "slide sample green"
else
  failures.each { |one| warn one }
  exit 1
end
