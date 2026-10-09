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
\alias{tt_question}
\alias{tt_plan}
\alias{tt_input}
\alias{tt_batch}
\title{Named native typed judging calls}
\usage{
tt_decide(question, input, options = list(), deadline_ms = NULL, completion = NULL)
tt_question(value = NULL, file, name, reference)
tt_input(kind, ...)
tt_plan(question, input, function_name = "decide", options = list())
tt_batch(function_name, question, input, options = list())
}
\arguments{
\item{question, value}{Literal wording or an ordinary authored question list.}
\item{input}{Text, JSON evidence, a record vector, list, data frame, or explicit input selector.}
\item{options}{Native canonical request controls in a named list.}
\item{deadline_ms}{Native call deadline in milliseconds.}
\item{completion}{An optional completion handle.}
\item{file, name, reference}{Explicit question selector authority.}
\item{kind, ...}{An explicit canonical input descriptor, without host validation.}
\item{function_name}{The native named judging function.}
}
\value{Calls return thinkthen_Call objects with generated results and native facts. Fields support ordinary R indexing and comparison. A batch provides next_result, poll, cancel, close and facts functions. A plan provides native packing estimates.}
\description{Rust admits every question, input and control. R converts ordinary values and delivers native conditions, interruption and cleanup.}
\details{Generated optional fields distinguish thinkthen_absent from explicit NULL. Native indexes and offsets remain zero-based. Objects print summaries that withhold content. Failed streamed calls retain their completed prefix and available terminal facts.}
