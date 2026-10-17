\name{tt_filter}
\alias{tt_filter}
\title{Keep the records that reach the mark}
\usage{
tt_filter(question, input, options = tt_options(), deadline_ms = NULL,
  completion = NULL)
}
\arguments{
\item{question}{Literal wording as one string, or an authored question list
carrying the question text, or a \code{\link{tt_question}} selector for a
question file, a saved name, or a reference.}
\item{input}{A record vector, list, or data frame; literal text and JSON
evidence become one record. A data frame converts by row. An explicit
\code{\link{tt_input}} or \code{\link{tt_files}} selector also serves. Missing
atomic records are refused.}
\item{options}{Request controls from \code{\link{tt_options}}; an empty list
keeps the native defaults. \code{threshold} sets the mark records must reach.}
\item{deadline_ms}{A whole-number millisecond deadline for this call, or
\code{NULL} for the native default.}
\item{completion}{A \code{\link{tt_completion}} handle that retains settlement
evidence for this call.}
}
\value{
A carrier of class \code{thinkthen_Call} with \code{$value},
\code{$probability}, complete native rows in \code{$results}, and actual
settlement evidence in \code{$facts}. The value is a list of kept original inputs. Probability is a list aligned with the kept inputs.
Atomic columns retain \code{$positions} and \code{$length} to identify
missing slots without changing native record identities. Generated optional
fields use \code{thinkthen_absent} for absence and \code{NULL} for explicit null.
Bounded feeds join into these same call fields.
}
\description{
Asks one yes-or-no question about every record and keeps the records whose
answer reaches the mark set by the threshold.
}
\details{
Native admission owns every rule for the question, the records, and the
controls. The \code{threshold} control names the mark; native admission
validates its form for this function. Unlike the primitive calls, a filter
refuses missing atomic records rather than skipping their slots.

A native failure raises a \code{thinkthen_error} condition with \code{$kind},
\code{$retryable}, and, once settled, the complete error carrier in
\code{$complete}. R interruption cancels the native work and raises R's own
interrupt condition. Native indexes are zero-based; generated optional fields
are \code{thinkthen_absent} when missing and \code{NULL} when explicitly null.
}
\examples{
\dontrun{
question <- "Is this a complaint?"
reviews <- c(
  "Arrived a day early. Thank you!",
  "The zipper broke the first time I used it.",
  "Does this come in blue?",
  "The strap snapped on day two."
)
complaints <- unlist(
  tt_filter(question, reviews)$value,
  use.names = FALSE
)
stopifnot(identical(complaints, reviews[c(2, 4)]))
}
}
\seealso{
\code{\link{tt_decide}} for the shared call behavior, result views, and
failure conditions, and \code{\link{tt_rank}} for ordering instead of
keeping.
}
