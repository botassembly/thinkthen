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
A complete call carrier of class \code{thinkthen_Call}. \code{$results} holds
one typed row of class \code{thinkthen_RankResult} per ranked position, each
with \code{$question}, \code{$meta}, \code{$answer_id}, \code{$members}
for aggregate readings, and, when the reading provides it, the zero-based
native \code{$index} of the record it places. \code{$answer} holds the
typed answer view: a list of the placed records best first, each with its
native \code{$index}. \code{$facts}
holds the call's facts, including \code{requests_sent}. The carrier also
carries \code{$positions} and \code{$length} beside those views.
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
prioritized <- tt_rank("Which message most clearly asks for a refund?",
                       c("Please refund my order.", "When will it arrive?"))
prioritized$answer[[1]]$index
}
}
\seealso{
\code{\link{tt_decide}} for the shared call behavior, result views, and
failure conditions, and \code{\link{tt_filter}} for keeping instead of
ordering.
}
