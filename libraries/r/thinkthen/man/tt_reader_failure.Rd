\name{tt_reader_failure}
\alias{tt_reader_failure}
\title{Report a failed reader}
\usage{
tt_reader_failure(kind, location = NULL)

\method{print}{thinkthen_reader_failure}(x, \dots)
}
\arguments{
\item{kind}{The failure's kind: \code{"io"}, \code{"utf8"}, or
\code{"invalid_input"}.}
\item{location}{The failure's optional physical location.}
\item{x}{A reader failure.}
\item{\dots}{Further arguments passed to or from other print methods;
ignored.}
}
\value{
A reader failure of class \code{thinkthen_reader_failure}, ready for a
producer's next item.
}
\description{
Builds the explicit failure a producer returns when its reader fails. It
finishes the feed's input with a native failure.
}
\details{
The completed prefix and final facts are retained: a named call joined on the
feed raises a \code{thinkthen_error} condition that carries its completed
results, and a stream settles the failure after already-sent work ends.
Printing withholds content.
}
\examples{
tt_reader_failure("io")
}
\seealso{
\code{\link{tt_feed}} for the producer that returns it and
\code{\link{tt_record}} for the record shape.
}
