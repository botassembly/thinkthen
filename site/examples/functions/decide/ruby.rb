require "thinkthen"

question = "Does the customer ask for a refund?"
broken = "Please refund my order. It arrived broken."
thanks = "Thanks for the quick help yesterday!"
raise unless ThinkThen.decide(question, broken) == true
raise unless ThinkThen.decide(question, thanks) == false
