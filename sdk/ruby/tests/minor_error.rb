# frozen_string_literal: true
require "thinkthen"
# Feature: call failures from a newer engine preserve unknown raw codes and messages.
if ARGV[1] == "child"
  ThinkThen::Client.open(backend: :stub) do |client|
    questions = ThinkThen::AnnotationQuestions.new(questions: {"future" => ThinkThen::DecideQuestion.new("Ready?")})
    row = client.annotate(questions, "record")
    error = row.answer.fetch("future").error
    abort "lost child name" unless error.code == :unknown && error.raw_name == "future.problem" && error.raw_code.nil?
    abort "lost child message" unless error.message == ARGV.fetch(0)
  end
  exit
end
if ARGV[1] == "row"
  ThinkThen::Client.open(backend: :stub) do |client|
    row = client.decide("Ready?", nil)
    abort "lost raw row code" unless row.error.code == :unknown && row.error.raw_code == 2147483647
    abort "lost row message" unless row.error.message == ARGV.fetch(0)
  end
  exit
end
begin
  ThinkThen::Client.open(backend: :stub) { abort "future error was ignored" }
rescue ThinkThen::UnknownCodeError => error
  abort "lost raw code" unless error.raw_code == 2147483647 && error.code == :unknown && error.raw_name == "future.code"
  abort "lost engine message" unless error.message == ARGV.fetch(0)
end
