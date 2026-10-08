//! The verbs and their command-level help.

use std::path::PathBuf;

use clap::{Args, Subcommand};

use super::{
    AnnotateArguments, CheckArguments, ChooseArguments, DecideArguments, FilterArguments,
    FindArguments, RankArguments, RecognizeArguments, RelateArguments, ScoreArguments,
    TagArguments,
};

/// The verbs the tool answers to.
///
/// Help lists the ten functions in enum order, then clap's own `help`, which
/// clap numbers 999, then the admin commands, numbered from 1000.
#[derive(Debug, Subcommand)]
pub(crate) enum Command {
    /// Report resolved local settings, cache size, and local usage counts.
    #[command(display_order = 1002)]
    Status(StatusArguments),

    /// Serve the ten judging functions over local MCP stdio.
    #[command(display_order = 1006)]
    Mcp(crate::mcp::startup::Arguments),

    /// Check that a named backend works with this tool.
    #[command(display_order = 1003, subcommand)]
    Backends(super::backends::BackendCommand),

    /// Published compatibility spelling for backends check.
    #[command(hide = true)]
    Check(CheckArguments),

    /// Answer one yes or no question about a text. A record run exits 0 when
    /// it completes without a partial or whole-run failure. The printed
    /// values carry the individual answers.
    ///
    /// Examples:
    ///
    /// printf 'Refund me please.' | thinkthen decide 'Does this ask for a refund?'
    ///
    /// printf 'Refund me please.' | thinkthen decide 'Does this ask for a refund?' --threshold 0.1:0.9
    ///
    /// The answer is a bare `true`, `false`, or `null`. The exit code is 0 for
    /// yes, 1 for no, 3 for not sure, and any other code when the run is broken
    /// or interrupted. Under `set -e` or `set -o pipefail` a no or not sure
    /// answer ends the script, so put the command in an `if`, a `case`, or a
    /// `||` list.
    ///
    /// A single cut answers no when the probability did not reach the cut. It
    /// never says the model is sure of no. A three-way gate takes a band, as
    /// `--threshold 0.1:0.9` writes one.
    ///
    /// A script that acts on the answer reads all four outcomes:
    ///
    /// thinkthen decide 'The customer asks for a refund.' --threshold 0.1:0.9 --quiet < m.txt && refund_code=0 || refund_code=$?
    ///
    /// case $refund_code in 0) route refunds ;; 1) route support ;; 3) route triage ;; *) exit 4 ;; esac
    ///
    /// Word the question in the form where yes permits the action. A broken run
    /// then never permits anything, because every outcome other than 0 leaves
    /// the action undone.
    Decide(DecideArguments),

    /// Keep the records where the answer is yes.
    ///
    /// `filter` asks one yes/no question of each record and prints the records
    /// that reach `--threshold` in input order. Line and JSONL records return
    /// as they arrived; CSV and TSV records become compact JSON objects. With no
    /// framing flag it reads lines, or JSON Lines when a pointer is given by
    /// --field or a question file's `on`. Records share requests by default;
    /// --batch 1 asks one record per request.
    ///
    /// A single cut keeps or drops, and there is no third pile. A run that
    /// wants one asks `decide --details` and splits with `jq`:
    ///
    /// thinkthen decide 'The report is reproducible.' --jsonl --field /body --details < i.jsonl | jq -c 'select(.answer.probability >= 0.9)'
    ///
    /// A finished run prints nothing on standard error, so two kept records
    /// out of five and two out of two look alike on the way out.
    ///
    /// A record run exits 0 when it completes without a partial or whole-run
    /// failure. The printed values carry the individual answers.
    Filter(FilterArguments),

