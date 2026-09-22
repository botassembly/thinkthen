# frozen_string_literal: true

# The error classes and the record text: one shared base, a bad deadline
# that raises instead of reading as no deadline, and records that cross as
# JSON rather than Ruby's `to_s` form.
#
# Before the fix the six kind classes had no shared base, so one
# `rescue ThinkThen::Error` heard nothing; a deadline that was not a
# number was dropped (`try_convert(...).ok()`), so `deadline: "soon"` read
# as no deadline at all; and every record crossed through `to_s`, so a
# Hash went as Ruby's `{:note=>"refund"}` form and a record object lost
# its own `to_json`.
#
# Offline; check.sh's null section runs it.
#
# Run with: ENGINE_NULL=1 ruby -I lib tests/test_error_classes.rb

require "thinkthen"

QUESTION = ThinkThen.question(decide: "Is this a complaint?")
failures = []

# 1. One shared base.
begin
  raise ThinkThen::Error, "the base class exists and is a StandardError"
rescue ThinkThen::Error => e
  failures << "the base is not a StandardError" unless e.is_a?(StandardError)
end
[ThinkThen::UsageError, ThinkThen::BackendError, ThinkThen::DeadlineError,
 ThinkThen::LocalError, ThinkThen::CancelledError, ThinkThen::DefectError].each do |klass|
  failures << "#{klass} does not inherit ThinkThen::Error" unless klass < ThinkThen::Error
end

builder = begin
  ThinkThen.question(decide: "", threshold: 0.2)
  nil
rescue ThinkThen::Error => e
  e
end
failures << "the builder's refusal is not a ThinkThen::Error" if builder.nil? || !builder.is_a?(ThinkThen::UsageError)

engine = begin
  ThinkThen.decide(QUESTION, "this text is malformed")
  nil
rescue ThinkThen::Error => e
  e
end
failures << "the engine's refusal is not a ThinkThen::Error" if engine.nil?
failures << "the engine's refusal is not a BackendError" if engine && !engine.is_a?(ThinkThen::BackendError)

# 2. A bad deadline raises instead of dropping.
bad = begin
  ThinkThen.decide(QUESTION, "I want a refund", deadline: "soon")
  nil
rescue ThinkThen::UsageError => e
  e
end
failures << "a text deadline was dropped instead of refused" if bad.nil?
failures << "the refusal's kind is #{bad.kind.inspect}, not usage" if bad && bad.kind != "usage"

# 3. Records cross as JSON, not `to_s`. The record's own `to_json` carries
#    the word the null backend judges; its `to_s` does not, so the answer
#    says which form crossed.
class QuietRefund
  def to_s
    "quiet"
  end

  def to_json(*)
    '{"note":"refund"}'
  end
end

records = [QuietRefund.new, "plain text"]
answers = begin
  ThinkThen.decide_many(QUESTION, records)
rescue StandardError => e
  failures << "the record batch raised #{e.class}: #{e.message}"
  nil
end
failures << "a record crossed as to_s, not its JSON text" if answers && answers[0] != true
failures << "a plain String record must cross unchanged" if answers && answers[1] != false

evidence = begin
  ThinkThen.decide(QUESTION, QuietRefund.new)
rescue StandardError => e
  failures << "the evidence call raised #{e.class}: #{e.message}"
  nil
end
failures << "evidence crossed as to_s, not its JSON text" if evidence != true

if failures.empty?
  puts "one base class, a refused bad deadline, and records that cross as JSON"
else
  failures.each { |one| warn one }
  exit 1
end
