# Validated public result/2 carriers; execution stays in the native engine.
 .tt_complete_models <- list(
  "Position" = list("file?"="str","first?"="positive","last?"="positive","images?"="[str]"),
  "Usage" = list("input_tokens?"="uint","output_tokens?"="uint"),
  "ProfileWarning" = list("tuned_for"="str","running"="str"),
  "BatchWarning" = list("tuned_for"="batch","running"="batch"),
  "Attempt" = list("ordinal"="positive","request_sha256"="Digest","wall_ms"="uint","outcome"="outcome","sdk_request_id"="SdkRequestId","status?"="uint","server_ms?"="uint","request_id?"="str"),
  "PersistenceObservation" = list("state"="=disabled|=pending|=written|=failed","observed_at"="str","advice?"="str"),
  "Facts" = list("usage_persistence?"="PersistenceObservation","largest_request_bytes?"="uint","largest_request_estimated_input_tokens?"="uint|null","token_estimate_method?"="str","call_id"="CallId","records"="uint","requests_sent"="uint","cache_answers"="uint","seconds"="number","input_tokens?"="uint","output_tokens?"="uint","model?"="str","estimated_cost_usd?"="cost","command_ms?"="uint","attempts?"="[Attempt]","held_model_mismatch?"="bool"),
  "QuestionSource" = list("origin"="origin","answered_by"="str","batch_size?"="positive"),
  "Observed" = list("observation_id"="ObservationId"),
  "FailedObservation" = list("failure_id"="FailureId"),
  "Meta" = list("tool"="str","url"="str","model"="str","requests_sent"="uint","cached"="bool","requests"="[Digest]","failed_questions"="uint","origin"="origin|null","question_sources"="[QuestionSource]","observations"="[Observation]","question_sha256?"="Digest","questions_sha256?"="Digest","answered_by?"="str","usage?"="Usage","profile_warning?"="ProfileWarning","batch_setting?"="batch","batch_warning?"="BatchWarning","context_sha256?"="Digest","attempts?"="[Attempt]"),
  "YesNo" = list("kind"="=yes_no","probability"="probability"),
  "Choice" = list("kind"="=choice","pick"="str","probabilities"="{probability}","confidence?"="probability"),
  "Tags" = list("kind"="=tag","probabilities"="{probability}"),
  "Score" = list("kind"="=score","level"="str","probabilities"="{probability}","confidence?"="probability"),
  "FindAnswer" = list("kind"="=find","pick"="str","probabilities"="{probability}","confidence?"="probability"),
  "DecideQuestion" = list("verb"="=decide","text"="text","true?"="description","false?"="description","name?"="str","wording_version?"="positive","item_schema?"="InputDeclaration","context_schema?"="InputDeclaration"),
  "ChooseQuestion" = list("verb"="=choose","text"="text","options"="[str]","name?"="str","wording_version?"="positive","item_schema?"="InputDeclaration","context_schema?"="InputDeclaration"),
  "TagQuestion" = list("verb"="=tag","text"="text","labels"="[str]","name?"="str","wording_version?"="positive","item_schema?"="InputDeclaration","context_schema?"="InputDeclaration"),
  "ScoreQuestion" = list("verb"="=score","text"="text","levels"="[str]","name?"="str","wording_version?"="positive","item_schema?"="InputDeclaration","context_schema?"="InputDeclaration"),
  "FindQuestion" = list("verb"="=find","text"="text","none"="bool","name?"="str","wording_version?"="positive","item_schema?"="InputDeclaration","context_schema?"="InputDeclaration"),
  "RelationRule" = list("name"="str","source"="str","target"="str","reads"="str","either"="bool","single?"="bool"),
  "RelateFields" = list("name"="str","kind"="str"),
  "RelateQuestion" = list("verb"="=relate","fields"="RelateFields|null","relations"="[RelationRule]","threshold"="threshold","profile?"="str","name?"="str","wording_version?"="positive","item_schema?"="InputDeclaration","context_schema?"="InputDeclaration"),
  "RecognizeQuestion" = list("verb"="=recognize","kinds"="{description}","instructions?"="str","entity_definition?"="str","relations?"="[RelationRule]","threshold"="threshold","relation_threshold"="threshold","on?"="str|[str]","profile?"="str","name?"="str","wording_version?"="positive","item_schema?"="InputDeclaration","context_schema?"="InputDeclaration"),
  "Failure" = list("kind"="=backend","cause"="cause"),
  "FailedField" = list("failed"="Failure"),
  "Entity" = list("text"="text","start"="uint","end"="uint","length"="uint","kind"="str","strength"="number","file?"="str","first_line?"="positive","last_line?"="positive"),
  "Endpoint" = list("name"="str","kind"="str","record?"="json","file?"="str","first_line?"="positive","last_line?"="positive"),
  "Edge" = list("relation"="str","source"="Endpoint","target"="Endpoint","probability"="probability","either?"="=true"),
  "EntityEdge" = list("relation"="str","source"="Entity","target"="Entity","probability"="probability","either?"="=true"),
  "Recognition" = list("entities"="[Entity]","relations?"="[EntityEdge]"),
  "PieceOdds" = list("start"="uint","end"="uint","tags"="{probability}"),
  "NameOdds" = list("start"="uint","end"="uint","kinds"="{probability}|null","edges"="{probability}|null"),
  "Span" = list("start"="uint","end"="uint"),
  "PairOdds" = list("relation"="str","source"="Span","target"="Span","probability"="probability"),
  "RecognitionAnswer" = list("pieces"="[PieceOdds]","names"="[NameOdds]","pairs"="[PairOdds]"),
  "AnnotationSuccess" = list("answer_id"="AnswerId","value"="SuccessValue","question"="AtomicQuestion","answer"="AtomicAnswer","threshold"="threshold","request"="Digest"),
  "AnnotationFailure" = list("failure_id"="FailureId","question"="AtomicQuestion","failure"="Failure","request"="Digest"),
  "RelationSuccess" = list("relation"="str","reads"="str","method"="str","direction"="str","source"="Endpoint","target"="Endpoint|null","request"="Digest","answer_id"="AnswerId","probability"="probability","accepted"="bool","answer?"="AtomicAnswer"),
  "RelationFailure" = list("relation"="str","reads"="str","method"="str","direction"="str","source"="Endpoint","target"="Endpoint|null","request"="Digest","failure_id"="FailureId","failure"="Failure"),
  "RelationAnswer" = list("questions"="[RelationEntry]"),
  "DecideResult" = list("schema"="=thinkthen.result/2","answer_id"="AnswerId","meta"="Meta","value"="bool|description","question"="DecideQuestion","answer"="YesNo","threshold"="threshold","input?"="json","position?"="Position","input_file?"="str","file?"="str","first_line?"="positive","last_line?"="positive","source?"="PhysicalSource","images?"="[NativeImage]","index?"="uint"),
  "ChooseResult" = list("schema"="=thinkthen.result/2","answer_id"="AnswerId","meta"="Meta","value"="str|null","question"="ChooseQuestion","answer"="Choice","threshold"="threshold","input?"="json","position?"="Position","input_file?"="str","file?"="str","first_line?"="positive","last_line?"="positive","source?"="PhysicalSource","images?"="[NativeImage]","index?"="uint"),
  "TagResult" = list("schema"="=thinkthen.result/2","answer_id"="AnswerId","meta"="Meta","value"="[str]","question"="TagQuestion","answer"="Tags","threshold"="threshold","input?"="json","position?"="Position","input_file?"="str","file?"="str","first_line?"="positive","last_line?"="positive","source?"="PhysicalSource","index?"="uint"),
  "ScoreResult" = list("schema"="=thinkthen.result/2","answer_id"="AnswerId","meta"="Meta","value"="number","question"="ScoreQuestion","answer"="Score","threshold"="null","input?"="json","position?"="Position","input_file?"="str","file?"="str","first_line?"="positive","last_line?"="positive","source?"="PhysicalSource","images?"="[NativeImage]","index?"="uint"),
  "FilterResult" = list("schema"="=thinkthen.result/2","answer_id"="AnswerId","meta"="Meta","value"="bool","input"="json","question"="DecideQuestion","answer"="YesNo","threshold"="threshold","position?"="Position","file?"="str","first_line?"="positive","last_line?"="positive","source?"="PhysicalSource","index?"="uint"),
  "RankMemberResult" = list("schema"="=thinkthen.result/2","answer_id"="AnswerId","value"="positive","question"="DecideQuestion","answer"="YesNo","threshold"="null","meta"="Meta","source?"="PhysicalSource"),
  "RankMember" = list("name"="str","result"="RankMemberResult"),
  "RankResult" = list("schema"="=thinkthen.result/2","answer_id"="AnswerId","meta"="Meta","value"="positive","input"="json","question"="AtomicQuestion","answer"="AtomicAnswer","threshold"="null","question_name?"="str","position?"="Position","file?"="str","first_line?"="positive","last_line?"="positive","source?"="PhysicalSource","index?"="uint","members?"="[RankMember]"),
  "FindResult" = list("schema"="=thinkthen.result/2","answer_id"="AnswerId","meta"="Meta","value"="json","question"="FindQuestion","answer"="FindAnswer","threshold"="null","position?"="Position","file?"="str","first_line?"="positive","last_line?"="positive","index?"="uint|null","candidates?"="[FindCandidate]"),
  "AnnotateResult" = list("schema"="=thinkthen.result/2","answer_id"="AnswerId","meta"="Meta","input"="json","value"="{AnnotatedValue}","answers"="{AnnotationEntry}","position?"="Position","file?"="str","first_line?"="positive","last_line?"="positive","index?"="uint","source?"="PhysicalSource"),
  "RecognizeResult" = list("schema"="=thinkthen.result/2","answer_id"="AnswerId","meta"="Meta","value"="Recognition","question"="RecognizeQuestion","answer"="RecognitionAnswer","input?"="json","file?"="str","first_line?"="positive","last_line?"="positive","index?"="uint","source?"="PhysicalSource"),
  "RelateResult" = list("schema"="=thinkthen.result/2","answer_id"="AnswerId","meta"="Meta","value"="[Edge]","question"="RelateQuestion","answer"="RelationAnswer","file?"="str","first_line?"="positive","last_line?"="positive","input?"="json","index?"="uint"),
  "CallError" = list("kind"="error_kind","message"="str","retryable"="bool","facts?"="Facts","attempts?"="[Attempt]","stopped?"="Stopped"),
  "DecideSpec" = list("decide"="text","true?"="description","false?"="description","threshold?"="threshold","model?"="str","profile?"="str","batch?"="batch","on?"="str|[str]","name?"="str","wording_version?"="positive","item_schema?"="InputDeclaration","context_schema?"="InputDeclaration"),
  "ChooseSpec" = list("choose"="text","options"="Labels","threshold?"="probability","model?"="str","profile?"="str","batch?"="batch","on?"="str|[str]","name?"="str","wording_version?"="positive","item_schema?"="InputDeclaration","context_schema?"="InputDeclaration"),
  "TagSpec" = list("tag"="text","labels"="Labels","threshold?"="probability","model?"="str","profile?"="str","batch?"="batch","on?"="str|[str]","name?"="str","wording_version?"="positive","item_schema?"="InputDeclaration","context_schema?"="InputDeclaration"),
  "ScoreSpec" = list("score"="text","levels"="Labels","model?"="str","profile?"="str","batch?"="batch","on?"="str|[str]","name?"="str","wording_version?"="positive","item_schema?"="InputDeclaration","context_schema?"="InputDeclaration"),
  "FindSpec" = list("find"="text","none?"="bool","model?"="str","profile?"="str","on?"="str|[str]","name?"="str","wording_version?"="positive","item_schema?"="InputDeclaration","context_schema?"="InputDeclaration"),
  "QuestionFile" = list("path"="str"),
  "QuestionSet" = list("version"="one","questions"="{AnnotationSpec}","batch?"="batch","threshold?"="threshold","profile?"="str"),
  "RecognitionPlan" = list("kinds?"="Labels","instructions?"="str","entity_definition?"="str","relations?"="[PlanRule]"),
  "PlanRule" = list("name"="str","source"="str","target"="str","reads?"="str","either?"="bool","single?"="bool"),
  "RecognitionSpec" = list("version"="one","recognize"="RecognitionPlan","threshold?"="probability","relation_threshold?"="probability","model?"="str","profile?"="str","on?"="str|[str]","name?"="str","wording_version?"="positive","item_schema?"="InputDeclaration","context_schema?"="InputDeclaration"),
  "RelationPlan" = list("relations"="[PlanRule]","fields?"="RelateFields"),
  "RelationSpec" = list("version"="one","relate"="RelationPlan","threshold?"="probability","model?"="str","profile?"="str","name?"="str","wording_version?"="positive","item_schema?"="InputDeclaration","context_schema?"="InputDeclaration"),
  "Files" = list("paths"="[str]","unit"="unit","window?"="positive","media?"="media"),
  "TextInput" = list("text"="json"),
  "RecordInput" = list("records"="[json]","context?"="json"),
  "CandidateInput" = list("units"="[json]","context?"="json"),
  "ImageBytes" = list("data"="bytes","name?"="str"),
  "ImageInput" = list("images"="[ImageBytes]","text?"="json"),
  "Controls" = list("batch?"="batch","context?"="json","on?"="str|[str]","threshold?"="threshold","top?"="positive","none?"="bool","model?"="str","attempts?"="bool","deadline_ms?"="uint"),
  "DecideMember" = list("decide"="text","true?"="description","false?"="description","threshold?"="threshold","on?"="str|[str]","name?"="str","wording_version?"="positive","item_schema?"="InputDeclaration","context_schema?"="InputDeclaration"),
  "ChooseMember" = list("choose"="text","options"="Labels","threshold?"="probability","on?"="str|[str]","name?"="str","wording_version?"="positive","item_schema?"="InputDeclaration","context_schema?"="InputDeclaration"),
  "TagMember" = list("tag"="text","labels"="Labels","threshold?"="probability","on?"="str|[str]","name?"="str","wording_version?"="positive","item_schema?"="InputDeclaration","context_schema?"="InputDeclaration"),
  "ScoreMember" = list("score"="text","levels"="Labels","on?"="str|[str]","name?"="str","wording_version?"="positive","item_schema?"="InputDeclaration","context_schema?"="InputDeclaration"),
  "StringDeclaration" = list("type"="=string"),
  "NumberDeclaration" = list("type"="=number"),
  "BooleanDeclaration" = list("type"="=boolean"),
  "ArrayDeclaration" = list("type"="=array","items"="StringDeclaration"),
  "ObjectDeclaration" = list("type"="=object","properties"="{PropertyDeclaration}","required?"="[str]"),
  "FindCandidate" = list("index"="uint|null","input"="json","probability"="probability","source?"="PhysicalSource"),
  "PhysicalSource" = list("file"="str","first_line?"="positive","last_line?"="positive"),
  "NativeImage" = list("media"="image_media","base64"="str","width"="positive","height"="positive"),
  "Stopped" = list("cause"="stop_cause","retryable"="bool","status?"="uint","at?"="uint"),
  "NativeInput" = list("original"="json","location?"="PhysicalSource","images"="[NativeImage]")
)
.tt_complete_aliases <- list(
  "AnnotationSpec" = list("DecideMember", "ChooseMember", "TagMember", "ScoreMember"),
  "SuccessValue" = list("bool","null","str","number","[str]"),
  "Observation" = list("Observed","FailedObservation"),
  "AtomicAnswer" = list("YesNo","Choice","Tags","Score","FindAnswer"),
  "AtomicQuestion" = list("DecideQuestion","ChooseQuestion","TagQuestion","ScoreQuestion","FindQuestion"),
  "AnnotationEntry" = list("AnnotationSuccess","AnnotationFailure"),
  "RelationEntry" = list("RelationSuccess","RelationFailure"),
  "Result" = list("DecideResult","ChooseResult","TagResult","ScoreResult","FilterResult","RankResult","FindResult","AnnotateResult","RecognizeResult","RelateResult"),
  "AnnotatedValue" = list("bool","null","str","number","[str]","FailedField")
)
.tt_complete_enums <- list(
  "origin" = list("live","cache","replay","proxy","memory"),
  "outcome" = list("ok","status","transport"),
  "cause" = list("missing_answer","wrong_kind","missing_probability","invalid_probability","invalid_distribution","unexpected_probability"),
  "error_kind" = list("usage","backend","local","cancelled","deadline","defect")
)
.tt_complete_ids <- c("CallId","SdkRequestId","ObservationId","FailureId","AnswerId","Digest")
.tt_absent <- structure(list(), class = "thinkthen_absent")