    /// Sort records by how likely the answer is yes, or give `rank` a saved
    /// score question to order by its weighted level value. It judges each
    /// record separately and sorts locally; it never compares two records.
    ///
    /// The printed order puts the highest value first. An exact tie keeps
    /// input order. `rank` never runs a tournament.
    ///
    /// It prints only after the input ends. Without `--top`, it holds every
    /// scored record; with `--top N`, it keeps N winners and bounded work in
    /// flight. Cut an endless stream into windows upstream. Every record is
    /// still judged, so `--top` saves no request on a completed input.
    ///
    /// `rank` orders and never selects. A floor is `filter` in front of it. With
    /// no framing flag it reads lines, or JSON Lines when a pointer is given by
    /// --field or a question file's `on`.
    ///
    /// A record run exits 0 when it completes without a partial or whole-run
    /// failure. The printed values carry the individual answers.
    Rank(RankArguments),

    /// Pick one option from your list.
    ///
    /// Use `tag` when more than one answer can apply.
    ///
    /// The answer is a bare JSON string, or `null` when the winning option
    /// falls under `--threshold` or the top two options tie exactly. Exit 0 is
    /// an option and exit 3 is not sure. `choose` never exits 1, because a pick
    /// is not a two-sided decision.
    ///
    /// --raw is available for a single text, --lines, and --jsonl. CSV and TSV
    /// always print JSONL and refuse --raw.
    ///
    /// A script reads the exit code first and the option second, so it takes
    /// two `case` blocks. `--raw` prints nothing at all for a not sure answer,
    /// and an empty string is no option, so only the exit code tells a not
    /// sure pick from a broken run:
    ///
    /// team=$(thinkthen choose 'Which team owns this?' billing shipping other --raw < m.txt) && team_code=0 || team_code=$?
    ///
    /// case $team_code in 0) ;; 3) team=not_sure ;; *) exit "$team_code" ;; esac
    ///
    /// case $team in billing) pay ;; not_sure) triage ;; *) exit 2 ;; esac
    ///
    /// "Not stated" is a different answer from "false". "Does the text
    /// establish X?" and "Is X true?" are different questions. When the
    /// difference matters, ask `choose` with options such as supported,
    /// contradicted, and not_stated rather than one yes/no question.
    ///
    /// Word the options so that they exclude one another, and type a catch-all
    /// such as other yourself, last. Keep the option order fixed once a cut is
    /// tuned, because a run with a reordered list is a different measurement.
    ///
    /// A record run exits 0 when it completes without a partial or whole-run
    /// failure. The printed values carry the individual answers.
    Choose(ChooseArguments),

    /// Pick the one line or record that best answers a question. Every line or
    /// record leaves together and sees every other one.
    ///
    /// Every line or record leaves together in one request and sees every other
    /// one. Input defaults to lines; --jsonl reads records and --field selects
    /// what the model sees. The set holds 2 to 255 lines or records, or 2 to
    /// 254 with --none, and at most 16 MiB across the original input.
    ///
    /// `find --none` prints nothing and exits 3 when `none` wins or ties for
    /// first.
    Find(FindArguments),

    /// Place a text on a scale you name.
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
    /// Measurement of the first System One model showed rubric scores rejecting
    /// 18% to 46% of work people had accepted. A later run ordered forty made-up
    /// reports well and ran one level high on 9 of 40, so tune a cut on labeled
    /// cases. A number belongs in a review queue a person reads. A gate that has
    /// to hold belongs in `decide` or `choose`, and the Bash way to branch on
    /// levels is `choose` with the levels as ordered options.
    ///
    /// A record run exits 0 when it completes without a partial or whole-run
    /// failure. The printed values carry the individual answers.
    Score(ScoreArguments),

