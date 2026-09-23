# frozen_string_literal: true

# The surface's own tests, offline against the null backend.
#
# Run with: ruby -I lib -I tests tests/test_surface.rb  (ENGINE_NULL=1)
#
# The stand-in's one synthesized partial failure (conformance case 74)
# fires only in a build that armed the compile-time `synthetic-partial`
# feature; check.sh builds the gate's copy that way.
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
    # The ruled pair (settled 2026-09-21): the record's place, the record,
    # and the probability the backend gave it.
    assert_equal "I want a refund", ranked.first.record
    assert_equal 1, ranked.first.index
    assert_in_delta 0.97, ranked.first.probability, 1e-9
    assert_equal 3, ranked.length
    top = ThinkThen.rank("Is this a complaint?", records, top: 2)
    assert_equal 2, top.length
  end

  def test_find_returns_the_unit_and_its_probability
    units = ["hello", "I want a refund for order 9", "weather talk"]
    found = ThinkThen.find("Which line asks for money back?", units)
    # The ruled pair (settled 2026-09-21): place, unit, and probability.
    assert_equal 1, found.index
    assert_equal "I want a refund for order 9", found.unit
    assert_in_delta 0.97, found.probability, 1e-9
  end

  def test_a_built_question_carries_its_members_once
    # Settled 2026-09-21: a built question plus members refuses, naming both.
    question = ThinkThen.question(choose: "Which team?", options: %w[billing shipping])
    error = assert_raises(ThinkThen::UsageError) do
      ThinkThen.choose(question, "some text", options: %w[billing])
    end
    assert_match(/options/, error.message)
    assert_equal "usage", error.kind
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
    # The nearest level's name on a score question; nil on every other
    # verb (settled 2026-09-21).
    assert_nil details["nearest"]
  end

  def test_details_nearest_on_a_score_question
    question = ThinkThen.question(score: "How strong is the refund claim?", levels: %w[low mid high])
    details = ThinkThen.details(question, "maybe later")
    assert_equal "mid", details["nearest"]
  end

  def test_details_carries_the_requests_list_and_the_failure_count
    # 0053 and 0054: one 64-figure digest a logical request, and
    # failed_questions always present, zero for one good question.
    details = ThinkThen.details("Does the customer ask for a refund?", "I want a refund for order 9")
    assert_kind_of Array, details["requests"]
    assert_equal 1, details["requests"].length
    details["requests"].each { |digest| assert_match(/\A[0-9a-f]{64}\z/, digest) }
    assert_equal 0, details["failed_questions"]
  end

  def test_annotate_preserves_the_good_answers_and_marks_the_failed_one
    # The stand-in's one synthesized partial failure (0054): the reply
    # answers one question and omits the last in name order, so its field
    # carries the ruled marker in this host's spelling (a Hash), never
    # nil.
    set = ThinkThen.send(:_parse_set, JSON.generate(
      "version" => 1,
      "questions" => {
        "refund" => { "decide" => "Is this a refund request?", "threshold" => 0.5 },
        "topic" => { "decide" => "Is this a billing problem?", "threshold" => 0.5 }
      }
    ))
    rows = ThinkThen.annotate(set, ["order 4471: charged twice, please refund"])
    assert_equal true, rows[0][:refund]
    assert_equal({ "failed" => { "kind" => "backend", "cause" => "missing_answer" } },
                 rows[0][:topic])
    clean = ThinkThen.annotate(set, ["I want a refund for order 4471"])
    assert_equal true, clean[0][:topic]
  end

  def test_usage_counts_sends
    before = ThinkThen.usage["requests"]
    ThinkThen.decide("Does the customer ask for a refund?", "I want a refund for order 9")
    assert_equal before + 1, ThinkThen.usage["requests"]
  end

  def test_usage_error_is_a_thinkthen_error
    error = assert_raises(ThinkThen::UsageError) do
      ThinkThen.question(decide: "", threshold: 0.2)
    end
    assert_kind_of ThinkThen::Error, error
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
    # The host error carries the kind and the retry signal the shim maps
    # onto it. The Rust half's unit test constructs the contract Error with
    # kind defect and asserts the class-name table (check.sh's shim test).
    defect = ThinkThen::DefectError.new("the engine broke its own contract", "defect", false)
    assert_equal "defect", defect.kind
    refute defect.retryable
  end

  def test_cancel_token_object
    token = ThinkThen::Cancel.new
    refute_nil token
    token.cancel
    assert_nil token.cancel
  end

  # The deck's recognize call, as drawn, against the recordings.
  def test_recognize_as_drawn
    text = "Maria Chen joined Northwind Freight in Chicago last spring."
    found = ThinkThen.recognize(
      text, kinds: %w[person organization place],
      relations: { works_for: %w[person organization] }
    )
    assert_equal "person", found.entities.first.kind
    assert_equal "Maria Chen", found.entities.first.text
    assert_equal "Maria Chen", text[found.entities.first.start...found.entities.first.end]
    assert_in_delta 0.98, found.entities.first.strength, 1e-9
    assert_equal ["works_for", 1, 2], [found.relations.first.name, found.relations.first.source,
                                      found.relations.first.target]
    assert_in_delta 1.0, found.relations.first.probability, 1e-9
  end

  # Offsets index Ruby characters: the emoji is one, and the name slices
  # clean in front of an accented letter.
  def test_recognize_offsets_count_ruby_characters
    text = "Le café 😀 Maria Chen arrived."
    found = ThinkThen.recognize(text, kinds: %w[person])
    name = found.entities.first
    assert_equal "Maria Chen", text[name.start...name.end]
  end

  # Punch-list item 3, repeated names and endpoints: the same place named
  # twice gives two entities with two ids, and a relation's endpoints are
  # those ids.
  def test_recognize_repeated_names_carry_distinct_ids
    text = "Chicago sent a delegation in March, and Chicago hosted the reply in June."
    found = ThinkThen.recognize(text, kinds: %w[place])
    assert_equal 2, found.entities.length
    first, second = found.entities
    assert_equal ["Chicago", "Chicago"], [first.text, second.text]
    refute_equal first.id, second.id
    assert_equal "Chicago", text[first.start...first.end]
    assert_equal "Chicago", text[second.start...second.end]
  end

  # Punch-list item 3, nested mutation isolation: everything the caller is
  # handed is the host's own copy. Mutating it at any depth must not change
  # the engine's next answer.
  def test_result_mutation_isolates_from_the_engine
    text = "Chicago sent a delegation in March, and Chicago hosted the reply in June."
    first = ThinkThen.recognize(text, kinds: %w[place])
    assert_equal 2, first.entities.length
    first.entities.first.text.replace("Oslo")
    first.entities.first.kind.replace("organization")
    first.entities.first.start = 999
    first.entities.clear
    first.relations << :junk

    second = ThinkThen.recognize(text, kinds: %w[place])
    assert_equal 2, second.entities.length
    assert_equal "Chicago", second.entities.first.text
    assert_equal "place", second.entities.first.kind
    assert_equal 0, second.entities.first.start
    assert_equal 7, second.entities.first.end
    assert_equal [], second.relations
  end

  # Punch-list item 3, result lifetime: the values stay readable after the
  # call returns, through a GC pass and a later call.
  def test_result_lifetime_across_gc_and_later_calls
    text = "Le café 😀 Maria Chen arrived."
    found = ThinkThen.recognize(text, kinds: %w[person])
    GC.start
    ThinkThen.usage
    name = found.entities.first
    assert_equal "Maria Chen", name.text
    assert_equal "person", name.kind
    assert_equal "Maria Chen", text[name.start...name.end]
  end

  # The slide's relate call against the four alerts' method-H recording.
  def test_relate_as_drawn
    alerts = ["Checkout returns 500 at the payment step.",
              "Card charges are failing for every customer.",
              "The nightly export ran two hours late.",
              "The payments database ran out of disk space."]
    edges = ThinkThen.relate(alerts, relations: %w[caused_by], either: %w[same_as])
    assert_equal 5, edges.length
    edge = edges.find { |one| one.name == "caused_by" && one.source == 1 && one.target == 4 }
    refute_nil edge, "the recorded 1-to-4 caused_by edge is missing"
    assert_in_delta 0.71, edge.probability, 1e-9
  end

  # relate takes every record at once, and the 255 limit refuses with the
  # usage kind before any question is asked.
  def test_relate_refuses_more_than_255_records
    records = Array.new(256) { |i| "alert #{i}" }
    error = assert_raises(ThinkThen::UsageError) { ThinkThen.relate(records, relations: %w[caused_by]) }
    assert_equal "usage", error.kind
    assert_match(/255/, error.message)
  end

  # The any-kind end is the one-character string "*" on this surface:
  # the recorded C36 run asks located_in from * to place.
  def test_the_any_kind_end_is_the_string_star
    text = "The road from Hull to Leeds was closed."
    found = ThinkThen.recognize(text, kinds: %w[place],
                                relations: { located_in: ["*", "place"] })
    assert_equal %w[Hull Leeds], found.entities.map(&:text)
    assert_equal [], found.relations
  end
end