.tt_complete_invalid <- function() stop("invalid complete result", call. = FALSE)
.tt_complete_has <- function(v, name) name %in% names(v)
.tt_complete_object <- function(v) is.list(v) && (length(v) == 0L || !is.null(names(v))) && !anyDuplicated(names(v))

.tt_complete_decode <- function(kind, value, preserve_unknown = FALSE) {
  fail <- .tt_complete_invalid
  if (kind %in% names(.tt_complete_aliases) || grepl("|", kind, fixed = TRUE)) {
    variants <- .tt_complete_aliases[[kind]]
    if (is.null(variants)) variants <- strsplit(kind, "|", fixed = TRUE)[[1L]]
    if (preserve_unknown && .tt_complete_object(value)) {
      identities <- lapply(variants, function(v) {
        shape <- .tt_complete_models[[v]]
        sub("\\?$", "", names(shape))[unlist(shape) %in% .tt_complete_ids]
      })
      shared <- Reduce(intersect, identities)
      if (sum(vapply(identities, function(keys) any(setdiff(keys, shared) %in% names(value)), TRUE)) > 1L) fail()
    }
    for (one in variants) {
      decoded <- tryCatch(list(ok = .tt_complete_decode(one, value, preserve_unknown)), error = function(e) NULL)
      if (!is.null(decoded)) return(decoded$ok)
    }
    fail()
  }
  if (startsWith(kind, "[")) {
    if (!is.list(value) || !is.null(names(value))) fail()
    return(lapply(value, function(v) .tt_complete_decode(substr(kind, 2L, nchar(kind)-1L), v, preserve_unknown)))
  }
  if (startsWith(kind, "{")) {
    if (!.tt_complete_object(value)) fail()
    return(lapply(value, function(v) .tt_complete_decode(substr(kind, 2L, nchar(kind)-1L), v, preserve_unknown)))
  }
  if (kind %in% names(.tt_complete_models)) {
    shape <- .tt_complete_models[[kind]]
    if (!.tt_complete_object(value)) fail()
    extra <- setdiff(names(value), sub("\\?$", "", names(shape)))
    if (!preserve_unknown && length(extra)) fail()
    held <- lapply(value[extra], .tt_complete_json)
    for (key in names(shape)) {
      name <- sub("\\?$", "", key)
      if (!.tt_complete_has(value, name)) {
        if (!endsWith(key, "?")) fail()
        held[name] <- list(.tt_absent)
      } else held[name] <- list(.tt_complete_decode(shape[[key]], value[[name]], preserve_unknown))
    }
    .tt_complete_check(kind, value)
    return(structure(held, class = c(paste0("thinkthen_", kind), "thinkthen_complete")))
  }
  scalar <- function(type) typeof(value) == type && length(value) == 1L && !is.na(value)
  if (kind %in% names(.tt_complete_enums)) {
    if (!scalar("character") || !value %in% unlist(.tt_complete_enums[[kind]])) fail()
    return(value)
  }
  if (kind == "one") { if (!is.numeric(value) || length(value) != 1L || is.na(value) || value != 1) fail(); return(value) }
  if (kind == "bytes") { if (!is.raw(value)) fail(); return(value) }
  if (kind %in% .tt_complete_ids) {
    if (!scalar("character") || !grepl("^[0-9a-f]{64}$", value)) fail()
    return(structure(value, class = c(paste0("thinkthen_", kind), "thinkthen_identity")))
  }
  if (startsWith(kind, "=")) {
    literal <- if (kind == "=true") TRUE else substring(kind, 2L)
    if (!identical(value, literal)) fail()
    return(value)
  }
  if (kind %in% c("json", "description", "text")) {
    if (kind %in% c("description", "text") && !is.null(value) && !is.list(value) && !scalar("character")) fail()
    if (kind == "text" && is.null(value)) fail()
    return(.tt_complete_json(value))
  }
  if (kind == "null") { if (!is.null(value)) fail(); return(NULL) }
  if (kind == "bool") { if (!scalar("logical")) fail(); return(value) }
  if (kind == "str") { if (!scalar("character")) fail(); return(value) }
  numeric <- is.numeric(value) && length(value) == 1L && !is.na(value) && is.finite(value)
  if (kind %in% c("uint", "positive", "batch")) {
    if (kind == "batch" && identical(value, "max")) return(value)
    if (!numeric || value > 9007199254740991 || value < if (kind == "uint") 0 else 1 || value != floor(value)) fail()
    return(value)
  }
  if (kind %in% c("number", "probability", "threshold")) {
    if (numeric && (kind == "number" || (value <= 1 && if (kind == "probability") value >= 0 else value > 0))) return(value)
    if (kind == "threshold") {
      if (is.null(value)) return(NULL)
      if (scalar("character") && grepl("^(0(\\.[0-9]+)?|1(\\.0+)?):(0(\\.[0-9]+)?|1(\\.0+)?)$", value)) {
        band <- as.numeric(strsplit(value, ":", fixed = TRUE)[[1L]])
        if (band[1L] < band[2L]) return(value)
      }
    }
    fail()
  }
  if (kind == "cost" && scalar("character") && grepl("^[0-9]+\\.[0-9]{6}$", value)) return(value)
  fail()
}

