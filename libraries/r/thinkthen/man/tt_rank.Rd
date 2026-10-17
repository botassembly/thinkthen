\name{tt_rank}
\alias{tt_rank}
\title{Order records best first}
\usage{
tt_rank(question, input, options = tt_options(), deadline_ms = NULL,
  completion = NULL)
}
\arguments{
\item{question}{Literal wording as one string, or an authored question set for
the ranking reading, or a \code{\link{tt_question}} selector for a question
file, a saved name, or a reference.}
\item{input}{A record vector, list, or data frame; literal text and JSON
evidence become one record. A data frame converts by row. An explicit
\code{\link{tt_input}} or \code{\link{tt_files}} selector also serves. Missing
atomic records are refused.}
\item{options}{Request controls from \code{\link{tt_options}}; an empty list
keeps the native defaults. \code{top} retains only the requested number of
ranked rows.}
\item{deadline_ms}{A whole-number millisecond deadline for this call, or
\code{NULL} for the native default.}
\item{completion}{A \code{\link{tt_completion}} handle that retains settlement
evidence for this call.}
}
\value{
A carrier of class \code{thinkthen_Call} with \code{$value},
\code{$probability}, complete native rows in \code{$results}, and actual
settlement evidence in \code{$facts}. The value is a list of zero-based original record indexes, best first. Probability is a list aligned with that retained order; score-based ranking can have no probability for a row.
Atomic columns retain \code{$positions} and \code{$length} to identify
missing slots without changing native record identities. Generated optional
fields use \code{thinkthen_absent} for absence and \code{NULL} for explicit null.
Bounded feeds join into these same call fields.
}
\description{
Orders the records best first. Rank orders by yes probability under a decide
question, by a saved score value, or by turns across an ordered decide set.
}
\details{
Native admission owns every rule for the question, the records, and the
controls. Unlike the primitive calls, a rank refuses missing atomic records
rather than skipping their slots.

A native failure raises a \code{thinkthen_error} condition with \code{$kind},
\code{$retryable}, and, once settled, the complete error carrier in
\code{$complete}. R interruption cancels the native work and raises R's own
interrupt condition. Native indexes are zero-based; generated optional fields
are \code{thinkthen_absent} when missing and \code{NULL} when explicitly null.
}
\examples{
\dontrun{
question <- "Is this urgent?"
inbox <- c(
  paste0(
    "Newsletter: our autumn catalog is here. ",
    "No reply needed."
  ),
  "Our checkout page is down and customers cannot pay",
  "Reminder: your invoice is due in 30 days",
  "Please send the signed quote by 5 pm today"
)
order <- unlist(
  tt_rank(question, inbox)$value,
  use.names = FALSE
)
by_urgency <- inbox[order + 1]
stopifnot(identical(by_urgency, inbox[c(2, 4, 3, 1)]))
}
}
\seealso{
\code{\link{tt_decide}} for the shared call behavior, result views, and
failure conditions, and \code{\link{tt_filter}} for keeping instead of
ordering.
}
