\name{tt_answer}
\alias{tt_answer}
\alias{tt_bind}
\title{Project and attach answers}
\description{Project answers from existing rows, or attach outcome columns to a copy of the original data by position. Failed answers raise a condition carrying all rows; errors = "na" returns missing values and warns instead.}
\usage{
tt_answer(rows, errors = "error")
tt_bind(data, rows, prefix)
}
\arguments{
\item{errors}{"error" raises thinkthen_failed_rows with code, codes, rows and failed_rows; its message names every code. "na" returns typed missing values for failed records, signals one thinkthen_failed_rows_warning with the same fields, and keeps the failures for \code{\link{tt_failures}}.}
\item{rows}{Result rows returned by a ThinkThen call.}
\item{data}{Original data frame, tibble, grouped tibble or data.table.}
\item{prefix}{Prefix for attached result columns. Annotation uses question names.}
}
\value{\code{tt_answer()} returns the typed answer vector or list column. \code{tt_bind()} returns a copy of the input table with outcome columns and its original table class.}
\seealso{\code{\link{tt_failures}}, \code{\link{thinkthen-package}}}
