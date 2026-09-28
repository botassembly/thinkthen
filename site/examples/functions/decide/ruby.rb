require "thinkthen"

question = "Does the customer ask for a refund?"
broken = "Please refund my order. It arrived broken."
thanks = "Thanks for the quick help yesterday!"
broken_is_refund = ThinkThen.decide(question, broken).value
thanks_is_refund = ThinkThen.decide(question, thanks).value
raise unless broken_is_refund == true
raise unless thanks_is_refund == false
