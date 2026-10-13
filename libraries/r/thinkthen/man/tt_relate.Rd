\name{tt_relate}
\alias{tt_relate}
\title{Find named relationships in one complete entity set}
\usage{
tt_relate(question, input, options = tt_options(), deadline_ms = NULL,
  completion = NULL)
}
\arguments{
\item{question}{A relation plan built by \code{\link{tt_relate_question}}, or
an ordinary authored relation question list, or a \code{\link{tt_question}}
selector for the file or saved name that holds one.}
\item{input}{The complete entity set: a record vector, list, or data frame
whose records carry the fields that identify each entity, such as
\code{name} and \code{kind}; literal text and JSON evidence become one record.
A data frame converts by row. An explicit \code{\link{tt_input}} or
\code{\link{tt_files}} selector also serves. Missing atomic records are
refused.}
\item{options}{Request controls from \code{\link{tt_options}}; an empty list
keeps the native defaults.}
\item{deadline_ms}{A whole-number millisecond deadline for this call, or
\code{NULL} for the native default.}
\item{completion}{A \code{\link{tt_completion}} handle that retains settlement
evidence for this call.}
}
\value{
A complete call carrier of class \code{thinkthen_Call}. \code{$results} holds
one typed row of class \code{thinkthen_RelateResult} for the whole set, with
\code{$input_sources} naming where each entity came from, \code{$question},
\code{$meta}, and \code{$answer_id}. \code{$answer} holds the typed answer
view: a list of edges, each with the relation's \code{$name}, its
\code{$source} and \code{$target} entity views, and its \code{$probability}.
\code{$facts} holds the call's facts, including \code{requests_sent}.
}
\description{
Finds the named relationships that hold in one complete entity set, under the
relation rules the question declares.
}
\details{
Native admission owns every rule for the relation plan, the entity set, and
the controls. A relation rule names its source and target kinds; a left-out
source or target means any kind. Unlike the primitive calls, a relate refuses
missing atomic records rather than skipping their slots.

A native failure raises a \code{thinkthen_error} condition with \code{$kind},
\code{$retryable}, and, once settled, the complete error carrier in
\code{$complete}. R interruption cancels the native work and raises R's own
interrupt condition. Native indexes are zero-based; generated optional fields
are \code{thinkthen_absent} when missing and \code{NULL} when explicitly null.
}
\examples{
\dontrun{
relation <- tt_relate_question(relations = list(list(name = "knows",
                                                     source = "person", target = "person")))
edges <- tt_relate(relation, list(list(name = "Ada", kind = "person"),
                                  list(name = "Bo", kind = "person")))$answer
edges[[1]]$source$name
}
}
\seealso{
\code{\link{tt_relate_question}} for building the relation plan,
\code{\link{tt_decide}} for the shared call behavior, result views, and
failure conditions, and \code{\link{tt_recognize}} for relations inside one
text.
}
