require "thinkthen"

question = "Does the customer ask for a refund?"
broken = "Please refund my order. It arrived broken."
thanks = "Thanks for the quick help yesterday!"
typesafe = ThinkThen::Client.new(backend: "typesafe")
liquid = ThinkThen::Client.new(backend: "liquid")
ollama = ThinkThen::Client.new(
  backend: "ollama", base_url: "http://localhost:11535/v1"
)
begin
[typesafe, liquid, ollama].each do |engine|
  broken_is_refund = engine.decide(question, broken).value
  thanks_is_refund = engine.decide(question, thanks).value
  raise unless broken_is_refund == true
  raise unless thanks_is_refund == false
end
ensure
  [typesafe, liquid, ollama].each(&:close)
end
