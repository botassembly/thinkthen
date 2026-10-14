\name{tt_plan}
\alias{tt_plan}
\title{Preview a call without sending}
\usage{
tt_plan(question, input, function_name = "decide", options = list())
}
\arguments{
\item{question}{The question the named function will ask: literal wording, an
authored question list, or a \code{\link{tt_question}} selector.}
\item{input}{The evidence or records the call would send, in any form the
named call takes.}
\item{function_name}{The named judging function to preview: \code{"decide"},
\code{"choose"}, \code{"tag"}, \code{"score"}, \code{"filter"}, or
\code{"rank"}; every other function refuses.}
\item{options}{Request controls from \code{\link{tt_options}}; an empty list
keeps the native defaults.}
}
\value{
A plan carrier of class \code{thinkthen_Plan} with \code{$records} and
\code{$requests} counts, \code{$estimated_bytes}, \code{$estimated_input_tokens}
(a list with \code{lower} and \code{upper} bounds), \code{$first_body_utf8} (the
first request body the call would send), \code{$largest_request_bytes},
\code{$largest_request_estimated_input_tokens}, \code{$token_estimate_method}, and \code{$upper_bound}, which is
\code{TRUE} when the request count is an upper bound because later requests
depend on answers; the byte estimate counts only the prepared requests.
}
\description{
Runs native preview admission for the \code{decide}, \code{choose},
\code{tag}, \code{score}, \code{filter}, and \code{rank} functions and
reports what a call would send; every other function refuses. It sends no
request and reads no key, cache, or answer, but it composes and reads the
call's selected explicit sources to build its estimate.
}
\details{
A plan shares explicit batch and context settings with the actual call, and
its first body is the body that call sends. The estimates bound the request,
not the model's own token count. Printing withholds content.
}
\examples{
\dontrun{
plan <- tt_plan("Q?", c("alpha", "beta"), options = list(batch = 1L))
plan$requests
}
}
\seealso{
\code{\link{tt_decide}} for the call this previews and
\code{\link{tt_options}} for the controls.
}
