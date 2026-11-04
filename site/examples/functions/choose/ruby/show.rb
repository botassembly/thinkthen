require "thinkthen"

ThinkThen::Client.open do |client|
  topic = client.choose("What is the message about?",
                        "Please refund my order.",
                        choices: ["billing", "shipping", "other"]).answer
  puts topic
end
