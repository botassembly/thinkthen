\name{tt_engine}
\alias{tt_engine}
\title{Select settings for the R session's engine}
\usage{
tt_engine(base_url = NULL, model = NULL, throttle = NULL,
  max_requests = NULL, max_requests_total = NULL,
  max_request_bytes = NULL, cache = NULL, timeout = NULL,
  max_retries = NULL, record = NULL, replay = NULL, profile = NULL,
  batch = NULL, ..., backend = NULL, refresh_cache = FALSE)
}
\arguments{
\item{base_url, model}{Override the selected backend's address and model.}
\item{throttle}{Number of concurrent requests, from 1 through 32.}
\item{max_requests, max_requests_total}{Request caps for a call and the process.}
\item{max_request_bytes}{Maximum bytes in one request.}
\item{cache}{Answer cache folder, or \code{FALSE} to disable it.}
\item{timeout, max_retries}{Request timeout and retry limit.}
\item{record, replay}{Recording folder and strict replay folder.}
\item{profile}{Backend profile file.}
\item{batch}{Positive record count per request, or \code{"max"}.}
\item{...}{Unknown arguments are refused.}
\item{refresh_cache}{Ignore held cache answers while retaining native recording and cache identity.}
\item{backend}{One backend name, or \code{NULL} to retain environment selection. Use the full keyword name.}
}
\value{Invisibly returns \code{NULL}.}
\description{Sets the engine once per R session. Equal settings can be repeated. Different settings raise a usage condition that names the settings in force. Start a new R session to change them.}
\details{The built-ins are \code{liquid}, \code{ollama}, \code{openrouter}, \code{perplexity}, and \code{typesafe}. Rust reads their respective key variables \code{LIQUIDAI_API_KEY}, \code{OLLAMA_API_KEY}, \code{OPENROUTER_API_KEY}, \code{PERPLEXITY_API_KEY}, and \code{TYPESAFE_API_KEY} when it constructs the engine. Configured backend names follow the Rust configuration. R accepts no key argument. An omitted argument retains the environment's setting. Explicit address and model settings retain the selected backend's path, question form, prices, and profile. Backend selection does not belong to an asking verb.}
\examples{
\dontrun{tt_engine(backend = "typesafe")}
}
