# A plan and call share explicit batch/context; actual loopback
# arrivals and captured bodies are independent of the planner's own count.
source(file.path(Sys.getenv("TT_TESTS"), "helper.R"))
tt_engine(base_url = arm("arm/full/capture/v1"), cache = FALSE)

alpha <- '{"state":"Each question quotes the text it asks about.","model":"jev-1.13.0","questions":{"q1":{"type":"noul","instructions":"The text is \\"alpha\\". Q?"}}}'
beta <- '{"state":"Each question quotes the text it asks about.","model":"jev-1.13.0","questions":{"q1":{"type":"noul","instructions":"The text is \\"beta\\". Q?"}}}'
one <- list(batch = 1L)
planned_sends <- sent_by(plan <- tt_plan("Q?", c("alpha", "beta"), options = one))
actual_sends <- sent_by(answer <- tt_decide("Q?", c("alpha", "beta"), options = one))
check("the batch-one plan has independently known bytes and token band without a send",
      planned_sends == 0L && identical(plan$records, 2) &&
      identical(plan$requests, 2) && identical(plan$estimated_bytes, 309) &&
      identical(plan$estimated_input_tokens, list(lower = 159, upper = 281)) &&
      identical(plan$first_body, alpha))
check("both actual batch-one bodies and the listener count match that plan",
      actual_sends == 2L && identical(answer$facts$requests_sent, 2) &&
      identical(vapply(answer$results, `[[`, TRUE, "value"), c(TRUE, TRUE)))

context <- list(context = "Shared reference")
context_plan_sends <- sent_by(context_plan <- tt_plan("Q?", "gamma", options = context))
context_sends <- sent_by(context_answer <- tt_decide("Q?", "gamma", options = context))
bodies <- capture()
check("a shared context also plans its own actual body and sends once",
      context_plan_sends == 0L && context_sends == 1L &&
      identical(context_plan$requests, 1) &&
      identical(sort(unlist(bodies[1:2])), sort(c(alpha,beta))) &&
      identical(bodies[[3L]],context_plan$first_body) &&
      grepl("Shared reference", bodies[[3L]], fixed = TRUE) &&
      identical(context_answer$facts$requests_sent, 1))
finish("plan identity", 3L)
