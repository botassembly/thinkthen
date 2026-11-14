\name{tt_decide}
\alias{tt_decide}
\alias{tt_choose}
\alias{tt_tag}
\alias{tt_score}
\alias{tt_filter}
\alias{tt_rank}
\alias{tt_find}
\alias{tt_annotate}
\alias{tt_recognize}
\alias{tt_relate}
\title{Ask typed questions about records}
\description{Judge vectors and structured records with admitted questions. Arguments after dots must be named. A matching admitted question supplies its own domain and question options.}
\usage{
tt_decide(x, question, ..., threshold = NULL, yes_means = NULL, no_means = NULL,
                      field = NULL, client = NULL, .progress = TRUE, name = NULL,
                      details = FALSE, errors = "error")
tt_choose(x, question, choices, ..., descriptions = NULL, threshold = NULL,
                      field = NULL, client = NULL, .progress = TRUE, name = NULL,
                      details = FALSE, errors = "error")
tt_tag(x, question, labels, ..., descriptions = NULL, threshold = NULL,
                   field = NULL, client = NULL, .progress = TRUE, name = NULL,
                   details = FALSE, errors = "error")
tt_score(x, question, levels, ..., descriptions = NULL, field = NULL,
                     client = NULL, .progress = TRUE, name = NULL,
                     details = FALSE, errors = "error")
tt_filter(x, question, ..., threshold = NULL, yes_means = NULL, no_means = NULL,
                      field = NULL, client = NULL, .progress = TRUE, name = NULL)
tt_rank(x, question, levels, ..., descriptions = NULL, top = NULL,
                    field = NULL, client = NULL, .progress = TRUE, name = NULL,
                    details = FALSE, errors = "error")
tt_find(x, question, ..., threshold = NULL, field = NULL, client = NULL,
                    .progress = TRUE, name = NULL, details = FALSE, errors = "error")
tt_annotate(x, questions, ..., field = NULL, client = NULL, .progress = TRUE,
                        name = NULL, details = FALSE, errors = "error")
tt_recognize(x, kinds, ..., threshold = NULL, field = NULL, client = NULL,
                         .progress = TRUE, name = NULL, details = FALSE, errors = "error")
tt_relate(x, rules, ..., threshold = NULL, field = NULL, client = NULL,
                      .progress = TRUE, name = NULL, details = FALSE, errors = "error")
}

\arguments{
\item{details}{TRUE returns every state in a details tibble. Unavailable on filter.}
\item{errors}{"error" raises a condition after any record or form entry fails; "na" returns missing answers, warns once, and keeps the failures for \code{\link{tt_failures}}.}
\item{yes_means, no_means}{Optional descriptions of true and false answers for decide and filter.}
\item{descriptions}{A named character vector mapping domain labels or levels to descriptions. Unknown names are refused.}
\item{x}{A character vector, list of records or data frame.}
\item{question}{Question text or an admitted question of the matching function.}
\item{...}{Named engine options. Unknown options are refused before any backend call.}
\item{threshold}{A numeric probability cut, or a low and high pair for decide; filter takes one bar.}
\item{field}{An optional JSON Pointer selecting field within each record.}
\item{client}{A client from \code{tt_client()}. NULL uses the scoped or package default.}
\item{.progress}{Whether to display progress in an interactive session.}
\item{name}{An optional execution name.}
\item{choices}{Character vector of allowed choices. Omit when the admitted question supplies it.}
\item{labels}{Character vector of allowed labels. Omit when the admitted question supplies it.}
\item{levels}{Character vector of score labels in increasing order. Omit when the admitted question supplies them.}
\item{top}{Optional maximum number of successful ranked rows; failed rows remain visible.}
\item{questions}{A toolbox or annotation question collection. Every child becomes one typed answer column.}
\item{kinds}{Character vector of entity kinds, or an admitted recognition question.}
\item{rules}{A single tt_rule, a list of rules, or an admitted relation question.}
}
\value{The plain verbs return typed answer vectors in input order. Not-sure answers are missing values. Rank fills positions beyond top with NA. Annotate returns a tibble with one typed column per question. Filter returns kept records in the original class. Its failure condition also carries the kept records. With details = TRUE, functions other than filter return full outcome tibbles, including every failed state, probability, provenance, detail and original item. Rank details remain in ranking order.}
\seealso{\code{\link{thinkthen-package}}}
\examples{
client <- tt_client(backend = "stub")
is_refund <- tt_decide("Please refund my order.", question = "Does this request a refund?", client = client)
stopifnot(is.logical(is_refund), length(is_refund) == 1L)
rows <- tt_decide(details = TRUE, "Please refund my order.", "Does this request a refund?", client = client)
stopifnot(identical(is_refund, tt_answer(rows)))
tt_close(client)
}
