require "thinkthen"

question = "Is this urgent?"
inbox = [
  "Newsletter: our autumn catalog is here. " \
    "No reply needed.",
  "Our checkout page is down and customers cannot pay",
  "Reminder: your invoice is due in 30 days",
  "Please send the signed quote by 5 pm today"
]
ranked = ThinkThen.rank(question, inbox)
raise unless ranked.map(&:index) == [1, 3, 2, 0]
