\name{tt_rule}
\alias{tt_rule}
\title{Construct a relation rule}
\description{Describe a relation between source and target entity kinds. Combine rules with \code{vctrs::list_of()} for relation questions.}
\usage{
tt_rule(source, target, relation, ..., reads = NULL, either = FALSE)
}
\arguments{
\item{source}{Source entity kind.}
\item{target}{Target entity kind.}
\item{relation}{Relation name.}
\item{...}{Named options passed to the engine. Vector helpers pass all arguments to the corresponding row function.}
\item{reads}{Optional verb wording used in the relation question.}
\item{either}{Whether either endpoint direction is accepted.}
}
\value{A size-one \code{tt_rule} record with the admitted relation fields.}
\seealso{\code{\link{thinkthen-package}}}
