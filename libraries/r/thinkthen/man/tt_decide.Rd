\name{tt_decide}
\alias{tt_decide}
\title{Answer a yes, no, or not-sure question}
\usage{
tt_decide(question, input, options = tt_options(), deadline_ms = NULL,
  completion = NULL)

\method{print}{thinkthen_complete}(x, \dots)

\method{print}{thinkthen_identity}(x, \dots)
}
\arguments{
\item{question}{Literal wording as one string, or an ordinary authored
question list such as
\code{list(decide = "Does this ask for a refund?", true = "Yes means refund",
false = "No means no refund")}, or a \code{\link{tt_question}} selector for a
question file, a saved name, or a reference.}
\item{input}{Evidence for the call: literal text, JSON evidence, a record
vector, list, or data frame, or an explicit \code{\link{tt_input}} or
\code{\link{tt_files}} selector. A data frame converts by row.}
\item{options}{Request controls from \code{\link{tt_options}}; an empty list
keeps the native defaults.}
\item{deadline_ms}{A whole-number millisecond deadline for this call, or
\code{NULL} for the native default.}
\item{completion}{A \code{\link{tt_completion}} handle that retains settlement
evidence for this call.}
\item{x}{A complete call carrier or a generated identity such as a call ID, an
answer ID, or a digest.}
\item{\dots}{Further arguments passed to or from other print methods;
ignored.}
}
\value{
A complete call carrier of class \code{thinkthen_Call}. \code{$results} holds
one typed row of class \code{thinkthen_DecideResult} per record, each with
\code{$value} (\code{TRUE} or \code{FALSE}, \code{NULL} when the answer is
not sure, or the question's authored reading for the side it landed on when it
declares one),
\code{$question}, \code{$meta}, and \code{$answer_id}, and, when the reading
provides them, the zero-based native \code{$index} and \code{$input}.
\code{$answer} holds the typed answer view; for an
ordinary atomic column it is a plain logical column whose not-sure cells are
\code{NA}, with \code{$value} and
\code{$probability} beside it filling missing slots with typed missing values.
\code{$facts} holds the call's facts, including \code{requests_sent}. For a
column the carrier also carries \code{$positions}, which maps compact native
record indexes to original zero-based slots, and \code{$length}, the full
column length. A feed input returns a carrier with \code{$results} and
\code{$facts} only, without the answer and column views.
}
\description{
Asks one yes-or-no question about each record and returns a typed answer per
record: \code{TRUE}, \code{FALSE}, or not sure, which a typed row carries as
\code{NULL} and an ordinary atomic column carries as \code{NA}.
}
\details{
Native admission owns every rule for the question, the records, and the
controls. A scalar character input is literal text; other scalars and
structured values become JSON evidence. An authored question carries the
question text beside what a yes and a no each mean, and may carry a threshold,
projection pointers, and a batch size. The \code{threshold} control sets the
cut or band the answer is read under.

A native failure raises a \code{thinkthen_error} condition with \code{$kind},
\code{$retryable}, and, once settled, the complete error carrier in
\code{$complete}. R interruption cancels the native work and raises R's own
interrupt condition. Native indexes and span offsets are zero-based; generated
optional fields are \code{thinkthen_absent} when missing and \code{NULL} when
explicitly null. Every named call returns a complete carrier whose generated
rows and identities print summaries that withhold content.
}
\examples{
\dontrun{
call <- tt_decide("Does this ask for a refund?", "Please refund my order.")
call$results[[1]]$value
}
}
\seealso{
\code{\link{tt_choose}}, \code{\link{tt_tag}}, \code{\link{tt_score}},
\code{\link{tt_filter}}, \code{\link{tt_rank}}, \code{\link{tt_find}},
\code{\link{tt_annotate}}, \code{\link{tt_recognize}}, and
\code{\link{tt_relate}} for the other named calls.
}
