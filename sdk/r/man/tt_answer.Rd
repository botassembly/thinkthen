\name{tt_answer}
\alias{tt_answer}
\alias{tt_bind}
\title{Project and attach answers}
\description{Project answers from existing rows, or attach outcome columns to a copy of the original data by position. Failed answers emit a classed warning.}
\usage{
tt_answer(rows)
tt_bind(data, rows, prefix)
}
\arguments{
\item{rows}{Result rows returned by a ThinkThen call.}
\item{data}{Original data frame, tibble, grouped tibble or data.table.}
\item{prefix}{Prefix for attached result columns. Annotation uses question names.}
}
\value{\code{tt_answer()} returns the typed answer vector or list column. \code{tt_bind()} returns a copy of the input table with outcome columns and its original table class.}
\seealso{\code{\link{thinkthen-package}}}
