//! The command line, as the clap types that parse it.

use std::ffi::OsString;
use std::path::PathBuf;

use crate::core::{DEFAULT_MODEL, Framing};
use clap::{Args, Parser};

mod command;
mod find;
mod relate;
pub(crate) use command::{CacheCommand, CheckArguments, Command, PruneArguments, StatusArguments};
pub(crate) use find::FindArguments;
pub(crate) use relate::RelateArguments;

/// Semantic commands for the shell: if, grep, and sort that understand meaning
#[derive(Debug, Parser)]
#[command(
    name = crate::core::NAME,
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

/// The options every judging verb takes.
#[derive(Args, Debug)]
pub(crate) struct Common {
    /// Print one machine-readable run-facts line last on standard error.
    #[arg(long, hide_short_help = true)]
    pub(crate) facts: bool,

    /// Print the full result object in place of the bare value.
    ///
    /// In record mode the object also carries `input`, the whole record as it
    /// arrived, so each answer names the record it answered. `input` repeats
    /// every record in the output, and a run over large records pays for that
    /// on every answer.
    #[arg(long)]
    pub(crate) details: bool,

    /// Read the records from FILE instead of from standard input.
    #[arg(long, value_name = "FILE")]
    pub(crate) input: Option<PathBuf>,

    /// Take each line as one text record.
    ///
    /// Value verbs keep `input` beside `value`. Record-returning verbs return
    /// records. `annotate` enriches object records.
    #[arg(long, conflicts_with_all = ["jsonl", "csv", "tsv"])]
    pub(crate) lines: bool,

    /// Take each line as one JSON record.
    ///
    /// Value verbs keep `input` beside `value`. Record-returning verbs return
    /// records. `annotate` enriches object records.
    #[arg(long, conflicts_with_all = ["csv", "tsv"])]
    pub(crate) jsonl: bool,

    /// Read a comma-separated table with a required header row.
    ///
    /// Every cell is a string, and every result is JSONL. The tool never
    /// guesses this framing from a filename.
    #[arg(long, conflicts_with = "tsv")]
    pub(crate) csv: bool,

    /// Read a tab-separated table with a required header row.
    ///
    /// Every cell is a string, and every result is JSONL. The tool never
    /// guesses this framing from a filename.
    #[arg(long)]
    pub(crate) tsv: bool,

    /// Send only the part of each record this RFC 6901 pointer names.
    ///
    /// Give it more than once to send an object of the named parts, keyed by
    /// the last part of each pointer. On a command that accepts a single text,
    /// no record framing reads the whole input as one JSON value. On `filter`
    /// and `rank` it reads JSON Lines. The pointer is the disclosure boundary:
    /// only the pointed value leaves the machine.
    #[arg(long, value_name = "POINTER")]
    pub(crate) field: Vec<String>,

    /// Print what would be sent and stop. No key is read and no connection opens.
    #[arg(long)]
    pub(crate) dry_run: bool,

    /// The base the request is posted under, which outranks THINKTHEN_BASE_URL.
    #[arg(long, value_name = "URL")]
    pub(crate) url: Option<String>,

    /// Read enforceable backend limits and a calibration name from FILE.
    ///
    /// A profile never selects an address, model, key, adapter, or cache.
    #[arg(long, value_name = "FILE")]
    pub(crate) profile: Option<PathBuf>,

