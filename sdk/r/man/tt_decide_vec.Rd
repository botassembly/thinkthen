\name{tt_decide_vec}
\alias{tt_decide_vec}
\alias{tt_choose_vec}
\alias{tt_tag_vec}
\alias{tt_score_vec}
\alias{tt_filter_vec}
\alias{tt_rank_vec}
\alias{tt_find_vec}
\alias{tt_annotate_vec}
\alias{tt_recognize_vec}
\alias{tt_relate_vec}
\alias{tt_keep}
\title{Return answer vectors}
\description{Call the corresponding row function and project its answers with \code{tt_answer()}. \code{tt_keep()} is an alias for \code{tt_filter_vec()}.}
\usage{
tt_decide_vec(...)
tt_choose_vec(...)
tt_tag_vec(...)
tt_score_vec(...)
tt_filter_vec(...)
tt_rank_vec(...)
tt_find_vec(...)
tt_annotate_vec(...)
tt_recognize_vec(...)
tt_relate_vec(...)
tt_keep(...)
}
\arguments{
\item{...}{Named options passed to the engine. Vector helpers pass all arguments to the corresponding row function.}
}
\value{The typed answer vector or list column of the corresponding row function. Failed answers retain typed missing values and emit a classed warning.}
\seealso{\code{\link{thinkthen-package}}}
