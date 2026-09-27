//! Which verb was typed, what it keeps, and the view it prints in.
//!
//! Each verb settles its question, says what a run does with each answered
//! record, and hands both to `asking.rs`, which owns the request and the row.

use std::io::{Read, Write};
use std::process::ExitCode;

use crate::core::{Json, Pointer, QuestionFileError, Resolved, Setting};

use crate::args::{
    Batching, ChooseArguments, Common, DecideArguments, FilterArguments, RankArguments, Refused,
    ScoreArguments, TagArguments,
};
use crate::asking::{Asks, fixed, run};
use crate::cli::asked;
use crate::edge::Environment;
use crate::failure::Failure;
use crate::schedule::Output;

/// One question, its rule, and the view its answer prints in.
#[derive(Debug)]
pub(crate) struct Asked<'a> {
    pub(crate) common: &'a Common,
    pub(crate) asks: Asks,
    pub(crate) settled: &'a Resolved,
    pub(crate) view: View,
    pub(crate) keeping: Keeping,
    /// Where `decide`, `filter` and `rank` read their batch setting, or `None`.
    pub(crate) batch: Option<Tiers<'a>>,
}

/// The typed `--batch` and a question file's `batch`, which with
/// `THINKTHEN_BATCH` and `max` settle a batch, by ADR 0048 item 4.
#[derive(Debug)]
pub(crate) struct Tiers<'a> {
    pub(crate) flag: Option<&'a str>,
    pub(crate) file: Option<Json>,
}

impl Tiers<'_> {
    /// The setting a stream runs at, or `None` on one document, where only a
    /// typed `--batch` is refused.
    pub(crate) fn setting(
        &self,
        environment: &Environment,
        streams: bool,
    ) -> Result<Option<Setting>, Failure> {
        if !streams {
            return match self.flag {
                Some(_) => Err(Failure::Usage(
                    "--batch groups the records of a stream, and a single text is one record",
                )),
                None => Ok(None),
            };
        }
        let setting = if let Some(flag) = self.flag {
            Setting::parse(flag).ok_or(Failure::Usage(
                "--batch takes max or a whole number of at least 1",
            ))?
        } else if let Some(variable) = environment.batch() {
            Setting::parse(variable).ok_or(Failure::Usage(
                "THINKTHEN_BATCH takes max or a whole number of at least 1",
            ))?
        } else if let Some(value) = &self.file {
            Setting::of_json(value).ok_or(Failure::Question(QuestionFileError::Shape {
                key: "batch",
                wanted: "takes max or a whole number of at least 1",
            }))?
        } else {
            Setting::Max
        };
        Ok(Some(setting))
    }
}

/// What a run does with each answered record.
///
/// `decide`, `choose`, and `score` print what the model said. `filter` and
/// `rank` print the records themselves, so the answer decides which record is
/// printed and in what order rather than what the line holds.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum Keeping {
    /// Print the bare value or the result row, as the three judging verbs do.
    Answers,
    /// Print the record itself when its answer reached the mark.
    Passing,
    /// Hold every record, and print them by the probability of yes.
    Ordered,
}

impl Keeping {
    /// The verb this way of keeping belongs to, which a refusal names.
    pub(crate) const fn verb(self) -> &'static str {
        match self {
            Self::Answers => "decide",
            Self::Passing => "filter",
            Self::Ordered => "rank",
        }
    }

    /// True when one document is no input at all, as both record verbs read it.
    pub(crate) const fn streams_only(self) -> bool {
        !matches!(self, Self::Answers)
    }
}

/// Which of the three views of one answer the command line asked for.
#[derive(Clone, Copy, Debug)]
pub(crate) struct View {
    pub(crate) quiet: bool,
    pub(crate) raw: bool,
    pub(crate) details: bool,
}

impl View {
    /// Refuse two views of one answer, which no run can print at once.
    ///
    /// `channels.md` makes an option that cannot act in the chosen mode a usage
    /// error. `--raw` prints a bare label, so it acts in neither other view.
    pub(crate) const fn checked(self) -> Result<Self, Failure> {
        if self.quiet && self.details {
            return Err(Failure::QuietWithDetails);
        }
        if self.raw && (self.quiet || self.details) {
            return Err(Failure::RawWithAnotherView);
        }
        Ok(self)
    }
}