.tt_complete_json <- function(v) {
  if (is.null(v)) return(NULL)
  if (is.list(v)) {
    if (anyDuplicated(names(v))) .tt_complete_invalid()
    return(lapply(v, .tt_complete_json))
  }
  if (length(v) != 1L || anyNA(v) || !typeof(v) %in% c("logical", "integer", "double", "character")) .tt_complete_invalid()
  if (is.numeric(v) && !is.finite(v)) .tt_complete_invalid()
  v
}

.tt_complete_check <- function(kind, v) {
  has <- .tt_complete_has
  fail <- .tt_complete_invalid
  if (kind %in% c("ChooseSpec","TagSpec","ChooseMember","TagMember","RecognitionSpec","RelationSpec","TagResult","FilterResult","RecognizeQuestion","RelateQuestion")) {
    for (key in c("threshold","relation_threshold")) {
      if (has(v,key) && (!is.numeric(v[[key]]) || length(v[[key]]) != 1L || v[[key]] <= 0 || v[[key]] > 1)) fail()
    }
  }
  if (kind == "ChooseResult" && !is.null(v$threshold) && !is.numeric(v$threshold)) fail()
  if (kind == "Meta") {
    if (length(v$requests) != length(v$question_sources) || length(v$requests) != length(v$observations) || has(v, "question_sha256") == has(v, "questions_sha256")) fail()
    sources <- v$question_sources
    if (any(!vapply(sources, function(s) s$origin %in% c("live", "cache", "replay"), TRUE))) fail()
    if (!length(sources)) {
      if (!is.null(v$origin) || v$cached || v$requests_sent != 0 || has(v, "answered_by")) fail()
    } else {
      origins <- vapply(sources, function(s) s$origin, "")
      origin <- if ("live" %in% origins) "live" else if ("replay" %in% origins) "replay" else "cache"
      if (v$origin != origin || v$cached != (origin != "live")) fail()
      names <- unique(vapply(sources, function(s) s$answered_by, ""))
      if (length(names) == 1L) {
        if (is.null(v$answered_by) || v$answered_by != names) fail()
      } else if (has(v, "answered_by")) fail()
    }
    if (v$failed_questions != sum(vapply(v$observations, function(o) has(o, "failure_id"), TRUE))) fail()
  }
  if (kind %in% c("Span", "NameOdds", "PieceOdds", "Entity")) {
    if (v$end < v$start || (kind == "Entity" && v$length != v$end - v$start)) fail()
  }
  if (kind == "Position" && (has(v, "first") != has(v, "last") || (has(v, "first") && (!has(v, "file") || v$last < v$first)))) fail()
  if (kind %in% c("Entity", "Endpoint") && (has(v, "first_line") != has(v, "last_line") || (has(v, "first_line") && (!has(v, "file") || v$last_line < v$first_line)))) fail()
  if (kind == "Usage" && !length(v)) fail()
  if (kind == "RankResult" && has(v, "members") && (!length(v$members) || !has(v, "question_name") || v$question$verb != "decide" || v$answer$kind != "yes_no")) fail()
  if (kind == "RankResult" && !v$answer$kind %in% c("yes_no", "score")) fail()
  if (kind %in% c("DecideResult", "FilterResult") && is.null(v$threshold)) fail()
  if (endsWith(kind, "Result") && (kind == "AnnotateResult") != has(v$meta, "questions_sha256")) fail()
}

