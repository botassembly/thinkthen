\name{tt_item}
\alias{tt_item}
\title{Build a record with its own controls}
\usage{
tt_item(original, context = NULL, options = NULL, seed_spans = NULL,
        examples = NULL, images = NULL)
}
\arguments{
\item{original}{The record's evidence, as text, a JSON value, or an input selector.}
\item{context}{Optional context for this record.}
\item{options}{Optional replacement ordered shortlist, as a list of named option records.}
\item{seed_spans}{Optional authored spans.}
\item{examples}{Optional record examples.}
\item{images}{Optional image inputs.}
}
\value{An item of class \code{thinkthen_item}, passed directly to a named call,
in a list of records, or to \code{\link{tt_record}}.}
\description{Keeps a record's evidence and optional controls together. Native admission validates the fields.}
\examples{
item <- tt_item("Refund please", context = "Customer message")
item$context
}
\seealso{\code{\link{tt_input}}, \code{\link{tt_record}}}
