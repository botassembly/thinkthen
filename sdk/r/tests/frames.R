source(file.path(dirname(sub("^--file=", "", grep("^--file=", commandArgs(), value = TRUE))), "support.R"))
stub <- tt_client(backend = "stub")
text <- c("alpha", "bravo", "alpha")
input <- data.frame(body = text, category = factor(c("b", "a", "b")),
                    day = as.Date("2026-01-01") + seq_len(3L), row.names = c("ten", "two", "again"))
rows <- tt_decide(input, "Ready?", evidence = "/body", client = stub)

# Feature: attachment rejects malformed prefixes and unsupported frame classes.
condition(tt_answer(input), "thinkthen_r_error")
for (prefix in list(NULL, NA_character_, "", c("a", "b"), 1)) condition(tt_bind(input, rows, prefix), "thinkthen_r_error")
condition(tt_bind(input, rows), "thinkthen_r_error")
condition(tt_bind(structure(input, class = c("unknown_frame", "data.frame")), rows, "out"), "thinkthen_r_error")

# Feature: column expressions use the same admitted evidence as a frame call.
answers <- tt_answer(tt_decide(text, "Ready?", client = stub))
tbl <- dplyr::mutate(tibble::as_tibble(input), answer = tt_decide_vec(body, "Ready?", client = stub))
DT <- data.table::as.data.table(input)
DT[, answer := tt_decide_vec(body, "Ready?", client = stub)]
stopifnot(identical(tbl$answer, answers), identical(DT$answer, answers))

# Feature: attachment preserves repeated names, custom attributes and empty prototypes.
names(input)[2:3] <- c("duplicate", "duplicate")
attr(input, "origin") <- "retain"
attr(input[[1]], "source") <- "column"
bound <- tt_bind(input, rows, "odd prefix")
stopifnot(identical(names(bound)[1:3], names(input)), identical(bound[[1]], input[[1]]),
          identical(bound[[2]], input[[2]]), identical(bound[[3]], input[[3]]),
          identical(attr(bound, "origin"), "retain"), identical(row.names(bound), row.names(input)))
for (data in list(input[FALSE, , drop = FALSE], tibble::tibble(body = character()))) {
  empty <- tt_decide(character(), "Ready?", client = stub)
  bound <- tt_bind(data, empty, "out")
  stopifnot(nrow(bound) == 0L, identical(class(bound), class(data)),
            is.logical(bound$out_answer), is.double(bound$out_probability),
            is.factor(bound$out_status), is.factor(bound$out_error_code), is.list(bound$out_detail))
}

# Feature: an answered absent find differs from a failed projection.
nothing <- tt_find(" \r\n\t\n", "Which line?", client = stub)
stopifnot(nothing$status == "answered", inherits(tt_answer(nothing), "tt_span"), is.na(tt_answer(nothing)))
cat("R eager table expressions, names, attributes and prototypes passed\n")

# Feature: factor IDs preserve labels through the public frame call.
input <- data.frame(id = factor(c("second", "first", "second")), body = text)
rows <- tt_decide(input, "Ready?", evidence = "/body", client = stub)
stopifnot(nrow(rows) == 3L, all(rows$status == "answered"),
          identical(rows$id, as.list(as.character(input$id))))

# Feature: frame encoding matches canonical per-row JSON for nested host values.
input <- data.frame(id = factor(c("second", "first", "third")),
  body = c('quoted "},[\\" text', "βeta", NA_character_),
  category = factor(c("b", "a", NA)), day = as.Date(c("2026-01-02", NA, "2026-01-04")),
  number = c(1 / 3, NA_real_, Inf))
input$nested <- I(list(list(values = list(1, list(text = "a,b}"))), NULL,
                       list(values = list(TRUE, FALSE))))
reference <- lapply(seq_len(nrow(input)), function(position) {
  json <- as.character(jsonlite::toJSON(input[position, , drop = FALSE],
    dataframe = "rows", auto_unbox = TRUE, null = "null", na = "null", digits = I(17), force = FALSE))
  jsonlite::fromJSON(json, simplifyVector = FALSE)[[1]]
})
actual <- tt_filter(input, "Ready?", client = stub)
expected <- tt_filter(reference, "Ready?", client = stub)
stopifnot(identical(actual[names(actual) != "item"], expected[names(expected) != "item"]),
  identical(actual$item, lapply(seq_len(nrow(input)), function(i) input[i, , drop = FALSE])))

# Feature: an unsupported cell fails its row while serializable neighbors survive.
input <- data.frame(body = c("alpha", "bravo", "charlie"))
input$nested <- I(list(list(value = 1), new.env(parent = emptyenv()), list(value = 3)))
actual <- tt_filter(input, "Ready?", client = stub)
expected <- tt_filter(lapply(seq_len(nrow(input)), function(i) {
  list(body = input$body[[i]], nested = input$nested[[i]])
}), "Ready?", client = stub)
stopifnot(identical(as.character(actual$status), c("answered", "failed", "answered")),
  identical(actual[names(actual) != "item"], expected[names(expected) != "item"]))

cat("R canonical frame encoding and isolated conversion failures passed\n")
