\name{tt_options}
\alias{tt_options}
\title{Build named request controls}
\usage{
tt_options(attempts = NULL, batch = NULL, context = NULL,
  context_field = NULL, deadline_ms = NULL, details = NULL,
  examples = NULL, examples_field = NULL, field = NULL,
  files_only = NULL, max_requests_total = NULL, mode = NULL,
  model = NULL, none = NULL, options_field = NULL,
  relation_threshold = NULL, seed_spans = NULL, seed_spans_field = NULL,
  snippet_pieces = NULL, stage_context = NULL, threshold = NULL,
  top = NULL)
}
\arguments{
\item{attempts}{Collect observed attempts.}
\item{batch}{Native batch scheduling: a positive whole number of records per
request, or \code{"max"}.}
\item{context}{Shared context fallback, including explicit empty text.}
\item{context_field}{Per-record context projection pointer.}
\item{deadline_ms}{The millisecond deadline for the call, including \code{-1}
for no deadline.}
\item{details}{Compatibility details output for scalar calls.}
\item{examples, examples_field}{Recognition boundary examples, or a per-record
example projection pointer.}
\item{field}{Evidence projection pointers, in caller order.}
\item{files_only}{Keep one selected record per physical file.}
\item{max_requests_total}{Process send budget for this call.}
\item{mode}{Explicit recognition stage selection.}
\item{model}{Explicit model on the engine's configured route.}
\item{none}{Offer a none candidate in find.}
\item{options_field}{Per-record ordered choose shortlist projection pointer.}
\item{relation_threshold}{Authored relation cut, applicable only to whole
recognition.}
\item{seed_spans, seed_spans_field}{Unconfirmed recognition proposals with
exact piece edges, or a per-record seed projection pointer.}
\item{snippet_pieces}{Recognition tokenizer pieces on each side; omission
retains the saved width.}
\item{stage_context}{Literal recognition context overrides, resolved member by
member.}
\item{threshold}{Explicit reading override.}
\item{top}{Retain the requested number of ranked rows.}
}
\value{
A named list holding only the controls actually supplied, ready for any named
call's \code{options} argument.
}
\description{
Builds the named request controls one call can carry. Native admission
validates every value and rejects a control the call's function does not take.
}
\details{
A control left out keeps the native default; an explicit \code{NULL} value
refuses at admission. The controls travel beside the question, not inside it;
the question's own authored settings keep their precedence. An empty
\code{tt_options()} is the same as an empty list.
}
\examples{
tt_options(threshold = 0.8, batch = 1L)
}
\seealso{
\code{\link{tt_decide}} for the calls that take these controls and
\code{\link{tt_engine}} for the session's engine settings.
}
