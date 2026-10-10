\name{tt_usage_persistence}
\alias{tt_usage_persistence}
\alias{tt_finish_usage_status}
\title{Observe live usage durability}
\usage{
tt_usage_persistence()
tt_finish_usage_status()
}
\value{A \code{thinkthen_UsageStatus} named list with the native \code{state} and optional fixed \code{advice}.}
\description{Observes the existing selected engine's usage durability without sending a model request. Observation does not wait for the writer. Finalization drains current usage deltas; only usage-lock acquisition has a deadline, and other filesystem work can take longer.}
\details{Written covers only this engine's current deltas. Failure leaves answers and in-memory counts usable but durable totals incomplete. The result is a live observation without a facts-snapshot timestamp, credentials or filesystem paths.}
