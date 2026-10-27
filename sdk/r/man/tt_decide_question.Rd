\name{tt_decide_question}
\alias{tt_decide_question}
\alias{tt_choose_question}
\alias{tt_tag_question}
\alias{tt_score_question}
\alias{tt_filter_question}
\alias{tt_rank_question}
\alias{tt_find_question}
\alias{tt_recognize_question}
\alias{tt_relate_question}
\title{Construct immutable questions}
\description{Admit question text, answer domains and options once for reuse with the matching row function.}
\usage{
tt_decide_question(question, ..., threshold = NULL, evidence = NULL)
tt_choose_question(question, choices, ..., threshold = NULL, evidence = NULL)
tt_tag_question(question, labels, ..., threshold = NULL, evidence = NULL)
tt_score_question(question, levels, ..., evidence = NULL)
tt_filter_question(question, ..., threshold = NULL, evidence = NULL)
tt_rank_question(question, levels, ..., top = NULL, evidence = NULL)
tt_find_question(question, ..., threshold = NULL, evidence = NULL)
tt_recognize_question(kinds, ..., threshold = NULL, evidence = NULL)
tt_relate_question(rules, ..., threshold = NULL, evidence = NULL)
}
\arguments{
\item{question}{Question text or an admitted question of the matching function.}
\item{...}{Named options passed to the engine. Vector helpers pass all arguments to the corresponding row function.}
\item{threshold}{A numeric probability cut, or a low and high pair for decide, choose and filter.}
\item{evidence}{An optional JSON Pointer selecting evidence within each record.}
\item{choices}{Character vector of allowed choices. Omit when the admitted question supplies it.}
\item{labels}{Character vector of allowed labels. Omit when the admitted question supplies it.}
\item{levels}{Character vector of score labels in increasing order. Omit when the admitted question supplies them.}
\item{top}{Optional maximum number of successful ranked rows; failed rows remain visible.}
\item{kinds}{Character vector of entity kinds, or an admitted recognition question.}
\item{rules}{Relation rules in \code{vctrs::list_of()}, or an admitted relation question.}
}
\value{An immutable admitted question. Question constructors raise a classed error if the engine rejects their arguments.}
\seealso{\code{\link{thinkthen-package}}}
