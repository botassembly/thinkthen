require "thinkthen"

message = "Morgan works for Example Group.\nPlease refund my order."
ThinkThen::Client.open do |client|
  # nil when no line answers the question.
  action = client.find("Which line states the requested action?",
                       message).answer
  puts action ? action.text : "no line answers"
end
