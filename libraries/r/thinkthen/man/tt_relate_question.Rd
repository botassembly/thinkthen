\name{tt_relate_question}
\alias{tt_relate_question}
\title{Build a relation plan}
\usage{
tt_relate_question(fields = NULL, relations = NULL)
}
\arguments{
\item{fields}{A named list with \code{name} and \code{kind} naming the record
fields that identify each entity in the set.}
\item{relations}{The relation rules, each a list with \code{name} and optional
\code{source} and \code{target} kinds; a left-out kind matches any kind.}
}
\value{
A question selector of class \code{thinkthen_question}, ready for
\code{\link{tt_relate}}'s \code{question} argument.
}
\description{
Builds a relation plan: the ordered relation rules and the record fields that
identify each entity in one complete entity set.
}
\details{
Each argument supplies one part of the relation question; a part left out is
left out of the question, and native admission validates the combination.
\code{\link{tt_relate}} owns what the built plan asks.
}
\examples{
tt_relate_question(relations = list(list(name = "knows", source = "person",
                                         target = "person")))
}
\seealso{
\code{\link{tt_relate}} for the call that takes this plan and
\code{\link{tt_question}} for the selector class.
}
