\name{tt_score}
\alias{tt_score}
\title{Place evidence on named levels}
\usage{
tt_score(question, input, options = tt_options(), deadline_ms = NULL,
  completion = NULL)
}
\arguments{
\item{question}{Literal wording as one string, or an authored question list
carrying the question text and its levels lowest first, such as
\code{list(score = "How urgent?", levels = c("low", "soon", "high"))}, or a
\code{\link{tt_question}} selector for a question file, a saved name, or a
reference.}
\item{input}{Evidence for the call: literal text, JSON evidence, a record
vector, list, or data frame, or an explicit \code{\link{tt_input}} or
\code{\link{tt_files}} selector. A data frame converts by row.}
\item{options}{Request controls from \code{\link{tt_options}}; an empty list
keeps the native defaults. A score takes no threshold.}
\item{deadline_ms}{A whole-number millisecond deadline for this call, or
\code{NULL} for the native default.}
\item{completion}{A \code{\link{tt_completion}} handle that retains settlement
evidence for this call.}
}
\value{
A complete call carrier of class \code{thinkthen_Call}. \code{$results} holds
one typed row of class \code{thinkthen_ScoreResult} per record, each with
\code{$question}, \code{$meta}, and \code{$answer_id}, and, when the reading
provides them, the zero-based native \code{$index} and \code{$input}.
\code{$answer} holds the typed answer view; for an
ordinary atomic column it is a plain double column, with \code{$value} beside
it filling missing slots with typed missing values. \code{$facts} holds the
call's facts, including \code{requests_sent}. For a column the carrier also
carries \code{$positions} and \code{$length} beside those views.
}
\description{
Places each record on the question's named levels, which are declared
lowest first, and returns a number.
}
\details{
Native admission owns every rule for the question, the records, and the
controls. The levels belong to the authored question, and a score takes no
threshold; the answer is a number.

A native failure raises a \code{thinkthen_error} condition with \code{$kind},
\code{$retryable}, and, once settled, the complete error carrier in
\code{$complete}. R interruption cancels the native work and raises R's own
interrupt condition. Native indexes are zero-based; generated optional fields
are \code{thinkthen_absent} when missing and \code{NULL} when explicitly null.
}
\examples{
\dontrun{
urgency <- tt_score(list(score = "How urgent?", levels = c("low", "soon", "high")),
                    "Later")
urgency$answer
}
}
\seealso{
\code{\link{tt_decide}} for the shared call behavior, result views, and
failure conditions.
}
