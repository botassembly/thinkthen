# The text crossing: encodings R strings carry, bytes that are not text,
# and native text under LC_ALL=C (R3-17, R5-10).
source(file.path(Sys.getenv("TT_TESTS"), "helper.R"))

# One child under a locale prints the digests and names it produced from
# native-marked UTF-8 bytes: a question and evidence, choose options, and a
# relate rule.
digests <- function(locale) {
  child(c(
    'word <- rawToChar(as.raw(c(0x63, 0x61, 0x66, 0xc3, 0xa9)))',
    'stopifnot(Encoding(word) == "unknown")',
    'd <- tt_details(paste("Is", word, "open?"), paste("the", word, "is open"))$value',
    'cat("decide", d$meta$question_sha256, unlist(d$meta$requests), "\\n")',
    'c <- tt_details(tt_question(choose = "Which?", options = c(word, "tea")), "a drink")$value',
    'cat("choose", c$meta$question_sha256, unlist(c$meta$requests), "\\n")',
    'e <- tt_relate(data.frame(name = c("a1", "b1"), kind = "x"), relations = word)$value',
    'cat("relate", sapply(e$relation, function(x) paste(charToRaw(enc2utf8(x)), collapse = "")), "\\n")'
  ), env = paste0("LC_ALL=", locale))$text
}
utf8 <- digests("C.UTF-8")
plain <- digests("C")
check("R3-17 and R5-10: native text gives the same digests and names under C and C.UTF-8",
      identical(plain, utf8) && grepl("^decide [0-9a-f]{64} [0-9a-f]{64}", utf8) && grepl("relate 636166c3a9", utf8, fixed = TRUE))

# Latin1 converts, and it asks what its UTF-8 twin asks.
latin1 <- "caf\xe9 complaint"
Encoding(latin1) <- "latin1"
check("latin1 and UTF-8 forms give one question digest", identical(
  tt_details(latin1, "x")$value$meta$question_sha256, tt_details(enc2utf8(latin1), "x")$value$meta$question_sha256))

# Invalid bytes and a bytes mark are refused by name, and send nothing.
native_invalid <- "caf\xe9"
Encoding(native_invalid) <- "unknown"
bytes <- "caf\xe9"
Encoding(bytes) <- "bytes"
for (bad in list(native_invalid, bytes)) {
  sent <- sent_by(said <- message_of(tt_decide("Is this a complaint?", bad)$value))
  check("bad bytes as evidence are refused before a send",
        sent == 0L && grepl("convert it with enc2utf8() or iconv() first", said, fixed = TRUE))
}
check("native invalid bytes in a question are usage", identical(kind_of(tt_decide(native_invalid, "x")$value), "usage"))

# A percent in the caller's text reaches R's error formatter intact.
check("a percent in evidence answers", isTRUE(tt_decide("Is this sure?", "100% sure %s %n")$value))
check("clean UTF-8 answers", isTRUE(tt_decide("Is this a complaint?", "café \U0001F600 complaint")$value))

finish("text", 9L)
