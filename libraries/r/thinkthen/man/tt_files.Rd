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
\details{Located rank indexes remain zero-based original record indexes. Recognition spans retain native zero-based Unicode scalar starts and exclusive ends, local to the record: use \code{substr(row$record, span$start + 1, span$end)}. They are not byte offsets. Record and span \code{first_line} and \code{last_line} are physical one-based inclusive source lines, including skipped blank lines. These differ from the one-based inclusive spans returned by \code{tt_recognize}. Find returns a located selection or NULL for none. This JSON compatibility helper does not establish typed result/2 or image support; 0431 owns typed R adoption.}