    // The default is the adapter's, so the sentence is built from the constant
    // rather than written again here. A doc comment cannot read a constant, so
    // clap is given the two texts as expressions instead.
    #[arg(
        long,
        value_name = "NAME",
        hide_short_help = true,
        help = format!("The model named in the request. [default: {DEFAULT_MODEL}]"),
        long_help = format!(
            "The model named in the request. [default: {DEFAULT_MODEL}]\n\n\
             It outranks a `model` key in a single question file. A question set holds no model."
        )
    )]
    pub(crate) model: Option<String>,

    /// Call the backend, then write the exchange into DIR. DIR is created when absent.
    ///
    /// An explicit recording folder suppresses the platform default cache. A
    /// folder that already holds an answer stops at exit 5 when the backend
    /// answers that request differently.
    #[arg(long, value_name = "DIR", hide_short_help = true)]
    pub(crate) record: Option<PathBuf>,

    /// Answer from DIR alone. No connection opens, and no key is read.
    ///
    /// An explicit replay folder suppresses the platform default cache.
    #[arg(long, value_name = "DIR", hide_short_help = true)]
    pub(crate) replay: Option<PathBuf>,

    /// Replay DIR and record into it, overriding THINKTHEN_CACHE and the platform default.
    ///
    /// It is --record DIR and --replay DIR together, so it stands beside
    /// neither of them. A finished record is answered from disk, and only the
    /// rest goes to the backend.
    #[arg(long, value_name = "DIR", hide_short_help = true)]
    pub(crate) cache: Option<PathBuf>,

    /// Do not read or write the answer cache for this run.
    ///
    /// Answers are otherwise cached by default in the platform cache folder.
    /// Entries contain the judged text. A new default folder is private to its
    /// owner. This option conflicts with --cache but may accompany an explicit
    /// --record or --replay folder.
    #[arg(long, conflicts_with = "cache")]
    pub(crate) no_cache: bool,

    /// Seconds from 1 to 86400 that bound one attempt from connect to last byte, and each retry wait.
    #[arg(
        long,
        value_name = "SECONDS",
        default_value_t = 30,
        hide_short_help = true
    )]
    pub(crate) timeout: u64,

    /// How many requests are in flight at once, from 1 to 32. [default: 4]
    ///
    /// It acts in record mode, on `annotate`, where a single text can make
    /// several grouped requests, and on `relate`, where each relation makes its
    /// own requests. Output follows the order the command defines.
    /// A run opens up to one connection for each request in flight, so --jobs N
    /// opens up to N connections.
    #[arg(
        long,
        value_name = "N",
        value_parser = clap::builder::RangedU64ValueParser::<u8>::new().range(1..=32),
        hide_short_help = true
    )]
    pub(crate) jobs: Option<u8>,

    /// How many times a retried status is sent again. A transport failure is never sent again.
    #[arg(long, value_name = "N", default_value_t = 3, hide_short_help = true)]
    pub(crate) max_retries: u32,
}

impl Common {
    /// The record framing the command line asked for.
    pub(crate) const fn framing(&self) -> Framing {
        if self.lines {
            Framing::Lines
        } else if self.jsonl {
            Framing::Jsonl
        } else if self.csv {
            Framing::Csv
        } else if self.tsv {
            Framing::Tsv
        } else {
            Framing::Document
        }
    }
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

    /// Taken so the command can say where the evidence goes.
    #[arg(value_name = "EVIDENCE", hide = true)]
    pub(crate) extra: Vec<OsString>,

    /// What a yes and a no mean.
    #[command(flatten)]
    pub(crate) meanings: Meanings,

    /// The rule: one cut T, or a band LOW:HIGH whose middle answers not sure.
    /// It defaults to 0.5.
    #[arg(long, value_name = "T|LOW:HIGH", allow_negative_numbers = true)]
    pub(crate) threshold: Option<String>,

    /// Print nothing on standard output. The exit code still carries the answer.
    #[arg(long)]
    pub(crate) quiet: bool,

    /// Taken so the command can name the command that prints a raw label.
    #[arg(long, hide = true)]
    pub(crate) raw: bool,

    /// How many records of a stream share one request.
    #[command(flatten)]
    pub(crate) batching: Batching,

    /// The options every judging verb takes.
    #[command(flatten)]
    pub(crate) common: Common,
}

