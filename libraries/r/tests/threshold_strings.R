# The same string band enters through a direct call, a built question, and
# raw saved JSON. The backend fixes yes probability at 0.9, inside the band.
source(file.path(Sys.getenv("TT_TESTS"), "helper.R"))
tt_engine(base_url = arm("arm/full/capture/v1"), cache = FALSE)
file <- tempfile(fileext = ".json")
writeLines('{"decide":"Q?","threshold":"0.2:0.95"}', file)
entrypoints <- list(
  direct = function() tt_decide("Q?", "x", threshold = "0.2:0.95"),
  built = function() tt_decide(tt_question(decide = "Q?", threshold = "0.2:0.95"), "x"),
  saved = function() tt_decide(tt_question(file = file), "x")
)
for (name in names(entrypoints)) {
  arrivals <- sent_by(answer <- entrypoints[[name]]())
  check(paste(name, "keeps unsure and yes probability in one send"),
        arrivals == 1L && identical(answer$value, NA) && identical(answer$probability, 0.9))
}
expected <- paste0('{"state":"Each question quotes the text it asks about.","model":"jev-1.13.0","questions":{',
                   '"q1":{"type":"noul","instructions":"The text is \\"x\\". Q?"}}}')
check("all three entrypoints send the same literal request body",
      identical(capture(), rep(list(expected), 3L)))

# Choose and tag take a string cut but cannot take a band.
choice <- sent_by(chosen <- tt_choose("Which?", "x", c("first", "second"), threshold = "0.95"))
tags <- sent_by(tagged <- tt_tag("Which?", "x", c("first", "second"), threshold = "0.95"))
check("string cuts keep choose and tag selection rules",
      choice == 1L && tags == 1L && identical(chosen$value, NA_character_) &&
      identical(chosen$probability, NA_real_) && identical(tagged$value, list(character())) &&
      is.null(tagged$probability))
constructed_sends <- sent_by({
  choose_question <- tt_question(choose = "Which?", options = c("first", "second"), threshold = "0.95")
  tag_question <- tt_question(tag = "Which?", labels = c("first", "second"), threshold = "0.95")
  check("constructed choose and tag string cuts reach the shared grammar",
        identical(choose_question$kind, "choose") && identical(tag_question$kind, "tag") &&
        grepl('"threshold":"0.95"', choose_question$json, fixed = TRUE) &&
        grepl('"threshold":"0.95"', tag_question$json, fixed = TRUE))
})
check("constructing valid string cuts sends nothing", constructed_sends == 0L)

invalid <- list(
  reversed_direct = function() tt_decide("Q?", "x", threshold = "0.95:0.2"),
  reversed_built = function() tt_question(decide = "Q?", threshold = "0.95:0.2"),
  blank_direct = function() tt_decide("Q?", "x", threshold = "0.2:"),
  blank_built = function() tt_question(decide = "Q?", threshold = "0.2:"),
  choose_band = function() tt_choose("Which?", "x", c("first", "second"), threshold = "0.2:0.95"),
  tag_band = function() tt_tag("Which?", "x", c("first", "second"), threshold = "0.2:0.95")
)
for (name in names(invalid)) {
  arrivals <- sent_by(kind <- kind_of(invalid[[name]]()))
  check(paste(name, "is usage before any request"), identical(kind, "usage") && arrivals == 0L)
}
finish("threshold strings", 5L)
