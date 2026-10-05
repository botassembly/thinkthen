require "thinkthen"

question = "Does the customer ask for a refund?"
broken = "Please refund my order. It arrived broken."
thanks = "Thanks for the quick help yesterday!"
typesafe = ThinkThen::Engine.new(backend: "typesafe")
liquid = ThinkThen::Engine.new(backend: "liquid")
ollama = ThinkThen::Engine.new(
  backend: "ollama", base_url: "http://localhost:11535/v1"
)
[typesafe, liquid, ollama].each do |engine|
  broken_is_refund = engine.decide(question, broken).value
  thanks_is_refund = engine.decide(question, thanks).value
  raise unless broken_is_refund == true
  raise unless thanks_is_refund == false
end
