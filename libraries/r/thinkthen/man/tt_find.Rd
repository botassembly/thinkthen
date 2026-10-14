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
A complete call carrier of class \code{thinkthen_Call}. \code{$results} holds
one typed row of class \code{thinkthen_FindResult} for the whole set, with
\code{$value} (the picked unit's own value), \code{$answer} (a typed view with
the pick's kind, confidence, and per-candidate probabilities),
\code{$candidates} (one view per unit with its zero-based \code{$index},
\code{$input}, and \code{$probability}), \code{$question}, \code{$meta},
and \code{$answer_id}. \code{$answer} holds the same typed find
view as that single row. \code{$facts} holds the call's facts, including
\code{requests_sent}.
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
picked <- tt_find("Which line?", c("first", "second"))
picked$answer$value
}
}
\seealso{
\code{\link{tt_decide}} for the shared call behavior, result views, and
failure conditions.
}
