\name{tt_feed}
\alias{tt_feed}
\title{Supply records through an R producer}
\usage{
tt_feed(next_item = NULL, name = "records",
  close = function() invisible(NULL), framing = NULL, reading = NULL,
  images = NULL)

\method{print}{thinkthen_feed}(x, \dots)
}
\arguments{
\item{next_item}{A function returning the next record as
\code{\link{tt_record}} output, ordinary evidence, or \code{NULL} at end of
input.}
\item{name}{The feed's name.}
\item{close}{A callback that releases the producer's reader; it runs at input
end, failure, cancellation, or garbage collection.}
\item{framing}{The feed's framing: \code{NULL} for composed records, or
\code{"jsonl"}, \code{"lines"}, \code{"csv"}, or \code{"tsv"} for raw
framed text.}
\item{reading}{Native reading settings for the feed.}
\item{images}{Image declarations for the feed.}
\item{x}{A feed.}
\item{\dots}{Further arguments passed to or from other print methods;
ignored.}
}
\value{
A feed of class \code{thinkthen_feed}, ready for any named call's
\code{input} argument or \code{\link{tt_batch}}'s.
}
\description{
Builds a bounded record producer that runs only on R's thread. A named call
that receives a feed joins its result packets into a carrier that holds only
\code{$results} and \code{$facts}, without the answer and column views of
the one-call form.
}
\details{
With \code{framing = "jsonl"} the producer supplies one raw JSON line per
item; with \code{"lines"}, one text line; blank records are decoded, skipped,
and caller locations retained. With \code{"csv"} or \code{"tsv"} the producer
supplies one complete encoded header first, then at most one complete encoded
data row per item; a quoted multiline cell belongs in one item. Native
admission refuses unsupported framing or projection combinations before the
producer is touched. Omitted fields use native defaults. Completion receipts
are unavailable for feed calls and refuse before producer access. A joined
call that fails carries the packets it already pulled as the condition's
\code{$completed} prefix. A feed with
no producer supplies \code{\link{tt_batch}}'s manual intake. Printing
withholds content.
}
\examples{
at <- 0L
messages <- c("Please refund my order.", "When will it arrive?")
feed <- tt_feed(function() {
  at <<- at + 1L
  if (at > length(messages)) return(NULL)
  tt_record(messages[[at]])
})
feed$header$name
}
\seealso{
\code{\link{tt_record}} for record shapes, \code{\link{tt_reader_failure}}
for reporting a failed reader, and \code{\link{tt_batch}} for session control.
}
