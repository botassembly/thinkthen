require "thinkthen"

messages = [
  "Please refund my order.",
  "Where is my parcel?",
  "I was charged twice for one invoice.",
]
questions = {
  "refund" => ThinkThen::DecideQuestion.new(
    "Does the customer request a refund?"
  ),
  "topic" => ThinkThen::ChooseQuestion.new(
    "What is the message about?",
    choices: ["billing", "shipping", "other"]
  ),
  "urgency" => ThinkThen::ScoreQuestion.new(
    "How urgent is the request?",
    levels: ["routine", "urgent"]
  ),
}
ThinkThen::Client.open do |client|
  client.annotate_each(questions, messages) do |form|
    is_refund = form[:refund].answer
    topic = form[:topic].answer
    urgency = form[:urgency].answer
    puts "#{is_refund} #{topic} #{urgency}"
  end
end
