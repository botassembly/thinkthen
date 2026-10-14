\name{tt_recognize_question}
\alias{tt_recognize_question}
\title{Build a recognition question}
\usage{
tt_recognize_question(entity_definition = NULL, instructions = NULL,
  kinds = NULL, mode = NULL, relations = NULL, snippet_pieces = NULL,
  stage_context = NULL)
}
\arguments{
\item{entity_definition}{Question text defining what counts as an entity.}
\item{instructions}{Question text instructing the recognition.}
\item{kinds}{A named list of entity kinds; each value is that kind's optional
description.}
\item{mode}{Explicit recognition stage selection.}
\item{relations}{Relation rules the recognition also asks for, each a list
with \code{name} and optional \code{source} and \code{target} kinds.}
\item{snippet_pieces}{Recognition tokenizer pieces on each side; omission
retains the saved width.}
\item{stage_context}{Literal recognition context overrides, resolved member by
member.}
}
\value{
A question selector of class \code{thinkthen_question}, ready for
\code{\link{tt_recognize}}'s \code{question} argument.
}
\description{
Builds a recognition question from its parts instead of an ordinary authored
list.
}
\details{
Each argument supplies one part of the recognition question; a part left out
is left out of the question, and native admission validates the combination.
\code{\link{tt_recognize}} owns what the built question asks.
}
\examples{
tt_recognize_question(kinds = list(person = NULL, organization = NULL))
}
\seealso{
\code{\link{tt_recognize}} for the call that takes this question and
\code{\link{tt_question}} for the selector class.
}
