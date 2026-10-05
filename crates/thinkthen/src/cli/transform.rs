//! The read-only catalog of built-in `jq` transforms.
//!
//! The command lists or prints bytes embedded at compile time. It never
//! applies, parses, or runs a transform, and it reads no setting, input, or
//! file. `sdlc/scripts/policy.py` holds this file to that.

use std::io::Write;

use clap::{Args, Subcommand};

use crate::failure::Failure;

#[derive(Args, Debug)]
pub(crate) struct TransformArguments {
    #[command(subcommand)]
    pub(crate) command: TransformCommand,
}

#[derive(Debug, Subcommand)]
pub(crate) enum TransformCommand {
    /// List the names of the built-in jq transforms.
    List,
    /// Print one built-in jq transform exactly as shipped.
    Show {
        /// The exact name `thinkthen transform list` prints.
        name: String,
    },
}

/// The closed catalog in bytewise ascending name order.
const CATALOG: [(&str, &[u8]); 10] = [
    ("band", include_bytes!("../../transforms/band.jq")),
    (
        "calibration",
        include_bytes!("../../transforms/calibration.jq"),
    ),
    ("compare", include_bytes!("../../transforms/compare.jq")),
    ("cost", include_bytes!("../../transforms/cost.jq")),
    ("counts", include_bytes!("../../transforms/counts.jq")),
    ("monitor", include_bytes!("../../transforms/monitor.jq")),
    ("score", include_bytes!("../../transforms/score.jq")),
    ("sweep", include_bytes!("../../transforms/sweep.jq")),
    ("triage", include_bytes!("../../transforms/triage.jq")),
    ("trials", include_bytes!("../../transforms/trials.jq")),
];

/// Write the list or one member to the writer and flush it.
pub(crate) fn run(command: &TransformCommand, mut writer: impl Write) -> Result<(), Failure> {
    let written = match command {
        TransformCommand::List => CATALOG
            .iter()
            .try_for_each(|(name, _)| writeln!(writer, "{name}")),
        TransformCommand::Show { name } => writer.write_all(lookup(name)?),
    };
    match written.and_then(|()| writer.flush()) {
        Err(error) if !Failure::closed_output(&error) => Err(Failure::Output(error)),
        _ => Ok(()),
    }
}

fn lookup(name: &str) -> Result<&'static [u8], Failure> {
    CATALOG
        .iter()
        .find(|(member, _)| *member == name)
        .map(|(_, bytes)| *bytes)
        .ok_or(Failure::Usage(
            "transform: unknown name; run `thinkthen transform list` to see the catalog",
        ))
}

#[cfg(test)]
mod tests {
    use super::{CATALOG, lookup};

    #[test]
    fn the_catalog_is_strictly_ascending_bytewise_and_free_of_duplicates() {
        assert!(
            CATALOG
                .windows(2)
                .all(|pair| pair[0].0.as_bytes() < pair[1].0.as_bytes())
        );
    }

    #[test]
    fn lookup_takes_only_an_exact_name() {
        assert!(lookup("band").is_ok());
        for name in ["Band", "band.jq", "./band", "ban", ""] {
            assert!(lookup(name).is_err(), "{name:?}");
        }
    }
}
