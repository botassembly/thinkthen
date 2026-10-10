# Native generated carriers own result fields; R supplies printing and ordinary views.
print.thinkthen_complete <- function(x, ...) { cat("<complete carrier: content withheld>\n"); invisible(x) }
print.thinkthen_identity <- function(x, ...) { cat("<complete identity>\n"); invisible(x) }

.tt_complete_plain <- function(value) {
  if (inherits(value, "thinkthen_identity")) return(unclass(value))
  if (is.list(value)) {
    if (inherits(value, "thinkthen_complete")) value <- value[!vapply(value, inherits, TRUE, "thinkthen_absent")]
    return(lapply(value, .tt_complete_plain))
  }
  value
}