print.thinkthen_complete <- function(x, ...) { cat("<complete carrier: content withheld>\n"); invisible(x) }
print.thinkthen_identity <- function(x, ...) { cat("<complete identity>\n"); invisible(x) }

.tt_complete_json_text <- function(value) {
  if (inherits(value, "thinkthen_identity")) value <- unclass(value)
  if (is.null(value)) return("null")
  if (is.list(value)) {
    if (inherits(value, "thinkthen_complete")) value <- value[!vapply(value, inherits, TRUE, "thinkthen_absent")]
    encoded <- vapply(value, .tt_complete_json_text, "")
    if (!is.null(names(value))) {
      if (!length(value)) return("{}")
      keys <- vapply(names(value), function(k) as.character(jsonlite::toJSON(k, auto_unbox = TRUE)), "")
      return(paste0("{", paste(paste0(keys, ":", encoded), collapse = ","), "}"))
    }
    return(paste0("[", paste(encoded, collapse = ","), "]"))
  }
  as.character(jsonlite::toJSON(value, auto_unbox = TRUE, digits = NA))
}

.tt_complete_aliases <- c(.tt_complete_aliases, list(
  "Labels" = list("[str]","{description}"),
  "QuestionSpec" = list("DecideSpec","ChooseSpec","TagSpec","ScoreSpec"),
  "RankSpec" = list("DecideSpec","ScoreSpec","QuestionSet","QuestionFile"),
  "Selection" = list("TextInput","RecordInput","CandidateInput","ImageInput","Files")
))
.tt_complete_enums <- c(.tt_complete_enums, list(
  "unit" = list("line","window","file"),
  "media" = list("text","image")
))

.tt_complete_plain <- function(value) {
  if (inherits(value, "thinkthen_identity")) return(unclass(value))
  if (is.list(value)) {
    if (inherits(value, "thinkthen_complete")) value <- value[!vapply(value, inherits, TRUE, "thinkthen_absent")]
    return(lapply(value, .tt_complete_plain))
  }
  value
}

.tt_complete_aliases$InputDeclaration <- list("StringDeclaration", "ObjectDeclaration")
.tt_complete_aliases$PropertyDeclaration <- list("StringDeclaration", "NumberDeclaration", "BooleanDeclaration", "ArrayDeclaration")
.tt_complete_enums$image_media <- list("image/png","image/jpeg")
.tt_complete_enums$stop_cause <- list("usage","local","no_key","transport","status","too_large","reply","backend","cancelled","deadline","defect")

.tt_complete_models$NativeInput <- list(original="json", "location?"="PhysicalSource", images="[NativeImage]")

.tt_complete_models$RelateResult[["input?"]] <- "json"
