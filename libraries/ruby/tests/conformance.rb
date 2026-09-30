# frozen_string_literal: true

# Every applicable shared case through the Ruby surface, on the 0092
# conformance backend. The parent starts the backend and runs this file
# again as a scrubbed child with a fake key. The child runs each success
# case on its own case arm. The expected request digests were recorded
# against the canonical address, so each is recomputed for the address the
# backend served. A record function's row lists question keys by ADR 0111,
# so its digests become the keys of the request each digest named. It prints one line a case and a count line, and exits
# nonzero after any failure or a count that does not add up.
#
# Two cases do not run here:
NOT_RUN = {
  "18-annotate-two-groups" => "its recording holds one request per group, and ADR 0111 section 5 packs a record's groups into one",
  "25-defect-fault" => "injects an internal invariant failure that no outside boundary reaches; src/lib.rs tests the guard"
}.freeze

LEGACY_BATCH_ONE = %w[13-filter-records 14-filter-none 15-rank-records 16-rank-stable-tie
                      27-decide-many 28-decide-many-repeated-texts 34-annotate-repeated-texts
                      35-annotate-score-repeated-texts].freeze

require "digest"
require "json"
require "pathname"
require "tmpdir"
require_relative "keys"

CASES = JSON.parse(File.read(File.expand_path("../../../conformance/cases.json", __dir__)))
CANONICAL = "https://api.typesafe.ai/v1/systemone"
ALL = CASES.fetch("cases")
IDS = ALL.map { |one| one.fetch("id") }
raise "shared case count differs from the document" unless ALL.size == CASES.fetch("case_count")
raise "duplicate shared case ID" unless IDS.uniq.size == IDS.size

selector = ENV["THINKTHEN_CONFORMANCE_IDS"]
if selector
  raise "THINKTHEN_CONFORMANCE_IDS takes an absolute path" unless Pathname.new(selector).absolute?

  chosen = File.readlines(selector, chomp: true).map(&:strip).reject { |id| id.empty? || id.start_with?("#") }
  raise "the selected case list is empty" if chosen.empty?
  raise "duplicate selected case ID" unless chosen.uniq.size == chosen.size

  absent = chosen - IDS
  raise "selected case #{absent.first} is absent from the shared corpus" unless absent.empty?
  SELECTED = chosen.freeze
else
  SELECTED = IDS.freeze
end

unless ARGV.first == "--child"
  require_relative "backend"
  backend = TestBackend::Backend.new
  status = Dir.mktmpdir do |root|
    extra = { "CONFORMANCE_PORT" => backend.port.to_s }
    extra["THINKTHEN_CONFORMANCE_IDS"] = selector if selector
    env = TestBackend.env(backend.url, root, extra)
    system(env, RbConfig.ruby, "-I", TestBackend::LIB, __FILE__, "--child", unsetenv_others: true)
  end
  backend.close
  exit(status ? 0 : 1)
end

require "thinkthen"
T = ThinkThen
ORIGIN = "http://127.0.0.1:#{ENV.fetch('CONFORMANCE_PORT')}".freeze

def digest(url, request) = Digest::SHA256.hexdigest("systemone\n#{url}\n#{request}")

def swap(value, renamed)
  case value
  when String then renamed.fetch(value, value)
  when Array then value.map { |item| swap(item, renamed) }
  when Hash then value.transform_values { |item| swap(item, renamed) }
  else value
  end
end

# Equal, holding numbers to a rounding tolerance.
def close?(one, other)
  case [one, other]
  in [Numeric, Numeric] then (one - other).abs < 1e-9
  in [Array, Array] then one.size == other.size && one.zip(other).all? { |a, b| close?(a, b) }
  in [Hash, Hash] then one.size == other.size && one.all? { |name, held| other.key?(name) && close?(held, other[name]) }
  else one == other
  end
end

def same(what, actual, expected)
  raise "#{what}: got #{JSON.generate(actual)}, expected #{JSON.generate(expected)}" unless close?(actual, expected)
end

def engine(base, **settings) = T::Engine.new(base_url: base, cache: false, **settings)

def question(held) = T.question(**held.transform_keys(&:to_sym))

