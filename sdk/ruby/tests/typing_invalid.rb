# frozen_string_literal: true

require "thinkthen"

ThinkThen::Client.open(backend: :stub) do |client|
  client.choose("Which?", "record") # type-error
  client.tag("Tags?", "record", labels: %w[a b], threshold: 0.2..0.8) # type-error
  client.rank("Urgency?", "record", levels: %w[low high], top: 1.5) # type-error
  ThinkThen::Client.new(backend: :unknown) # type-error
  row = client.decide("Ready?", "record")
  row.answer # type-error
  rank = ThinkThen::RankQuestion.new("Urgency?", levels: %w[low high])
  ThinkThen::AnnotationQuestions.new(questions: {"bad" => rank}) # type-error
  toolbox = ThinkThen::Toolbox.new(questions: {"ready" => ThinkThen::DecideQuestion.new("Ready?")})
  client.annotate(toolbox, "record") # type-error
end
