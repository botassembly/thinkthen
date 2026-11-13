\name{tt_client}
\alias{tt_client}
\alias{tt_close}
\alias{tt_usage}
\alias{local_tt_client}
\alias{with_tt_client}
\alias{tt_retry}
\title{Create and scope clients}
\description{Clients hold engine settings. Scoped defaults restore the previous client on exit and do not close the borrowed client.}
\usage{
tt_client(backend = NULL, ..., api_key = NULL, url = NULL,
                      model = NULL, batch = NULL, max_requests = NULL, retry = NULL)
tt_close(client)
tt_usage(client)
local_tt_client(client, .env = parent.frame())
with_tt_client(client, code)
tt_retry(..., max_retries = NULL, initial_backoff_ms = NULL,
                     attempt_timeout_ms = NULL)
}
\arguments{
\item{backend}{Backend name. NULL reads THINKTHEN_BACKEND, then uses the engine default.}
\item{...}{Named engine options. Unknown options are refused before any backend call.}
\item{api_key}{Optional backend API key.}
\item{url}{Optional backend address.}
\item{model}{Optional model name.}
\item{batch}{Optional whole integer batch size.}
\item{max_requests}{Optional exact whole send allowance; NULL is unlimited and zero forbids sends. Numeric values must fit the exact R integer range.}
\item{retry}{Retry settings returned by \code{tt_retry()}.}
\item{client}{A client from \code{tt_client()}.}
\item{.env}{The scope whose exit restores the previous default client.}
\item{code}{Expression evaluated with the supplied default client.}
\item{max_retries}{Optional whole integer retry count.}
\item{initial_backoff_ms}{Optional whole integer initial retry delay in milliseconds.}
\item{attempt_timeout_ms}{Optional whole integer attempt timeout in milliseconds.}
}
\value{\code{tt_usage()} returns a copied list of cumulative counters and token subtotals; integers above exact R numeric range remain strings. A snapshot with active calls is not a final total. \code{tt_client()} returns a client. \code{tt_retry()} returns immutable retry settings. \code{tt_close()} returns NULL invisibly. \code{local_tt_client()} returns the client invisibly. \code{with_tt_client()} returns the value of \code{code}.}
\seealso{\code{\link{thinkthen-package}}}
