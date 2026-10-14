\name{tt_finish_usage_status}
\alias{tt_finish_usage_status}
\title{Drain usage deltas and report durability}
\usage{
tt_finish_usage_status()
}
\value{A \code{thinkthen_UsageStatus} named list with the native \code{state} and optional fixed \code{advice}.}
\description{Drains the selected engine's current usage deltas to durable storage and reports the live durability state. It sends no model request.}
\details{Draining waits for the writer to finish; only usage-lock acquisition has a deadline, and other filesystem work can take longer. Written covers only this engine's current deltas. Failure leaves answers and in-memory counts usable but durable totals incomplete. The result is a live observation without a facts-snapshot timestamp, credentials or filesystem paths.}
\seealso{
\code{\link{tt_usage_persistence}} for observing without draining and
\code{\link{tt_usage}} for the in-memory totals.
}
