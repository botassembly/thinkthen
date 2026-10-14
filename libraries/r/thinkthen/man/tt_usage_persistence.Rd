\name{tt_usage_persistence}
\alias{tt_usage_persistence}
\title{Observe live usage durability}
\usage{
tt_usage_persistence()
}
\value{A \code{thinkthen_UsageStatus} named list with the native \code{state} and optional fixed \code{advice}.}
\description{Observes the existing selected engine's usage durability without sending a model request. Observation does not wait for the writer.}
\details{Written covers only this engine's current deltas. Failure leaves answers and in-memory counts usable but durable totals incomplete. The result is a live observation without a facts-snapshot timestamp, credentials or filesystem paths.}
\seealso{
\code{\link{tt_finish_usage_status}} for draining current deltas and
\code{\link{tt_usage}} for the in-memory totals.
}
