# recognize and relate in R's own shapes: one frame a record with R's
# one-based offsets, relations in an attribute, relate's frame in and edge
# frame out, the dedupe by name and kind, and the 255 cap. The generic arm
# picks the names pinned below. Case 42 of cases.json gives the relation
# shape through the case arm.
source(file.path(Sys.getenv("TT_TESTS"), "helper.R"))
library(tidyr)
columns <- c("text", "start", "end", "length", "kind", "strength")
sentence <- "Maria Chen joined Northwind Freight in Chicago last spring."

found <- tt_recognize(c(sentence, NA), c("person", "organization"))
check("a frame a record, NA an empty frame", length(found) == 2L && identical(names(found[[1]]), columns) &&
      identical(nrow(found[[2]]), 0L) && identical(names(found[[2]]), columns))
check("the one-based offsets slice each name in R", identical(substring(sentence, found[[1]]$start, found[[1]]$end), found[[1]]$text) &&
      identical(found[[1]]$length, found[[1]]$end - found[[1]]$start + 1))
check("tidyr::unnest gives one row a name", identical(nrow(unnest(data.frame(body = sentence, names = I(found[1])), names)), 2L))
emoji <- "Le café \U0001F600 Maria Chen arrived."
whole <- tt_recognize(emoji, "person")[[1]]
check("offsets count code points past an accent and an emoji", identical(c(whole$start, whole$end, whole$length), c(22, 29, 8)) &&
      identical(substr(emoji, whole$start, whole$end), "arrived."))

# Ownership: the values are R's own. Mutating them and a gc() pass change
# nothing the next call returns, and the answer comes from the cache.
held <- found
held[[1]]$text[[1]] <- "Oslo"
invisible(gc())
check("a mutated result leaves the next answer alone", sent_by(again <- tt_recognize(sentence, c("person", "organization"))) == 0L &&
      identical(again[[1]], found[[1]]))

# The relation attribute through case 42's question file and exact bytes.
case <- jsonlite::fromJSON(file.path(Sys.getenv("TT_TESTS"), "../../../conformance/cases.json"), simplifyVector = FALSE)$cases
spec <- tempfile(fileext = ".json")
writeLines(jsonlite::toJSON(Filter(function(c) startsWith(c$id, "42-"), case)[[1]]$question, auto_unbox = TRUE), spec)
related <- child(c(sprintf('r <- tt_recognize("%s", "@%s")[[1]]', sentence, spec), 'a <- attr(r, "relations")',
                   'cat(names(a), "|", unlist(a[1, ]), "|", r$text[a$source == r$text], "\\n")'),
                 env = paste0("THINKTHEN_BASE_URL=", arm("case/42-recognize-C01-relations/v1")))$text
check("relations ride in the attribute with names and kinds", identical(related,
  "source source_kind target target_kind relation probability | Maria Chen person Northwind Freight organization works_for 0.84 | Maria Chen "))

# relate: a frame in, deduped by name and kind in first-seen order, and an
# edge frame out whose first two columns igraph reads.
entities <- data.frame(name = c("a1", "b1", "a1", "a1"), kind = c("x", "x", "x", "y"))
edges <- tt_relate(entities, relations = "caused_by")
check("a repeated name and kind is one entity", identical(nrow(edges), 6L) &&
      identical(names(edges), c("source", "target", "relation", "probability", "source_kind", "target_kind")))
check("each edge carries names and kinds", identical(edges[1, "source"], "a1") && identical(edges[1, "target_kind"], "x"))
check("igraph reads the edge frame", identical(igraph::ecount(igraph::graph_from_data_frame(edges)), 6))
check("relate refuses NA names", identical(kind_of(tt_relate(data.frame(name = c("a", NA), kind = "x"), "r")), "usage"))

# The 255 cap counts unique pairs: 256 names refuse before a send, and 300
# rows over ten names answer.
many <- sent_by(capped <- message_of(tt_relate(data.frame(name = paste0("n", 1:256), kind = "x"), "r")))
check("256 unique entities are refused before a send", many == 0L && identical(capped, "relate takes at most 255 entities"))
check("300 rows over ten names answer", identical(nrow(tt_relate(data.frame(name = paste0("n", rep(1:10, 30)), kind = "x"), "r")), 90L))

# One deadline covers the whole column: three texts at 600 ms each under a
# 1 s deadline end in the deadline kind with at most two sent.
late <- NULL
sent <- sent_by(late <- child(c('r <- tryCatch(tt_recognize(c("d1", "d2", "d3"), "person", deadline = 1), error = function(e) e)',
                                'cat(class(r)[[1]], "\\n")'), env = paste0("THINKTHEN_BASE_URL=", arm("arm/delay/600/v1")))$text)
check("recognize shares one deadline across its texts", identical(late, "thinkthen_deadline ") && sent <= 2L)
cat("recognize under the deadline sent", sent, "\n")
finish("recognize", 13L)
