# frozen_string_literal: true

require "open3"
require "tmpdir"

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
    @url = "http://127.0.0.1:#{@output.readline.strip}/v1"
  end

  def command(value)
    @input.puts(value)
    @input.flush
    @output.readline.strip
  end

  def close
    @input.puts("shutdown")
    @input.close
    raise "loopback failed" unless @wait.value.success?
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
                     translated = rule.to_h { |key, entry| [{"reads" => :phrase, "either" => :symmetric}.fetch(key, key.to_sym), entry] }
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

def invoke_rows(client, request)
  question = request.fetch("question")
  function = question.fetch("function")
  records = request.fetch("records")
  options = ruby_options(question)
  if function == "annotate"
    questions = question.fetch("questions").transform_values { |entry| typed_question(entry) }
    client.annotate_each(ThinkThen::AnnotationQuestions.new(questions: questions), records, **options)
  elsif %w[recognize relate].include?(function)
    client.public_send("#{function}_each", records, **options)
  else
    client.public_send("#{function}_each", question.fetch("question"), records, **options)
  end
end

# Feature: every function has backend-independent typed Ruby answers and detail.
def assert_typed_row(row, expected)
  function = expected.fetch("function")
  equal(function.to_sym, row.function, "typed function")
  equal(expected.fetch("status").to_sym, row.status, "typed status")
  klass = {"answered" => ThinkThen::Answered, "unsure" => ThinkThen::Unsure, "failed" => ThinkThen::Failed}.fetch(expected.fetch("status"))
  assert(row.is_a?(klass) && row.is_a?(Data), "status Data variant")
  equal([row.status == :answered, row.status == :unsure, row.status == :failed],
        [row.answered?, row.unsure?, row.failed?], "status predicates")
  assert(row.frozen?, "immutable row")
  assert(row.source.is_a?(ThinkThen::Source), "typed source")
  assert(!row.respond_to?(:p), "no old probability spelling")
  unless row.answered?
    assert(!row.respond_to?(:answer), "non-answer variant has no answer accessor")
    assert(!row.to_h.key?("answer"), "non-answer wire shape omits answer")
  end
  if row.failed?
    assert(row.error.is_a?(ThinkThen::RowError), "typed row error")
    equal(expected.fetch("error").fetch("code").to_sym, row.error.code, "typed error code")
    assert(!row.respond_to?(:probability), "failed variant has no probability")
    return
  end
  return unless row.answered?

  answer = row.answer
  expected_type = {"choose" => String, "tag" => Array, "score" => Float, "rank" => Integer,
                   "annotate" => Hash, "recognize" => Array, "relate" => Array}
  assert(answer.is_a?(expected_type.fetch(function)), "function answer type") if expected_type.key?(function)
  assert([true, false].include?(answer), "boolean answer") if %w[decide filter].include?(function)
  assert(answer.nil? || answer.is_a?(ThinkThen::Found), "typed found answer") if function == "find"
  if function == "recognize"
    answer.each { |name| assert(name.is_a?(ThinkThen::Name), "typed recognized name") }
  elsif function == "relate"
    answer.each do |relation|
      assert(relation.is_a?(ThinkThen::Relation), "typed relation")
      assert(relation.source.is_a?(ThinkThen::RelationName), "typed relation endpoint")
    end
  elsif function == "annotate"
    answer.each_value do |entry|
      assert(entry.is_a?(Data), "typed annotation entry")
      assert(!entry.respond_to?(:position), "annotation omits enclosing metadata")
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
  config[:retry] = ThinkThen::Retry.new(**config[:retry].transform_keys(&:to_sym)) if config[:retry]
  ThinkThen::Client.open(**config) do |client|
    stream = invoke_rows(client, test.fetch("request"))
    begin
      stream.cancel if test["cancelled"]
      stream.each do |value|
        row = value.is_a?(ThinkThen::FilterRow) ? value.row : value
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
  {"error" => error.code.to_s}
end

def expected_fields(expected, actual)
  if expected.is_a?(Hash)
    expected.to_h { |key, value| [key, expected_fields(value, actual.fetch(key))] }
  elsif expected.is_a?(Array)
    equal(expected.length, actual.length, "exact contract array count")
    expected.zip(actual).map { |want, got| expected_fields(want, got) }
  else
    actual
  end
end

# Feature: installed Ruby calls consume canonical cases for both wire formats.
def shared_cases
  folder = File.join(ENV.fetch("THINKTHEN_CASES"), "typed")
  manifest = JSON.parse(File.read(File.join(folder, "manifest.json"), encoding: Encoding::UTF_8))
  registry = JSON.parse(File.read(File.expand_path(manifest.fetch("registry"), folder), encoding: Encoding::UTF_8))
  equal(registry.fetch("codes").to_h { |code| [code.fetch("value"), code.fetch("name")] },
        ThinkThen::NativeContract::CODE_NAMES, "generated code mapping")
  stub = JSON.parse(File.read(ENV.fetch("THINKTHEN_STUB_CASES"), encoding: Encoding::UTF_8), max_nesting: false)
  stub.each { |test| equal(test.fetch("expected"), execute_case(test, test.fetch("client")), "stub contract") }
  manifest.fetch("backends").each do |backend_name|
    manifest.fetch("cases").each do |filename|
      path = File.join(folder, filename)
      test = JSON.parse(File.read(path, encoding: Encoding::UTF_8), max_nesting: false)
      Backend.open(path) do |backend|
        settings = manifest.fetch("client").merge(test.fetch("client", {})).merge("backend" => backend_name, "url" => backend.url,
                                                   "api_key" => "synthetic-loopback-key")
        rows = execute_case(test, settings)
        equal(test.fetch("expected"), expected_fields(test.fetch("expected"), rows), "#{backend_name}: #{filename}")
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
    route = {"question" => "projection: public Question constructor in request envelope",
             "toolbox" => "direct: saved toolbox JSON",
             "settings" => "projection: public Retry value in client options"}.fetch(boundary)
    input = test.fetch("input")
    expected = boundary == "question" ? "request.#{test.fetch('message')}" : test.fetch("message")
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
        options[:retry] = ThinkThen::Retry.new(**options.fetch(:retry).transform_keys(&:to_sym))
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
