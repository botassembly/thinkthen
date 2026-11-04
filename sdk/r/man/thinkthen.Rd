\name{thinkthen-package}
\alias{thinkthen-package}
\alias{thinkthen}
\docType{package}
\title{Ask typed questions about records}
\description{Judge vectors and structured records with admitted questions. Calls return typed answers. The _rows forms retain every outcome.}
\details{Use a named backend with \code{tt_client()}. Required answer domains follow the question. Arguments after dots must be named. Thresholds are numeric cuts or numeric low and high pairs for decide, choose and filter. Field selects a JSON Pointer. An execution name is independent of toolbox selection.

Question constructors admit immutable values. A matching question carries its domain into its call. Toolboxes hold named questions; annotation accepts a toolbox or \code{tt_annotation_questions()} collection. Relation domains accept a single rule or a list of rules.

Results contain position, exact ID, status, typed answer, full-precision probability, source, error fields and typed detail. Spans count Unicode code points from one with an inclusive end. Missing records retain their positions as failed rows without a backend call. Call-level problems raise classed rlang conditions. Interrupts cancel and raise \code{thinkthen_cancelled_error}.

Answer calls raise a condition with all rows on failure. Use errors = "na" for typed missing values. \code{tt_bind()} attaches results by position to a copy of a supported data frame. \code{tt_close()} releases a client. Scoped defaults restore the prior client on every exit.

See \code{\link{tt_decide}} for calls and \code{\link{tt_client}} for clients.}
\examples{
client <- tt_client(backend = "stub")
team <- tt_choose(c("Refund please", "Where is my delivery?"),
                  "Which team?", c("billing", "shipping"), client = client)
stopifnot(is.character(team))
tt_close(client)
}
