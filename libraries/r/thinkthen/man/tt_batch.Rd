\name{tt_batch}
\alias{tt_batch}
\title{Own a native request session}
\usage{
tt_batch(function_name, question, input, options = list())

\method{print}{thinkthen_batch}(x, \dots)
}
\arguments{
\item{function_name}{The named judging function, such as \code{"decide"},
\code{"choose"}, \code{"filter"}, or \code{"relate"}.}
\item{question}{The question that function asks: literal wording, an authored
question list, or a \code{\link{tt_question}} selector.}
\item{input}{The session's input: records in any form the named call takes, or
a \code{\link{tt_feed}} producer for bounded intake, with \code{tt_feed()} and
no producer for manual pushing.}
\item{options}{Request controls from \code{\link{tt_options}}; an empty list
keeps the native defaults. A session's deadline and attempt controls belong
here, not on the individual members.}
\item{x}{A batch handle.}
\item{\dots}{Further arguments passed to or from other print methods;
ignored.}
}
\value{
A batch handle of class \code{thinkthen_batch} with the member functions
\code{$next_result()}, which blocks in R's synchronous idiom while checking R
interrupts; \code{$poll()}, which returns a ready result packet or
\code{NULL}; \code{$push(record)} and \code{$finish(failure = NULL)} for
manual intake; \code{$cancel()}; \code{$close()}; and \code{$facts()}, which
returns the terminal facts after settlement. The handle also carries
\code{$positions} and \code{$length} for column input. Dropping the last
handle cancels through native finalization.
}
\description{
Starts one native request session for a named function and returns a handle
that drives it result by result.
}
\details{
\code{$push(record)} returns \code{"accepted"}, \code{"full"}, or
\code{"closed"}; retry the same record after polling when full, and stop
advancing intake when closed. A record takes the same shapes as
\code{\link{tt_record}}'s \code{original} or a whole \code{tt_record}. A
producer from \code{\link{tt_feed}} feeds the session on R's thread; the
handle retains one pending record while the native input cell is full. A
native failure raises a \code{thinkthen_error} condition that retains the
completed prefix and final facts when they exist.
}
\examples{
\dontrun{
stream <- tt_batch("decide", "Q?", c("first", "second"), options = list(batch = 1L))
repeat {
  value <- stream$next_result()
  if (is.null(value)) break
}
}
}
\seealso{
\code{\link{tt_feed}} for a producer, \code{\link{tt_record}} for record
shapes, and \code{\link{tt_decide}} for the one-call form.
}
