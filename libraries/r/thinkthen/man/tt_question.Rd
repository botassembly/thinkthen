\name{tt_question}
\alias{tt_question}
\title{Select a question with explicit authority}
\usage{
tt_question(value = NULL, file, name, reference)

\method{print}{thinkthen_question}(x, \dots)
}
\arguments{
\item{value}{Literal wording as one string, an ordinary authored question
list, or another question selector to rewrap.}
\item{file}{A question file path; \code{~} expands.}
\item{name}{A saved question name.}
\item{reference}{A saved question reference, such as
\code{"@support.refund"}.}
\item{x}{A question selector.}
\item{\dots}{Further arguments passed to or from other print methods;
ignored.}
}
\value{
A question selector of class \code{thinkthen_question}, ready for any named
call's \code{question} argument.
}
\description{
Marks how a question reaches the judge: literal text, an authored question, a
question file, a saved name, or a reference. An explicit selector keeps
filesystem authority at the edge.
}
\details{
One authority wins, in this order: \code{file}, then \code{name}, then
\code{reference}, then \code{value}. A string is literal wording, including
strings beginning with \code{@}; an authored question is an ordinary named
list. Passing a question selector to this function rewraps it. Printing
withholds content. \code{\link{tt_recognize_question}} and
\code{\link{tt_relate_question}} return these selectors.
}
\examples{
tt_question("Does this ask for a refund?")
tt_question(value = list(decide = "Refund?", true = "Yes means refund"))
}
\seealso{
\code{\link{tt_decide}} for the calls that take a question, and
\code{\link{tt_options}} for the controls beside it.
}
