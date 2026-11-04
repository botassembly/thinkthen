require "thinkthen"

message = "Please refund my order."
messages = [
  "Please refund my order.",
  "Where is my parcel?",
  "I was charged twice for one invoice.",
]
two_lines = "Morgan works for Example Group.\nPlease refund my order."

ThinkThen::Client.open do |client|
  is_refund = client.decide?("Does the customer request a refund?", message)
  puts "decide: #{is_refund}"

  topic = client.choose("What is the message about?", message,
                        choices: ["billing", "shipping", "other"]).answer
  puts "choose: #{topic}"

  topics = client.tag("Which topics appear?",
                      "I was charged twice and my parcel is late again.",
                      labels: ["billing", "shipping"]).answer
  puts "tag: #{topics.inspect}"

  urgency = client.score("How urgent is the request?", message,
                         levels: ["routine", "urgent"]).answer
  puts "score: #{urgency}"

  client.filter(
    "Does the customer request a refund?", messages
  ).each do |refund_request|
    puts "filter: #{refund_request}"
  end

  by_urgency = client.rank("How urgent is the request?", messages,
                           levels: ["routine", "urgent"])
  by_urgency.each.with_index(1) { |request, rank|
    puts "rank: #{rank} #{request}"
  }

  action = client.find(
    "Which line states the requested action?", two_lines
  ).answer
  puts "find: #{action ? action.text : "no line answers"}"

  names = client.recognize("Morgan works for Example Group.",
                           kinds: ["person", "organization"]).answer
  names.each { |name| puts "recognize: #{name.kind} #{name.text}" }

  works_for = ThinkThen::RelationRule.new(
    source: "person", target: "organization", relation: "works_for")
  relations = client.relate(two_lines, rules: [works_for]).answer
  relations.each do |relation|
    puts "relate: #{relation.source.text} " \
         "#{relation.relation} #{relation.target.text}"
  end

  questions = {
    "refund" => ThinkThen::DecideQuestion.new(
      "Does the customer request a refund?"),
    "topic" => ThinkThen::ChooseQuestion.new("What is the message about?",
      choices: ["billing", "shipping", "other"]),
    "urgency" => ThinkThen::ScoreQuestion.new("How urgent is the request?",
                                              levels: ["routine", "urgent"]),
  }
  form = client.annotate(questions, message)
  puts "annotate: #{form[:refund].answer} " \
       "#{form[:topic].answer} #{form[:urgency].answer}"
end