    /// Name every label that fits.
    ///
    /// The answer is one JSON array holding every applicable label. A record
    /// run exits 0 when it completes without a partial or whole-run failure.
    /// The printed values carry the individual answers.
    #[command(
        after_help = "Examples:\n\nthinkthen tag 'Which topics?' --label billing='About charges.' --label urgent='Needs prompt attention.' < message.txt\nthinkthen tag 'Which topics?' billing urgent < message.txt\n"
    )]
    Tag(TagArguments),

    /// Answer a saved set of questions about every record.
    ///
    /// The answer is one annotated JSON object. A record run exits 0 when it
    /// completes without a partial or whole-run failure. The printed values
    /// carry the individual answers. A completed run with one or more failed
    /// questions exits 6. In JSONL detailed batch-one mode, --on-error
    /// continue emits a safe error record for a missing on pointer and continues;
    /// a completed run with skipped records exits 7.
    #[command(
        after_help = "Examples:\n\nthinkthen annotate checks.json < message.txt\nthinkthen annotate checks.json --input message.txt\nthinkthen annotate checks.json --jsonl --details --batch 1 --on-error continue < cases.jsonl\n"
    )]
    Annotate(AnnotateArguments),

    /// Find every name in a text and assign one of the given kinds.
    ///
    /// Relations are beta. `--threshold` gates computed name strength;
    /// `--relation-threshold` gates a relation's model probability.
    ///
    /// Without custom wording, the default questions seek names of people, organisations, places, products, works, events or other things. --instructions, --entity-definition or described --kind values define the entity task for every recognition step. Accuracy outside the measured examples is unknown.
    ///
    /// Each record can make paid requests in three steps: one boundary question
    /// per text piece; one kind question per found name when kinds are given,
    /// plus an edge question when its span can change; then questions for the
    /// relation pairs allowed by rules. --plan prints the first record's exact
    /// boundary requests and upper bounds for the whole input.
    ///
    /// A record run exits 0 when it completes without a partial or whole-run
    /// failure. The printed values carry the individual answers.
    Recognize(RecognizeArguments),

    /// Find named relations across one complete entity set.
    ///
    /// Relations are beta. Every entity leaves together as one complete entity
    /// set and sees every other entity admitted by a rule. Inline rules use
    /// NAME=SOURCE_KIND:TARGET_KIND. A bare NAME means NAME=*:*.
    ///
    /// A run makes paid requests. A relation between two kinds asks one
    /// question for every entity of the larger kind, or of the source kind when
    /// the counts are equal. A same-kind relation asks one yes-or-no question
    /// for every pair, in both directions unless --either. --plan prints the
    /// questions and requests and sends nothing.
    ///
    /// A run that answers some relation questions and fails others prints what
    /// it has and exits 6. A run whose relation questions all fail prints
    /// nothing and exits 4.
    Relate(RelateArguments),

    /// Inspect and maintain answer-cache folders without sending a request.
    ///
    /// The cache never trims itself. It grows until you run cache prune, and
    /// it holds the judged text until then.
    #[command(display_order = 1004)]
    Cache(CacheArguments),

    /// List or print the built-in jq transforms without running them.
    #[command(display_order = 1005)]
    Transform(crate::cli::transform::TransformArguments),

    /// Grade or compare saved answers without sending a request.
    #[command(display_order = 1000, subcommand)]
    Runs(super::runs::RunsCommand),

    /// Published compatibility spelling for runs audit.
    #[command(hide = true)]
    Audit(crate::cli::audit::AuditArguments),

    /// Published compatibility spelling for runs diff.
    #[command(hide = true)]
    Diff(crate::cli::diff::DiffArguments),
}

#[derive(Args, Debug)]
pub(crate) struct CacheArguments {
    #[command(subcommand)]
    pub(crate) command: CacheCommand,
}

#[derive(Debug, Subcommand)]
pub(crate) enum CacheCommand {
    /// Remove selected answers, then the oldest answers until thinkthen.sqlite fits the size target.
    Prune(PruneArguments),
    /// Report stored question keys absent from a complete caller-supplied key list.
    Unused(UnusedArguments),
    /// Merge a folder's old entries, live file and fixture into thinkthen.jsonl.
    Convert(ConvertArguments),
}

#[derive(Args, Debug)]
pub(crate) struct ConvertArguments {
    /// The recording or cache folder to convert. Its old files stay.
    pub(crate) directory: PathBuf,
    /// Also write each single-record exchange in the quoted form.
    #[arg(long)]
    pub(crate) quote: bool,
}