/// Answer the question about the evidence, and set the exit code from the answer.
///
/// # Errors
///
/// Returns [`Failure`] for every outcome `specification/channels.md` gives an
/// exit code other than 0, 1, and 3.
pub(crate) fn decide(
    arguments: &DecideArguments,
    environment: &Environment,
    input: impl Read + Send + 'static,
    writer: impl Write,
) -> Result<ExitCode, Failure> {
    if arguments.raw {
        return Err(Failure::Usage(
            "`decide` prints JSON; `choose --raw` prints a bare label",
        ));
    }
    let (settled, file) = asked::decide(arguments)?;
    let view = View {
        quiet: arguments.quiet,
        raw: false,
        details: arguments.common.details,
    };
    judging(
        Asked {
            common: &arguments.common,
            asks: fixed(&settled)?,
            settled: &settled,
            view,
            keeping: Keeping::Answers,
            batch: Some(tiers(&arguments.batching, file)),
        },
        environment,
        input,
        writer,
    )
}

/// Keep the records that reach the mark, and print each one as it arrived.
///
/// # Errors
///
/// Returns [`Failure`] for a band in either home, for a view that prints no
/// record, and for every outcome `channels.md` gives a code above 3.
pub(crate) fn filter(
    arguments: &FilterArguments,
    environment: &Environment,
    input: impl Read + Send + 'static,
    writer: impl Write,
) -> Result<ExitCode, Failure> {
    views(&arguments.refused, Keeping::Passing)?;
    if arguments.top.is_some() {
        return Err(Failure::Usage(
            "`filter` keeps records and has no order to cut, so --top belongs to `rank`",
        ));
    }
    let (settled, file) = asked::filter(arguments)?;
    over_kept(
        Keeping::Passing,
        &arguments.common,
        tiers(&arguments.batching, file),
        &settled,
        None,
        environment,
        input,
        writer,
    )
}

/// Print every record, with the most likely yes first.
///
/// # Errors
///
/// Returns [`Failure`] for a rule in either home, for a view that prints no
/// record, and for every outcome `channels.md` gives a code above 3.
pub(crate) fn rank(
    arguments: &RankArguments,
    environment: &Environment,
    input: impl Read + Send + 'static,
    writer: impl Write,
) -> Result<ExitCode, Failure> {
    views(&arguments.refused, Keeping::Ordered)?;
    let top = arguments
        .top
        .as_deref()
        .map(|value| {
            value
                .parse::<usize>()
                .ok()
                .filter(|number| *number > 0)
                .ok_or(Failure::TopIsZero)
        })
        .transpose()?;
    let (settled, file) = asked::rank(arguments)?;
    over_kept(
        Keeping::Ordered,
        &arguments.common,
        tiers(&arguments.batching, file),
        &settled,
        top,
        environment,
        input,
        writer,
    )
}

fn tiers(batching: &Batching, file: Option<Json>) -> Tiers<'_> {
    Tiers {
        flag: batching.batch.as_deref(),
        file,
    }
}

/// Refuse a view that prints no record, before anything else is read.
///
/// Both record verbs call this first, so a command line holding two mistakes
/// is refused for the same one whichever verb was typed.
fn views(refused: &Refused, keeping: Keeping) -> Result<(), Failure> {
    if refused.quiet {
        return Err(Failure::QuietOverKept(keeping.verb()));
    }
    if refused.raw {
        return Err(Failure::RawOverKept(keeping.verb()));
    }
    Ok(())
}