/// The batch size of `decide`, `filter` and `rank` over a stream.
#[derive(Args, Debug)]
pub(crate) struct Batching {
    /// Share the exact UTF-8 contents of FILE as evidence for every batch.
    #[arg(long, value_name = "FILE", hide_short_help = true)]
    pub(crate) context: Option<PathBuf>,

    /// Send at most N records of a stream in one request, or `max`. [default: max]
    ///
    /// `max` fills each request to the backend's limits. `--batch 1` asks one
    /// record a request, as before batching. It beats `THINKTHEN_BATCH`, which
    /// beats a question file's `batch`.
    #[arg(long, value_name = "N|max", hide_short_help = true)]
    pub(crate) batch: Option<String>,

    /// Close a batch before its request exceeds N bytes. [default: 96000]
    #[arg(
        long,
        value_name = "N",
        hide_short_help = true,
        allow_negative_numbers = true
    )]
    pub(crate) max_request_bytes: Option<String>,
}

/// The two texts that say what a yes and a no mean, which every yes/no verb takes.
#[derive(Args, Debug)]
pub(crate) struct Meanings {
    /// One sentence saying what a yes means, sent beside the question.
    #[arg(long = "true", value_name = "TEXT")]
    pub(crate) yes: Option<String>,

    /// One sentence saying what a no means, sent beside the question.
    #[arg(long = "false", value_name = "TEXT")]
    pub(crate) no: Option<String>,
}

/// The two views `filter` and `rank` take so they can refuse them in their own words.
///
/// Left to the parser, `--quiet` drew "unexpected argument" and a tip about
/// another option. The tool says which command carries the view instead.
#[derive(Args, Debug)]
pub(crate) struct Refused {
    /// Taken so that the tool refuses it in its own words.
    #[arg(long, hide = true)]
    pub(crate) quiet: bool,

    /// Taken so that the tool refuses it in its own words.
    #[arg(long, hide = true)]
    pub(crate) raw: bool,
}

/// Everything `filter` was asked, before any of it is read.
#[derive(Args, Debug)]
pub(crate) struct FilterArguments {
    /// The question asked of each record, or `@` and the path of a question file.
    ///
    /// As `@FILE` it is a question file holding one `decide` question, and a
    /// value typed beside it replaces the file's value.
    pub(crate) question: String,

    /// Taken so the command can say where the evidence goes.
    #[arg(value_name = "EVIDENCE", hide = true)]
    pub(crate) extra: Vec<OsString>,

    /// One cut T on the probability of yes. A band is a usage error. It defaults to 0.5.
    #[arg(long, value_name = "T", allow_negative_numbers = true)]
    pub(crate) threshold: Option<String>,

    /// Taken so the command can explain that only an order can be cut.
    #[arg(long, value_name = "N", hide = true, allow_negative_numbers = true)]
    pub(crate) top: Option<String>,

    /// What a yes and a no mean.
    #[command(flatten)]
    pub(crate) meanings: Meanings,

    /// The two views `filter` refuses in its own words.
    #[command(flatten)]
    pub(crate) refused: Refused,

    /// How many records of a stream share one request.
    #[command(flatten)]
    pub(crate) batching: Batching,

    /// The options every judging verb takes.
    #[command(flatten)]
    pub(crate) common: Common,
}

/// Everything `rank` was asked, before any of it is read.
#[derive(Args, Debug)]
pub(crate) struct RankArguments {
    /// The question asked of each record, or `@` and the path of a question file.
    ///
    /// As `@FILE` it is a question file holding one `decide` question, and a
    /// value typed beside it replaces the file's value.
    pub(crate) question: String,

    /// Taken so the command can say where the evidence goes.
    #[arg(value_name = "EVIDENCE", hide = true)]
    pub(crate) extra: Vec<OsString>,

    /// Print the first N records of the order, from 1 upward.
    ///
    /// It saves no request, because every record is judged before anything is
    /// sorted.
    #[arg(long, value_name = "N", allow_negative_numbers = true)]
    pub(crate) top: Option<String>,

