# frozen_string_literal: true

# The surface's own tests, offline against the null backend.
#
# Run with: ruby -I lib -I tests tests/test_surface.rb  (ENGINE_NULL=1)

require "minitest/autorun"
require "json"
require "thinkthen"

# The null backend answers by evidence: "refund" 0.97, "maybe" 0.55, else
# 0.03, with the file grammar's default cut.
class TestSurface < Minitest::Test
  def test_decide_cut
    assert_equal true, ThinkThen.decide("Does the customer ask for a refund?", "I want a refund for order 9")
    assert_equal false, ThinkThen.decide("Does the customer ask for a refund?", "Thanks, all good here")
  end

  def test_band_gives_nil_for_not_sure
    question = ThinkThen.question(decide: "Does the customer ask for a refund?", threshold: 0.2..0.8)
    assert_equal true, ThinkThen.decide(question, "I want a refund for order 9")
    assert_equal nil, ThinkThen.decide(question, "maybe I asked for one earlier, not sure")
    assert_equal false, ThinkThen.decide(question, "Thanks, all good here")
  end

  def test_cut_threshold_as_number
    question = ThinkThen.question(decide: "Does the customer ask for a refund?", threshold: 0.5)
    assert_equal true, ThinkThen.decide(question, "I want a refund for order 9")
  end

  def test_question_from_text_uses_default_cut
    assert_equal true, ThinkThen.decide("Is this a complaint?", "refund please")
  end

  def test_choose_is_nil_on_the_null_backend
    # The null backend answers one probability and cannot distinguish
    # options, so a choice needs the wire or a recording; the conformance
    # file carries those cases and skips them offline on every surface.
    picked = ThinkThen.choose("Which team fits?", "I want a refund for order 9", options: ["billing", "sales", "support"])
    assert_equal nil, picked
  end

  def test_score_returns_the_position
    value, level = ThinkThen.score_with_level("How urgent is this?", "I want a refund for order 9", levels: ["low", "mid", "high"])
    assert_kind_of Float, value
    assert_includes ["low", "mid", "high"], level
    assert_equal value, ThinkThen.score("How urgent is this?", "I want a refund for order 9", levels: ["low", "mid", "high"])
  end

  def test_tag_returns_labels_in_order
    labels = ThinkThen.tag("What does this mention?", "I want a refund for order 9", labels: ["money", "shipping", "account"])
    assert_kind_of Array, labels
    labels.each { |label| assert_includes ["money", "shipping", "account"], label }
  end

  def test_filter_returns_the_kept_records
    records = ["I want a refund", "thanks", "a refund please", "hello"]
    assert_equal ["I want a refund", "a refund please"], ThinkThen.filter("Is this a complaint?", records)
  end

  def test_filter_takes_any_enumerable
    lazy = ["I want a refund", "hello"].lazy
    assert_equal ["I want a refund"], ThinkThen.filter("Is this a complaint?", lazy)
  end

  def test_decide_many_keeps_order
    records = ["I want a refund", "hello", "maybe a refund"]
    assert_equal [true, false, true], ThinkThen.decide_many("Is this a complaint?", records)
  end

  def test_rank_orders_most_likely_yes_first
    records = ["hello", "I want a refund", "maybe"]
    ranked = ThinkThen.rank("Is this a complaint?", records)
    assert_equal "I want a refund", ranked.first
    assert_equal 3, ranked.length
    top = ThinkThen.rank("Is this a complaint?", records, top: 2)
    assert_equal 2, top.length
  end

  def test_find_returns_the_unit
    units = ["hello", "I want a refund for order 9", "weather talk"]
    found = ThinkThen.find("Which line asks for money back?", units)
    assert_equal "I want a refund for order 9", found
  end

  def test_annotate_over_strings
    set = ThinkThen.set(File.expand_path("fixture/form.json", __dir__))
    answers = ThinkThen.annotate(set, ["I want a refund", "hello"])
    assert_equal 2, answers.length
    assert answers[0].key?(:refund)
    assert_equal true, answers[0][:refund]
    assert_equal false, answers[1][:refund]
  end

  def test_annotate_over_hashes_with_on
    set = ThinkThen.set(File.expand_path("fixture/form.json", __dir__))
    rows = [{ "body" => "I want a refund", "id" => 1 }, { "body" => "hello", "id" => 2 }]
    answers = ThinkThen.annotate(set, rows, on: "body")
    assert_equal 1, answers[0][:id]
    assert_equal true, answers[0][:refund]
    assert_equal false, answers[1][:refund]
  end

  def test_details
    details = ThinkThen.details("Does the customer ask for a refund?", "I want a refund for order 9")
    assert_equal 0.97, details["probability"]
    assert_equal true, details["answer"]
    assert_equal "jev-latest", details["model"]
    assert_match(/\A[0-9a-f]{64}\z/, details["digest"])
    assert_equal 1, details["sends"]
  end

  def test_usage_counts_sends
    before = ThinkThen.usage["requests"]
    ThinkThen.decide("Does the customer ask for a refund?", "I want a refund for order 9")
    assert_equal before + 1, ThinkThen.usage["requests"]
  end

  def test_usage_error_is_an_argument_error
    error = assert_raises(ThinkThen::UsageError) do
      ThinkThen.question(decide: "", threshold: 0.2)
    end
    assert_kind_of ArgumentError, error
    assert_equal "usage", error.kind
    assert_equal false, error.retryable
  end

  def test_filter_refuses_a_band
    question = ThinkThen.question(decide: "Is this a complaint?", threshold: 0.2..0.8)
    error = assert_raises(ThinkThen::UsageError) do
      ThinkThen.filter(question, ["hello"])
    end
    assert_equal "usage", error.kind
  end

  def test_the_six_error_classes_exist
    %w[UsageError BackendError DeadlineError LocalError CancelledError DefectError].each do |name|
      assert ThinkThen.const_defined?(name, false), "#{name} is missing"
    end
  end

  def test_cancel_token_object
    token = ThinkThen::Cancel.new
    refute_nil token
    token.cancel
    assert_nil token.cancel
  end
end