#[derive(Args, Debug)]
pub(crate) struct UnusedArguments {
    /// The existing cache or recording folder to inspect.
    pub(crate) directory: PathBuf,
    /// A UTF-8 file with one lowercase question key per line.
    #[arg(long, value_name = "KEYS")]
    pub(crate) used: PathBuf,
}

#[derive(Args, Debug)]
pub(crate) struct PruneArguments {
    /// The cache or recording folder to maintain.
    pub(crate) directory: PathBuf,
    /// Show selected question keys and allocated bytes without changing the folder.
    #[arg(long)]
    pub(crate) dry_run: bool,
    /// Trim to this many allocated bytes. Without it, the configuration's
    /// cache_bytes applies, or 100000000.
    #[arg(long, value_name = "BYTES")]
    pub(crate) max_size: Option<String>,
    /// Remove answers strictly older than a duration such as 30d or 12h.
    #[arg(long, value_name = "Nd|Nh|Nm|Ns")]
    pub(crate) older_than: Option<String>,
    /// Remove answers whose reply names another model. Give the version that
    /// answered, as a result's meta.model shows it, not the alias passed to
    /// --model. A name no reply in the folder carries is refused, and nothing
    /// is removed.
    #[arg(long, value_name = "MODEL")]
    pub(crate) answered_by_other_than: Option<String>,
}

impl Command {
    /// Settle positional file routes before opening any evidence.
    pub(crate) fn route_inputs(&mut self) -> Result<(), crate::failure::Failure> {
        use crate::failure::Failure;
        let (common, paths) = match self {
            Self::Decide(a) => (
                &mut a.common,
                a.extra.drain(..).map(PathBuf::from).collect(),
            ),
            Self::Filter(a) => (
                &mut a.common,
                a.extra.drain(..).map(PathBuf::from).collect(),
            ),
            Self::Rank(a) => (
                &mut a.common,
                a.extra.drain(..).map(PathBuf::from).collect(),
            ),
            Self::Annotate(a) => (&mut a.common, std::mem::take(&mut a.extra_input)),
            Self::Recognize(_) | Self::Relate(_) => return Ok(()),
            _ => return Ok(()),
        };
        if !paths.is_empty() && !common.input.is_empty() {
            return Err(Failure::Usage("positional files cannot accompany --input"));
        }
        common.input.extend(paths);
        Ok(())
    }

    /// Whether a one-question verb received a loose second argument.
    pub(crate) fn stray(&self) -> bool {
        match self {
            Self::Decide(arguments) => !arguments.extra.is_empty(),
            Self::Filter(arguments) => !arguments.extra.is_empty(),
            Self::Rank(arguments) => !arguments.extra.is_empty(),
            Self::Find(arguments) => !arguments.extra.is_empty(),
            _ => false,
        }
    }

    /// Whether the command line itself selected JSON Lines with no pointer.
    pub(crate) fn typed_jsonl(&self) -> bool {
        match self {
            Self::Find(arguments) => arguments.common.jsonl && arguments.common.field.is_empty(),
            Self::Decide(arguments) => arguments.common.jsonl && arguments.common.field.is_empty(),
            Self::Choose(arguments) => arguments.common.jsonl && arguments.common.field.is_empty(),
            Self::Tag(arguments) => arguments.common.jsonl && arguments.common.field.is_empty(),
            Self::Score(arguments) => arguments.common.jsonl && arguments.common.field.is_empty(),
            Self::Filter(arguments) => arguments.common.jsonl && arguments.common.field.is_empty(),
            Self::Rank(arguments) => arguments.common.jsonl && arguments.common.field.is_empty(),
            Self::Annotate(arguments) => {
                arguments.common.jsonl && arguments.common.field.is_empty()
            }
            Self::Recognize(arguments) => {
                arguments.common.jsonl && arguments.common.field.is_empty()
            }
            _ => false,
        }
    }

    pub(crate) const fn reads_input(&self) -> bool {
        !matches!(
            self,
            Self::Cache(_)
                | Self::Status(_)
                | Self::Mcp(_)
                | Self::Check(_)
                | Self::Backends(_)
                | Self::Transform(_)
                | Self::Runs(_)
                | Self::Audit(_)
                | Self::Diff(_)
        )
    }

