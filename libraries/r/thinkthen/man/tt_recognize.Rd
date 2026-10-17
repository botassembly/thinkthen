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
A carrier of class \code{thinkthen_Call} with \code{$value},
\code{$probability}, complete native rows in \code{$results}, and actual
settlement evidence in \code{$facts}. Scalar input returns an entity carrier, or a boundary-proposal carrier in boundary-only mode. Record input returns a list of those carriers. Probability contains aligned lists from retained span proposals or boundary proposals; entity strength is a separate field. Complete rows retain relations when requested.
Atomic columns retain \code{$positions} and \code{$length} to identify
missing slots without changing native record identities. Generated optional
fields use \code{thinkthen_absent} for absence and \code{NULL} for explicit null.
Bounded feeds join into these same call fields.
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
kinds <- list(
  person = NULL,
  product = NULL,
  place = NULL
)
text <- paste0(
  "Sam Patel says his Kindle arrived broken ",
  "in Denver."
)
facts <- tt_recognize(
  list(version = 1, recognize = list(kinds = kinds)),
  text
)$value
entity_names <- vapply(facts$entities, `[[`, "", "text")
found_kinds <- vapply(facts$entities, `[[`, "", "kind")
stopifnot(
  identical(
    entity_names,
    c("Sam Patel", "Kindle", "Denver")
  ),
  identical(found_kinds, names(kinds))
)
}
}
\seealso{
\code{\link{tt_recognize_question}} for building the question,
\code{\link{tt_decide}} for the shared call behavior, result views, and
failure conditions, and \code{\link{tt_relate}} for relations over a complete
entity set.
}
