# frozen_string_literal: true

require "thinkthen"

ThinkThen::Client.open(backend: :stub) do |client|
  row = client.choose("Which team?", "refund", choices: %w[billing shipping])
  if row.is_a?(ThinkThen::Answered)
    # @type var answer: String
    answer = row.answer
    puts answer
  end
  questions = ThinkThen::AnnotationQuestions.new(questions: {"refund" => ThinkThen::DecideQuestion.new("Refund?")})
  client.annotate(questions, "refund")
  rows = client.filter_each("Ready?", ["first", nil])
  rows.each { |value| puts value.position }
  rows.close
  case client.decide("Ready?", "record")
  in ThinkThen::Answered[answer:, probability:]
    puts answer, probability
  in ThinkThen::Unsure
    puts "unsure"
  in ThinkThen::Failed[error:]
    puts error.code
  end
end
