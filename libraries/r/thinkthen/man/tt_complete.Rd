\name{tt_complete}
\alias{tt_decide_complete}
\alias{tt_choose_complete}
\alias{tt_tag_complete}
\alias{tt_score_complete}
\alias{tt_filter_complete}
\alias{tt_rank_complete}
\alias{tt_find_complete}
\alias{tt_annotate_complete}
\alias{tt_recognize_complete}
\alias{tt_relate_complete}
\alias{tt_decide_batch}
\alias{tt_choose_batch}
\alias{tt_tag_batch}
\alias{tt_score_batch}
\alias{tt_filter_batch}
\alias{tt_annotate_batch}
\title{Typed native complete calls and lazy batches}
\usage{
tt_decide_complete(question, input, ...)
tt_decide_batch(question, input, ...)
}
\arguments{
\item{question}{Explicit native question source: role and exactly one body, raw, path, name or reference.}
\item{input}{Explicit records or files with native reading options.}
\item{...}{attempts, deadline_ms, cancel and context controls.}
}
\value{Complete calls return typed results, final native facts, original inputs and ordinals. A batch returns next_row, poll, cancel, close and facts functions.}
\description{Runs the named native engine API. All ten complete functions share this interface; six record functions also expose native lazy batches.}
\details{Batch preparation waits for the first pull. poll returns NULL while pending; final facts distinguish exhaustion. Close an unfinished batch. Error conditions expose complete native failures with available facts. Ordinals and recognition starts are zero-based; recognition ends are exclusive Unicode scalar offsets. Physical source lines are one-based inclusive. Native rank values are one-based ranks. Cache, record, replay, images and admission remain native.}
