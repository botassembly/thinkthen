# frozen_string_literal: true

# The Ruby surface's slice of the conformance file, offline against the
# null backend. One line a case: ok, skip with a reason, or FAIL. The exit
# is nonzero after any FAIL. Ported from the Rust surface's runner, case
# for case.
#
# Run with: ruby -I lib tests/conformance.rb  (ENGINE_NULL=1)

require "json"
require "thinkthen"

PATH = File.expand_path("../../../conformance/conformance.json", __dir__)

def wire_set?
  ENV.key?("ENGINE_BASE_URL") || ENV.key?("THINKTHEN_BASE_URL")
end

def built(text)
  ThinkThen._parse_question(text)
rescue ThinkThen::UsageError => e
  raise "the call failed: #{e.kind} (#{e.message})"
end

def ok_if(condition, what)
  raise "FAIL: #{what}" unless condition
end

def expect_answer(value)
  return true if value == true
  return false if value == false

  nil
end

def raise_kind
  yield
  nil
rescue ThinkThen::UsageError, ThinkThen::BackendError, ThinkThen::DeadlineError,
       ThinkThen::LocalError, ThinkThen::CancelledError, ThinkThen::DefectError => e
  e.kind
end

def check_error(verb, question_text, evidence, records, kind)
  begin
    question = ThinkThen._parse_question(question_text)
  rescue ThinkThen::UsageError => e
    return if e.kind == kind

    raise "FAIL: expected the #{kind} kind, got #{e.kind}"
  end
  if kind == "deadline"
    error = raise_kind { ThinkThen.decide(question, evidence, deadline: 0.0) }
    return ok_if(error == kind, "expected the #{kind} kind, got #{error || 'an answer'}")
  end
  error = case verb
          when "filter" then raise_kind { ThinkThen.filter(question, records) }
          when "decide" then raise_kind { ThinkThen.decide(question, evidence) }
          when "choose" then raise_kind { ThinkThen.choose(question, evidence) }
          when "decide_many" then raise_kind { ThinkThen.decide_many(question, records) }
          when "rank" then raise_kind { ThinkThen.rank(question, records) }
          else nil
          end
  ok_if(error == kind, "expected the #{kind} kind, got #{error || 'an answer'}")
end

def check_details(expect, details)
  # The audit's identity fields and the two 0053/0054 additions; the
  # recorded probability is not compared because the null backend's own
  # rule cannot reproduce case 73's recorded number.
  ok_if(details["model"] == expect.dig("details", "model"),
        "expected model #{expect.dig('details', 'model').inspect}, got #{details['model'].inspect}")
  ok_if(details["digest"] == expect.dig("details", "question_sha256"), "the digest diverged")
  wanted_requests = expect.dig("details", "requests")
  ok_if(details["requests"] == wanted_requests, "the requests list diverged") if wanted_requests
  wanted_failed = expect.dig("details", "failed_questions")
  ok_if(details["failed_questions"] == wanted_failed, "failed_questions diverged") unless wanted_failed.nil?
end

