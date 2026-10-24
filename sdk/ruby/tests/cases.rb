# frozen_string_literal: true

require "open3"

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

class Backend
  attr_reader :url

  def initialize(*arguments)
    environment = ENV.to_h.select { |name, _| %w[PATH HOME TMPDIR LLVM_PROFILE_FILE].include?(name) }
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

def normalize(row, function)
  value = row.to_h
  value["error"]&.delete("message")
  if function == "annotate" && value["answer"].is_a?(Hash)
    value["answer"].each_value { |entry| entry["error"]&.delete("message") }
  end
  value
end

def execute_case(test, settings)
  output = []
  request = test.fetch("request")
  tagged = request.fetch("question")
  question = case tagged.fetch("type")
             when "inline" then tagged.fetch("value")
             when "toolbox" then ThinkThen.toolbox(tagged.fetch("value"), name: tagged["name"])
             when "reference" then ThinkThen.reference(tagged.fetch("name"))
             end
  ThinkThen::Client.open(**settings) do |client|
    rows = client.public_send(request.fetch("function"), question,
                              request.fetch("records"), **request.fetch("options", {}))
    rows.cancel if test["cancelled"]
    rows.each { |row| output << normalize(row, request.fetch("function")) }
    output
  end
rescue ThinkThen::Error => error
  {"error" => error.code}
ensure
  equal(test["expected_rows"], output, "rows before terminal error") if test.key?("expected_rows")
end

def shared_cases
  stub = JSON.parse(File.read(ENV.fetch("THINKTHEN_STUB_CASES")), max_nesting: false)
  stub.each do |test|
    equal(test.fetch("expected"), execute_case(test, test.fetch("client")),
          "stub #{test.fetch('request').fetch('function')}")
  end
  puts "installed Ruby gem: all #{stub.length} construction functions pass"

  options = JSON.parse(File.read(File.join(File.dirname(ENV.fetch("THINKTHEN_STUB_CASES")), "options.json")))
  options.each do |test|
    actual = execute_case(test, test.fetch("client"))
    expected = if test.key?("expected_error")
                 {"error" => test.fetch("expected_error")}
               else
                 execute_case({"request" => test.fetch("equivalent_request")}, test.fetch("client"))
               end
    equal(expected, actual, test.fetch("name"))
    equal(test.fetch("request").fetch("records").length, actual.length, "option row count") unless test.key?("expected_error")
  end
  puts "installed Ruby gem: shared option combinations pass"


  paths = Dir[File.join(ENV.fetch("THINKTHEN_CASES"), "*.json")].sort
  assert(!paths.empty?, "no shared conformance cases")
  paths.each do |path|
    test = JSON.parse(File.read(path), max_nesting: false)
    Backend.open(path) do |backend|
      settings = test.fetch("client").merge("url" => backend.url)
      equal(test.fetch("expected"), execute_case(test, settings), File.basename(path))
      equal(test.fetch("requests"), backend.command("count").to_i, "backend requests #{path}")
    end
  end
  puts "installed Ruby gem: #{paths.length} shared conformance cases pass"
end
