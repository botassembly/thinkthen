import thinkthen as tt

question = "Does the customer ask for a refund?"
broken = "Please refund my order. It arrived broken."
thanks = "Thanks for the quick help yesterday!"
typesafe = tt.Engine(backend="typesafe")
liquid = tt.Engine(backend="liquid")
ollama = tt.Engine(
    backend="ollama", base_url="http://localhost:11535/v1",
)
for engine in (typesafe, liquid, ollama):
    broken_is_refund = engine.decide(question, broken).value
    thanks_is_refund = engine.decide(question, thanks).value
    assert broken_is_refund is True
    assert thanks_is_refund is False
