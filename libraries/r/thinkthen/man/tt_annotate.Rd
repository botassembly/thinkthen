\name{tt_annotate}
\alias{tt_annotate}
\title{Ask a saved question set}
\usage{
tt_annotate(question, input, options = tt_options(), deadline_ms = NULL,
  completion = NULL)
}
\arguments{
\item{question}{An authored question set: an ordinary named list with
\code{version} and \code{questions}, each entry one named question for a
member of the set, such as
\code{list(version = 1L, questions = list(approved = list(decide = "Approved?")))},
or a \code{\link{tt_question}} selector for the file or saved name that holds
the set.}
\item{input}{The records to annotate: a record vector, list, or data frame;
literal text and JSON evidence become one record. A data frame converts by
row. An explicit \code{\link{tt_input}} or \code{\link{tt_files}} selector
also serves. Missing atomic records are refused.}
\item{options}{Request controls from \code{\link{tt_options}}; an empty list
keeps the native defaults.}
\item{deadline_ms}{A whole-number millisecond deadline for this call, or
\code{NULL} for the native default.}
\item{completion}{A \code{\link{tt_completion}} handle that retains settlement
evidence for this call.}
}
\value{
A complete call carrier of class \code{thinkthen_Call}. \code{$results} holds
one typed row of class \code{thinkthen_AnnotateResult} per record, each with
\code{$value} (one named field per question in the set), \code{$meta}, and
\code{$answer_id}, and, when the reading provides them, the zero-based native
\code{$index} and \code{$input}. \code{$answer} holds the typed answer view: a list of
annotated row views, one per record, with one field per question name.
\code{$facts} holds the call's facts, including \code{requests_sent}. The
carrier also carries \code{$positions} and \code{$length} beside those views. A feed
input returns a carrier with \code{$results} and \code{$facts} only,
without the answer and column views.
}
\description{
Asks a saved question set about each record and adds one typed field per
question to it.
}
\details{
Native admission owns every rule for the question set, the records, and the
controls. Annotate question set entries take no batch. Unlike the primitive
calls, an annotate refuses missing atomic records rather than skipping their
slots.

A native failure raises a \code{thinkthen_error} condition with \code{$kind},
\code{$retryable}, and, once settled, the complete error carrier in
\code{$complete}. R interruption cancels the native work and raises R's own
interrupt condition. Native indexes are zero-based; generated optional fields
are \code{thinkthen_absent} when missing and \code{NULL} when explicitly null.
}
\examples{
\dontrun{
approved <- tt_annotate(list(version = 1L,
                              questions = list(approved = list(decide = "Approved?"))),
                        "Yes")
approved$answer[[1]]$approved
}
}
\seealso{
\code{\link{tt_decide}} for the shared call behavior, result views, and
failure conditions.
}