    /// The input file one command named, if any.
    pub(crate) fn input(&self) -> Option<&std::path::Path> {
        match self {
            Self::Decide(arguments) => arguments.common.input.first().map(PathBuf::as_path),
            Self::Choose(arguments) => arguments.common.input.first().map(PathBuf::as_path),
            Self::Tag(arguments) => arguments.common.input.first().map(PathBuf::as_path),
            Self::Score(arguments) => arguments.common.input.first().map(PathBuf::as_path),
            Self::Filter(arguments) => arguments.common.input.first().map(PathBuf::as_path),
            Self::Rank(arguments) => arguments.common.input.first().map(PathBuf::as_path),
            Self::Find(arguments) => arguments.common.input.first().map(PathBuf::as_path),
            Self::Annotate(arguments) => arguments.common.input.first().map(PathBuf::as_path),
            Self::Recognize(arguments) => arguments.common.input.first().map(PathBuf::as_path),
            Self::Relate(arguments) => arguments.common.input.first().map(PathBuf::as_path),
            Self::Cache(_)
            | Self::Status(_)
            | Self::Mcp(_)
            | Self::Check(_)
            | Self::Backends(_)
            | Self::Transform(_)
            | Self::Runs(_)
            | Self::Audit(_)
            | Self::Diff(_) => None,
        }
    }

    /// The attempt timeout parsed in either shared argument home.
    pub(crate) const fn timeout(&self) -> u64 {
        match self {
            Self::Decide(arguments) => arguments.common.timeout,
            Self::Choose(arguments) => arguments.common.timeout,
            Self::Tag(arguments) => arguments.common.timeout,
            Self::Score(arguments) => arguments.common.timeout,
            Self::Filter(arguments) => arguments.common.timeout,
            Self::Rank(arguments) => arguments.common.timeout,
            Self::Find(arguments) => arguments.common.timeout,
            Self::Annotate(arguments) => arguments.common.timeout,
            Self::Recognize(arguments) => arguments.common.timeout,
            Self::Relate(arguments) => arguments.common.timeout,
            Self::Check(arguments)
            | Self::Backends(super::backends::BackendCommand::Check(arguments)) => {
                arguments.timeout
            }
            Self::Cache(_)
            | Self::Status(_)
            | Self::Mcp(_)
            | Self::Transform(_)
            | Self::Runs(_)
            | Self::Audit(_)
            | Self::Diff(_) => 1,
        }
    }
}

#[derive(Args, Debug)]
pub(crate) struct StatusArguments {
    /// Print one closed JSON object instead of name-value lines.
    #[arg(long)]
    pub(crate) json: bool,
    /// The named backend: a base with its own key variable and model. It outranks THINKTHEN_BACKEND.
    #[arg(long, value_name = "NAME")]
    pub(crate) backend: Option<String>,
}

impl Command {
    pub(crate) fn check_images(&self) -> Result<(), crate::failure::Failure> {
        use crate::failure::Failure;
        let refusal = match self {
            Self::Tag(a) if a.common.images() => {
                Some("tag accepts text only; images are unsupported")
            }
            Self::Filter(a) if a.common.images() => {
                Some("filter accepts text only; images are unsupported")
            }
            Self::Rank(a) if a.common.images() => {
                Some("rank accepts text only; images are unsupported")
            }
            Self::Annotate(a) if a.common.images() => {
                Some("annotate accepts text only; images are unsupported")
            }
            Self::Find(a)
                if !a.common.image.is_empty() || a.common.media.as_deref() == Some("image") =>
            {
                Some("find accepts text only; images are unsupported")
            }
            Self::Recognize(a) if a.common.images() => {
                Some("recognize accepts text only; images are unsupported")
            }
            Self::Relate(a) if a.common.images() => {
                Some("relate accepts text only; images are unsupported")
            }
            _ => None,
        };
        refusal.map_or(Ok(()), |message| Err(Failure::Usage(message)))
    }
}
