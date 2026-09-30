# A plan reads exactly the locked judge's batch/context; actual loopback
# arrivals and captured bodies are independent of the planner's own count.
source(file.path(Sys.getenv("TT_TESTS"), "helper.R"))
tt_engine(base_url = arm("arm/full/capture/v1"), cache = FALSE)

alpha <- '{"state":"alpha","model":"jev-1.13.0","questions":{"q1":{"type":"noul","instructions":"Q?"}}}'
beta <- '{"state":"beta","model":"jev-1.13.0","questions":{"q1":{"type":"noul","instructions":"Q?"}}}'
one <- tt_decide("Q?", batch = 1L)
planned_sends <- sent_by(plan <- tt_plan(one, c("alpha", "beta")))
actual_sends <- sent_by(answer <- one(c("alpha", "beta")))
check("the bound batch-one plan has independently known bytes and token band without a send",
      planned_sends == 0L && identical(plan$records, 2) &&
      identical(plan$requests, 2) && identical(plan$estimated_bytes, 185) &&
      identical(plan$estimated_input_tokens, list(lower = 95, upper = 168)) &&
      identical(plan$first_body, alpha))
check("both actual batch-one bodies and the listener count match that plan",
      actual_sends == 2L && identical(answer$facts$requests_sent, 2) &&
      identical(answer$value, c(TRUE, TRUE)))

context <- tt_decide("Q?", context = "Shared reference")
context_plan_sends <- sent_by(context_plan <- tt_plan(context, "gamma"))
context_sends <- sent_by(context_answer <- context("gamma"))
bodies <- capture()
check("a bound context also plans its own actual body and sends once",
      context_plan_sends == 0L && context_sends == 1L &&
      identical(context_plan$requests, 1) &&
      identical(bodies, list(alpha, beta, context_plan$first_body)) &&
      grepl("Shared reference", bodies[[3L]], fixed = TRUE) &&
      identical(context_answer$facts$requests_sent, 1))
finish("plan identity", 3L)
