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
\alias{tt_decide_rows}
\alias{tt_choose_rows}
\alias{tt_tag_rows}
\alias{tt_score_rows}
\alias{tt_filter_rows}
\alias{tt_rank_rows}
\alias{tt_find_rows}
\alias{tt_annotate_rows}
\alias{tt_recognize_rows}
\alias{tt_relate_rows}
\alias{tt_keep}
\title{Ask typed questions about records}
\description{Judge vectors and structured records with admitted questions. Arguments after dots must be named. A matching admitted question supplies its own domain and question options.}
\usage{
tt_decide(x, question, ..., threshold = NULL, yes_means = NULL, no_means = NULL, field = NULL, client = NULL,
                      .progress = TRUE, name = NULL, errors = "error")
tt_choose(x, question, choices, ..., descriptions = NULL, threshold = NULL, field = NULL,
                      client = NULL, .progress = TRUE, name = NULL, errors = "error")
tt_tag(x, question, labels, ..., descriptions = NULL, threshold = NULL, field = NULL,
                   client = NULL, .progress = TRUE, name = NULL, errors = "error")
tt_score(x, question, levels, ..., descriptions = NULL, field = NULL,
                     client = NULL, .progress = TRUE, name = NULL, errors = "error")
tt_filter(x, question, ..., threshold = NULL, yes_means = NULL, no_means = NULL, field = NULL,
                      client = NULL, .progress = TRUE, name = NULL, errors = "error")
tt_rank(x, question, levels, ..., descriptions = NULL, top = NULL, field = NULL,
                    client = NULL, .progress = TRUE, name = NULL, errors = "error")
tt_find(x, question, ..., threshold = NULL, field = NULL,
                    client = NULL, .progress = TRUE, name = NULL, errors = "error")
tt_annotate(x, toolbox, ..., field = NULL,
                        client = NULL, .progress = TRUE, name = NULL, errors = "error")
tt_recognize(x, kinds, ..., threshold = NULL, field = NULL,
                         client = NULL, .progress = TRUE, name = NULL, errors = "error")
tt_relate(x, rules, ..., threshold = NULL, field = NULL,
                      client = NULL, .progress = TRUE, name = NULL, errors = "error")
tt_decide_rows(x, question, ..., threshold = NULL, yes_means = NULL, no_means = NULL, field = NULL, client = NULL,
                      .progress = TRUE, name = NULL)
tt_choose_rows(x, question, choices, ..., descriptions = NULL, threshold = NULL, field = NULL,
                      client = NULL, .progress = TRUE, name = NULL)
tt_tag_rows(x, question, labels, ..., descriptions = NULL, threshold = NULL, field = NULL,
                   client = NULL, .progress = TRUE, name = NULL)
tt_score_rows(x, question, levels, ..., descriptions = NULL, field = NULL,
                     client = NULL, .progress = TRUE, name = NULL)
tt_filter_rows(x, question, ..., threshold = NULL, yes_means = NULL, no_means = NULL, field = NULL,
                      client = NULL, .progress = TRUE, name = NULL)
tt_rank_rows(x, question, levels, ..., descriptions = NULL, top = NULL, field = NULL,
                    client = NULL, .progress = TRUE, name = NULL)
tt_find_rows(x, question, ..., threshold = NULL, field = NULL,
                    client = NULL, .progress = TRUE, name = NULL)
tt_annotate_rows(x, toolbox, ..., field = NULL,
                        client = NULL, .progress = TRUE, name = NULL)
tt_recognize_rows(x, kinds, ..., threshold = NULL, field = NULL,
                         client = NULL, .progress = TRUE, name = NULL)
tt_relate_rows(x, rules, ..., threshold = NULL, field = NULL,
                      client = NULL, .progress = TRUE, name = NULL)
tt_keep(x, question, ..., threshold = NULL, yes_means = NULL, no_means = NULL, field = NULL, client = NULL, .progress = TRUE, name = NULL, errors = "error")
}
\arguments{
\item{errors}{"error" raises a condition carrying every outcome and failed row; "na" returns missing answers or drops failed records from a kept-only filter.}
\item{yes_means, no_means}{Optional descriptions of true and false answers for decide and filter.}
\item{descriptions}{A named character vector mapping domain labels or levels to descriptions. Unknown names are refused.}
\item{x}{A character vector, list of records or data frame.}
\item{question}{Question text or an admitted question of the matching function.}
\item{...}{Named engine options. Unknown options are refused before any backend call.}
\item{threshold}{A numeric probability cut, or a low and high pair for decide, choose and filter.}
\item{field}{An optional JSON Pointer selecting field within each record.}
\item{client}{A client from \code{tt_client()}. NULL uses the scoped or package default.}
\item{.progress}{Whether to display progress in an interactive session.}
\item{name}{An optional execution name.}
\item{choices}{Character vector of allowed choices. Omit when the admitted question supplies it.}
\item{labels}{Character vector of allowed labels. Omit when the admitted question supplies it.}
\item{levels}{Character vector of score labels in increasing order. Omit when the admitted question supplies them.}
\item{top}{Optional maximum number of successful ranked rows; failed rows remain visible.}
\item{toolbox}{A toolbox or annotation question collection. Every child becomes one typed answer column.}
\item{kinds}{Character vector of entity kinds, or an admitted recognition question.}
\item{rules}{A single tt_rule, a list of rules, or an admitted relation question.}
}
\value{The plain verbs return typed answer vectors in input order. Not-sure answers are missing values. Rank fills positions beyond top with NA. Annotate returns a tibble with one typed column per question. Filter returns kept records in the original class. Its failure condition also carries the kept records. The _rows variants return full outcome tibbles, including failed records, probability, provenance and detail. Filter and rank rows include each original item; rank rows remain in ranking order.}
\seealso{\code{\link{thinkthen-package}}}
\examples{
client <- tt_client(backend = "stub")
is_refund <- tt_decide("Please refund my order.", question = "Does this request a refund?", client = client)
stopifnot(is.logical(is_refund), length(is_refund) == 1L)
rows <- tt_decide_rows("Please refund my order.", "Does this request a refund?", client = client)
stopifnot(identical(is_refund, tt_answer(rows)))
tt_close(client)
}
