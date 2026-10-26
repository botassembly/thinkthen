# Feature: R retains all positions in repeated vector calls on one thread.
library(thinkthen)
test <- jsonlite::fromJSON(Sys.getenv("THINKTHEN_STRESS_CASE"), simplifyVector = FALSE)
client <- do.call(tt_client, test$client)
count <- test$records
records_for <- function(offset) lapply(seq_len(count), function(i) list(id = paste0("record-", sprintf("%.0f", offset + i - 1L)), body = paste("record", sprintf("%.0f", offset + i - 1L))))
failed <- 0L
widths <- 0L
if (test$cancel) {
  result <- tryCatch(tt_decide(records_for(0L), "Ready?", evidence = "/body", client = client), thinkthen_cancelled_error = identity)
  stopifnot(inherits(result, "thinkthen_cancelled_error"), result$code == "local.cancelled")
} else {
  for (round in seq_len(test$concurrency * test$rounds)) {
    offset <- test$identifier_base + (round - 1L) * count
    records <- records_for(offset)
    rows <- suppressWarnings(tt_decide(records, "Ready?", evidence = "/body", client = client))
    stopifnot(nrow(rows) == count, all(rows$position == seq_len(count)),
              identical(unlist(rows$id, use.names = FALSE), paste0("record-", sprintf("%.0f", offset + seq_len(count) - 1L))))
    for (width in unique(nchar(unlist(rows$id, use.names = FALSE)))) widths <- bitwOr(widths, bitwShiftL(1L, width))
    failures <- rows$status == "failed"
    failed <- failed + sum(failures)
    stopifnot(all(as.character(rows$error_code[failures]) == test$scenario$code),
              all(rows$status[!failures] == "answered"), all(rows$answer[!failures]),
              all(rows$probability[!failures] == test$probability))
    hook <- Sys.getenv("THINKTHEN_STRESS_CHECKPOINT_COMMAND")
    if (nzchar(hook)) stopifnot(system(hook) == 0L)
  }
  stopifnot(if (test$scenario$status == "failed") failed > 0 else failed == 0)
}
tt_close(client)
cat(jsonlite::toJSON(list(identifier_width_mask = widths, calls = if (test$cancel) 1L else test$concurrency * test$rounds,
  rows = if (test$cancel) 0L else count * test$concurrency * test$rounds,
  failed = failed, cancelled = test$cancel), auto_unbox = TRUE), "\n", sep = "")
