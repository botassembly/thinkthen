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

def engine(base, **settings) = T::Client.new(base_url: base, cache: false, **settings)

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

def single(client, verb, asked, text, success, base)
  expected = success["answers"][0]
  result = client.public_send(verb, asked, text)
  document = result.results.first.to_h
  detailed(document, expected, base)
  same("typed", result.value, expected["bare"])
  counters = success["counters"] or return
  Dir.mktmpdir do |folder|
    T::Client.open(base_url: base, cache: folder) do |cached|
      facts = Array.new(counters["calls"]) { cached.public_send(verb, asked, text).facts }
      same("counters", { "calls" => facts.size, "requests" => facts.sum(&:requests_sent),
                         "cache_answers" => facts.sum(&:cache_answers) }, counters)
    end
  end
end

def annotated(client, set, texts, success, one)
  result = client.annotate(set, texts)
  records = result.results.map(&:to_h)
  failed = 0
  success["answers"].each do |expected|
    answer = records[one ? 0 : expected["exchange"]].fetch("answers").fetch(expected["name"])
    failure = answer["failure"]
    value = answer["value"]
    failed += 1 if failure
    same("bare #{expected['name']}", failure ? { "failed" => failure } : value, expected["bare"])
    same("unresolved #{expected['name']}", !failure && value.nil?, expected["bare"].nil?)
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
    [digest(CANONICAL, request), QuestionKeys.of("#{base}/systemone", request, exchange.fetch("response").fetch("model"))]
  end
  success = swap(one["expect"]["success"], renamed)
  texts = exchanges.map { |exchange| exchange["evidence"] }
  settings = LEGACY_BATCH_ONE.include?(id) ? {batch: 1} : {}
  client = T::Client.new(base_url: base, cache: false, **settings)
  held = one["question"]
  case [one["verb"], success["kind"]]
  in ["recognize", _]
    result = client.recognize(held, one.fetch("text"))
    raise "untyped recognition" unless result.results.first.is_a?(T::Results::NativeRecognition)
    bare = result.value.to_h
    same("result", bare, success["answers"][0]["bare"])
  in ["relate", _]
    edges = client.relate(held, one["entities"]).value
    same("result", edges.map(&:to_h), success["answers"][0]["bare"])
  in ["annotate", _]
    whole = one.key?("record")
    annotated(client, one["question_set"], whole ? [one["record"]] : texts, success, whole)
  in ["find", _]
    result = client.find(held["find"], held["units"], none: held["none"])
    found = result.results.first
    selected = success["operation"]["selected"]
    picked = success["operation"]["probabilities"].find { |row| row["index"] == selected }
    want = selected.nil? ? nil : [selected, held["units"][selected], picked["probability"]]
    candidate = found.candidates.find { |row| row.index == found.index }
    same("found", found.index.nil? ? nil : [found.index, found.value, candidate.probability], want)
    same("selected value", result.value, want&.[](1))
  in ["rank", _]
    ranked = client.rank(held, texts).results
    same("ranking", ranked.map { |row| { "index" => row.index, "probability" => row.answer.probability } }, success["operation"]["ranking"])
  in [_, "filter"]
    kept = client.filter(held, texts).results
    same("indexes", kept.map(&:index), success["operation"]["indexes"])
  in [_, "decide_many"]
    same("bare", client.decide(held, texts).value, success["answers"].map { |answer| answer["bare"] })
  else
    single(client, one["verb"], held, texts[0], success, base)
    if id == "01-decide-yes-captured"
      Dir.mktmpdir do |folder|
        file = File.join(folder, "question.json")
        File.write(file, JSON.generate(held))
        before = client.usage[:requests_sent]
        named = client.decide(T::Client.question_file(file), texts[0])
        detailed(named.results.first.to_h, success["answers"][0], base)
        same("named-file sends", client.usage[:requests_sent] - before, 1)
      end
    end
  end
ensure
  client&.close
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
        T::Client.open(base_url: generic, cache: file) { |client| client.decide(text, "Is this urgent?") }
      end
    when "23-cancelled-fault"
      token = T::Cancel.new
      token.cancel
      counted.decide(text, "Is this urgent?", cancel: token)
    when "24-deadline-fault" then counted.decide(text, "Is this urgent?", deadline_ms: 0)
    when "29-usage-json-text" then counted.decide(one["question"], "Is this urgent?")
    when "30-local-question-file"
      Dir.mktmpdir do |folder|
        file = File.join(folder, "question.json")
        File.write(file, JSON.generate(one["question"]))
        counted.decide(T::Client.question_file(file), one["evidence"])
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
ensure
  counted&.close
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
