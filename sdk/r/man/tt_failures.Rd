\name{tt_failures}
\alias{tt_failures}
\title{List failed records}
\description{List each failure of an errors = "na" result or of any result rows, so a failure can be told apart from not sure.}
\usage{
tt_failures(x)
}
\arguments{
\item{x}{A result of a call with errors = "na", or result rows.}
}
\value{A tibble with one row per failure, sorted by position, with columns \code{position}, \code{id}, \code{question}, \code{error_code}, \code{error_message} and \code{error_http_status}. \code{question} names the annotation question and is \code{NA} otherwise. Positions count the input of the call. A clean result or any other value gives zero rows. Subsetting with \code{[} drops the failures, as base R does.}
\seealso{\code{\link{tt_answer}}, \code{\link{thinkthen-package}}}
