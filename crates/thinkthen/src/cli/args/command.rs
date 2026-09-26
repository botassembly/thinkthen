//! The verbs and their command-level help.

use std::path::PathBuf;

use clap::{Args, Subcommand};

use super::{
    AnnotateArguments, ChooseArguments, DecideArguments, FilterArguments, FindArguments,
    RankArguments, RecognizeArguments, RelateArguments, ScoreArguments, TagArguments,
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

    /// Check that a backend you name works with this tool, over four fixed requests.
    ///
    /// It exits 0 only when no finding is critical. Every request is real spend.
    /// The report names the model asked for, the model sent, and the model each
    /// reply names.
    #[command(display_order = 1003)]
    Check(CheckArguments),

    /// Answer one yes or no question about a text. A record run exits 0 when
    /// it completes without a partial or whole-run failure. The printed
    /// values carry the individual answers.
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
    /// thinkthen decide 'The customer asks for a refund.' --threshold 0.1:0.9 --quiet < m.txt
    ///
    /// case $? in 0) route refunds ;; 1) route support ;; 3) route triage ;; *) exit 4 ;; esac
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
    /// --field or a question file's `on`. It makes one paid request for every
    /// record.
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

    /// Sort records by how likely the answer is yes. `rank` asks one yes/no
    /// question of each record and sorts locally; it never compares two
    /// records.
    ///
    /// The printed order puts the most likely yes first. An exact tie keeps
    /// input order. `rank` never runs a tournament.
    ///
    /// It holds every record until the input ends, because a final order needs
    /// the whole set, so an endless stream is cut into windows upstream.
    /// `--top N` prints the first N of the order and saves no request.
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
    /// pick=$(thinkthen choose 'Which team owns this?' billing shipping other --raw < m.txt) && rc=0 || rc=$?
    ///
    /// case $rc in 0) ;; 3) pick=not_sure ;; *) exit "$rc" ;; esac
    ///
    /// case $pick in billing) pay ;; not_sure) triage ;; *) exit 2 ;; esac
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
    /// questions exits 6.
    #[command(
        after_help = "Examples:\n\nthinkthen annotate checks.json < message.txt\nthinkthen annotate checks.json --input message.txt\n"
    )]
    Annotate(AnnotateArguments),

    /// Find every name in a text and assign one of the given kinds.
    ///
    /// Relations are beta. `--threshold` gates computed name strength;
    /// `--relation-threshold` gates a relation's model probability.
    ///
    /// Each record makes paid requests: a detection question for every word, a
    /// kind question for every word when two or more kinds are given, and
    /// relation questions when rules are given. --dry-run prints the exact
    /// requests for the first record.
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
    /// for every pair, in both directions unless --either. --dry-run prints the
    /// questions and requests and sends nothing.
    ///
    /// A run that answers some relation questions and fails others prints what
    /// it has and exits 6. A run whose relation questions all fail prints
    /// nothing and exits 4.
    #[command(mut_arg("jobs", |arg| arg.hide(true)))]
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

    /// Grade saved answers against an answer key and suggest a bar.
    ///
    /// RESULTS holds the lines `decide`, `filter`, `choose`, `tag`, `score`,
    /// `rank`, `find`, or `annotate` printed with --details, and KEY holds one
    /// JSON object per record: its id, the right value, and an optional part of
    /// tune or held. audit prints agreement with its 95% interval, both kinds of
    /// disagreement, precision and f1, AUC, calibration, a coverage curve, and a
    /// suggested bar tuned on one part and checked on the other. An answer
    /// inside a band is not sure, and it counts apart from right and wrong.
    ///
    /// A key may give each record a part of tune or held; without parts audit
    /// splits the records itself and shows how steady its bar is.
    ///
    /// --write changes one threshold in the file and prints the old value on
    /// standard error.
    ///
    /// audit sends no request and reads no key.
    ///
    /// To grade a recording, replay it with --details and pass the output:
    ///
    /// thinkthen decide 'Is it red?' --jsonl --details --replay runs/red < records.jsonl | thinkthen audit - key.jsonl
    #[command(display_order = 1000)]
    Audit(crate::cli::audit::AuditArguments),

    /// Show which saved answers changed between two runs or two cuts.
    ///
    /// A and B hold the lines `decide` or `choose` printed for the same
    /// records. Without B, diff compares A under --threshold with A under
    /// --compare-threshold. Two cuts on one run cost nothing. The probabilities
    /// are already saved. Each changed answer prints on one line, and a summary
    /// with its McNemar test prints last. With --key, each change says whether
    /// it gained or lost a right answer. An answer inside a band is not sure.
    ///
    /// diff sends no request and reads no key.
    ///
    /// thinkthen diff runs/before.jsonl runs/after.jsonl --key key.jsonl --table
    #[command(display_order = 1001)]
    Diff(crate::cli::diff::DiffArguments),
}