/// The one flow both record verbs take, which prints records and not answers.
#[expect(
    clippy::too_many_arguments,
    reason = "the two verbs share every step, and splitting the call would split the flow"
)]
fn over_kept(
    keeping: Keeping,
    common: &Common,
    batch: Tiers<'_>,
    settled: &Resolved,
    top: Option<usize>,
    environment: &Environment,
    input: impl Read + Send + 'static,
    mut writer: impl Write,
) -> Result<ExitCode, Failure> {
    let writer: &mut dyn Write = &mut writer;
    let mut output = match keeping {
        Keeping::Ordered => Output::Ordered {
            held: Vec::new(),
            top,
            writer,
        },
        _ => Output::Streaming(writer),
    };
    run(
        Asked {
            common,
            asks: fixed(settled)?,
            settled,
            view: View {
                quiet: false,
                raw: false,
                details: common.details,
            },
            keeping,
            batch: Some(batch),
        },
        environment,
        input,
        &mut output,
    )
}

/// Run one judging verb, whose rows print as their places come.
fn judging(
    asked: Asked<'_>,
    environment: &Environment,
    input: impl Read + Send + 'static,
    mut writer: impl Write,
) -> Result<ExitCode, Failure> {
    let writer: &mut dyn Write = &mut writer;
    run(asked, environment, input, &mut Output::Streaming(writer))
}

/// Pick one label from the options, and set the exit code from the answer.
///
/// # Errors
///
/// Returns [`Failure`] for a band on `choose`, for a list of options the verb
/// does not take, and for every outcome `channels.md` gives a code above 3.
pub(crate) fn choose(
    arguments: &ChooseArguments,
    environment: &Environment,
    input: impl Read + Send + 'static,
    writer: impl Write,
) -> Result<ExitCode, Failure> {
    if arguments.raw && (arguments.common.csv || arguments.common.tsv) {
        return Err(Failure::TableRaw);
    }
    let settled = asked::choose(arguments)?;
    let asks = match arguments.options_pointer.as_deref() {
        Some(typed) => {
            if !arguments.common.jsonl {
                return Err(Failure::OptionsOutsideJsonl);
            }
            Asks::FromRecord {
                text: settled.text().clone(),
                pointer: Pointer::new(typed)
                    .map_err(|error| Failure::Pointer("--options", typed.to_owned(), error))?,
            }
        }
        None => fixed(&settled)?,
    };
    let view = View {
        quiet: arguments.quiet,
        raw: arguments.raw,
        details: arguments.common.details,
    };
    judging(
        Asked {
            common: &arguments.common,
            asks,
            settled: &settled,
            view,
            keeping: Keeping::Answers,
            batch: None,
        },
        environment,
        input,
        writer,
    )
}

/// Return every label whose independent yes probability reaches the cut.
pub(crate) fn tag(
    arguments: &TagArguments,
    environment: &Environment,
    input: impl Read + Send + 'static,
    writer: impl Write,
) -> Result<ExitCode, Failure> {
    if arguments.raw {
        return Err(Failure::TagRaw);
    }
    if arguments.quiet {
        return Err(Failure::TagQuiet);
    }
    let settled = asked::tag(arguments)?;
    judging(
        Asked {
            common: &arguments.common,
            asks: fixed(&settled)?,
            settled: &settled,
            view: View {
                quiet: false,
                raw: false,
                details: arguments.common.details,
            },
            keeping: Keeping::Answers,
            batch: None,
        },
        environment,
        input,
        writer,
    )
}

/// Place the evidence on the levels and print the weighted position.
///
/// # Errors
///
/// Returns [`Failure`] for a rule, which `score` has none of, for a list of
/// levels the verb does not take, and for every outcome `channels.md` gives a
/// code above 3.
pub(crate) fn score(
    arguments: &ScoreArguments,
    environment: &Environment,
    input: impl Read + Send + 'static,
    writer: impl Write,
) -> Result<ExitCode, Failure> {
    if arguments.raw {
        return Err(Failure::Usage(
            "`score` prints a JSON number; `choose --raw` prints a bare label",
        ));
    }
    if arguments.quiet {
        return Err(Failure::Usage(
            "`score` has no answer exit code, so --quiet would discard its result",
        ));
    }
    let settled = asked::score(arguments)?;
    let view = View {
        quiet: false,
        raw: false,
        details: arguments.common.details,
    };
    judging(
        Asked {
            common: &arguments.common,
            asks: fixed(&settled)?,
            settled: &settled,
            view,
            keeping: Keeping::Answers,
            batch: None,
        },
        environment,
        input,
        writer,
    )
}
