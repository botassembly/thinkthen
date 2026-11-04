library(thinkthen)

client <- tt_client()
messages <- data.frame(
  id = c("a1", "a2", "a3"),
  text = c("Please refund my order.", "Where is my parcel?",
           "I was charged twice for one invoice.")
)
text <- "Morgan works for Example Group.\nPlease refund my order.\n"
urgency <- c("routine", "urgent")

tt_decide(messages, "Does the customer request a refund?",
          field = "/text", client = client)
tt_filter(messages, "Does the customer request a refund?",
          field = "/text", client = client)
tt_choose(messages, "What is the message about?",
          c("billing", "shipping", "other"),
          field = "/text", client = client)
tt_tag("I was charged twice and my parcel is late again.",
       "Which topics appear?", c("billing", "shipping"), client = client)
tt_score(messages, "How urgent is the request?", urgency,
         field = "/text", client = client)
tt_rank(messages, "How urgent is the request?", urgency,
        field = "/text", client = client)
tt_find(text, "Which line states the requested action?", client = client)
tt_recognize("Morgan works for Example Group.", c("person", "organization"),
             client = client)
tt_relate(text, vctrs::list_of(tt_rule("person", "organization", "works_for")),
          client = client)
tt_annotate(messages, tt_annotation_questions(list(
  refund = tt_decide_question("Does the customer request a refund?"),
  topic = tt_choose_question("What is the message about?",
                             c("billing", "shipping", "other"))
)), field = "/text", client = client)

tt_close(client)
