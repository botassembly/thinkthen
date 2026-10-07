# frozen_string_literal: true
# Pure carrier fixtures, independent of the native result/2 bridge.
require "minitest/autorun"
require_relative "../lib/thinkthen/requests"
class CompleteCarrierTest < Minitest::Test
  C = ThinkThen.const_get(:Complete)
  FIXTURE = JSON.parse(File.read(File.expand_path("../../python/tests/fixtures/complete.json", __dir__)))
  def test_all_ten_carriers_keep_probabilities_locations_and_failure_states
    FIXTURE.fetch("results").each do |row|
      result = C.decode(row.fetch("type"), row.fetch("result"))
      assert_equal row.fetch("result"), C.to_json_value(result)
      assert_instance_of C::AnswerId, result.answer_id
      assert_equal ["e" * 64, "e" * 64], result.meta.requests
      refute_includes result.inspect, "reported"
      assert result.frozen?
      assert_equal 0, C.decode(row.fetch("type"),row.fetch("result").merge("index"=>0)).index
      assert_raises(ArgumentError) { C.decode(row.fetch("type"),row.fetch("result").merge("index"=>-1)) }
    end
    candidate=C.decode("FindCandidate",{"index"=>0,"input"=>false,"probability"=>0.25,"source"=>{"file"=>"é.txt","first_line"=>2,"last_line"=>2}})
    assert_equal false,candidate.input
    assert_equal 2,candidate.source.first_line
    assert_equal 0.25,candidate.probability
    decide = C.decode("DecideResult", FIXTURE["results"][0]["result"])
    assert_equal false, decide.value
    assert_nil decide.question.true
    choose = C.decode("ChooseResult", FIXTURE["results"][1]["result"])
    assert_nil choose.value
    assert_equal 0, choose.answer.confidence
    assert_equal %w[b a], choose.answer.probabilities.keys
    assert_equal %w[red.png blue.png red.png], choose.position.images
    assert_same C::ABSENT, choose.position.first
    annotated = C.decode("AnnotateResult", FIXTURE["results"][7]["result"])
    assert_nil annotated.answers["ok"].value
    assert_equal "missing_answer", annotated.answers["bad"].failure.cause
    recognized = C.decode("RecognizeResult", FIXTURE["results"][8]["result"])
    assert_equal 2, recognized.value.entities.first.end
    assert_nil recognized.answer.names.first.edges
    relation = C.decode("RelateResult", FIXTURE["results"][9]["result"])
    assert_equal "é.txt", relation.value.first.source.file
    assert_equal "é.txt", relation.value.first.target.file
    assert_instance_of C::RelationFailure, relation.answer.questions.last
    empty = C.decode("RelateResult", FIXTURE["empty"]["result"])
    assert_nil empty.meta.origin
    assert_same C::ABSENT, empty.meta.answered_by
    assert_equal false, empty.meta.cached
  end
  def test_ids_facts_and_started_failures_refuse_synthetic_metadata
    facts = C.decode("Facts", FIXTURE["facts"])
    assert_instance_of C::CallId, facts.call_id
    assert_same C::ABSENT, facts.command_ms
    FIXTURE["errors"].each { |error| assert_same C::ABSENT, C.decode("CallError", error).facts }
    failed = C.decode("CallError", FIXTURE["started_error"])
    assert_equal 0, failed.attempts.first.server_ms
    assert_instance_of C::SdkRequestId, failed.attempts.first.sdk_request_id
    ["A" * 64, "a" * 63, 0].each { |id| assert_raises(ArgumentError) { C::AnswerId.new(id) } }
    [{"schema" => "thinkthen.result/1"}, {"answer_id" => "a" * 63}, {"value" => 0},
     {"answer" => {"kind" => "yes_no", "probability" => true}}, {"proxy" => nil},
     {"position" => {"file" => "x", "first" => 4}}].each do |change|
      assert_raises(ArgumentError) { C.decode("DecideResult", FIXTURE["results"][0]["result"].merge(change)) }
    end
    [{"origin" => "proxy"}, {"cached" => false}, {"answered_by" => "invented"},
     {"observations" => []}, {"failed_questions" => 1}].each do |change|
      assert_raises(ArgumentError) { C.decode("Meta", FIXTURE["results"][0]["result"]["meta"].merge(change)) }
    end
  end
  def test_named_requests_preserve_payloads_descriptions_files_and_duplicate_images
    record = C.decode("RecordInput", {"records" => [false, nil, {"id" => 1}, {"id" => 1}], "context" => {"context" => []}})
    specs = {
      "decide" => ["DecideSpec", {"decide" => ["Q", {"active" => false}], "false" => nil}],
      "choose" => ["ChooseSpec", {"choose" => "Q", "options" => {"b" => nil, "a" => {"nested" => [false]}}}],
      "tag" => ["TagSpec", {"tag" => "Q", "labels" => ["a"]}],
      "score" => ["ScoreSpec", {"score" => "Q", "levels" => %w[low high]}],
      "filter" => ["DecideSpec", {"decide" => "Q"}],
      "rank" => ["ScoreSpec", {"score" => "Q", "levels" => %w[low high]}],
      "find" => ["FindSpec", {"find" => "Q", "none" => true}],
      "annotate" => ["QuestionSet", {"version" => 1, "questions" => {"a" => {"decide" => "Q", "false" => nil}}}],
      "recognize" => ["RecognitionSpec", {"version" => 1, "recognize" => {"kinds" => {"person" => {"nested" => [false]}}}}],
      "relate" => ["RelationSpec", {"version" => 1, "relate" => {"relations" => [{"name" => "knows", "source" => "*", "target" => "*"}]}}]
    }
    specs.each do |verb, (type, raw)|
      request = C.public_send(verb, C.decode(type, raw), record)
      assert_equal raw, request.question_json
      assert_equal [false, nil, {"id" => 1}, {"id" => 1}], request.input.records
    end
    data = "\x01\x02\x03".b
    image = {"data" => data, "name" => "red.png"}
    images = C.decode("ImageInput", {"images" => [image, {"data" => "b"}, image], "text" => nil})
    request = C.decide(C::DecideSpec.new(decide: "Q"), images)
    data.setbyte(0, 99)
    assert_equal "\x01\x02\x03".b, request.input.images.first.data
    assert_equal request.input.images.first.data, request.input.images.last.data
    assert_nil request.input.text
    %w[tag filter rank find annotate recognize relate].each do |verb|
      type, raw = specs.fetch(verb)
      error = assert_raises(ArgumentError) { C.public_send(verb, C.decode(type, raw), images) }
      assert_equal "this function is text-only", error.message
    end
    named = C.decide(C.decode("QuestionFile", {"path" => "question.json"}), record)
    assert_raises(ArgumentError) { named.question_json }
  end
  def test_input_carriers_keep_the_shared_question_grammar
    corpus = JSON.parse(File.read(File.expand_path("../../../specification/fixtures/question-file/corpus.json", __dir__)))
    corpus.fetch("cases").select { |row| row["valid"] }.each do |row|
      kind = row["verb"] == "relate" ? "RelationSpec" : row["verb"].capitalize + "Spec"
      assert_equal row["file"], C.to_json_value(C.decode(kind, row["file"]))
    end
    [["DecideSpec", {"decide" => false}], ["ChooseSpec", {"choose" => "Q", "options" => {"a" => true}}],
     ["ChooseSpec", {"choose" => "Q", "options" => %w[a b], "threshold" => 0}],
     ["QuestionSet", {"version" => 1, "questions" => {"ready" => {"decide" => "Q", "profile" => "other"}}}]].each do |kind, raw|
      assert_raises(ArgumentError) { C.decode(kind, raw) }
    end
    raw = {"decide" => {}, "on" => "/body", "false" => nil}
    assert_equal raw, C.to_json_value(C.decode("DecideSpec", raw))
  end

end
