# recognize and relate in R's own shapes: one frame a record with R's
# one-based offsets, relations in an attribute, relate's frame in and edge
# frame out, the dedupe by name and kind, and the 255 cap. The generic arm
# picks the names pinned below. Case 42 of cases.json gives the relation
# shape through the case arm.
source(file.path(Sys.getenv("TT_TESTS"), "helper.R"))
library(tidyr)
columns <- c("text", "start", "end", "length", "kind", "strength")
sentence <- "Maria Chen joined Northwind Freight in Chicago last spring."

called <- tt_recognize(c(sentence, NA, sentence), c("person", "organization"))
found <- called$value
check("a frame a record, NA an empty frame", length(found) == 3L && identical(names(found[[1]]), columns) &&
      identical(nrow(found[[2]]), 0L) && identical(names(found[[2]]), columns))
check("recognize aggregates live positions and keeps their original indexes",
      identical(called$facts$records, 2L) && called$facts$requests_sent >= 1 &&
      identical(sort(unique(vapply(called$details, `[[`, 0, "index"))), c(0, 2)))
all_missing <- tt_recognize(c(NA_character_, NA_character_), "person")
check("all missing recognition has no-work facts and no question event",
      identical(all_missing$facts$records, 0L) && identical(all_missing$facts$requests_sent, 0L) &&
      is.null(all_missing$facts$model) && length(all_missing$details) == 0L)
check("empty recognition has no spans and sends nothing", sent_by(check("empty recognition frame",
  identical(tt_recognize("", "person")$value[[1]], found[[2]]))) == 0L)
check("the one-based offsets slice each name in R", identical(substring(sentence, found[[1]]$start, found[[1]]$end), found[[1]]$text) &&
      identical(found[[1]]$length, found[[1]]$end - found[[1]]$start + 1))
check("tidyr::unnest gives one row a name", identical(nrow(unnest(data.frame(body = sentence, names = I(found[1])), names)), 2L))
emoji <- "Le café \U0001F600 Maria Chen arrived."
whole <- tt_recognize(emoji, "person")$value[[1]]
check("offsets count code points past an accent and an emoji", identical(c(whole$start, whole$end, whole$length), c(22, 29, 8)) &&
      identical(substr(emoji, whole$start, whole$end), "arrived."))

# Ownership: the values are R's own. Mutating them and a gc() pass change
# nothing the next call returns, and the answer comes from the cache.
held <- found
held[[1]]$text[[1]] <- "Oslo"
invisible(gc())
check("a mutated result leaves the next answer alone", sent_by(again <- tt_recognize(sentence, c("person", "organization"))$value) == 0L &&
      identical(again[[1]], found[[1]]))

# The relation attribute through case 42's question file and exact bytes.
case <- jsonlite::fromJSON(file.path(Sys.getenv("TT_TESTS"), "../../../conformance/cases.json"), simplifyVector = FALSE)$cases
spec <- tempfile(fileext = ".json")
writeLines(jsonlite::toJSON(Filter(function(c) startsWith(c$id, "42-"), case)[[1]]$question, auto_unbox = TRUE), spec)
related <- child(c(sprintf('r <- tt_recognize("%s", "@%s")$value[[1]]', sentence, spec), 'a <- attr(r, "relations")',
                   'cat(names(a), "|", unlist(a[1, ]), "|", r$text[a$source == r$text], "\\n")'),
                 env = paste0("THINKTHEN_BASE_URL=", arm("case/42-recognize-C01-relations/v1")))$text
check("relations ride in the attribute with names and kinds", identical(related,
  "source source_kind target target_kind relation probability either | Maria Chen person Northwind Freight organization works_for 0.84 FALSE | Maria Chen "))

# relate: a frame in, deduped by name and kind in first-seen order, and an
# edge frame out whose first two columns igraph reads.
entities <- data.frame(name = c("a1", "b1", "a1", "a1"), kind = c("x", "x", "x", "y"))
edges <- tt_relate(entities, relations = "caused_by")$value
check("a repeated name and kind is one entity", identical(nrow(edges), 6L) &&
      identical(names(edges), c("source", "target", "relation", "probability", "source_kind", "target_kind", "either")))
check("each edge carries names and kinds", identical(edges[1, "source"], "a1") && identical(edges[1, "target_kind"], "x"))
check("igraph reads the edge frame", identical(igraph::ecount(igraph::graph_from_data_frame(edges)), 6))
unnested <- unnest(data.frame(body = sentence, names = I(found[1])), names)
check("relate reads tidyr::unnest of tt_recognize by its text, and name wins over text",
      identical(tt_relate(unnested, "knows")$value, tt_relate(data.frame(name = found[[1]]$text, kind = found[[1]]$kind), "knows")$value) &&
      identical(tt_relate(data.frame(name = found[[1]]$text, text = "not this", kind = found[[1]]$kind), "knows")$value$source,
                found[[1]]$text))
check("relate refuses NA names", identical(kind_of(tt_relate(data.frame(name = c("a", NA), kind = "x"), "r")$value), "usage"))

# The 255 cap counts unique pairs: 256 names refuse before a send, and 300
# rows over ten names answer.
many <- sent_by(capped <- message_of(tt_relate(data.frame(name = paste0("n", 1:256), kind = "x"), "r")$value))
check("256 unique entities are refused before a send", many == 0L && identical(capped, "relate takes at most 255 entities"))
check("300 rows over ten names answer", identical(nrow(tt_relate(data.frame(name = paste0("n", rep(1:10, 30)), kind = "x"), "r")$value), 90L))

# One deadline covers the whole column: three texts at 600 ms each under a
# 1 s deadline end in the deadline kind with at most two sent.
late <- NULL
sent <- sent_by(late <- child(c('r <- tryCatch(tt_recognize(c("d1", "d2", "d3"), "person", deadline_ms = 1000), error = function(e) e)',
                                'cat(class(r)[[1]], "\\n")'), env = paste0("THINKTHEN_BASE_URL=", arm("arm/delay/600/v1")))$text)
check("recognize shares one deadline across its texts", identical(late, "thinkthen_deadline ") && sent <= 2L)
cat("recognize under the deadline sent", sent, "\n")
# A later oversized text fails before its own request. The first text's
# accounted prefix, details, and measured outer elapsed stay on error.
prefix <- child(c(
  'e <- tryCatch(tt_recognize(c("Maria Chen", paste(rep("x", 600001L), collapse = "")), "person"), error = function(e) e)',
  'cat(e$kind, e$facts$records, e$facts$requests_sent, is.null(e$facts$model),',
  '    length(e$details), identical(unique(vapply(e$details, `[[`, 0, "index")), 0),',
  '    e$facts$seconds > 0, "\\n")'
))
check("later recognition usage retains the first accounted call only",
      identical(trimws(prefix$text), "usage 1 2 FALSE 3 TRUE TRUE"))
# Repeated equivalent relate inputs reuse their cached answer across the
# frame-shape comparisons above.
finish("recognize", 12L + sent)
