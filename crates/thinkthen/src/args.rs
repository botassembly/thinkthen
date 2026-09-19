//! The command line, as the clap types that parse it.

use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};

/// Put a decider model in the shell.
#[derive(Debug, Parser)]
#[command(
    name = thinkthen_core::NAME,
    about,
    disable_version_flag = true,
    arg_required_else_help = true
)]
pub(crate) struct Cli {
    /// Print the version and exit.
    #[arg(short = 'V', long = "version")]
    pub(crate) version: bool,

    /// The command to run.
    #[command(subcommand)]
    pub(crate) command: Option<Command>,
}

/// The verbs the tool answers to.
#[derive(Debug, Subcommand)]
pub(crate) enum Command {
    /// Answer a yes/no question about the evidence and set the exit code.
    ///
    /// The answer is a bare `true`, `false`, or `null`, and the exit code is 0
    /// for yes, 1 for no, and 3 for unresolved. Under `set -e` or `set -o
    /// pipefail` a no ends the script, so put the command in an `if`, a `case`,
    /// or a `||` list.
    ///
    /// A single cut answers no when the probability did not reach the mark. It
    /// never says the model is sure of no. A three-way gate takes a band, as
    /// `--threshold 0.1:0.9` writes one.
    ///
    /// A script that acts on the answer reads all four outcomes:
    ///
    /// thinkthen decide 'The customer asks for a refund.' --threshold 0.1:0.9 --quiet < m.txt
    ///
    /// case $? in 0) route refunds ;; 1) route support ;; 3) route triage ;; *) exit 4 ;; esac
    ///
    /// Word the question in the form where yes permits the action. A failure
    /// then never permits anything, because every outcome other than 0 leaves
    /// the action undone.
    Decide(DecideArguments),

    /// Pick one label from a fixed list and print it.
    ///
    /// The answer is a bare JSON string, or `null` when the winning option
    /// falls under `--threshold` or the top two options tie exactly. Exit 0 is
    /// a label and exit 3 is unresolved. `choose` never exits 1, because a pick
    /// is not a two-sided decision.
    ///
    /// A script reads the exit code first and the label second, so it takes two
    /// `case` blocks. `--raw` prints nothing at all for an unresolved answer,
    /// and an empty string is no label, so only the exit code tells an
    /// unresolved pick from a command that failed:
    ///
    /// label=$(thinkthen choose 'Which team owns this?' billing shipping other --raw < m.txt) && rc=0 || rc=$?
    ///
    /// case $rc in 0) ;; 3) label=unresolved ;; *) exit "$rc" ;; esac
    ///
    /// case $label in billing) pay ;; unresolved) triage ;; *) exit 2 ;; esac
    ///
    /// "Not stated" is a different answer from "false". "Does the document
    /// establish X?" and "Is X true?" are different questions. When the
    /// difference matters, ask `choose` with labels such as supported,
    /// contradicted, and not_stated rather than one yes/no question.
    ///
    /// Word the options so that they exclude one another, and type a catch-all
    /// such as other yourself, last. Keep the option order fixed once a cut is
    /// tuned, because a run with a reordered list is a different measurement.
    Choose(ChooseArguments),

    /// Place the evidence on named levels and print the number.
    ///
    /// The levels come lowest first, and the number is the backend's
    /// probability-weighted position on them, from 0 to the number of levels
    /// minus one. `score` takes no threshold, and no answer sets the exit code.
    /// `jq -e` cuts on the number in one line and sets one:
    ///
    /// thinkthen score 'How much disruption?' none workaround blocked < t.txt | jq -e '. >= 2' > /dev/null
    ///
    /// One number hides the shape of the distribution. The level odds 0, 1, 0
    /// and 0.5, 0, 0.5 both score 1. Read `--details` for the odds of every
    /// level when the difference matters.
    ///
    /// Measurement of the first decider model showed rubric judgments rejecting
    /// 18% to 46% of work people had accepted. A later run ordered forty made-up
    /// reports well and ran one level high on 9 of 40, so tune a cut on labeled
    /// cases. A number belongs in a review queue a person reads. A gate that has
    /// to hold belongs in `decide` or `choose`, and the Bash way to branch on
    /// levels is `choose` with the levels as ordered labels.
    Score(ScoreArguments),
}

