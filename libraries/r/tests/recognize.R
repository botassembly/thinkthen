# Recognition returns generated native objects, without a second frame API.
source(file.path(Sys.getenv("TT_TESTS"), "helper.R"))
question <- list(version=1L,recognize=list(kinds=list(person=NULL,organization=NULL)))
sentence <- "Maria Chen joined Northwind Freight in Chicago last spring."
called <- tt_recognize(question,c(sentence,NA_character_,sentence))
check("recognition preserves present originals and the missing-column map", called$facts$records==2 &&
  identical(called$positions,c(0,2)) && called$length==3 && length(called$results)==2L)
row <- called$results[[1]]
check("generated spans keep code-point offsets", all(vapply(row$value$entities,function(entity)
  identical(substr(sentence,entity$start+1L,entity$end),entity$text),TRUE)))
held <- row$value
held$entities[[1]]$text <- "mutated"
gc()
again <- tt_recognize(question,sentence)
check("mutating a returned value does not alter cached answers", again$facts$requests_sent==0 &&
  identical(again$results[[1]]$value,row$value))
check("all missing recognition produces native no-work facts", tt_recognize(question,c(NA_character_,NA_character_))$facts$requests_sent==0)
entities <- data.frame(name=c("a1","b1","a1","a1"),kind=c("x","x","x","y"))
relation <- list(version=1L,relate=list(relations=list(list(name="caused_by",source="*",target="*"))))
edges <- tt_relate(relation,entities)$results[[1]]$value
check("relation keeps typed edges and deduplicates name-kind pairs", length(edges)==6L &&
  all(vapply(edges,inherits,TRUE,"thinkthen_Edge")))
check("relation refuses missing names before sending", sent_by(check("missing name",kind_of(
  tt_relate(relation,data.frame(name=c("a",NA),kind="x")))=="usage"))==0L)
finish("recognition ownership",8L)
