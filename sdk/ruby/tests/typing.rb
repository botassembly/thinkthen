# frozen_string_literal: true
require "thinkthen"

ThinkThen::Client.open(backend: :stub) do |client|
  row = client.choose("Which team?", "refund", choices: %i[billing shipping])
  # @type var answer: String | Symbol
  answer = row.answer
  puts answer
  question = ThinkThen::DecideQuestion.new("Ready?", threshold: 0.2..0.8)
  questions = ThinkThen::AnnotationQuestions.new(refund: question)
  client.annotate(questions, "refund")
  client.decide_each(question, ["first", nil]) do |value|
    puts value.yes? if value.answered?
  end
  choice = ThinkThen::Toolbox.new(questions: {topic: ThinkThen::ChooseQuestion.new("Team?", choices: %i[billing shipping])})
  client.choose(choice.choose(:topic), "refund")
  result = client.decide("Ready?", "record")
  case result.status
  in :answered then puts result.answer
  in :unsure then puts "unsure"
  in :failed then puts result.error&.code
  end
end
