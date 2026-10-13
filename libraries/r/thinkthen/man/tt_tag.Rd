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
A complete call carrier of class \code{thinkthen_Call}. \code{$results} holds
one typed row of class \code{thinkthen_TagResult} per record, each with
\code{$value} (the applicable labels as an R list of strings),
\code{$question}, \code{$meta}, and \code{$answer_id}, and, when the
reading provides them, the zero-based native \code{$index} and
\code{$input}. \code{$answer} holds the typed answer view: a list with one
label list per record for explicit selectors and structured inputs; for an
ordinary atomic column it is a list of character vectors, one per record,
with a missing atomic slot retaining an empty character vector.
\code{$facts} holds the
call's facts, including \code{requests_sent}. For a column the carrier also
carries \code{$positions} and \code{$length} beside those views. A feed
input returns a carrier with \code{$results} and \code{$facts} only,
without the answer and column views.
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
tags <- tt_tag(list(tag = "Which labels?", labels = c("refund", "shipping")),
               "Refund shipping")
tags$answer[[1]]
}
}
\seealso{
\code{\link{tt_decide}} for the shared call behavior, result views, and
failure conditions.
}
