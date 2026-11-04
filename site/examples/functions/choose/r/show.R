library(thinkthen)

topic <- tt_choose("Please refund my order.",
                   question = "What is the message about?",
                   choices = c("billing", "shipping", "other"))
print(topic)
