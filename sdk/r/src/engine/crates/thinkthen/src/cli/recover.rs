//! Offline export of completed typed rows, without client or credential setup.
use crate::Error;
use crate::files::FileError;
use crate::files::recovery::{self, Frame};
use clap::{Arg, ArgMatches, Command};
use serde_json::json;
use std::io::{self, Read, Seek, SeekFrom, Write};
use std::path::Path;

pub(super) fn command() -> Command {
    Command::new("recover")
        .about("Export completed typed answers from a recovery artifact without model calls")
        .arg(Arg::new("artifact").required(true).value_name("FILE"))
}

pub(super) fn run(args: &ArgMatches) -> Result<u8, Error> {
    for id in args.ids() {
        if id.as_str() != "artifact"
            && args.value_source(id.as_str()) == Some(clap::parser::ValueSource::CommandLine)
        {
            return Err(super::options::usage(
                "recover",
                "only a recovery artifact",
                "execution options",
            ));
        }
    }
    let path = args
        .get_one::<String>("artifact")
        .ok_or_else(|| super::local("a recovery artifact is required"))?;
    export(Path::new(path)).map_err(|error| super::local(&error.to_string()))
}

fn export(path: &Path) -> Result<u8, FileError> {
    let mut admitted = None;
    let mut occurrences = 0_u64;
    let mut committed = 0_u64;
    let mut positions = tempfile::tempfile().map_err(local)?;
    let mut closing = None;
    let mut output = io::stdout().lock();
    let torn_tail = recovery::read(path, |frame| {
        if closing.is_some() {
            return Err(local("recovery contains data after its closing frame"));
        }
        match frame {
            Frame::Manifest {
                admitted: count, ..
            } if admitted.is_none() => {
                positions.set_len(count).map_err(local)?;
                admitted = Some(count);
            }
            Frame::Occurrence { position, .. }
                if admitted.is_some_and(|count| position <= count)
                    && position == occurrences + 1
                    && committed == 0 =>
            {
                occurrences += 1
            }
            Frame::Row { position, row }
                if admitted == Some(occurrences) && position > 0 && position <= occurrences =>
            {
                positions
                    .seek(SeekFrom::Start(position - 1))
                    .map_err(local)?;
                let mut present = [0];
                positions.read_exact(&mut present).map_err(local)?;
                if present != [0] {
                    return Err(local("duplicate recovery row position"));
                }
                positions
                    .seek(SeekFrom::Start(position - 1))
                    .map_err(local)?;
                positions.write_all(&[1]).map_err(local)?;
                committed += 1;
                let typed: crate::results::AnyRow = serde_json::from_value(row).map_err(local)?;
                if typed.position() != position {
                    return Err(local("recovery row position does not match its frame"));
                }
                serde_json::to_writer(&mut output, &typed).map_err(local)?;
                output.write_all(b"\n").map_err(local)?;
            }
            Frame::Closing {
                complete,
                emitted,
                output,
            } if admitted == Some(occurrences) => {
                if emitted != committed {
                    return Err(local(
                        "recovery closing count does not match its emitted rows",
                    ));
                }
                closing = Some((complete, output))
            }
            _ => return Err(local("invalid recovery frame order or position")),
        }
        Ok(())
    })?;
    if closing.is_some() && torn_tail {
        return Err(local("recovery contains data after its closing frame"));
    }
    let admitted = admitted.ok_or_else(|| local("recovery has no manifest"))?;
    if admitted != occurrences {
        return Err(local("recovery input occurrence map is incomplete"));
    }
    let complete = closing.as_ref().is_some_and(|(complete, _)| *complete) && !torn_tail;
    output.flush().map_err(local)?;
    let report = json!({"type":"recovery", "admitted":admitted,"committed":committed,"complete":complete,"operation_status":if complete {"complete"} else {"incomplete"},"torn_tail":torn_tail,"completed_output":closing.and_then(|(_, path)| path)});
    let mut stderr = io::stderr().lock();
    let encoded = serde_json::to_string(&report).map_err(local)?;
    stderr
        .write_all(
            encoded
                .strip_suffix('}')
                .ok_or_else(|| local("invalid recovery summary"))?
                .as_bytes(),
        )
        .map_err(local)?;
    let ranges: &[u8] = if complete {
        b",\"missing_ranges\":[],\"unemitted_ranges\":["
    } else {
        b",\"unemitted_ranges\":[],\"missing_ranges\":["
    };
    stderr.write_all(ranges).map_err(local)?;
    positions.rewind().map_err(local)?;
    let mut start = None;
    let mut first = true;
    for position in 1..=admitted {
        let mut present = [0];
        positions.read_exact(&mut present).map_err(local)?;
        if present == [0] {
            start.get_or_insert(position);
        } else if let Some(begin) = start.take() {
            write_range(&mut stderr, &mut first, begin, position - 1)?;
        }
    }
    if let Some(begin) = start {
        write_range(&mut stderr, &mut first, begin, admitted)?;
    }
    stderr.write_all(b"]}\n").map_err(local)?;
    Ok(0)
}

fn local(error: impl std::fmt::Display) -> FileError {
    FileError::Local(error.to_string())
}

fn write_range(
    output: &mut impl Write,
    first: &mut bool,
    begin: u64,
    end: u64,
) -> Result<(), FileError> {
    if !*first {
        output.write_all(b",").map_err(local)?;
    }
    *first = false;
    serde_json::to_writer(output, &(begin, end)).map_err(local)
}
