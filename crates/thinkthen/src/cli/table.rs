//! Shared file framing translates command arguments into the existing call.

use super::{Command, CommonArgs, family_code, local};
use clap::Args;
use serde_json::{Value, json};
use std::io::{self, Write};
use std::path::PathBuf;
use thinkthen::files::{FileError, FileFormat, FileJob, FileOutput, FileReceipt};
use thinkthen::{Cancel, Client, Error, SdkRequest};

#[derive(Debug, Args)]
pub(super) struct TableArgs {
    /// Read a UTF-8 comma-separated file and append judgment columns.
    #[arg(long, requires="input", group="structured", conflicts_with_all=["tsv", "excel", "lines"])]
    csv: bool,
    /// Read a UTF-8 tab-separated file and append judgment columns.
    #[arg(long, requires="input", group="structured", conflicts_with_all=["csv", "excel", "lines"])]
    tsv: bool,
    /// Preserve an .xlsx workbook and append judgment columns to one sheet.
    #[arg(long, requires="input", group="structured", conflicts_with_all=["csv", "tsv", "lines"])]
    excel: bool,
    /// The local table to read.
    #[arg(long)]
    input: Option<PathBuf>,
    /// A worksheet name; required when the workbook has several sheets.
    #[arg(long)]
    sheet: Option<String>,
    /// A literal evidence-column header. Repeat for several columns.
    #[arg(long = "column")]
    columns: Vec<String>,
    /// The new answer column; annotate uses its question names instead.
    #[arg(long)]
    into: Option<String>,
    /// A new output file; defaults to NAME.thinkthen.EXT beside the input.
    #[arg(long, conflicts_with = "in_place")]
    output: Option<PathBuf>,
    /// Replace the input after retaining an exclusive backup beside it.
    #[arg(long, conflicts_with = "output")]
    in_place: bool,
}

impl TableArgs {
    pub(super) fn enabled(&self) -> bool {
        self.csv
            || self.tsv
            || self.excel
            || self.input.is_some()
            || self.sheet.is_some()
            || !self.columns.is_empty()
            || self.into.is_some()
            || self.output.is_some()
            || self.in_place
    }

    pub(super) fn run(
        &self,
        request: &SdkRequest,
        client: &Client,
        cancel: &Cancel,
    ) -> Result<u8, Error> {
        let result = self
            .job()
            .and_then(|job| job.execute(request, client, cancel));
        Self::render(result)
    }

    pub(super) fn run_entry(
        &self,
        entry: &thinkthen::runs::RunEntry,
        client: &Client,
        cancel: &Cancel,
    ) -> Result<u8, Error> {
        Self::render(
            self.job()
                .and_then(|job| entry.execute_file(&job, client, cancel)),
        )
    }

    fn render(result: Result<FileReceipt, FileError>) -> Result<u8, Error> {
        match result {
            Ok(receipt) => {
                writeln!(io::stdout().lock(), "{}", receipt.output.display())
                    .map_err(|_| local("standard output could not be written"))?;
                if let Some(backup) = receipt.backup {
                    writeln!(
                        io::stderr().lock(),
                        "thinkthen: backup: {}",
                        backup.display()
                    )
                    .map_err(|_| local("standard error could not be written"))?;
                }
                if let Some(error) = receipt.first_failure {
                    super::report(&error);
                    Ok(family_code(error.kind().family()))
                } else {
                    Ok(0)
                }
            }
            Err(error) => {
                writeln!(io::stderr().lock(), "thinkthen: {}: {error}", error.code())
                    .map_err(|_| local("standard error could not be written"))?;
                Ok(family_code(error.family()))
            }
        }
    }

    fn job(&self) -> Result<FileJob, FileError> {
        let input = self
            .input
            .clone()
            .ok_or_else(|| FileError::Invalid("file framing requires --input PATH".into()))?;
        let format = if self.csv {
            FileFormat::Csv
        } else if self.tsv {
            FileFormat::Tsv
        } else if self.excel {
            FileFormat::Xlsx
        } else {
            return Err(FileError::Invalid("choose --csv, --tsv or --excel".into()));
        };
        let output = if self.in_place {
            FileOutput::InPlace
        } else {
            FileOutput::NewFile(self.output.clone())
        };
        FileJob::new(
            input,
            format,
            self.sheet.clone(),
            self.columns.clone(),
            self.into.clone(),
            output,
        )
    }
}

