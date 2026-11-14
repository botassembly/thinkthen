# frozen_string_literal: true

require "open3"
require "tmpdir"
require "timeout"

module Checks
  def assert(value, message = "assertion failed")
    raise message unless value
  end

  def equal(expected, actual, name)
    assert(expected == actual, "#{name}: expected #{expected.inspect}, got #{actual.inspect}")
  end

  def raises(type)
    begin
      yield
    rescue type => error
      return error
    end
    raise "expected #{type}"
  end
end
include Checks

# Feature: Ruby callers do not inherit credentials or personal configuration.
def isolated_ruby_environment
  original = ENV.to_h
  Dir.mktmpdir("thinkthen-ruby-") do |home|
    keep = %w[PATH SystemRoot SYSTEMROOT WINDIR COMSPEC PATHEXT TEMP TMP TMPDIR
              GEM_HOME GEM_PATH RUBYLIB LLVM_PROFILE_FILE THINKTHEN_LOOPBACK THINKTHEN_CASES
              THINKTHEN_STUB_CASES THINKTHEN_RUBY_COVERAGE THINKTHEN_NATIVE_FIXTURES THINKTHEN_TEST_ASSETS]
    ENV.replace(original.select { |key, _| keep.include?(key) })
    ENV.delete("THINKTHEN_API_KEY")
    ENV["HOME"] = ENV["USERPROFILE"] = ENV["XDG_CONFIG_HOME"] = ENV["XDG_CACHE_HOME"] = home
    yield
  ensure
    ENV.replace(original)
  end
end

class Backend
  attr_reader :url

  def initialize(*arguments)
    environment = ENV.to_h.select { |name, _| %w[PATH HOME TMPDIR SystemRoot LLVM_PROFILE_FILE].include?(name) }
    @input, @output, @wait = Open3.popen2(environment, ENV.fetch("THINKTHEN_LOOPBACK"),
                                       *arguments, unsetenv_others: true)
    @url = "http://127.0.0.1:#{Timeout.timeout(30) { @output.readline.strip }}/v1"
  end

  def command(value)
    @input.puts(value)
    @input.flush
    Timeout.timeout(30) { @output.readline.strip }
  end

  def close
    @input.puts("shutdown")
    @input.close
    raise "loopback failed" unless Timeout.timeout(30) { @wait.value }.success?
    @output.close
  end

  def self.open(*arguments)
    backend = new(*arguments)
    begin
      yield backend
    ensure
      backend.close
    end
  end
end

def ruby_options(question)
  question.reject { |key, _| %w[function question questions].include?(key) }.to_h do |key, value|
    translated = case key
                 when "threshold"
                   value.is_a?(Hash) ? value.fetch("low")..value.fetch("high") : value
                 when "rules"
                   value.map do |rule|
                     translated = rule.to_h { |key, entry| [key.to_sym, entry] }
                     ThinkThen::RelationRule.new(**translated)
                   end
                 else value
                 end
    [key.to_sym, translated]
  end
end

def typed_question(question)
  name = question.fetch("function")
  klass = ThinkThen.const_get("#{name.capitalize}Question")
  options = ruby_options(question)
  return klass.new(**options) if %w[recognize relate].include?(name)
  klass.new(question.fetch("question"), **options)
end

def invoke_rows(client, request, cancel: nil)
  question = request.fetch("question")
  function = question.fetch("function")
  records = request.fetch("records", [])
  options = ruby_options(question)
  options[:name] = request["name"] if request.key?("name")
  options[:cancel] = cancel unless cancel.nil?
  if function == "annotate"
    questions = question.fetch("questions").transform_values { |entry| typed_question(entry) }
    client.annotate_each(ThinkThen::AnnotationQuestions.new(**questions), records, **options)
  elsif %w[recognize relate].include?(function)
    client.public_send("#{function}_each", records, **options)
  else
    client.public_send(function == "rank" ? :rank : "#{function}_each", question.fetch("question"), records, **options)
  end
end

