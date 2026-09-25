# frozen_string_literal: true

# The deadline's bounds, checked at the contract's one door: a NaN, an
# infinity, a negative other than the sentinel, and an oversized budget
# raise the usage kind; minus one means no deadline; zero stays a spent
# deadline. Before the shim adopted the checked conversion, a huge budget
# panicked inside the host process instead of raising, and a NaN read as a
# spent deadline.
#
# Offline; check.sh's null section runs it.
#
# Run with: ENGINE_NULL=1 ruby -I lib tests/test_deadline_bounds.rb

require "thinkthen"

QUESTION = ThinkThen.question(decide: "Is this a complaint?")

def raised_kind
  yield
  nil
rescue StandardError => e
  e.respond_to?(:kind) ? e.kind : "not a ThinkThen error: #{e.class}: #{e.message}"
end

failures = []

[Float::NAN, Float::INFINITY, 1e300, -5.0].each do |held|
  kind = raised_kind { ThinkThen.decide(QUESTION, "refund please", deadline: held) }
  unless kind == "usage"
    failures << format("deadline %s: expected the usage kind, got %s", held.inspect, kind.inspect)
  end
end

begin
  answer = ThinkThen.decide(QUESTION, "refund please", deadline: -1)
  failures << "minus one means no deadline; the call answered #{answer.inspect}" unless answer == true
rescue StandardError => e
  failures << "minus one means no deadline; the call raised #{e.class}: #{e.message}"
end

kind = raised_kind { ThinkThen.decide(QUESTION, "refund please", deadline: 0) }
failures << "zero stays a spent deadline; got #{kind.inspect}" unless kind == "deadline"

if failures.empty?
  puts "the deadline bounds hold: usage for hostile budgets, minus one means none, zero stays spent"
else
  failures.each { |one| warn one }
  exit 1
end