pub(super) fn dispatch(command: &Command, cancel: &Cancel) -> Option<Result<u8, Error>> {
    let common = match command {
        Command::Decide(a) | Command::Find(a) | Command::Filter(a) => &a.common,
        Command::Choose(a) => &a.common,
        Command::Tag(a) => &a.common,
        Command::Score(a) => &a.common,
        Command::Rank(a) => &a.common,
        Command::Recognize(a) => &a.common,
        Command::Relate(a) => &a.common,
        Command::Annotate(a) => &a.common,
        Command::Run(_) => return None,
    };
    if !common.table.enabled() {
        return None;
    }
    Some((|| {
        let mut options = serde_json::Map::new();
        let (function, question) = match command {
            Command::Decide(a) | Command::Find(a) | Command::Filter(a) => {
                if let Some(cut) = &a.threshold {
                    options.insert("threshold".into(), cut.clone().into());
                }
                let function = match command {
                    Command::Find(_) => "find",
                    Command::Filter(_) => "filter",
                    _ => "decide",
                };
                (function, Value::String(a.question.clone()))
            }
            Command::Choose(a) => {
                options.insert("options".into(), json!(a.options));
                if let Some(cut) = &a.threshold {
                    options.insert("threshold".into(), cut.clone().into());
                }
                ("choose", a.question.clone().into())
            }
            Command::Tag(a) => {
                options.insert("labels".into(), json!(a.labels));
                if let Some(cut) = &a.threshold {
                    options.insert("threshold".into(), cut.clone().into());
                }
                ("tag", a.question.clone().into())
            }
            Command::Score(a) => {
                options.insert("levels".into(), json!(a.levels));
                ("score", a.question.clone().into())
            }
            Command::Rank(a) => {
                if !a.levels.is_empty() {
                    options.insert("levels".into(), json!(a.levels));
                }
                if let Some(top) = &a.top {
                    let top: thinkthen::RankLimit = top.parse()?;
                    options.insert("top".into(), top.get().into());
                }
                ("rank", a.question.clone().into())
            }
            Command::Recognize(a) => {
                if let Some(cut) = &a.threshold {
                    options.insert("threshold".into(), cut.clone().into());
                }
                ("recognize", json!(a.kinds))
            }
            Command::Relate(a) => {
                let rules = a
                    .rules
                    .iter()
                    .map(|rule| {
                        thinkthen::RelationRule::parse(rule).map(|rule| rule.either(a.either))
                    })
                    .collect::<Result<Vec<_>, _>>()?;
                if let Some(cut) = &a.threshold {
                    options.insert("threshold".into(), cut.clone().into());
                }
                (
                    "relate",
                    serde_json::to_value(rules)
                        .map_err(|_| local("relation arguments could not be encoded"))?,
                )
            }
            Command::Annotate(a) => (
                "annotate",
                serde_json::from_str(&a.toolbox).map_err(|_| {
                    Error::new(
                        thinkthen::ErrorKind::Toolbox,
                        "annotate expects an inline JSON toolbox",
                    )
                })?,
            ),
            Command::Run(_) => return Err(local("saved entries use their own file framing")),
        };
        if let Some(on) = &common.on {
            options.insert("on".into(), on.clone().into());
        }
        let request = SdkRequest::from_json(&json!({"function":function,"question":{"type":"inline","value":question},"options":options}).to_string())?;
        let client = client(common)?;
        common.table.run(&request, &client, cancel)
    })())
}

fn client(common: &CommonArgs) -> Result<Client, Error> {
    let mut settings = serde_json::Map::new();
    settings.insert("adapter".into(), common.adapter.clone().into());
    if let Some(model) = &common.model {
        settings.insert("model".into(), model.clone().into());
    }
    if let Some(url) = &common.url {
        settings.insert("url".into(), url.clone().into());
    }
    if let Some(batch) = &common.batch {
        let batch = batch
            .parse::<serde_json::Number>()
            .map_err(|_| {
                Error::new(
                    thinkthen::ErrorKind::Batch,
                    "batch must be a whole number of at least 1",
                )
            })?
            .into();
        settings.insert("batch".into(), batch);
    }
    Client::from_json(&Value::Object(settings).to_string(), |name| {
        std::env::var(name).ok()
    })
}