# Feature: every function has backend-independent typed Ruby answers and detail.
def assert_typed_row(row, expected)
  function = expected.fetch("function")
  equal(function.to_sym, row.function, "typed function")
  equal(expected.fetch("status").to_sym, row.status, "typed status")
  klass = function == "filter" ? ThinkThen::Result : ThinkThen.const_get("#{function.capitalize}Row")
  assert(row.is_a?(klass), "function result class")
  equal([row.status == :answered, row.status == :unsure, row.status == :failed],
        [row.answered?, row.unsure?, row.failed?], "status predicates")
  assert(row.frozen?, "immutable row")
  assert(row.source.is_a?(ThinkThen::Source), "typed source")
  assert(!row.respond_to?(:p), "no old probability spelling")
  unless row.answered?
    raises(row.unsure? ? ThinkThen::UnsureError : ThinkThen::FailedError) { row.answer }
    assert(!row.to_h.key?("answer"), "non-answer wire shape omits answer")
  end
  if row.failed?
    assert(row.error.is_a?(ThinkThen::RowError), "typed row error")
    equal(expected.fetch("error").fetch("code").to_sym, row.error.code, "typed error code")
    assert(row.probability.nil?, "failed result has no probability")
    return
  end
  return unless row.answered?

  answer = row.answer
  expected_type = {"choose" => String, "tag" => Array, "score" => Float, "rank" => Integer,
                   "annotate" => Hash, "recognize" => Array, "relate" => Array}
  assert(answer.is_a?(expected_type.fetch(function)), "function answer type") if expected_type.key?(function)
  assert([true, false].include?(answer), "boolean answer") if %w[decide filter].include?(function)
  assert(answer.nil? || answer.is_a?(ThinkThen::Span), "typed found answer") if function == "find"
  if function == "recognize"
    answer.each { |name| assert(name.is_a?(ThinkThen::Name), "typed recognized name") }
  elsif function == "relate"
    answer.each do |relation|
      assert(relation.is_a?(ThinkThen::Relation), "typed relation")
      assert(relation.source.is_a?(ThinkThen::RelationName), "typed relation endpoint")
    end
  elsif function == "annotate"
    answer.each_value do |entry|
      assert(entry.is_a?(ThinkThen::Result), "typed annotation entry")
      equal(row.position, entry.position, "entry keeps parent position")
      assert(entry.item.equal?(row.item), "entry keeps original record")
      assert(entry.error.is_a?(ThinkThen::RowError), "typed child failure") if entry.failed?
    end
  end
  detail_type = {"choose" => ThinkThen::LabelDetail, "tag" => ThinkThen::LabelDetail,
                 "score" => ThinkThen::LevelDetail, "rank" => ThinkThen::LevelDetail,
                 "recognize" => ThinkThen::RecognizeDetail, "relate" => ThinkThen::RelationDetail}
  assert(row.detail.is_a?(detail_type.fetch(function)), "function detail type") if expected["detail"]
end

def execute_case(test, settings)
  output = []
  config = settings.transform_keys(&:to_sym).merge(backend: settings.fetch("backend").to_sym)
  if config[:retry]
    policy = config[:retry].transform_keys(&:to_sym)
    policy[:initial_backoff] = policy.delete(:initial_backoff_ms) / 1000.0 if policy.key?(:initial_backoff_ms)
    policy[:attempt_timeout] = policy.delete(:attempt_timeout_ms) / 1000.0 if policy.key?(:attempt_timeout_ms)
    config[:retry] = ThinkThen::RetryPolicy.new(**policy)
  end
  ThinkThen::Client.open(**config) do |client|
    cancel = ThinkThen::Cancel.new
    cancel.cancel if test["cancelled"]
    if test.fetch("request").fetch("question").fetch("function") == "filter"
      request = test.fetch("request")
      definition = request.fetch("question")
      kept, failures = [], []
      begin
        options = ruby_options(definition)
        options[:name] = request["name"] if request.key?("name")
        client.filter(definition.fetch("question"), request.fetch("records", []), **options, cancel: cancel) do |item|
          assert(request.fetch("records").any? { |record| record.equal?(item) }, "filter keeps caller object identity")
          kept << item
        end
      rescue ThinkThen::FailedError => error
        failures = error.rows
        failures.each { |row| assert(row.item.equal?(request.fetch("records")[row.position - 1]), "failed filter keeps original object") }
      end
      return {"kept" => kept, "failures" => failures.map(&:to_h)}
    end
    stream = invoke_rows(client, test.fetch("request"), cancel: cancel)
    begin
      stream.each do |value|
        row = value
        expected = test.fetch("expected", test.fetch("expected_rows", []))[output.length]
        assert_typed_row(row, expected) if expected
        output << row.to_h
      end
    ensure
      stream.close
    end
  end
  output
