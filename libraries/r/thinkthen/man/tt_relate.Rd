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
A carrier of class \code{thinkthen_Call} with \code{$value},
\code{$probability}, complete native rows in \code{$results}, and actual
settlement evidence in \code{$facts}. The value is a list of typed edges for the complete entity set. Each edge retains its relation, source and target entity views and probability. The call probability is a list of those edge probabilities.
Atomic columns retain \code{$positions} and \code{$length} to identify
missing slots without changing native record identities. Generated optional
fields use \code{thinkthen_absent} for absence and \code{NULL} for explicit null.
Bounded feeds join into these same call fields.
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
entities <- data.frame(
  name = c(
    "Apple",
    "Amazon",
    "iPhone",
    "Kindle"
  ),
  kind = c("company", "company", "product", "product")
)
makes <- list(
  name = "makes",
  source = "company",
  target = "product"
)
who_makes <- tt_relate(
  list(version = 1, relate = list(relations = list(makes))),
  entities
)$value
companies <- vapply(who_makes, function(edge) edge$source$name, "")
products <- vapply(who_makes, function(edge) edge$target$name, "")
stopifnot(
  identical(companies, c("Apple", "Amazon")),
  identical(products, c("iPhone", "Kindle"))
)
}
}
\seealso{
\code{\link{tt_relate_question}} for building the relation plan,
\code{\link{tt_decide}} for the shared call behavior, result views, and
failure conditions, and \code{\link{tt_recognize}} for relations inside one
text.
}
