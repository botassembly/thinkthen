library(thinkthen)

topics <- tt_tag("I was charged twice and my parcel is late again.",
                 question = "Which topics appear?",
                 labels = c("billing", "shipping"))[[1]]
print(topics)
