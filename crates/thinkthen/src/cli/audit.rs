//! Offline command input/output; scoring and row admission belong to the library.
use crate::analysis::{self, AuditOptions, GoldCase, Manifest};
use crate::{Cancel, Code, Error};
use clap::{Arg, ArgMatches, Command, parser::ValueSource};
use std::io::{self, Read, Write};

pub(super) fn command() -> Command {
    Command::new("audit")
        .about("Grade saved answers against labels without a key or model call")
        .arg(Arg::new("rows").required(true).value_name("ROWS")
            .help("Saved typed JSON Lines; - reads standard input"))
        .arg(Arg::new("manifest").long("manifest").required(true).value_name("RUN.json")
            .help("Versioned question, route and expected case inventory"))
        .arg(Arg::new("key").long("key").required(true).value_name("GOLD.jsonl")
            .help("Gold labels joined by exact ID and question identity"))
        .arg(Arg::new("bins").long("bins").default_value("10").value_name("N")
            .value_parser(clap::value_parser!(std::num::NonZeroUsize))
            .help("Equal-width probability bins"))
        .after_help("Prints one JSON report. Failed, unsure, unlabeled and missing cases remain visible. Poor accuracy is a successful audit, not a command error. Each input is limited to 64 MiB and 100,000 cases; the Rust analysis API supports explicit bounds.")
}

pub(super) fn run(args: &ArgMatches, cancel: &Cancel) -> Result<u8, Error> {
    offline_options(args, &command())?;
    let mut options = AuditOptions::default();
    if let Some(bins) = args.get_one::<std::num::NonZeroUsize>("bins") {
        options.bins = *bins;
    }
    let path = |name| {
        args.get_one::<String>(name)
            .map(String::as_str)
            .ok_or_else(|| super::options::usage(name, "a file path", "absent path"))
    };
    if path("manifest")? == "-" || path("key")? == "-" {
        return Err(super::options::usage(
            "manifest/key",
            "named files; only ROWS may read stdin",
            "standard input",
        ));
    }
    let manifest: Manifest = serde_json::from_str(&read(
        path("manifest")?,
        options.max_input_bytes.get(),
        cancel,
    )?)
    .map_err(|_| {
        Error::from_contract(
            Code::UsageQuestion,
            "the audit manifest is not valid current JSON metadata",
        )
    })?;
    let rows = lines(
        &read(path("rows")?, options.max_input_bytes.get(), cancel)?,
        options.max_cases.get(),
        analysis::decode_row,
    )?;
    let key = lines(
        &read(path("key")?, options.max_input_bytes.get(), cancel)?,
        options.max_cases.get(),
        |line| {
            serde_json::from_str::<GoldCase>(line).map_err(|_| {
                Error::from_contract(
                    Code::UsageQuestion,
                    "a key entry is not a current typed gold label",
                )
            })
        },
    )?;
    if cancel.is_cancelled() {
        return Err(crate::retry::cancelled());
    }
    let report = analysis::audit(&manifest, rows, key, options)?;
    output(args, cancel, &report)?;
    Ok(0)
}
pub(super) fn read(path: &str, maximum: usize, cancel: &Cancel) -> Result<String, Error> {
    let mut path = Some(path.to_owned());
    let receiver = super::input::spawn(
        move || {
            path.take()
                .map(|path| read_document(&path, maximum))
                .transpose()
        },
        cancel.clone(),
    )?;
    super::input::receive(&receiver, cancel)?
        .ok_or_else(|| crate::core::local("the audit input reader ended without a document"))
}
fn read_document(path: &str, maximum: usize) -> Result<String, Error> {
    let mut reader: Box<dyn Read> = if path == "-" {
        Box::new(io::stdin())
    } else {
        Box::new(
            std::fs::File::open(path)
                .map_err(|_| crate::core::local("an audit input file could not be opened"))?,
        )
    };
    let mut bytes = Vec::new();
    reader
        .by_ref()
        .take(maximum.saturating_add(1) as u64)
        .read_to_end(&mut bytes)
        .map_err(|_| crate::core::local("an audit input could not be read"))?;
    if bytes.len() > maximum {
        return Err(crate::core::local(
            "an audit input exceeds its configured byte limit",
        ));
    }
    String::from_utf8(bytes)
        .map_err(|_| Error::from_contract(Code::UsageQuestion, "audit inputs must be UTF-8"))
}
pub(super) fn lines<T>(
    text: &str,
    maximum: usize,
    mut parse: impl FnMut(&str) -> Result<T, Error>,
) -> Result<Vec<T>, Error> {
    let mut values = Vec::new();
    for line in text.lines().filter(|line| !line.trim().is_empty()) {
        if values.len() == maximum {
            return Err(Error::from_contract(
                Code::UsageQuestion,
                "audit input exceeds its configured case limit",
            ));
        }
        values.push(parse(line)?);
    }
    Ok(values)
}

pub(super) fn offline_options(args: &ArgMatches, command: &Command) -> Result<(), Error> {
    for id in args.ids() {
        if args.value_source(id.as_str()) == Some(ValueSource::CommandLine)
            && !command
                .get_arguments()
                .any(|argument| argument.get_id() == id)
            && !command.get_groups().any(|group| group.get_id() == id)
            && !["quiet", "color"].contains(&id.as_str())
        {
            return Err(super::options::usage(
                id.as_str(),
                "offline analysis options",
                "inference or incompatible output option",
            ));
        }
    }
    Ok(())
}

pub(super) fn output(
    args: &ArgMatches,
    cancel: &Cancel,
    report: &impl serde::Serialize,
) -> Result<(), Error> {
    if !super::options::switch(args, "quiet")? {
        let mut output = io::stdout().lock();
        let encoded = serde_json::to_vec(report)
            .map_err(|_| crate::core::local("the analysis report could not be encoded"))?;
        output
            .write_all(&encoded)
            .map_err(|error| super::output_error(error, cancel))?;
        output
            .write_all(b"\n")
            .map_err(|error| super::output_error(error, cancel))?;
    }
    Ok(())
}
