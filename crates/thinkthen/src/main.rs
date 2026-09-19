//! The `thinkthen` command line.
//!
//! This crate owns every edge: arguments, standard output, and the exit code.
//! It hands typed values to `thinkthen-core` and prints what comes back.

#![forbid(unsafe_code)]

use std::io::{self, Write};
use std::process::ExitCode;

use clap::Parser;

/// Put a decider model in the shell.
#[derive(Debug, Parser)]
#[command(
    name = thinkthen_core::NAME,
    about,
    disable_version_flag = true,
    arg_required_else_help = true
)]
struct Cli {
    /// Print the version and exit.
    #[arg(short = 'V', long = "version")]
    version: bool,
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    let stdout = io::stdout();
    match run(&cli, stdout.lock()) {
        Ok(()) => ExitCode::SUCCESS,
        Err(_) => ExitCode::FAILURE,
    }
}

fn run(cli: &Cli, mut writer: impl Write) -> io::Result<()> {
    if cli.version {
        let line = thinkthen_core::version_line(env!("CARGO_PKG_VERSION"));
        writeln!(writer, "{line}")?;
    }
    writer.flush()
}