    /// Taken so that the tool refuses it in its own words. `rank` has no rule.
    #[arg(long, value_name = "T", hide = true, allow_negative_numbers = true)]
    pub(crate) threshold: Option<String>,

    /// What a yes and a no mean.
    #[command(flatten)]
    pub(crate) meanings: Meanings,

    /// The two views `rank` refuses in its own words.
    #[command(flatten)]
    pub(crate) refused: Refused,

    /// How many records of a stream share one request.
    #[command(flatten)]
    pub(crate) batching: Batching,

    /// The options every judging verb takes.
    #[command(flatten)]
    pub(crate) common: Common,
}

/// Everything `choose` was asked, before any of it is read.
#[derive(Args, Debug)]
pub(crate) struct ChooseArguments {
    /// The question that states what decides the pick, or `@` and a file path.
    pub(crate) question: String,

    /// The options to pick between, 2 to 255 of them, in the order they are sent.
    pub(crate) options: Vec<String>,

    /// One option and what it means, as LABEL=DESCRIPTION. Give it once per option.
    ///
    /// The first `=` splits the option from the description, and the
    /// description travels beside the option so the model reads both. It does
    /// not stand beside the positional options, because two lists have no
    /// order between them.
    #[arg(long = "option", value_name = "LABEL=DESCRIPTION")]
    pub(crate) described: Vec<String>,

    /// Take each record's own options from this RFC 6901 pointer.
    ///
    /// The record holds a list of options, or a map from each option to the
    /// description that travels with it. It needs --jsonl, because a pointer
    /// needs a JSON record, and it takes no list on the command line.
    #[arg(long = "options", value_name = "POINTER")]
    pub(crate) options_pointer: Option<String>,

    /// One cut T on the winning option's probability. A band is a usage error.
    #[arg(long, value_name = "T", allow_negative_numbers = true)]
    pub(crate) threshold: Option<String>,

    /// Print the option without quotation marks, and nothing when not sure.
    ///
    /// This view is available for a single text, --lines, and --jsonl. CSV and
    /// TSV always print JSONL and refuse --raw.
    #[arg(long)]
    pub(crate) raw: bool,

    /// Print nothing on standard output. The exit code still carries the answer.
    #[arg(long)]
    pub(crate) quiet: bool,

    /// The options every judging verb takes.
    #[command(flatten)]
    pub(crate) common: Common,
}

/// Everything `tag` was asked, before any of it is read.
#[derive(Args, Debug)]
pub(crate) struct TagArguments {
    /// The question that frames the labels, or `@` and a question-file path.
    pub(crate) question: String,

    /// The labels to test independently, 1 to 20, in request and output order.
    #[arg(value_name = "LABEL")]
    pub(crate) labels: Vec<String>,

    /// One label and what it means, as LABEL=DESCRIPTION. Give it once per label.
    #[arg(long = "label", value_name = "LABEL=DESCRIPTION")]
    pub(crate) described: Vec<String>,

    /// One cut applied to every label's probability of yes. It defaults to 0.5.
    #[arg(long, value_name = "T", allow_negative_numbers = true)]
    pub(crate) threshold: Option<String>,

    /// Taken so the command can explain that its output is always JSON.
    #[arg(long, hide = true)]
    pub(crate) raw: bool,

    /// Taken so the command can explain that an empty array is an answer.
    #[arg(long, hide = true)]
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
    #[arg(long, value_name = "T", hide = true, allow_negative_numbers = true)]
    pub(crate) threshold: Option<String>,

    /// Taken so the command can explain that a number has no answer exit code.
    #[arg(long, hide = true)]
    pub(crate) quiet: bool,

    /// Taken so the command can name the command that prints a raw label.
    #[arg(long, hide = true)]
    pub(crate) raw: bool,

    /// The options every judging verb takes.
    #[command(flatten)]
    pub(crate) common: Common,
}

