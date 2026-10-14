\name{tt_annotate}
\alias{tt_annotate}
\title{Ask a saved question set}
\usage{
tt_annotate(question, input, options = tt_options(), deadline_ms = NULL,
  completion = NULL)
}
\arguments{
\item{question}{An authored question set: an ordinary named list with
\code{version} and \code{questions}, each entry one named question for a
member of the set, such as
\code{list(version = 1L, questions = list(approved = list(decide = "Approved?")))},
or a \code{\link{tt_question}} selector for the file or saved name that holds
the set.}
\item{input}{The records to annotate: a record vector, list, or data frame;
literal text and JSON evidence become one record. A data frame converts by
row. An explicit \code{\link{tt_input}} or \code{\link{tt_files}} selector
also serves. Missing atomic records are refused.}
\item{options}{Request controls from \code{\link{tt_options}}; an empty list
keeps the native defaults.}
\item{deadline_ms}{A whole-number millisecond deadline for this call, or
\code{NULL} for the native default.}
\item{completion}{A \code{\link{tt_completion}} handle that retains settlement
evidence for this call.}
}
\value{
A carrier of class \code{thinkthen_Call} with \code{$value},
\code{$probability}, complete native rows in \code{$results}, and actual
settlement evidence in \code{$facts}. The value is a list of named annotation fields, one list per record. Probability is NULL. Complete rows retain each field's native answer.
Atomic columns retain \code{$positions} and \code{$length} to identify
missing slots without changing native record identities. Generated optional
fields use \code{thinkthen_absent} for absence and \code{NULL} for explicit null.
Bounded feeds join into these same call fields.
}
\description{
Asks a saved question set about each record and adds one typed field per
question to it.
}
\details{
Native admission owns every rule for the question set, the records, and the
controls. Annotate question set entries take no batch. Unlike the primitive
calls, an annotate refuses missing atomic records rather than skipping their
slots.

A native failure raises a \code{thinkthen_error} condition with \code{$kind},
\code{$retryable}, and, once settled, the complete error carrier in
\code{$complete}. R interruption cancels the native work and raises R's own
interrupt condition. Native indexes are zero-based; generated optional fields
are \code{thinkthen_absent} when missing and \code{NULL} when explicitly null.
}
\examples{
\dontrun{
form <- tt_question(file = "form.json")
report <- "Steps: click Log in. Nobody gets in."
triage <- tt_annotate(form, report)$value[[1]]
stopifnot(
  identical(triage$steps, TRUE),
  identical(triage$area, "login"),
  identical(triage$impact, 1.98)
)
}
}
\seealso{
\code{\link{tt_decide}} for the shared call behavior, result views, and
failure conditions.
}
