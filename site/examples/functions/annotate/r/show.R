library(thinkthen)

messages <- c("Please refund my order.",
              "Where is my parcel?",
              "I was charged twice for one invoice.")
inbox_form <- tt_annotation_questions(list(
  "refund" = tt_decide_question("Does the customer request a refund?"),
  "topic" = tt_choose_question("What is the message about?",
                               choices = c("billing", "shipping", "other")),
  "urgency" = tt_score_question("How urgent is the request?",
                                levels = c("routine", "urgent"))
))
triage <- tt_annotate(messages, inbox_form)
print(triage)
