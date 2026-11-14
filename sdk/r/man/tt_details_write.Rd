\name{tt_details_write}
\alias{tt_details_write}
\title{Save canonical outcome rows}
\description{Write the current details table as shared JSON Lines, with exact identifiers and Unicode character offsets from zero, end excluded. Original caller items are omitted.}
\usage{tt_details_write(details, path)}
\arguments{
\item{details}{A details table returned by a ThinkThen function.}
\item{path}{The file to write.}}
\value{The details table, invisibly.}
