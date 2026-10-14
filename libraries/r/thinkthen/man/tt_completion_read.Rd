\name{tt_completion_read}
\alias{tt_completion_read}
\title{Read settled completion evidence}
\usage{
tt_completion_read(handle)}
\arguments{
\item{handle}{A completion handle from \code{\link{tt_completion}}.}
}
\value{
An ordinary named list with \code{$state}, one of \code{"unused"},
\code{"claimed"}, \code{"running"}, or \code{"terminal"}. A terminal receipt
also carries \code{$kind} (\code{"success"} or the failure kind), the final
\code{$facts} when they exist, and \code{$details}, the retained question
events with their record indexes. Each read returns independently owned
values.
}
\description{
Reads a completion handle's settlement evidence as ordinary values at any
time.}
\details{
The receipt is an observation of the handle's own state; a call interrupted in
R still settles natively, and reading after that returns the settled
evidence. An admission refusal settles a claimed handle without invented
facts.}
\examples{
\dontrun{
receipt <- tt_completion()
answer <- tt_decide("Q?", "x", completion = receipt)
settled <- tt_completion_read(receipt)
settled$state
settled$facts$requests_sent
}
}
\seealso{
\code{\link{tt_completion}} for the handle and \code{\link{tt_decide}} for the
calls that accept one.
}
