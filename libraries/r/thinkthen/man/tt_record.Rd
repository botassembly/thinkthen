\name{tt_record}
\alias{tt_record}
\title{Compose one record for a producer}
\usage{
tt_record(original, location = NULL, ...)

\method{print}{thinkthen_record}(x, \dots)
}
\arguments{
\item{original}{The record's original: literal text, JSON evidence, or an
explicit \code{\link{tt_input}} selector. \code{tt_record(NULL)} supplies a
present JSON null.}
\item{location}{The record's physical location, such as
\code{list(file = "rows.jsonl", first_line = 1L, last_line = 1L)}.}
\item{\dots}{For \code{tt_record}, additional record fields such as
\code{context}, \code{options}, and \code{images}, passed to native admission
as written; for the print method, further arguments, ignored.}
\item{x}{A record.}
}
\value{
A record of class \code{thinkthen_record}, ready for a producer's next item or
a session's \code{$push()}.
}
\description{
Composes one record with its original, optional physical location, and
optional per-record fields.
}
\details{
A producer returns this value, ordinary evidence, or \code{NULL} at end of
input. The original reaches native admission unchanged; per-record context and
options reach it beside it. Printing withholds content.
}
\examples{
tt_record("Please refund my order.")
tt_record("Later", location = list(file = "rows.jsonl", first_line = 4L,
                                   last_line = 4L))
}
\seealso{
\code{\link{tt_feed}} for the producer that returns records and
\code{\link{tt_reader_failure}} for reporting a failed reader.
}