rescue ThinkThen::Error => error
  equal(test.fetch("expected_rows"), output, "rows before call failure") if test.key?("expected_rows")
  {"error" => error.code.to_s, "http_status" => error.http_status}.compact
end

# Filter exposes original kept records and retained failed results, never its raw decisions.
def expected_projection(test)
  expected = test.fetch("expected")
  return expected unless expected.is_a?(Array) && test.fetch("request").fetch("question").fetch("function") == "filter"
  records = test.fetch("request").fetch("records", [])
  {"kept" => expected.select { |row| row["status"] == "answered" && row["answer"] == true }.map { |row| records[row.fetch("position") - 1] },
   "failures" => expected.select { |row| row["status"] == "failed" }}
end

def normalized_contract(value)
  case value
  when Array then value.map { |item| normalized_contract(item) }
  when Hash
    value.each_with_object({}) do |(key, item), result|
      next if key == "message" && value["code"].is_a?(String)
      if key == "source" && item.is_a?(Hash) && item.key?("question_identity")
        item = item.reject { |name, _| %w[backend model question_identity].include?(name) }
        next if item.empty?
      end
      result[key] = normalized_contract(item)
    end
  else value
  end
end

def strict_row_contract(expected, actual, backend, model)
  source = actual.fetch("source")
  equal(backend, source.fetch("backend"), "source backend")
  equal(model, source.fetch("model"), "source model")
  assert(source.fetch("question_identity").is_a?(String), "source question identity")
  strict_contract(expected, normalized_contract(actual))
end

def strict_contract(expected, actual)
  equal(expected.class, actual.class, "exact JSON type")
  case expected
  when Hash
    equal(expected.keys.sort, actual.keys.sort, "exact JSON fields")
    expected.each { |key, value| strict_contract(value, actual.fetch(key)) }
  when Array
    equal(expected.length, actual.length, "exact JSON array length")
    expected.zip(actual).each { |want, got| strict_contract(want, got) }
  else equal(expected, actual, "exact JSON value")
  end
end

# Feature: installed Ruby calls consume canonical cases for both wire formats.
def shared_cases
  folder = File.join(ENV.fetch("THINKTHEN_CASES"), "typed")
  manifest = JSON.parse(File.read(File.join(folder, "manifest.json"), encoding: Encoding::UTF_8))
  registry = JSON.parse(File.read(File.expand_path(manifest.fetch("registry"), folder), encoding: Encoding::UTF_8))
  equal(registry.fetch("codes").to_h { |code| [code.fetch("value"), code.fetch("name")] },
        ThinkThen.const_get(:NativeContract)::CODE_NAMES, "generated code mapping")
  registry.fetch("codes").each do |code|
    equal(code.fetch("family"), ThinkThen.send(:native).thinkthen_code_family(code.fetch("value")), "ABI error family")
    expected_class = {"usage" => ThinkThen::UsageError, "backend" => ThinkThen::SetupError,
                      "internal" => ThinkThen::InternalError, "record" => ThinkThen::FailedError,
                      "local" => ThinkThen::SetupError}.fetch(code.fetch("family"))
    expected_class = ThinkThen::CancelledError if code.fetch("name") == "local.cancelled"
    expected_class = ThinkThen::SetupError if code.fetch("name") == "usage.key-missing"
    equal(expected_class, ThinkThen.send(:native).send(:error_class, code.fetch("value"), code.fetch("name")), "public error family class")
  end
  equal(nil, ThinkThen.send(:native).thinkthen_code_family(2147483647), "unknown ABI code has no family")
  # The shortcut client owns process-wide configuration. Keep its lifetime in a child.
  pid = fork do
    path = File.join(folder, "decide-precision.json")
    test = JSON.parse(File.read(path))
    Backend.open(path) do |backend|
      ThinkThen.configure do |config|
        config.backend = :systemone
        config.url = backend.url
        config.api_key = "synthetic-loopback-key"
        config.model = manifest.fetch("client").fetch("model")
      end
      request = test.fetch("request")
      question = typed_question(request.fetch("question"))
      rows = [ThinkThen.decide(question, request.fetch("records").first)]
      rows.concat(ThinkThen.decide_each(question, request.fetch("records")).to_a)
      rows.each { |row| strict_row_contract(test.fetch("expected").first, row.to_h, "systemone", manifest.fetch("client").fetch("model")) }
      equal(test.fetch("requests") * 2, backend.command("count").to_i, "shortcut request count")
      ThinkThen.close
    end
  end
  Process.wait(pid)
  assert($?.success?, "shared case through module shortcuts")
  stub = JSON.parse(File.read(ENV.fetch("THINKTHEN_STUB_CASES"), encoding: Encoding::UTF_8), max_nesting: false)
  stub.each { |test| equal(expected_projection(test), execute_case(test, test.fetch("client")), "stub contract") }
  manifest.fetch("backends").each do |backend_name|
    manifest.fetch("cases").each do |filename|
      path = File.join(folder, filename)
      test = JSON.parse(File.read(path, encoding: Encoding::UTF_8), max_nesting: false)
      Backend.open(path) do |backend|
        settings = manifest.fetch("client").merge(test.fetch("client", {})).merge("backend" => backend_name, "url" => backend.url,
                                                   "api_key" => "synthetic-loopback-key")
        rows = execute_case(test, settings)
        if rows.is_a?(Array)
          equal(test.fetch("expected").length, rows.length, "exact row count")
          test.fetch("expected").zip(rows).each do |expected, actual|
            strict_row_contract(expected, actual, backend_name, settings.fetch("model"))
          end
        else
          strict_contract(normalized_contract(expected_projection(test)), normalized_contract(rows))
        end
        equal(test.fetch("requests"), backend.command("count").to_i, "#{backend_name}: #{filename} request count")
      end
    end
  end