def entity(one) = %w[text start end length kind strength].to_h { |key| [key, one[key]] }

def detailed(document, expected, base)
  wanted = expected["details"]
  answer = wanted["answer"]
  %w[probability probabilities level].each { |name| same(name, document["answer"][name], answer[name]) if answer.key?(name) }
  same("confidence", document["answer"].fetch("confidence", "absent"), answer.fetch("confidence", "absent"))
  %w[model question_sha256 usage requests_sent cached].each do |name|
    same(name, document["meta"].fetch(name, "absent"), wanted.fetch(name, "absent"))
  end
  # A keyed request stands for the list of its keys, so a row reads the flattened list.
  requests = wanted.key?("requests") ? wanted["requests"].flat_map { |held| Array(held) } : "absent"
  same("requests", document["meta"].fetch("requests", "absent"), requests)
  same("url", document["meta"]["url"], "#{base}/systemone")
end

def single(engine, asked, text, success, base)
  expected = success["answers"][0]
  document = engine.details(asked, text).value
  detailed(document, expected, base)
  typed = case document["answer"]["kind"]
          when "yes_no" then engine.decide(asked, text).value
          when "score" then engine.score(asked, text).value
          when "choice" then engine.choose(asked, text).value
          when "tag" then engine.tag(asked, text).value
          end
  same("typed", typed, expected["bare"])
  counters = success["counters"] or return
  Dir.mktmpdir do |folder|
    cached = T::Engine.new(base_url: base, cache: folder)
    before = cached.usage
    counters["calls"].times { cached.details(asked, text) }
    after = cached.usage
    same("counters", { "calls" => counters["calls"], "requests" => after[:requests_sent] - before[:requests_sent],
                       "cache_answers" => after[:cache_answers] - before[:cache_answers] }, counters)
  end
end

def annotated(engine, set, texts, success, one)
  records = Dir.mktmpdir do |folder|
    File.write(File.join(folder, "set.json"), JSON.generate(set))
    engine.annotate(T.set(File.join(folder, "set.json")), texts).value
  end
  failed = 0
  success["answers"].each do |expected|
    value = records[one ? 0 : expected["exchange"]].fetch(expected["name"].to_sym)
    failed += 1 if value.is_a?(Hash) && value.key?("failed")
    same("bare #{expected['name']}", value, expected["bare"])
  end
  same("failed", failed, success.fetch("failed_questions", 0))
end

def check(one)
  id = one["id"]
  return refused(one, one["expect"]["error"]["kind"]) if one["expect"].key?("error")

  base = "#{ORIGIN}/case/#{id}/v1"
  exchanges = one["exchanges"]
  # Every row lists question keys, by ADR 0111.
  renamed = exchanges.to_h do |exchange|
    request = exchange["request"]
    [digest(CANONICAL, request), QuestionKeys.of("#{base}/systemone", request)]
  end
  success = swap(one["expect"]["success"], renamed)
  texts = exchanges.map { |exchange| exchange["evidence"] }
  engine = engine(base, batch: LEGACY_BATCH_ONE.include?(id) ? 1 : nil)
  held = one["question"]
  case [one["verb"], success["kind"]]
  in ["recognize", _]
    rules = held["recognize"]
    found = engine.recognize(one["text"], kinds: rules["kinds"], relations: rules["relations"],
                                          threshold: held["threshold"], relation_threshold: held["relation_threshold"]).value
    bare = { "entities" => found.entities.map { |e| entity(e) } }
    unless found.relations.nil?
      bare["relations"] = found.relations.map do |r|
        { "relation" => r.relation, "source" => entity(r.source), "target" => entity(r.target), "probability" => r.probability }
      end
    end
    same("result", bare, success["answers"][0]["bare"])
  in ["relate", _]
    pair = ->(e) { { "name" => e.name, "kind" => e.kind } }
    edges = engine.relate(one["entities"], relations: held["relate"]["relations"], threshold: held["threshold"]).value
    same("result", edges.map { |e| { "relation" => e.relation, "source" => pair.(e.source), "target" => pair.(e.target), "probability" => e.probability } },
         success["answers"][0]["bare"])
  in ["annotate", _]
    whole = one.key?("record")
    annotated(engine, one["question_set"], whole ? [JSON.generate(one["record"])] : texts, success, whole)
  in ["find", _]
    found = engine.find(held["find"], held["units"], none: held["none"]).value
    selected = success["operation"]["selected"]
    picked = success["operation"]["probabilities"].find { |row| row["index"] == selected }
    want = selected.nil? ? nil : [selected, held["units"][selected], picked["probability"]]
    same("found", found.index.nil? ? nil : found.to_a, want)
  in ["rank", _]
    ranked = engine.rank(held["decide"], texts).value
    same("ranking", ranked.map { |row| { "index" => row.index, "probability" => row.probability } }, success["operation"]["ranking"])
  in [_, "filter"]
    kept = engine.filter(question(held), texts).value
    same("indexes", kept.map { |text| texts.index { |held_text| held_text.equal?(text) } }, success["operation"]["indexes"])
  in [_, "decide_many"]
    same("bare", engine.decide_many(question(held), texts).value, success["answers"].map { |answer| answer["bare"] })
  else
    single(engine, question(held), texts[0], success, base)
    if id == "01-decide-yes-captured"
      Dir.mktmpdir do |folder|
        file = File.join(folder, "question.json")
        File.write(file, JSON.generate(held))
        before = engine.usage[:requests_sent]
        detailed(engine.details(T.question(file: file), texts[0]).value, success["answers"][0], base)
        same("named-file sends", engine.usage[:requests_sent] - before, 1)
      end
    end
  end
