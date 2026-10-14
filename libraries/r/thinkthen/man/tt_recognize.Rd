\name{tt_recognize}
\alias{tt_recognize}
\title{Find every name in one text and give each one a kind}
\usage{
tt_recognize(question, input, options = tt_options(), deadline_ms = NULL,
  completion = NULL)
}
\arguments{
\item{question}{A recognition question built by
\code{\link{tt_recognize_question}}, or an ordinary authored recognition
question list, or a \code{\link{tt_question}} selector for the file or saved
name that holds one.}
\item{input}{The text to read: literal text, JSON evidence, a record vector,
list, or data frame, or an explicit \code{\link{tt_input}} or
\code{\link{tt_files}} selector. A data frame converts by row.}
\item{options}{Request controls from \code{\link{tt_options}}; an empty list
keeps the native defaults. \code{relation_threshold} applies only to a
recognition asked for relations.}
\item{deadline_ms}{A whole-number millisecond deadline for this call, or
\code{NULL} for the native default.}
\item{completion}{A \code{\link{tt_completion}} handle that retains settlement
evidence for this call.}
}
\value{
A complete call carrier of class \code{thinkthen_Call}. \code{$results} holds
one typed row of class \code{thinkthen_RecognizeResult} per record, each
with \code{$question}, \code{$meta}, and \code{$answer_id}, and, when the
reading provides them, the zero-based native \code{$index} and
\code{$input}. \code{$answer} is an outer list with one entry per
record, in the same order as \code{$results}. Each entry is that
record's typed answer view: whole recognition gives the entry
\code{$entities}, each carrying its \code{$text}, \code{$kind}, and
zero-based Unicode-scalar \code{$start} and \code{$end} offsets, and,
when the question asks for relations, the recognized \code{$relations}.
A question that selects boundary-only mode gives the entry
\code{$mode} and \code{$proposals} instead, each proposal carrying its
\code{$text}, zero-based \code{$start} and \code{$end}, \code{$length},
and \code{$probability}, with no entities. Read the fields per record,
as in \code{result$answer[[1]]$entities} or
\code{result$answer[[1]]$proposals}. \code{$facts} holds the call's
facts, including \code{requests_sent}.
}
\description{
Reads one text, finds every name in it, and gives each one a kind. A
recognition question can also ask for named relationships between the names it
finds, or ask only for unclassified boundary proposals.
}
\details{
Native admission owns every rule for the question, the text, and the controls.
Entity spans are inclusive-start, exclusive-end offsets in Unicode scalars;
add one to a start offset for \code{substr}. A \code{mode} selection of
\code{"boundary_only"} returns decoded boundary proposals with no kinds or
entities; \code{"whole"} recognition returns the entities.

A native failure raises a \code{thinkthen_error} condition with \code{$kind},
\code{$retryable}, and, once settled, the complete error carrier in
\code{$complete}. R interruption cancels the native work and raises R's own
interrupt condition. Native indexes and offsets are zero-based; generated
optional fields are \code{thinkthen_absent} when missing and \code{NULL} when
explicitly null.
}
\examples{
\dontrun{
recognition <- tt_recognize_question(kinds = list(person = NULL, organization = NULL))
names <- tt_recognize(recognition, "Maria Chen joined Northwind Freight.")
names$answer[[1]]$entities[[1]]$text
}
}
\seealso{
\code{\link{tt_recognize_question}} for building the question,
\code{\link{tt_decide}} for the shared call behavior, result views, and
failure conditions, and \code{\link{tt_relate}} for relations over a complete
entity set.
}
