\name{tt_tag}
\alias{tt_tag}
\title{Return every applicable label}
\usage{
tt_tag(question, input, options = tt_options(), deadline_ms = NULL,
  completion = NULL)
}
\arguments{
\item{question}{Literal wording as one string, or an authored question list
carrying the question text and its declared labels, such as
\code{list(tag = "Which labels?", labels = c("refund", "shipping"))}, or a
\code{\link{tt_question}} selector for a question file, a saved name, or a
reference.}
\item{input}{Evidence for the call: literal text, JSON evidence, a record
vector, list, or data frame, or an explicit \code{\link{tt_input}} or
\code{\link{tt_files}} selector. A data frame converts by row.}
\item{options}{Request controls from \code{\link{tt_options}}; an empty list
keeps the native defaults.}
\item{deadline_ms}{A whole-number millisecond deadline for this call, or
\code{NULL} for the native default.}
\item{completion}{A \code{\link{tt_completion}} handle that retains settlement
evidence for this call.}
}
\value{
A carrier of class \code{thinkthen_Call} with \code{$value},
\code{$probability}, complete native rows in \code{$results}, and actual
settlement evidence in \code{$facts}. The value is a list of character vectors, one per record. Probability is NULL.
Atomic columns retain \code{$positions} and \code{$length} to identify
missing slots without changing native record identities. Generated optional
fields use \code{thinkthen_absent} for absence and \code{NULL} for explicit null.
Bounded feeds join into these same call fields.
}
\description{
Asks one question whose answer is any number of labels from the declared
label list, and returns every applicable label for each record.
}
\details{
Native admission owns every rule for the question, the records, and the
controls. The labels belong to the authored question; each label is read under
the same single cut, and an authored question may also carry projection
pointers and a batch size.

A native failure raises a \code{thinkthen_error} condition with \code{$kind},
\code{$retryable}, and, once settled, the complete error carrier in
\code{$complete}. R interruption cancels the native work and raises R's own
interrupt condition. Native indexes are zero-based; generated optional fields
are \code{thinkthen_absent} when missing and \code{NULL} when explicitly null.
}
\examples{
\dontrun{
question <- "Which labels fit this message?"
labels <- c("praise", "bug", "billing")
message <- paste0(
  "Love the new dashboard, but export crashes the app,\n",
  "and I was charged twice.\n"
)
fitting_labels <- tt_tag(
  list(tag = question, labels = labels),
  message
)$value[[1]]
stopifnot(identical(
  fitting_labels,
  c("praise", "bug", "billing")
))
}
}
\seealso{
\code{\link{tt_decide}} for the shared call behavior, result views, and
failure conditions.
}