def run_case(verb, question_text, evidence, records, expect, set_json, text = nil, form = nil)
  if (error = expect["error"])
    kind = error["kind"]
    raise "SKIP: the stand-in ignores a pre-fired token; conformance/DIVERGENCES.md carries this as a real-engine requirement" if verb == "cancel"
    raise "SKIP: the backend kind needs the wire or a dead address; the null backend answers" if kind == "backend" && !wire_set?

    return check_error(verb, question_text, evidence, records, kind)
  end

  case verb
  when "decide"
    question = built(question_text)
    details = ThinkThen.details(question, evidence)
    check_details(expect, details)
    ok_if(details["answer"] == expect_answer(expect["answer"]),
          "expected #{expect['answer'].inspect}, got #{details['answer'].inspect}")
  when "decide_many"
    question = built(question_text)
    judgments = ThinkThen.decide_many_with_probabilities(question, records)
    answers = judgments.map { |one| one[:answer] }
    ok_if(answers == expect["answers"].map { |value| expect_answer(value) },
          "expected #{expect['answers'].inspect}, got #{answers.inspect}")
    if expect["probabilities"]
      judgments.zip(expect["probabilities"]).each do |one, wanted|
        ok_if((one[:probability] - wanted).abs < 1e-9,
              "expected probability #{wanted}, got #{one[:probability]}")
      end
    end
    if expect["rows"]
      # The ruled record row (go-ahead item 4): this host's own pair, the
      # record and the value it carries, in input order.
      rows = records.zip(answers).map { |record, value| { "input" => record, "value" => value } }
      ok_if(rows == expect["rows"], "expected rows #{expect['rows'].inspect}, got #{rows.inspect}")
    end
  when "filter"
    question = built(question_text)
    kept = ThinkThen.filter(question, records)
    wanted = expect["indexes"].map { |index| records[index] }
    ok_if(kept == wanted, "expected #{wanted.inspect}, got #{kept.inspect}")
    if expect["rows"]
      rows = kept.map { |record| { "input" => record, "value" => true } }
      ok_if(rows == expect["rows"], "expected rows #{expect['rows'].inspect}, got #{rows.inspect}")
    end
  when "choose"
    question = built(question_text)
    picked = ThinkThen.choose(question, evidence)
    wanted = expect["answer"]
    ok_if(wanted.nil? ? picked.nil? : picked == wanted,
          "expected #{wanted.inspect}, got #{picked.inspect}")
  when "score"
    question = built(question_text)
    value, level = ThinkThen.score_with_level(question, evidence)
    ok_if((value - expect["answer"]).abs < 1e-9,
          "expected #{expect['answer'].inspect}, got #{value}")
    ok_if(level == expect.dig("details", "nearest_level"),
          "expected level #{expect.dig('details', 'nearest_level').inspect}, got #{level.inspect}")
  when "tag"
    question = built(question_text)
    labels = ThinkThen.tag(question, evidence)
    ok_if(labels == expect["answer"], "expected #{expect['answer'].inspect}, got #{labels.inspect}")
  when "annotate"
    set = ThinkThen._parse_set(JSON.generate({ "questions" => set_json }))
    held = records.empty? ? [evidence] : records
    answers = ThinkThen.annotate(set, held)
    first = answers.first or raise "FAIL: no annotated record came back"
    expect["answers"].each do |name, wanted|
      field = first.key?(name.to_sym) ? first[name.to_sym] : (raise "FAIL: no #{name} field in the answer")
      if wanted.key?("failed")
        # The ruled marker (0054), in this host's own spelling.
        ok_if(field == { "failed" => wanted["failed"] },
              "#{name}: expected the marker #{wanted['failed'].inspect}, got #{field.inspect}")
        next
      end
      if wanted["answer"].is_a?(Array) || wanted["answer"].is_a?(Float)
        raise "SKIP: #{name} holds a field the runner does not check"
      end

      ok_if(field == wanted["answer"],
            "#{name}: expected #{wanted['answer'].inspect}, got #{field.inspect}")
    end
    wanted_failed = expect["failed_questions"]
    if wanted_failed
      counted = first.count { |_name, field| field.is_a?(Hash) && field.key?("failed") }
      ok_if(counted == wanted_failed,
            "expected #{wanted_failed} failed fields, got #{counted}")
    end
  when "details"
    question = built(question_text)
    check_details(expect, ThinkThen.details(question, evidence))
  when "usage"
    raise "SKIP: the cache half needs the disk cache, which the stand-in does not carry"
  when "recognize"
    spec = JSON.parse(question_text)
    rules = (spec["relations"] || []).to_h do |rule|
      [rule["name"], { source: rule["source"], target: rule["target"], either: rule["either"] }]
    end
    found = ThinkThen.recognize(text.to_s, kinds: spec["kinds"],
                                relations: rules.empty? ? nil : rules,
                                threshold: spec["threshold"],
                                relation_threshold: spec["relation_threshold"])
    wanted_entities = expect["entities"]
    ok_if(found.entities.length == wanted_entities.length,
          "expected #{wanted_entities.length} names, got #{found.entities.length}")
    found.entities.zip(wanted_entities).each do |got, wanted|
      ok_if([got.id, got.text, got.kind, got.start, got.end] ==
              [wanted["id"], wanted["text"], wanted["kind"], wanted["start"], wanted["end"]],
            "the name #{wanted['text'].inspect} diverged: " \
            "#{[got.id, got.text, got.kind, got.start, got.end].inspect}")
      ok_if((got.strength - wanted["strength"]).abs < 1e-9,
            "strength diverged for #{wanted['text'].inspect}: #{got.strength}")
    end
    wanted_relations = expect["relations"]
    ok_if(found.relations.length == wanted_relations.length,
          "expected #{wanted_relations.length} relations, got #{found.relations.length}")
    found.relations.zip(wanted_relations).each do |got, wanted|
      ok_if([got.name, got.source, got.target] == [wanted["name"], wanted["source"], wanted["target"]] &&
              (got.probability - wanted["probability"]).abs < 1e-9,
            "the relation #{wanted['name']} diverged: " \
            "#{[got.name, got.source, got.target, got.probability].inspect}")
    end
  when "relate"
    if form == "per-subject"
      raise "SKIP: the per-subject arm is engine-internal; the stand-in serves the ruled pairs " \
            "form (conformance/DIVERGENCES.md)"
    end
    spec = JSON.parse(question_text)
    rules = (spec["relations"] || []).map do |rule|
      rule.is_a?(Hash) ? { rule["name"] => [rule["source"], rule["target"]] } : rule
    end
    edges = ThinkThen.relate(records, relations: rules.empty? ? nil : rules, either: spec["either"],
                             threshold: spec["threshold"], kind_field: spec["kind_field"])
    wanted_edges = expect["edges"]
    ok_if(edges.length == wanted_edges.length,
          "expected #{wanted_edges.length} edges, got #{edges.length}")
    edges.zip(wanted_edges).each do |got, wanted|
      ok_if([got.name, got.source, got.target] == [wanted["name"], wanted["source"], wanted["target"]] &&
              (got.probability - wanted["probability"]).abs < 1e-9,
            "edge #{[wanted['name'], wanted['source'], wanted['target']].inspect} diverged: " \
            "#{[got.name, got.source, got.target, got.probability].inspect}")
      %w[source_kind target_kind].each do |key|
        ok_if(got[key.to_sym] == wanted[key], "#{key} diverged on #{wanted['name']}") if wanted.key?(key)
      end
    end
  else
    raise "SKIP: no case shape for #{verb}"
  end
end

file = JSON.parse(File.read(PATH))
failed = 0
file["cases"].each do |one|
  id = one["id"]
  verb = one["verb"]
  if one["question_file"]
    puts "skip     #{id}: the local kind needs a file door this surface does not carry"
    next
  end
  question_text = JSON.generate(one["question"])
  evidence = one["evidence"].to_s
  records = one["records"].to_a
  begin
    run_case(verb, question_text, evidence, records, one["expect"], one["set"], one["text"], one["form"])
    puts "ok       #{id}"
  rescue RuntimeError => e
    message = e.message.sub(/\AFAIL: /, "")
    if message.start_with?("SKIP: ")
      puts "skip     #{id}: #{message.sub('SKIP: ', '')}"
    elsif message.start_with?("DIVERGE")
      puts "diverge  #{id}: #{message}"
    else
      failed += 1
      puts "FAIL     #{id}: #{message}"
    end
  end
end

if failed.positive?
  warn "#{failed} conformance case(s) failed"
  exit 1
end
puts "conformance slice green for the Ruby surface"
