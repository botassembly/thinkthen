\name{tt_input}
\alias{tt_input}
\title{Mark an explicit input descriptor}
\usage{
tt_input(kind, ...)

\method{print}{thinkthen_input}(x, \dots)
}
\arguments{
\item{kind}{The canonical input kind name.}
\item{\dots}{Named fields of the canonical input descriptor, passed through as
written.}
\item{x}{An input selector.}
}
\value{
An input selector of class \code{thinkthen_input}, ready for any named call's
\code{input} argument.
}
\description{
Builds an explicit canonical input descriptor without validating it in R.
Native admission validates the descriptor when the call runs.
}
\details{
Use this for shapes the ordinary conversions cannot name: ordered items with
per-record context or options, image attachments, or a single JSON array that
should remain one original. An ordinary \code{NULL} passed as \code{input}
supplies an empty record collection; \code{tt_input("json", value = NULL)}
retains an explicit JSON null instead. \code{\link{tt_files}} and
\code{\link{tt_feed}} headers also carry this class. Printing withholds
content.
}
\examples{
tt_input("json", value = NULL)
tt_input("records", items = list(list(
  original = list(kind = "json", value = list(body = "Please refund my order.")),
  context = list(queue = "support")
)))
}
\seealso{
\code{\link{tt_files}} for explicit native file reading and
\code{\link{tt_decide}} for the calls that take an input.
}
