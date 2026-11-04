require "thinkthen"

ThinkThen::Client.open do |client|
  topics = client.tag("Which topics appear?",
                      "I was charged twice and my parcel is late again.",
                      labels: ["billing", "shipping"]).answer
  p topics
end
