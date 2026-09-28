# frozen_string_literal: true

# Every verb's Ruby shape through the real engine on the generic arm, which
# gives the first option, level, or yes 0.9 and shares the rest.
require "minitest/autorun"
require_relative "backend"

class TestSurface < Minitest::Test
  def run_child(script, arm: "generic")
    TestBackend.run(script, arm: arm)
  end

  def test_the_single_verbs_answer_in_their_ruby_shapes
    lines, count = run_child(<<~RUBY)
      say T.decide("Is it urgent?", "text")
      say T.decide(T.question(decide: "Is it urgent?", threshold: 0.2..0.95), "text")
      say T.decide(T.question(decide: "Is it urgent?", threshold: 0.95), "text")
      say T.choose("Which team?", "text", options: %w[billing shipping])
      say T.score("How urgent?", "text", levels: %w[low mid high])
      say T.score_with_level("How urgent?", "text", levels: %w[low mid high])
      say T.tag("Which labels?", "text", labels: %w[money shipping])
    RUBY
    assert_equal [true, nil, false, "billing", 0.15, [0.15, "low"], %w[money shipping]], lines
    # The three decide questions send the same request, and so do the two
    # score calls. The cache answers the repeats.
    assert_equal 4, count
  end

  def test_details_is_the_commands_details_document
    lines, = run_child(<<~RUBY)
      found = T.details("Is it urgent?", "text")
      say [found["schema"], found["value"], found["answer"], found["meta"]["requests_sent"], found["meta"]["requests"].size]
    RUBY
    assert_equal [["thinkthen.result/1", true, { "kind" => "yes_no", "probability" => 0.9 }, 1, 1]], lines
  end

  def test_the_bulk_verbs_keep_input_order_and_map_places_back
    lines, count = run_child(<<~RUBY)
      records = ["one", { note: "two" }, "three"]
      say T.decide_many("Is it urgent?", records)
      say T.filter("Is it urgent?", records.lazy)
      say T.filter(T.question(decide: "Is it urgent?", threshold: 0.95), records)
      say T.rank("Is it urgent?", records, top: 2).map(&:to_a)
      say T.find("Which line?", %w[first second]).to_a
    RUBY
    assert_equal [true, true, true], lines[0]
    assert_equal ["one", { "note" => "two" }, "three"], lines[1]
    assert_equal [], lines[2]
    assert_equal [[0, "one", 0.9], [1, { "note" => "two" }, 0.9]], lines[3]
    assert_equal [0, "first", 0.9], lines[4]
    # filter and rank ask the same three requests again, from the cache.
    assert_equal 4, count
  end

  # G4: the probabilities come from the same requests as the answers.
  def test_decide_many_with_probabilities_sends_one_request_a_record
    lines, count = run_child(<<~RUBY)
      rows = T.decide_many_with_probabilities("Is it urgent?", (1..20).map { |n| "record \#{n}" })
      say [rows.size, rows.first]
    RUBY
    assert_equal [[20, { "answer" => true, "probability" => 0.9 }]], lines
    assert_equal 20, count
  end

  def test_annotate_names_each_answer_and_joins_the_on_keys
    lines, = run_child(<<~RUBY)
      set = T.set(#{File.expand_path("fixture/form.json", __dir__).inspect})
      say set.names
      say T.annotate(set, ["a refund please"])
      say T.annotate(set, [{ "body" => "a refund please", "id" => 7 }], on: "body")
    RUBY
    assert_equal [%w[refund complaint], [{ "refund" => true, "complaint" => true }],
                  [{ "body" => "a refund please", "id" => 7, "refund" => true, "complaint" => true }]], lines
  end

  # R3-15b: every record's keys count, and the refusal sends nothing.
  def test_annotate_refuses_a_question_landing_on_any_records_key
    lines, count = run_child(<<~RUBY)
      set = T.set(refund: { "decide" => "Does it ask for a refund?" })
      begin
        T.annotate(set, [{ "body" => "a" }, { "body" => "b", "refund" => 1 }], on: "body")
      rescue T::UsageError => e
        say e.message
      end
    RUBY
    assert_equal ["annotate cannot add a question named 'refund': the input already has a key by that name; rename one"], lines
    assert_equal 0, count
  end

  # find's none is true or false, refused before any request.
  def test_find_refuses_a_none_that_is_not_true_or_false
    lines, count = run_child(<<~RUBY)
      begin
        T.find("Which?", %w[a b], none: "yes")
      rescue T::UsageError => e
        say e.message
      end
    RUBY
    assert_equal ["none is true or false"], lines
    assert_equal 0, count
  end

  def test_rank_and_find_refuse_a_built_profile_before_sending
    lines, count = run_child(<<~RUBY)
      calibrated = T.question(decide: "Is this urgent?", profile: "old")
      [:rank, :find].each do |verb|
        begin
          verb == :rank ? T.rank(calibrated, %w[a b]) : T.find(calibrated, %w[a b], none: true)
        rescue T::UsageError => e
          say [e.kind, e.message]
        end
      end
    RUBY
    assert_equal [["usage", "rank takes a decide question with no profile"],
                  ["usage", "find takes a decide question with no profile"]], lines
    assert_equal 0, count
  end

  # R3-16 and G11: a nil, invalid UTF-8, or NUL record refuses by its index
  # before any request.
  def test_a_record_with_no_honest_text_refuses_by_index
    lines, count = run_child(<<~RUBY)
      [["one", nil], ["one", "two", "\\xff".b], ["one\\0two"]].each do |records|
        T.decide_many("Is it urgent?", records)
      rescue T::UsageError => e
        say e.message
      end
    RUBY
    assert_equal ["record 1 is nil; a record is text or a JSON value", "record 2 is not valid UTF-8",
                  "record 0 holds a NUL byte"], lines
    assert_equal 0, count
  end

  # R2-25: a record that is not a String crosses as its JSON text. The
  # generic arm names the text's last piece, which ends at the JSON text's
  # 17th character.
  def test_a_hash_record_crosses_as_its_json_text
    lines, = run_child(<<~RUBY)
      say T.recognize({ note: "refund" }, kinds: %w[thing]).entities.map { |one| [one.text, one.start, one.end] }
    RUBY
    assert_equal [[['"}', 15, 17]]], lines
  end

  def test_recognize_and_relate_return_entity_ends
    lines, = run_child(<<~RUBY)
      found = T.recognize("Ana Lima", kinds: { "person" => "A person's name." }, relations: { knows: %w[person person] })
      say [found.entities.map(&:to_a), found.relations]
      found = T.recognize("Ana Lima", kinds: %w[person])
      say found.relations
      edges = T.relate([["Ana", "person"], { name: "Acme", kind: "organization" }], relations: { works_for: %w[person organization] })
      say edges.map { |edge| [edge.relation, edge.source.to_a, edge.target.to_a, edge.probability] }
    RUBY
    assert_equal [[["Ana Lima", 0, 8, 8, "person", 0.81]], []], lines[0]
    assert_nil lines[1]
    assert_equal [["works_for", ["Ana", "person"], ["Acme", "organization"], 0.9]], lines[2]
  end

  # ADR 0056: a found name carries text in place of name, and relate reads it
  # as the name. name wins when a Hash holds both.
  def test_relate_reads_what_recognize_found
    lines, = run_child(<<~RUBY)
      rules = { knows: %w[person person] }
      found = T.recognize("Maria Chen arrived.", kinds: %w[person]).entities
      forms = [found, found.map { |one| { text: one.text, kind: one.kind } },
               found.map { |one| { "name" => one.text, "text" => "not this", "kind" => one.kind } },
               [["Maria Chen", "person"], ["arrived.", "person"]]]
      say(forms.map { |form| T.relate(form, relations: rules).map { |edge| [edge.source.to_a, edge.target.to_a] } }.uniq)
    RUBY
    assert_equal [[[["Maria Chen", "person"], ["arrived.", "person"]], [["arrived.", "person"], ["Maria Chen", "person"]]]], lines[0]
  end

  def test_a_built_question_carries_its_members_once
    lines, count = run_child(<<~RUBY)
      question = T.question(choose: "Which team?", options: %w[billing shipping])
      begin
        T.choose(question, "text", options: %w[billing])
      rescue T::UsageError => e
        say [e.message, e.kind]
      end
      banded = T.question(decide: "Is it urgent?", threshold: 0.2..0.8)
      begin
        T.filter(banded, ["one"])
      rescue T::UsageError => e
        say e.message
      end
    RUBY
    assert_equal [["choose: a built question carries its own options; pass them on the question", "usage"],
                  "filter does not take a banded question"], lines
    assert_equal 0, count
  end

  def test_usage_counts_sends_and_cache_answers
    lines, = run_child(<<~RUBY)
      2.times { T.decide("Is it urgent?", "the same text") }
      say T.usage
    RUBY
    assert_equal [{ "requests_sent" => 1, "retries" => 0, "cache_answers" => 1, "input_tokens" => 0, "output_tokens" => 0 }], lines
  end
end
