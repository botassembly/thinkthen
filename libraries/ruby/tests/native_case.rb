# frozen_string_literal: true
require "thinkthen"
doc = JSON.parse(File.read(ARGV.fetch(0), encoding: "UTF-8"))
settings = JSON.parse(ARGV.fetch(1)).transform_keys(&:to_sym)
client = ThinkThen::Client.new(**settings)
def json_value(value)
  return value.to_h.transform_values { |v| json_value(v) } if value.is_a?(Hash) || (value.respond_to?(:to_h) && !value.nil? && !value.is_a?(Array))
  return value.map { |v| json_value(v) } if value.is_a?(Array)
  value
end
begin
  question = doc.fetch("question")
  question = case question.fetch("kind")
    when "definition" then question.fetch("value")
    when "text" then question.fetch("text")
    when "file" then ThinkThen::Client.question_file(question.fetch("path"))
    when "name" then ThinkThen::Client.question_name(question.fetch("name"))
    when "reference" then ThinkThen::Client.question_reference(question.fetch("reference"))
  end
  input = doc.fetch("input")
  input = if input.fetch("kind") == "source"
    source = input.fetch("source")
    ThinkThen::Client.files(source.fetch("paths"), framing: source["framing"], media: source.fetch("media", "text"), **source.fetch("reading", {}).transform_keys(&:to_sym))
  else
    input.fetch("items").map do |item|
      original = item["original"]
      value = original && original.fetch(original.fetch("kind") == "text" ? "text" : "value")
      ThinkThen::Client::Item.new(item.transform_keys(&:to_sym))
    end
  end
  cancel = ThinkThen::Cancel.new
  cancel.cancel if doc["cancel"]
  if doc["held_cancel"]
    Thread.new { STDIN.gets; cancel.cancel; puts "cancel-fired"; STDOUT.flush }
  end
  done = client.public_send(doc.fetch("verb"), question, input, cancel: cancel, **doc.fetch("options").transform_keys(&:to_sym))
  raise "untyped facts" unless done.facts.is_a?(ThinkThen::Results::NativeFacts)
  done.results.each { |row| raise "untyped result" unless row.class.name.start_with?("ThinkThen::Results::Native") }
  puts JSON.generate(packets: json_value(done.observations) + [{kind: "aggregate", value: json_value(done.results)}, json_value(done.terminal)])
rescue ThinkThen::Error => error
  if error.terminal
    puts JSON.generate(packets: [{kind: "aggregate", value: json_value(error.results)}, json_value(error.terminal)])
  else
    puts JSON.generate(admission: {code: error.code, message: error.message, requests_sent: 0})
  end
ensure
  client.close
end