impl Command {
    /// The options every judging verb takes, whichever verb was named.
    pub(crate) const fn common(&self) -> &Common {
        match self {
            Self::Decide(arguments) => &arguments.common,
            Self::Choose(arguments) => &arguments.common,
            Self::Score(arguments) => &arguments.common,
        }
    }
}

/// The options every judging verb takes.
#[derive(Args, Debug)]
pub(crate) struct Common {
    /// Print the full result object in place of the bare value.
    ///
    /// In record mode the object also carries `input`, the whole record as it
    /// arrived, so a row names the record it answered. `input` repeats every
    /// record in the output, and a run over large records pays for that on
    /// every row.
    #[arg(long)]
    pub(crate) details: bool,

    /// Read the records from FILE instead of from standard input.
    #[arg(long, value_name = "FILE")]
    pub(crate) input: Option<PathBuf>,

    /// Take each line as one text record.
    ///
    /// One value prints per record, in input order. The bare values alone tie
    /// no line to a record, so a script that names records reads --details.
    #[arg(long, conflicts_with = "jsonl")]
    pub(crate) lines: bool,

    /// Take each line as one JSON record.
    ///
    /// One value prints per record, in input order. The bare values alone tie
    /// no line to a record, so a script that names records reads --details.
    #[arg(long)]
    pub(crate) jsonl: bool,

    /// Send only the part of each record this RFC 6901 pointer names.
    ///
    /// Give it more than once to send an object of the named parts, keyed by
    /// the last part of each pointer. Without --jsonl it reads the whole input
    /// as one JSON value. The pointer is the disclosure boundary: only the
    /// pointed value leaves the machine.
    #[arg(long, value_name = "POINTER")]
    pub(crate) field: Vec<String>,

    /// Print what would be sent and stop. No key is read and no connection opens.
    #[arg(long)]
    pub(crate) dry_run: bool,

    /// The base the request is posted under, which outranks THINKTHEN_BASE_URL.
    #[arg(long, value_name = "URL", hide_short_help = true)]
    pub(crate) url: Option<String>,

    /// The model named in the request. [default: jev-latest]
    ///
    /// It outranks a `model` key in a question file.
    #[arg(long, value_name = "NAME", hide_short_help = true)]
    pub(crate) model: Option<String>,

    /// Call the backend, then write the exchange into DIR. DIR is created when absent.
    #[arg(long, value_name = "DIR", hide_short_help = true)]
    pub(crate) record: Option<PathBuf>,

    /// Answer from DIR alone. No connection opens, and no key is read.
    #[arg(long, value_name = "DIR", hide_short_help = true)]
    pub(crate) replay: Option<PathBuf>,

    /// Replay DIR and record into it, which resumes a run that stopped.
    ///
    /// It is --record DIR and --replay DIR together, so it stands beside
    /// neither of them. A finished record is answered from disk, and only the
    /// rest goes to the backend.
    #[arg(long, value_name = "DIR", hide_short_help = true)]
    pub(crate) cache: Option<PathBuf>,

    /// Seconds one attempt may take, from connect to the last byte.
    #[arg(
        long,
        value_name = "SECONDS",
        default_value_t = 30,
        hide_short_help = true
    )]
    pub(crate) timeout: u64,

    /// How many requests are in flight at once, from 1 to 32.
    ///
    /// It acts in record mode alone, because one document sends one request.
    /// Output never depends on it: a run with any number prints the bytes one
    /// job prints, in input order.
    #[arg(
        long,
        value_name = "N",
        value_parser = clap::builder::RangedU64ValueParser::<u8>::new().range(1..=32),
        hide_short_help = true
    )]
    pub(crate) jobs: Option<u8>,

    /// How many times a transport failure or a retried status is sent again.
    #[arg(long, value_name = "N", default_value_t = 2, hide_short_help = true)]
    pub(crate) max_retries: u32,
}

