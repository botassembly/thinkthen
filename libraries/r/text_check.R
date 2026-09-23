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

# Native-marked bytes that are already valid utf-8 cross unchanged under
# any locale, and native-marked bytes that are not valid utf-8 refuse by
# name - the fourth review's LC_ALL=C probe. Pinned here after the
# second-review fix: both cases hold at the tip, and this file runs under
# whatever locale check.sh was invoked with, so an explicit C-locale
# child pins the first case where it was reported.
native_valid <- "caf\u00e9 complaint"
Encoding(native_valid) <- "native"
held <- tryCatch(tt_recognize(native_valid), error = function(e) conditionMessage(e))
check("native-marked valid utf-8 crosses unchanged",
      is.character(held) && validUTF8(held) && grepl(enc2utf8("caf\u00e9 complaint"), held, fixed = TRUE))
c_locale <- Sys.setlocale("LC_CTYPE", "C")
held <- tryCatch(tt_recognize(native_valid), error = function(e) conditionMessage(e))
Sys.setlocale("LC_CTYPE", c_locale)
# The comparison is by bytes, not by grepl: the message the refusal
# carried came back under a C-locale checkpoint and R marked it native,
# and grepl under any locale may refuse to translate the utf-8 needle.
# The bytes are the fact; nothing locale-sensitive may judge them.
bytes_contain <- function(haystack, needle) {
  h <- charToRaw(haystack)
  n <- charToRaw(needle)
  if (length(n) == 0L) return(TRUE)
  hits <- which(h == n[[1]])
  for (start in hits) {
    end <- start + length(n) - 1L
    if (end <= length(h) && identical(h[start:end], n)) return(TRUE)
  }
  FALSE
}
check("native-marked valid utf-8 crosses unchanged under LC_CTYPE=C",
      is.character(held) && bytes_contain(held, enc2utf8("caf\u00e9 complaint")))
native_invalid <- "caf\xe9 complaint"
Encoding(native_invalid) <- "native"
held <- tryCatch(tt_recognize(native_invalid), error = function(e) conditionMessage(e))
check("native-marked invalid bytes refuse by name",
      is.character(held) && grepl("not valid UTF-8", held, fixed = TRUE))

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
