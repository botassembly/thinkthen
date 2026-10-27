source(file.path(dirname(sub("^--file=", "", grep("^--file=", commandArgs(), value = TRUE))), "support.R"))
client <- tt_client(backend = "stub")

# Feature: table attachment restores supported classes without reference mutation.
base <- data.frame(id = c("same", "same", "third"),
                   body = c("alpha", "bravo", "charlie"), group = c("a", "b", "a"))
base$day <- as.Date("2026-01-01") + seq_len(3L)
inputs <- list(base, tibble::as_tibble(base),
               dplyr::group_by(tibble::as_tibble(base), group),
               data.table::as.data.table(base))
for (input in inputs) {
  before <- unserialize(serialize(input, NULL))
  rows <- tt_decide(input, "Ready?", evidence = "/body", client = client)
  result <- tt_bind(input, rows, "decision")
  stopifnot(identical(class(result), class(input)),
            identical(as.data.frame(input), as.data.frame(before)),
            identical(result$body, before$body), identical(result$day, before$day),
            identical(result$decision_answer, rows$answer),
            identical(result$decision_probability, rows$probability))
  if (inherits(input, "grouped_df")) {
    stopifnot(identical(dplyr::group_data(result), dplyr::group_data(input)))
  }
  condition(tt_bind(result, rows, "decision"), "thinkthen_r_error")
  condition(tt_bind(input[1, ], rows, "decision"), "thinkthen_r_error")
}

# Feature: filter rows retain originals and failed rows in input order.
input <- list("alpha", NULL, "bravo")
rows <- tt_filter(input, "Ready?", client = client)
check_rows(rows, "filter", 3L)
stopifnot(identical(rows$position, c(1, 2, 3)), identical(rows$item, input),
          is.logical(rows$kept), is.na(rows$kept[[2]]), rows$status[[2]] == "failed",
          as.character(rows$error_code[[2]]) == "record.missing",
          identical(rows$kept, rows$answer))
mask <- warned(tt_keep(input, "Ready?", client = client))
stopifnot(is.logical(mask), length(mask) == 3L, is.na(mask[[2]]))

# Feature: filter originals keep reconstructed grouping and custom attributes.
input <- dplyr::group_by(tibble::as_tibble(base), group)
attr(input, "origin") <- "caller"
rows <- tt_filter(input, "Ready?", evidence = "/body", client = client)
for (position in seq_len(nrow(input))) {
  item <- rows$item[[position]]
  expected <- input[position, , drop = FALSE]
  dplyr::validate_grouped_df(item, check_bounds = TRUE)
  stopifnot(identical(class(item), class(input)),
            identical(item$day, input$day[position]),
            identical(attr(item, "origin"), "caller"),
            identical(dplyr::group_data(item), dplyr::group_data(expected)))
}

# Feature: full rank attachment joins on input positions and top retains failures.
records <- list(list(id = "same", body = "alpha"), NULL,
                list(id = "same", body = "bravo"))
ranked <- tt_rank(records, "How urgent?", c("low", "high"), client = client)
check_rows(ranked, "rank", 3L)
stopifnot(is.double(ranked$answer), ranked$position[[3]] == 2,
          ranked$status[[3]] == "failed")
data <- data.frame(body = c("alpha", NA_character_, "bravo"))
bound <- tt_bind(data, ranked, "priority")
stopifnot(identical(bound$priority_answer,
                    ranked$answer[match(seq_len(3L), ranked$position)]))
limited <- tt_rank(records, "How urgent?", c("low", "high"), top = 1L, client = client)
stopifnot(nrow(limited) == 2L, sum(limited$status == "failed") == 1L,
          limited$position[limited$status == "failed"] == 2)
condition(tt_bind(data, limited, "priority"), "thinkthen_r_error")
# Feature: annotation attachment retains declared columns without answered input.
questions <- tt_annotation_questions(list(ready = tt_decide_question("Ready?")))
for (input in list(data.frame(id = logical(), body = character()),
                   data.frame(id = TRUE, body = "text"))) {
  annotated <- tt_annotate(input, questions, client = client)
  attached <- tt_bind(input, annotated)
  stopifnot(identical(names(attached), c("id", "body", "ready", "ready_status", "ready_probability", "ready_detail")),
            nrow(attached) == nrow(input), is.logical(attached$ready), is.double(attached$ready_probability))
  if (nrow(input)) stopifnot(is.na(attached$ready), as.character(attached$ready_status) == "failed")
}
tt_close(client)
cat("R table restoration, complete filters and rank attachment passed\n")
