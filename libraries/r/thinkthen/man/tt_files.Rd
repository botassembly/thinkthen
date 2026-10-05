\name{tt_files}
\alias{tt_files}
\title{Judge Explicit Files with Source Coordinates}
\usage{tt_files(question, paths, unit = "line", window = NULL, deadline_ms = NULL)}
\arguments{
\item{question}{A named list using any of the ten JSON question grammars.}
\item{paths}{Explicit file or folder paths in order.}
\item{unit}{Physical line, nonoverlapping window, or whole file.}
\item{window}{Positive line count required only for window units.}
\item{deadline_ms}{Whole-call millisecond deadline, or NULL for none.}
}
\description{Uses the shared native file reader. Source metadata stays outside evidence and annotated values. Existing text and column methods retain their input shapes.}
\value{A ThinkThen result with located JSON values and call facts.}
