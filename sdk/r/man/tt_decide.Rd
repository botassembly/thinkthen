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
tt_decide(x, question, ..., threshold = NULL, evidence = NULL, client = NULL,
                      .progress = TRUE, name = NULL)
tt_choose(x, question, choices, ..., threshold = NULL, evidence = NULL,
                      client = NULL, .progress = TRUE, name = NULL)
tt_tag(x, question, labels, ..., threshold = NULL, evidence = NULL,
                   client = NULL, .progress = TRUE, name = NULL)
tt_score(x, question, levels, ..., evidence = NULL,
                     client = NULL, .progress = TRUE, name = NULL)
tt_filter(x, question, ..., threshold = NULL, evidence = NULL,
                      client = NULL, .progress = TRUE, name = NULL)
tt_rank(x, question, levels, ..., top = NULL, evidence = NULL,
                    client = NULL, .progress = TRUE, name = NULL)
tt_find(x, question, ..., threshold = NULL, evidence = NULL,
                    client = NULL, .progress = TRUE, name = NULL)
tt_annotate(x, toolbox, ..., evidence = NULL,
                        client = NULL, .progress = TRUE, name = NULL)
tt_recognize(x, kinds, ..., threshold = NULL, evidence = NULL,
                         client = NULL, .progress = TRUE, name = NULL)
tt_relate(x, rules, ..., threshold = NULL, evidence = NULL,
                      client = NULL, .progress = TRUE, name = NULL)
}
\arguments{
\item{x}{A character vector, list of records or data frame.}
\item{question}{Question text or an admitted question of the matching function.}
\item{...}{Named options passed to the engine. Vector helpers pass all arguments to the corresponding row function.}
\item{threshold}{A numeric probability cut, or a low and high pair for decide, choose and filter.}
\item{evidence}{An optional JSON Pointer selecting evidence within each record.}
\item{client}{A client from \code{tt_client()}. NULL uses the scoped or package default.}
\item{.progress}{Whether to display progress in an interactive session.}
\item{name}{An optional execution name.}
\item{choices}{Character vector of allowed choices. Omit when the admitted question supplies it.}
\item{labels}{Character vector of allowed labels. Omit when the admitted question supplies it.}
\item{levels}{Character vector of score labels in increasing order. Omit when the admitted question supplies them.}
\item{top}{Optional maximum number of successful ranked rows; failed rows remain visible.}
\item{toolbox}{An annotation question collection from \code{tt_annotation_questions()}.}
\item{kinds}{Character vector of entity kinds, or an admitted recognition question.}
\item{rules}{Relation rules in \code{vctrs::list_of()}, or an admitted relation question.}
}
\value{A typed tibble with position, exact ID, status, answer, probability, source, error fields and detail. Decide and filter answers are logical; choose is character; score and rank are double; tag holds character vectors; find, recognize and relate hold typed records; annotate holds named outcomes. Failed records remain visible. Filter also returns item and kept. Rank sorts successful rows before failures.}
\seealso{\code{\link{thinkthen-package}}}
\examples{
client <- tt_client(backend = "stub")
rows <- tt_decide("Please refund my order.", "Does this request a refund?",
                  client = client)
rows$answer
tt_close(client)
}
