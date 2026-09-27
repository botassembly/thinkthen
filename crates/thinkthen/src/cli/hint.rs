//! Fixed, input-free guidance for command lines that already fail.

use std::io::{self, Write};
use std::process::ExitCode;

use clap::error::{ContextKind, ContextValue, ErrorKind};

const CHOOSE_OR_TAG: &str = "`choose` picks one option, and `tag` names every label that fits";
const WRITES_NONE: &str = "thinkthen judges text and writes none";

const GUESSED: [(&str, &str); 7] = [
    ("grep", "`filter` keeps the records where the answer is yes"),
    (
        "if",
        "`decide` answers one yes or no question in its exit code",
    ),
    ("classify", CHOOSE_OR_TAG),
    ("switch", CHOOSE_OR_TAG),
    (
        "sort",
        "`rank` sorts records by how likely the answer is yes",
    ),
    ("summarize", WRITES_NONE),
    ("rewrite", WRITES_NONE),
];

pub(crate) const NOT_JSON_LINES: &str = "the record is not valid JSON; read a table with `--csv` or `--tsv`, and plain text with `--lines`";
pub(crate) const ONE_QUESTION: &str = "the question is one argument and each option takes one value; quote a question of several words, and send evidence on standard input or as `--input FILE`";

/// Replace clap's refusal only for an exact guessed command word.
pub(crate) fn refused(error: clap::Error) -> ExitCode {
    if error.kind() == ErrorKind::InvalidSubcommand
        && let Some(ContextValue::String(word)) = error.get(ContextKind::InvalidSubcommand)
        && let Some((known, tail)) = GUESSED.iter().find(|(known, _)| *known == word)
    {
        let mut writer = io::stderr().lock();
        let _unwritten = writeln!(writer, "thinkthen: `{known}` is not a command; {tail}")
            .and_then(|()| writer.flush());
        return ExitCode::from(2);
    }
    error.exit()
}
