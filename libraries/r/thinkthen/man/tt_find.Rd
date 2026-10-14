\name{tt_find}
\alias{tt_find}
\title{Pick the unit that best answers}
\usage{
tt_find(question, input, options = tt_options(), deadline_ms = NULL,
  completion = NULL)
}
\arguments{
\item{question}{Literal wording as one string, or an authored question list
carrying the question text, or a \code{\link{tt_question}} selector for a
question file, a saved name, or a reference.}
\item{input}{The ordered units to choose among: a record vector, list, or data
frame; literal text and JSON evidence become one unit. A data frame converts
by row. An explicit \code{\link{tt_input}} or \code{\link{tt_files}} selector
also serves. Missing atomic records are refused.}
\item{options}{Request controls from \code{\link{tt_options}}; an empty list
keeps the native defaults. \code{none = TRUE} offers a none candidate, so the
answer can say that nothing fits.}
\item{deadline_ms}{A whole-number millisecond deadline for this call, or
\code{NULL} for the native default.}
\item{completion}{A \code{\link{tt_completion}} handle that retains settlement
evidence for this call.}
}
\value{
A carrier of class \code{thinkthen_Call} with \code{$value},
\code{$probability}, complete native rows in \code{$results}, and actual
settlement evidence in \code{$facts}. The value is the selected original input or NULL when nothing fits. Probability is the selected probability or NULL. The complete row retains candidates and the typed find answer.
Atomic columns retain \code{$positions} and \code{$length} to identify
missing slots without changing native record identities. Generated optional
fields use \code{thinkthen_absent} for absence and \code{NULL} for explicit null.
Bounded feeds join into these same call fields.
}
\description{
Picks the one unit, out of a set the model sees at once, that best answers the
question.
}
\details{
Native admission owns every rule for the question, the units, and the
controls. Unlike the primitive calls, a find refuses missing atomic records
rather than skipping their slots.

A native failure raises a \code{thinkthen_error} condition with \code{$kind},
\code{$retryable}, and, once settled, the complete error carrier in
\code{$complete}. R interruption cancels the native work and raises R's own
interrupt condition. Native indexes are zero-based; generated optional fields
are \code{thinkthen_absent} when missing and \code{NULL} when explicitly null.
}
\examples{
\dontrun{
question <- "Which line gives the refund deadline?"
policy <- c(
  "Returns need the original receipt.",
  "Refunds are issued within 30 days of purchase.",
  "Shipping is free on orders over $50.",
  "Gift cards cannot be exchanged for cash."
)
refund_deadline <- tt_find(
  question, policy
)$value
stopifnot(identical(refund_deadline, policy[2]))
}
}
\seealso{
\code{\link{tt_decide}} for the shared call behavior, result views, and
failure conditions.
}
