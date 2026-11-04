\name{tt_toolbox}
\alias{tt_toolbox}
\alias{tt_question}
\alias{tt_annotation_questions}
\alias{tt_toolbox_from_json}
\alias{tt_toolbox_to_json}
\alias{tt_toolbox_read}
\alias{tt_toolbox_write}
\title{Collect and select questions}
\description{Toolboxes hold named admitted questions. Annotation collections exclude rank and annotate questions. JSON helpers read and write the canonical toolbox representation.}
\usage{
tt_toolbox(questions)
tt_question(toolbox, name)
tt_annotation_questions(questions)
tt_toolbox_from_json(json)
tt_toolbox_to_json(toolbox)
tt_toolbox_read(path)
tt_toolbox_write(toolbox, path)
}
\arguments{
\item{path}{A toolbox JSON file to read or write.}
\item{questions}{Named list of admitted questions.}
\item{toolbox}{A toolbox from \code{tt_toolbox()} or \code{tt_toolbox_from_json()}.}
\item{name}{The toolbox entry name.}
\item{json}{A JSON string containing a toolbox.}
}
\value{\code{tt_toolbox()} and \code{tt_toolbox_from_json()} return an immutable toolbox. \code{tt_question()} returns an admitted question. \code{tt_annotation_questions()} returns an annotation collection. \code{tt_toolbox_to_json()} returns a JSON string.}
\seealso{\code{\link{thinkthen-package}}}