#[derive(Args, Debug)]
pub(crate) struct CacheArguments {
    #[command(subcommand)]
    pub(crate) command: CacheCommand,
}

#[derive(Debug, Subcommand)]
pub(crate) enum CacheCommand {
    /// Remove selected entries, then the oldest entries until the folder fits the size target.
    Prune(PruneArguments),
}

#[derive(Args, Debug)]
pub(crate) struct PruneArguments {
    /// The cache or recording folder to maintain.
    pub(crate) directory: PathBuf,
    /// Trim to this many allocated bytes. Without it, the configuration's
    /// cache_bytes applies, or 100000000.
    #[arg(long, value_name = "BYTES")]
    pub(crate) max_size: Option<String>,
    /// Remove entries strictly older than a duration such as 30d or 12h.
    #[arg(long, value_name = "Nd|Nh|Nm|Ns")]
    pub(crate) older_than: Option<String>,
    /// Remove entries whose reply names another model. Give the version that
    /// answered, as a result's meta.model shows it, not the alias passed to
    /// --model. A name no reply carries removes every entry.
    #[arg(long, value_name = "MODEL")]
    pub(crate) answered_by_other_than: Option<String>,
}

impl Command {
    pub(crate) const fn reads_input(&self) -> bool {
        !matches!(
            self,
            Self::Cache(_)
                | Self::Status(_)
                | Self::Check(_)
                | Self::Transform(_)
                | Self::Audit(_)
                | Self::Diff(_)
        )
    }

    /// The input file one command named, if any.
    pub(crate) fn input(&self) -> Option<&std::path::Path> {
        match self {
            Self::Decide(arguments) => arguments.common.input.as_deref(),
            Self::Choose(arguments) => arguments.common.input.as_deref(),
            Self::Tag(arguments) => arguments.common.input.as_deref(),
            Self::Score(arguments) => arguments.common.input.as_deref(),
            Self::Filter(arguments) => arguments.common.input.as_deref(),
            Self::Rank(arguments) => arguments.common.input.as_deref(),
            Self::Find(arguments) => arguments.common.input.as_deref(),
            Self::Annotate(arguments) => arguments.common.input.as_deref(),
            Self::Recognize(arguments) => arguments.common.input.as_deref(),
            Self::Relate(arguments) => arguments.common.input.as_deref(),
            Self::Cache(_)
            | Self::Status(_)
            | Self::Check(_)
            | Self::Transform(_)
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
            Self::Check(arguments) => arguments.timeout,
            Self::Cache(_)
            | Self::Status(_)
            | Self::Transform(_)
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
}

#[derive(Args, Debug)]
pub(crate) struct CheckArguments {
    /// The base the requests are posted under, which outranks THINKTHEN_BASE_URL.
    #[arg(long, value_name = "URL")]
    pub(crate) url: Option<String>,
    /// The model named in each request, resolved as every command resolves it.
    #[arg(long, value_name = "NAME")]
    pub(crate) model: Option<String>,
    /// Positive seconds that bound one attempt from connect to last byte, and each retry wait.
    #[arg(long, value_name = "SECONDS", default_value_t = 30)]
    pub(crate) timeout: u64,
    /// Print the four request bodies and stop. No key is read and nothing is sent.
    #[arg(long)]
    pub(crate) dry_run: bool,
}
