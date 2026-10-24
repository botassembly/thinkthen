\name{thinkthen}
\alias{tt_client}
\alias{tt_question}
\alias{tt_reference}
\alias{tt_detail}
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
\title{Ask typed questions about records}
\description{Judge character vectors, data frames and lists through a shared native engine. Filter retains selected originals. Other functions return a data frame of result rows.}
\usage{
tt_client(adapter = NULL, ..., mode = NULL, url = NULL,
          key_source = NULL, model = NULL, batch = NULL)
tt_question(toolbox, name = NULL)
tt_reference(name)
tt_detail(rows)
tt_answer(rows)
tt_bind(data, rows, prefix)
tt_decide(x, question, ..., threshold = NULL, on = NULL, client = NULL)
tt_choose(x, question, options = NULL, ..., threshold = NULL, on = NULL, client = NULL)
tt_tag(x, question, labels = NULL, ..., threshold = NULL, on = NULL, client = NULL)
tt_score(x, question, levels = NULL, ..., on = NULL, client = NULL)
tt_filter(x, question, ..., threshold = NULL, on = NULL, client = NULL)
tt_rank(x, question, ..., levels = NULL, top = NULL, on = NULL, client = NULL)
tt_find(x, question, ..., threshold = NULL, on = NULL, client = NULL)
tt_annotate(x, toolbox, ..., on = NULL, client = NULL)
tt_recognize(x, kinds = character(), ..., threshold = NULL, on = NULL, client = NULL)
tt_relate(x, rules, ..., threshold = NULL, on = NULL, client = NULL)
}
\arguments{
\item{x}{A character vector, data frame, or list of JSON records.}
\item{question}{Inline question text or an explicit tagged question.}
\item{adapter}{A backend name resolved by the engine.}
\item{mode, url, key_source, model, batch}{Client settings admitted once by Rust.}
\item{...}{No additional arguments are accepted.}
\item{toolbox}{A named list describing a toolbox, or a tagged question.}
\item{name}{An optional toolbox question name, or a reference name.}
\item{rows}{A complete ThinkThen result data frame.}
\item{data}{A base data frame or an ungrouped tibble with the same unchanged input rows.}
\item{prefix}{One nonempty nonmissing string used to name attached result columns.}
\item{options, labels, levels, kinds}{Character vectors for the selected function.}
\item{rules}{A list of relation rule objects, or a tagged question.}
\item{threshold}{A numeric threshold or band string.}
\item{on}{An evidence JSON Pointer.}
\item{top}{Limit the retained ranked rows.}
\item{client}{An explicit client, or NULL for the shared default client.}
}
\value{A client or tagged question for the corresponding helper; a list of decoded JSON values for \code{tt_detail}; selected original records for \code{tt_filter}; a typed answer vector or list for \code{tt_answer}; the input frame with prefixed answer, p, status, error and detail columns for \code{tt_bind}; otherwise a data frame with id, status, answer, p, error and raw JSON detail columns.}
\details{All R inputs use collection failure behavior. Failed rows remain data unless the engine reports a final call error, including filter and rank failures. Admission and final errors raise coded conditions. R interruption cancels the shared call. Span offsets count Unicode characters from zero with an exclusive end; use \code{substr(text, start + 1L, end)}. Client settings never appear in a call. JSON detail stays unparsed until \code{tt_detail} is called. The result helpers make no engine calls. \code{tt_answer} rejects ordinary frames and filter output; rank answers remain in rank order. Absent nested answers become NA list elements while successfully answered null remains NULL. \code{tt_bind} preserves original table columns, names, row names and classes. It requires exactly one result row per input row and never recycles. It rejects rank and filter output, invalid prefixes and column name collisions with ordinary R errors. Inspect status and error to distinguish unsure and failed rows.}
\examples{
client <- tt_client(adapter = "stub")
rows <- tt_decide(c("alpha", "beta"), "Ready?", client = client)
rows[c("status", "answer", "p")]
}
