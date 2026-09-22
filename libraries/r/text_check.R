# The text crossing: the percent sign that used to reach R's error
# formatter, the encodings R strings carry, and bytes that are not text.
# Offline: the stand-in answers from the recordings, no network, no key.
#
# Source: sdlc/issues/2026-09-22-surfaces-branch-second-review-new-defects-and-leftovers.md
# (items 5 and 6: the percent crash and the encodings).
.libPaths(c("rlib", .libPaths()))
library(thinkthen)
stopifnot(Sys.getenv("ENGINE_NULL") == "1")

passed <- 0
fail <- function(what) stop(paste("failed:", what), call. = FALSE)
check <- function(what, held) {
  if (!isTRUE(held)) fail(what)
  passed <<- passed + 1
}

# The percent sign. The stand-in's refusal quotes the text back, and that
# quoted message crosses through R's own error formatter (`Rf_error`,
# reached from the shim's Err), which reads a bare `%` as a format and
# reads varargs that do not exist: before the fix, this whole script died
# with a segfault before the message could be caught.
held <- tryCatch(tt_recognize("100% sure %s"), error = function(e) conditionMessage(e))
check("a percent text errors instead of crashing", is.character(held))
check("the quoted text keeps its percent signs", grepl("100% sure %s", held, fixed = TRUE))
check("the quoted message is valid utf-8", validUTF8(held))

# A latin1 string converts: the bytes R marks latin1 arrive as the utf-8
# text the caller meant, which the refusal's quote proves.
latin1 <- "caf\xe9 complaint"
Encoding(latin1) <- "latin1"
held <- tryCatch(tt_recognize(latin1), error = function(e) conditionMessage(e))
check("latin1 text converts to utf-8", is.character(held) && validUTF8(held) &&
        grepl(enc2utf8(latin1), held, fixed = TRUE))
check("the latin1 and utf-8 forms answer the same",
      identical(tt_decide("Is this a complaint?", latin1),
                tt_decide("Is this a complaint?", enc2utf8(latin1))))

# Clean utf-8 keeps answering as before.
clean <- tt_decide("Is this a complaint?", "caf\u00e9 complaint")
check("clean utf-8 still answers", is.logical(clean) && length(clean) == 1L)

# Bytes that are not valid utf-8 are refused loudly, never answered.
bad <- rawToChar(as.raw(c(0x63, 0x61, 0x66, 0xe9)))
res <- tryCatch(tt_decide("Is this a complaint?", bad), error = function(e) conditionMessage(e))
check("invalid bytes refuse with a clear error",
      is.character(res) && grepl("not valid UTF-8", res, fixed = TRUE))
res <- tryCatch(tt_recognize(bad), error = function(e) conditionMessage(e))
check("invalid bytes refuse before the engine sees them",
      is.character(res) && grepl("not valid UTF-8", res, fixed = TRUE))

# A string R marks as bytes is refused by name too.
bytes <- "caf\xe9"
Encoding(bytes) <- "bytes"
res <- tryCatch(tt_decide("Is this a complaint?", bytes), error = function(e) conditionMessage(e))
check("byte-marked strings refuse with a clear error",
      is.character(res) && grepl("enc2utf8", res, fixed = TRUE))

cat("text checks passed:", passed, "\n")
