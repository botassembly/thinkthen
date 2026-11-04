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
tt_decide_question(question, ..., threshold = NULL, yes_means = NULL, no_means = NULL, field = NULL)
tt_choose_question(question, choices, ..., descriptions = NULL, threshold = NULL, field = NULL)
tt_tag_question(question, labels, ..., descriptions = NULL, threshold = NULL, field = NULL)
tt_score_question(question, levels, ..., descriptions = NULL, field = NULL)
tt_filter_question(question, ..., threshold = NULL, yes_means = NULL, no_means = NULL, field = NULL)
tt_rank_question(question, levels, ..., descriptions = NULL, top = NULL, field = NULL)
tt_find_question(question, ..., threshold = NULL, field = NULL)
tt_recognize_question(kinds, ..., threshold = NULL, field = NULL)
tt_relate_question(rules, ..., threshold = NULL, field = NULL)
}
\arguments{
\item{yes_means, no_means}{Optional meanings of true and false answers.}
\item{descriptions}{A named character vector of descriptions for domain labels or levels.}
\item{question}{Question text or an admitted question of the matching function.}
\item{...}{Named engine options. Unknown options are refused before any backend call.}
\item{threshold}{A numeric probability cut, or a low and high pair for decide, choose and filter.}
\item{field}{An optional JSON Pointer selecting field within each record.}
\item{choices}{Character vector of allowed choices. Omit when the admitted question supplies it.}
\item{labels}{Character vector of allowed labels. Omit when the admitted question supplies it.}
\item{levels}{Character vector of score labels in increasing order. Omit when the admitted question supplies them.}
\item{top}{Optional maximum number of successful ranked rows; failed rows remain visible.}
\item{kinds}{Character vector of entity kinds, or an admitted recognition question.}
\item{rules}{Relation rules in \code{vctrs::list_of()}, or an admitted relation question.}
}
\value{An immutable admitted question. Question constructors raise a classed error if the engine rejects their arguments.}
\seealso{\code{\link{thinkthen-package}}}
