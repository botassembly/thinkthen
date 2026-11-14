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
    # A failed child fails the form; each consumed child has its own loud read.
    entries = form.raise_if_failed.answer
    is_refund = entries[:refund].answer
    topic = entries[:topic].answer
    urgency = entries[:urgency].answer
    puts "#{is_refund} #{topic} #{urgency}"
  end
end
