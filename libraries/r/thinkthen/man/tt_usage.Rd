\name{tt_usage}
\alias{tt_usage}
\title{Report the session's running usage totals}
\usage{
tt_usage()}
\value{
A named list with \code{requests_sent}, \code{retries}, \code{input_tokens},
\code{output_tokens}, and \code{cache_answers}.
}
\description{
Reports the selected engine's usage totals for this session.
}
\details{
The totals cover one engine since it was built, or since the process was
forked. Failed calls and retries count, and nothing resets them. A fork keeps
the parent's totals and each side then counts alone.
}
\examples{
\dontrun{
tt_usage()$requests_sent
}
}
\seealso{
\code{\link{tt_engine}} for selecting the engine these totals describe and
\code{\link{tt_usage_persistence}} for its durable usage.
}
