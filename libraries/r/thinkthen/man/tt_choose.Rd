\name{tt_choose}
\alias{tt_choose}
\title{Pick one label from a fixed list}
\usage{
tt_choose(question, input, options = tt_options(), deadline_ms = NULL,
  completion = NULL)
}
\arguments{
\item{question}{Literal wording as one string, or an authored question list
carrying the question text and its fixed option list, such as
\code{list(choose = "Which category?", options = c("refund", "other"))}, or a
\code{\link{tt_question}} selector for a question file, a saved name, or a
reference.}
\item{input}{Evidence for the call: literal text, JSON evidence, a record
vector, list, or data frame, or an explicit \code{\link{tt_input}} or
\code{\link{tt_files}} selector. A data frame converts by row.}
\item{options}{Request controls from \code{\link{tt_options}}; an empty list
keeps the native defaults. \code{options_field} names a per-record ordered
shortlist projection.}
\item{deadline_ms}{A whole-number millisecond deadline for this call, or
\code{NULL} for the native default.}
\item{completion}{A \code{\link{tt_completion}} handle that retains settlement
evidence for this call.}
}
\value{
A carrier of class \code{thinkthen_Call} with \code{$value},
\code{$probability}, complete native rows in \code{$results}, and actual
settlement evidence in \code{$facts}. The value is a selected label or NULL for not sure. Atomic columns return character columns with typed missing slots. Probability follows the same scalar or record shape.
Atomic columns retain \code{$positions} and \code{$length} to identify
missing slots without changing native record identities. Generated optional
fields use \code{thinkthen_absent} for absence and \code{NULL} for explicit null.
Bounded feeds join into these same call fields.
}
\description{
Asks one question whose answer is exactly one label from the fixed option
list the question declares, and returns that label for each record.
}
\details{
Native admission owns every rule for the question, the records, and the
controls. The options belong to the authored question, not to the call's
controls; an authored question may also carry a single threshold cut,
projection pointers, and a batch size.

A native failure raises a \code{thinkthen_error} condition with \code{$kind},
\code{$retryable}, and, once settled, the complete error carrier in
\code{$complete}. R interruption cancels the native work and raises R's own
interrupt condition. Native indexes are zero-based; generated optional fields
are \code{thinkthen_absent} when missing and \code{NULL} when explicitly null.
}
\examples{
\dontrun{
question <- "Which team owns this?"
teams <- list(
  billing = "Invoices, fees, and refunds.",
  shipping = "Parcels and delivery.",
  account = "Logins and passwords."
)
parcel <- "My parcel went to the wrong address."
team <- tt_choose(
  list(choose = question, options = teams),
  parcel
)$value
stopifnot(identical(team, "shipping"))
}
}
\seealso{
\code{\link{tt_decide}} for the shared call behavior, result views, and
failure conditions.
}
