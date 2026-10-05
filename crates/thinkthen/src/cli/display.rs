//! Filter, rank and find text decoration at the output boundary.

use std::io::Write;

use clap::Args;

use crate::args::Common;
use crate::cli::intake::{Position, Snapshot};
use crate::edge;
use crate::failure::Failure;
use crate::schedule::Judged;

#[derive(Args, Clone, Debug, Default)]
pub(crate) struct Arguments {
    /// Prefix selected records with their physical line number.
    #[arg(short = 'n', long)]
    pub(crate) line_number: bool,

    /// Print the selected record's score.
    #[arg(long)]
    pub(crate) scores: bool,

    /// Print an independent group with N physical neighbors on either side.
    #[arg(long, value_name = "N", allow_hyphen_values = true)]
    pub(crate) around: Option<String>,
}

#[derive(Default)]
pub(crate) struct Display {
    pub(crate) arguments: Arguments,
    pub(crate) around: Option<usize>,
    pub(crate) snapshot: Option<Snapshot>,
    multiple: bool,
}

impl Display {
    pub(crate) fn validate(&mut self, common: &Common) -> Result<(), Failure> {
        self.around = self
            .arguments
            .around
            .as_deref()
            .map(|text| {
                text.parse::<usize>()
                    .ok()
                    .filter(|_| !text.is_empty() && text.bytes().all(|byte| byte.is_ascii_digit()))
                    .ok_or(Failure::Usage(
                        "--around takes an ASCII whole number of at least 0",
                    ))
            })
            .transpose()?;
        let active = self.arguments.line_number || self.arguments.scores || self.around.is_some();
        if active && common.details {
            return Err(Failure::Usage(
                "text display flags cannot accompany --details",
            ));
        }
        if active && (common.csv || common.tsv) {
            return Err(Failure::Usage(
                "text display flags need lines or JSONL, not CSV or TSV",
            ));
        }
        self.multiple = common.input.len() > 1;
        Ok(())
    }

    fn prefix(&self, position: &Position, line: usize, selected: bool) -> String {
        if !self.arguments.line_number {
            return String::new();
        }
        let file = if self.multiple {
            position
                .file
                .as_ref()
                .map(|file| format!("{file}:"))
                .unwrap_or_default()
        } else {
            String::new()
        };
        format!("{file}{line}{}", if selected { ':' } else { '-' })
    }

    pub(crate) fn emit(&self, writer: &mut dyn Write, judged: &Judged) -> Result<bool, Failure> {
        self.emit_row(
            writer,
            judged.printed.as_deref(),
            judged.position.as_ref(),
            judged.order_value,
        )
    }

    pub(crate) fn emit_row(
        &self,
        writer: &mut dyn Write,
        printed: Option<&str>,
        position: Option<&Position>,
        score: Option<f64>,
    ) -> Result<bool, Failure> {
        let Some(printed) = printed else {
            return Ok(true);
        };
        if !self.arguments.line_number && !self.arguments.scores && self.around.is_none() {
            return edge::write_line(writer, printed);
        }
        let position = position.ok_or(Failure::Defect("a displayed row carries no position"))?;
        let score = if self.arguments.scores {
            format!(
                "{}",
                score.ok_or(Failure::Defect("a displayed row carries no ordering value",))?
            )
        } else {
            String::new()
        };
        let Some(around) = self.around else {
            let score = if self.arguments.scores {
                format!("{score} ")
            } else {
                score
            };
            return edge::write_line(
                writer,
                &format!(
                    "{score}{}{printed}",
                    self.prefix(position, position.first, true)
                ),
            );
        };
        let delimiter = if self.arguments.scores {
            format!("-- {score}")
        } else {
            "--".to_owned()
        };
        if !edge::write_line(&mut *writer, &delimiter)? {
            return Ok(false);
        }
        let snapshot = self
            .snapshot
            .as_ref()
            .ok_or(Failure::Defect("a neighbor view has no snapshot"))?;
        snapshot.lines(
            position.source,
            position.first.saturating_sub(around).max(1),
            position.last.saturating_add(around),
            |line, bytes| {
                let prefix = self.prefix(
                    position,
                    line,
                    (position.first..=position.last).contains(&line),
                );
                match writer
                    .write_all(prefix.as_bytes())
                    .and_then(|()| writer.write_all(bytes))
                    .and_then(|()| writer.write_all(b"\n"))
                    .and_then(|()| writer.flush())
                {
                    Err(error) if Failure::closed_output(&error) => Ok(false),
                    Err(error) => Err(Failure::Output(error)),
                    Ok(()) => Ok(true),
                }
            },
        )
    }
}
