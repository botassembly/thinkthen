library(thinkthen)

messages <- c("Please refund my order.",
              "Where is my parcel?",
              "I was charged twice for one invoice.")
refund_requests <- tt_filter(messages,
                             question = "Does the customer request a refund?")
print(refund_requests)