end

# Feature: malformed dynamic inputs retain the shared admission diagnostics.
def admission_cases
  path = File.join(ENV.fetch("THINKTHEN_CASES"), "public-contract", "usage-admission.json")
  JSON.parse(File.read(path, encoding: Encoding::UTF_8)).each do |test|
    equal("admission", test.fetch("kind"), "admission fixture kind")
    boundary = test.fetch("boundary")
    route = {"question" => "direct: public Question constructor",
             "toolbox" => "direct: saved toolbox JSON",
             "settings" => "projection: public Retry value in client options"}.fetch(boundary)
    input = test.fetch("input")
    expected = test.fetch("message")
    error = raises(ThinkThen::Error) do
      case boundary
      when "question"
        # Named subclasses can reject malformed domains before engine admission.
        # The public base constructor accepts dynamic options without coercion.
        options = input.reject { |key, _| %w[function question].include?(key) }.transform_keys(&:to_sym)
        ThinkThen::Question.new(input.fetch("function"), input["question"], **options)
      when "toolbox"
        ThinkThen::Toolbox.from_json(JSON.generate(input))
      when "settings"
        options = input.transform_keys(&:to_sym)
        options[:backend] = options[:backend].to_sym if options[:backend]
        options[:retry] = ThinkThen::RetryPolicy.new(**options.fetch(:retry).transform_keys(&:to_sym))
        ThinkThen::Client.open(**options) { raise "malformed settings admitted" }
      else
        raise "unclassified admission boundary: #{boundary}"
      end
    end
    equal(test.fetch("code"), error.code.to_s, "#{test.fetch('name')} code")
    equal(expected, error.message, "#{test.fetch('name')} message")
    assert(!error.message.include?("secret-placeholder"), "admission redacts string contents")
    puts "public admission: #{test.fetch('name')} (#{route})"
  end
end

# Feature: settings and named options use the same admitted contract as other hosts.
def shared_options
  path = File.expand_path("../construction/options.json", ENV.fetch("THINKTHEN_CASES"))
  JSON.parse(File.read(path, encoding: Encoding::UTF_8)).each do |test|
    if test.key?("expected_error")
      error = raises(ThinkThen::Error) do
        ThinkThen::Toolbox.from_json(JSON.generate("entries" => {"fixture" => {"question" => test.fetch("request").fetch("question")}}))
      end
      equal(test.fetch("expected_error"), error.code.to_s, "shared saved-question admission")
    else
      strict_contract(normalized_contract(expected_projection(test)), normalized_contract(execute_case(test, test.fetch("client"))))
    end
    puts "Settings options: #{test.fetch('name')}"
  end
end
