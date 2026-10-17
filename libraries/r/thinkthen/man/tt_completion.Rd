\name{tt_completion}
\alias{tt_completion}
\title{Retain asynchronous settlement evidence}
\usage{
tt_completion()}
\value{
A fresh completion handle, ready for a named call's \code{completion}
argument.}
\description{
Creates a completion handle that retains settlement evidence for one named
call after the synchronous caller returns or is interrupted.}
\details{
A handle is claimed by the first call that receives it and cannot be reused.
Passing it to a named call settles it whatever the outcome; reading it with
\code{\link{tt_completion_read}} is safe at any time, including while the call
still runs.}
\examples{
\dontrun{
receipt <- tt_completion()
answer <- tt_decide("Q?", "x", completion = receipt)
tt_completion_read(receipt)$state
}
}
\seealso{
\code{\link{tt_completion_read}} for reading the settled evidence and
\code{\link{tt_decide}} for the calls that accept a handle.
}
