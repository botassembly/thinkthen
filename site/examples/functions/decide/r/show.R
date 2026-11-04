library(thinkthen)

is_refund <- tt_decide("Please refund my order.",
                       question = "Does the customer request a refund?")
print(is_refund)