/// Everything `decide` was asked, before any of it is read.
#[derive(Args, Debug)]
pub(crate) struct DecideArguments {
    /// The question to answer, or `@` and the path of a question file.
    ///
    /// As text it is one argument naming one visible fact. As `@FILE` it is a
    /// question file holding one `decide` question, and a value typed beside
    /// it replaces the file's value. A question that must begin with `@` is
    /// written in a file.
    pub(crate) question: String,

    /// One sentence saying what a yes means, sent beside the question.
    #[arg(long = "true", value_name = "TEXT")]
    pub(crate) yes: Option<String>,

    /// One sentence saying what a no means, sent beside the question.
    #[arg(long = "false", value_name = "TEXT")]
    pub(crate) no: Option<String>,

    /// The rule: one cut T, or a band LOW:HIGH that leaves a middle unresolved.
    #[arg(long, value_name = "T|LOW:HIGH")]
    pub(crate) threshold: Option<String>,

    /// Print nothing on standard output. The exit code still carries the answer.
    #[arg(long)]
    pub(crate) quiet: bool,

    /// The options every judging verb takes.
    #[command(flatten)]
    pub(crate) common: Common,
}

/// Everything `choose` was asked, before any of it is read.
#[derive(Args, Debug)]
pub(crate) struct ChooseArguments {
    /// The question that states what decides the pick, or `@` and a file path.
    pub(crate) question: String,

    /// The labels to pick between, 2 to 255 of them, in the order they are sent.
    pub(crate) options: Vec<String>,

    /// One label and what it means, as LABEL=DESCRIPTION. Give it once per label.
    ///
    /// The first `=` splits the label from the description, and the
    /// description travels beside the label so the model reads both. It does
    /// not stand beside the positional labels, because two lists have no order
    /// between them.
    #[arg(long = "option", value_name = "LABEL=DESCRIPTION")]
    pub(crate) described: Vec<String>,

    /// Take each record's own options from this RFC 6901 pointer.
    ///
    /// The record holds a list of labels, or a map from each label to the
    /// description that travels with it. It needs --jsonl, because a pointer
    /// needs a JSON record, and it takes no list on the command line.
    #[arg(long = "options", value_name = "POINTER")]
    pub(crate) options_pointer: Option<String>,

    /// One cut T on the winning option's probability. A band is a usage error.
    #[arg(long, value_name = "T")]
    pub(crate) threshold: Option<String>,

    /// Print the label without quotation marks, and nothing when unresolved.
    #[arg(long)]
    pub(crate) raw: bool,

    /// Print nothing on standard output. The exit code still carries the answer.
    #[arg(long)]
    pub(crate) quiet: bool,

    /// The options every judging verb takes.
    #[command(flatten)]
    pub(crate) common: Common,
}

/// Everything `score` was asked, before any of it is read.
#[derive(Args, Debug)]
pub(crate) struct ScoreArguments {
    /// The question that names what is being placed, or `@` and a file path.
    pub(crate) question: String,

    /// The levels, 2 to 10 of them, lowest first.
    pub(crate) levels: Vec<String>,

    /// Taken so that the tool refuses it in its own words. `score` has no rule.
    ///
    /// Left to the parser, `--threshold` drew a tip naming `--record` and a
    /// usage line that read as if `--record` were required.
    #[arg(long, value_name = "T", hide = true)]
    pub(crate) threshold: Option<String>,

    /// The options every judging verb takes.
    #[command(flatten)]
    pub(crate) common: Common,
}
