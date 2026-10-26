\name{thinkthen}
\alias{tt_client}
\alias{tt_question}
\alias{tt_answer}
\alias{tt_bind}
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
\alias{tt_id_integer}
\alias{tt_retry}
\alias{tt_client_from_env}
\alias{local_tt_client}
\alias{with_tt_client}
\alias{tt_close}
\alias{tt_toolbox}
\alias{tt_annotation_questions}
\alias{tt_rule}
\alias{tt_decide_question}
\alias{tt_choose_question}
\alias{tt_tag_question}
\alias{tt_score_question}
\alias{tt_filter_question}
\alias{tt_rank_question}
\alias{tt_find_question}
\alias{tt_recognize_question}
\alias{tt_relate_question}
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
\title{Ask typed questions about records}
\description{Judge vectors and structured records with admitted questions. Calls return typed tibbles and retain failed records.}
\details{Use a named backend with \code{tt_client()}. Required answer domains follow the question. Arguments after dots must be named. Thresholds are numeric cuts or numeric low and high pairs for decide, choose and filter. Evidence selects a JSON Pointer. An execution name is independent of toolbox selection.

Question constructors admit immutable values. A matching question carries its domain into its call. Toolboxes hold named questions; annotation uses the narrower \code{tt_annotation_questions()} collection. Relation domains use \code{vctrs::list_of(tt_rule(...))}.

Results contain position, exact ID, status, typed answer, full-precision probability, source, error fields and typed detail. Spans count Unicode code points from one with an inclusive end. Missing records retain their positions as failed rows without a backend call. Call-level problems raise classed rlang conditions. Interrupts cancel and raise \code{thinkthen_cancelled_error}.

Vector helpers project answers and warn once about failures. \code{tt_bind()} attaches results by position to a copy of a supported data frame. \code{tt_close()} releases a client. Scoped defaults restore the prior client on every exit.

See the package API page for complete signatures and representation rules.}
\examples{
client <- tt_client(backend = "stub")
rows <- tt_choose(c("Refund please", "Where is my delivery?"),
                  "Which team?", c("billing", "shipping"), client = client)
rows$answer
tt_close(client)
}
