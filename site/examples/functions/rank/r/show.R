library(thinkthen)

messages <- c("Please refund my order.",
              "Where is my parcel?",
              "I was charged twice for one invoice.")
urgency_rank <- tt_rank(messages,
                        question = "How urgent is the request?",
                        levels = c("routine", "urgent"))
print(urgency_rank)
