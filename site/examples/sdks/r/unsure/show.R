library(thinkthen)

question <- "Does the customer request a refund?"
is_refund <- tt_decide("I was charged twice for one invoice.",
                       question = question,
                       threshold = c(0.2, 0.8))
print(is_refund)

failure <- tryCatch(tt_decide("", question = question),
                    thinkthen_record_error = identity)
print(failure$code)
