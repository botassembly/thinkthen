library(thinkthen)

urgency <- tt_score("Please refund my order.",
                    question = "How urgent is the request?",
                    levels = c("routine", "urgent"))
print(urgency)