/// Everything `recognize` was asked before its input is read.
#[derive(Args, Debug)]
pub(crate) struct RecognizeArguments {
    /// Kinds to assign, or one `@FILE` recognize question file.
    #[arg(value_name = "KIND")]
    pub(crate) kinds: Vec<String>,

    /// One kind and what it means, as KIND=DESCRIPTION.
    #[arg(long = "kind", value_name = "KIND=DESCRIPTION")]
    pub(crate) described: Vec<String>,

    /// A beta relation rule, as NAME or NAME=SOURCE:TARGET. `*` or `ANY` means any kind.
    #[arg(long = "relation", value_name = "NAME=SOURCE:TARGET")]
    pub(crate) relations: Vec<String>,

    /// Keep names whose computed strength reaches this cut. [default: 0.5]
    #[arg(long, value_name = "T", allow_negative_numbers = true)]
    pub(crate) threshold: Option<String>,

    /// Keep beta relation edges whose model probability reaches this cut. [default: 0.5]
    #[arg(long, value_name = "T", allow_negative_numbers = true)]
    pub(crate) relation_threshold: Option<String>,

    /// Refuse a text over this many UTF-8 bytes before any request. [default: 600000]
    #[arg(
        long,
        value_name = "N",
        value_parser = clap::builder::RangedU64ValueParser::<usize>::new().range(1..=9_007_199_254_740_991),
        hide_short_help = true
    )]
    pub(crate) max_text_bytes: Option<usize>,

    /// Split relation plans before a request exceeds N bytes. [default: 96000]
    #[arg(
        long,
        value_name = "N",
        hide_short_help = true,
        allow_negative_numbers = true
    )]
    pub(crate) max_request_bytes: Option<String>,

    /// The options every judging verb takes.
    #[command(flatten)]
    pub(crate) common: Common,
}

/// Everything `annotate` was asked, before the set or evidence is read.
#[derive(Args, Debug)]
pub(crate) struct AnnotateArguments {
    /// A JSON question set containing the named questions to ask.
    pub(crate) questions: PathBuf,

    /// A likely input file written without `--input`.
    #[arg(value_name = "INPUT", hide = true)]
    pub(crate) extra_input: Option<PathBuf>,

    /// Taken so the command can explain that thresholds belong to questions.
    #[arg(long, value_name = "T", hide = true, allow_negative_numbers = true)]
    pub(crate) threshold: Option<String>,

    /// Taken so the command can explain that every named answer prints.
    #[arg(long, hide = true)]
    pub(crate) quiet: bool,

    /// Taken so the command can explain that its output is always JSON.
    #[arg(long, hide = true)]
    pub(crate) raw: bool,

    /// The options shared with record-oriented judging commands.
    #[command(flatten)]
    pub(crate) common: Common,
}

#[cfg(test)]
mod annotate_tests {
    use super::{Cli, Command};
    use clap::Parser as _;

    #[test]
    fn tag_accepts_plain_labels() {
        let cli = Cli::try_parse_from(["thinkthen", "tag", "Which topics?", "billing"])
            .expect("valid tag command");
        let Some(Command::Tag(arguments)) = cli.command else {
            panic!("tag command");
        };
        assert_eq!(arguments.labels, ["billing"]);
    }

    #[test]
    fn annotate_accepts_a_question_set_and_record_options() {
        let cli = Cli::try_parse_from([
            "thinkthen",
            "annotate",
            "checks.json",
            "--jsonl",
            "--field",
            "/body",
            "--details",
            "--jobs",
            "4",
        ])
        .expect("valid annotate command");
        let Some(Command::Annotate(arguments)) = cli.command else {
            panic!("annotate command");
        };
        assert_eq!(arguments.questions.to_string_lossy(), "checks.json");
        assert!(arguments.extra_input.is_none());
        assert!(arguments.common.jsonl);
        assert_eq!(arguments.common.field, ["/body"]);
        assert_eq!(arguments.common.jobs, Some(4));
    }
}