end

# Each error case at its public boundary: the kind, not retryable, a usage
# message that names something, and one send only for the refusing arm.
def refused(one, kind)
  generic = "#{ORIGIN}/generic/v1"
  text = one["question"]["decide"]
  counted = engine(generic)
  before = counted.usage[:requests_sent]
  begin
    case one["id"]
    when "20-usage-fault" then counted.decide(text, "   ")
    when "21-backend-fault" then (counted = engine("#{ORIGIN}/arm/refuse/v1")).decide(text, "Is this urgent?")
    when "22-local-fault"
      Dir.mktmpdir do |folder|
        file = File.join(folder, "not-a-folder")
        File.write(file, "not a folder")
        T::Engine.new(base_url: generic, cache: file).decide(text, "Is this urgent?")
      end
    when "23-cancelled-fault"
      token = T::Cancel.new
      token.cancel
      counted.decide(text, "Is this urgent?", cancel: token)
    when "24-deadline-fault" then counted.decide(text, "Is this urgent?", deadline: 0)
    when "29-usage-json-text" then T.question(decide: text, threshold: one["question"]["threshold"])
    when "30-local-question-file"
      Dir.mktmpdir do |folder|
        file = File.join(folder, "question.json")
        File.write(file, JSON.generate(one["question"]))
        counted.decide(T.question(file: file), one["evidence"])
      end
    when "31-usage-rank-blank-question" then counted.rank(text, %w[one two])
    else raise "no public boundary is written for #{one['id']}"
    end
  rescue T::Error => e
    same("kind", e.kind, kind)
    same("retryable", e.retryable, false)
    raise "a usage error names nothing" if kind == "usage" && e.message.strip.empty?

    sent = counted.usage[:requests_sent] - before
    same("sent", sent, one["id"] == "21-backend-fault" ? 1 : 0)
    return
  end
  raise "the case succeeded"
end

passed = failed = skipped = 0
CASES["cases"].each do |one|
  id = one["id"]
  next unless SELECTED.include?(id)
  if (why = NOT_RUN[id])
    skipped += 1
    puts "not run #{id}: #{why}"
    next
  end
  begin
    check(one)
    passed += 1
    puts "ok #{id}"
  rescue StandardError => e
    failed += 1
    puts "FAIL #{id}: #{e.class}: #{e.message}"
  end
end
total = passed + failed + skipped
puts "conformance: total=#{CASES['case_count']} selected=#{SELECTED.size} pass=#{passed} fail=#{failed} not_run=#{skipped} unselected=#{CASES['case_count'] - SELECTED.size}"
exit(failed.zero? && total == SELECTED.size ? 0 : 1)
