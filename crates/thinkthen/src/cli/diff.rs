//! Offline diff dispatch uses the same document and typed-row admission as audit.
use crate::analysis::{self, AuditOptions, GoldCase, Manifest, QuestionMapping, SavedRun};
use crate::{Cancel, Code, Error, Threshold};
use clap::{Arg, ArgGroup, ArgMatches, Command};

pub(super) fn command() -> Command {
    let mut command = Command::new("diff")
        .about("Compare saved runs or decide/filter thresholds without a model call")
        .arg(
            Arg::new("rows")
                .required(true)
                .value_name("LEFT")
                .help("Left saved typed JSON Lines; - reads stdin"),
        );
    for (name, label, help) in [
        (
            "manifest",
            "RUN.json",
            "Original manifest for threshold-only comparison",
        ),
        (
            "left-manifest",
            "L.json",
            "Left manifest for actual-run comparison",
        ),
        ("right", "RIGHT", "Right saved typed JSON Lines"),
        (
            "right-manifest",
            "R.json",
            "Right manifest for actual-run comparison",
        ),
        (
            "key",
            "GOLD.jsonl",
            "Optional labels keyed by exact case and question identity",
        ),
        (
            "question-map",
            "MAP.json",
            "Explicit one-to-one array of left/right question identities",
        ),
    ] {
        command = command.arg(Arg::new(name).long(name).value_name(label).help(help));
    }
    command
        .arg(Arg::new("threshold").long("threshold").value_name("P").value_parser(clap::value_parser!(f64))
            .conflicts_with_all(["low","high"]).help("Compare one new decide/filter cut"))
        .arg(Arg::new("low").long("low").value_name("L").requires("high").value_parser(clap::value_parser!(f64)).help("New band's low bar"))
        .arg(Arg::new("high").long("high").value_name("H").requires("low").value_parser(clap::value_parser!(f64)).help("New band's high bar"))
        .group(ArgGroup::new("mode").args(["manifest","left-manifest"]).required(true))
        .group(ArgGroup::new("cut").args(["threshold","low"]))
        .mut_arg("manifest",|arg|arg.requires("cut").conflicts_with_all(["right","right-manifest","question-map"]))
        .mut_arg("left-manifest",|arg|arg.requires_all(["right","right-manifest"]).conflicts_with_all(["threshold","low","high"]))
        .mut_arg("right",|arg|arg.requires("left-manifest"))
        .mut_arg("right-manifest",|arg|arg.requires("left-manifest"))
        .mut_arg("question-map",|arg|arg.requires("left-manifest"))
        .after_help("Prints one JSON report. Actual saved-run comparison supports decide/filter, choose and find. Threshold-only comparison supports decide/filter probabilities, never guesses missing winners and never calls a backend. Different question identities require --question-map. Without labels, correctness improvement is unavailable.")
}
pub(super) fn run(args: &ArgMatches, cancel: &Cancel) -> Result<u8, Error> {
    super::audit::offline_options(args, &command())?;
    let options = AuditOptions::default();
    let gold = if args.contains_id("key") {
        Some(super::audit::lines(
            &document(args, "key", options, cancel)?,
            options.max_cases.get().saturating_mul(2),
            |line| {
                serde_json::from_str::<GoldCase>(line)
                    .map_err(|_| invalid("a diff key entry is not a current typed gold label"))
            },
        )?)
    } else {
        None
    };
    let report = if args.contains_id("manifest") {
        let manifest = metadata(args, "manifest", options, cancel)?;
        let rows = rows(args, "rows", options, cancel)?;
        let threshold = if let Some(value) = args.get_one::<f64>("threshold") {
            Threshold::cut(*value)?
        } else {
            Threshold::band(
                *args
                    .get_one::<f64>("low")
                    .ok_or_else(|| invalid("threshold diff requires a cut or complete band"))?,
                *args
                    .get_one::<f64>("high")
                    .ok_or_else(|| invalid("threshold diff requires a cut or complete band"))?,
            )?
        };
        analysis::compare_threshold(SavedRun { manifest, rows }, threshold, gold, options)?
    } else {
        let left = SavedRun {
            manifest: metadata(args, "left-manifest", options, cancel)?,
            rows: rows(args, "rows", options, cancel)?,
        };
        let right = SavedRun {
            manifest: metadata(args, "right-manifest", options, cancel)?,
            rows: rows(args, "right", options, cancel)?,
        };
        let mapping = if args.contains_id("question-map") {
            serde_json::from_str::<Vec<QuestionMapping>>(&document(
                args,
                "question-map",
                options,
                cancel,
            )?)
            .map_err(|_| invalid("question map must be an array of left/right identity pairs"))?
        } else {
            vec![]
        };
        analysis::compare(left, right, gold, &mapping, options)?
    };
    if cancel.is_cancelled() {
        return Err(crate::retry::cancelled());
    }
    super::audit::output(args, cancel, &report)?;
    Ok(0)
}
fn invalid(message: &str) -> Error {
    Error::from_contract(Code::UsageQuestion, message)
}
fn document(
    args: &ArgMatches,
    name: &str,
    options: AuditOptions,
    cancel: &Cancel,
) -> Result<String, Error> {
    let path = args
        .get_one::<String>(name)
        .ok_or_else(|| invalid("diff requires its mode's input paths"))?;
    if name != "rows" && path == "-" {
        return Err(invalid(
            "only LEFT may read standard input; other diff inputs require named files",
        ));
    }
    super::audit::read(path, options.max_input_bytes.get(), cancel)
}
fn metadata(
    args: &ArgMatches,
    name: &str,
    options: AuditOptions,
    cancel: &Cancel,
) -> Result<Manifest, Error> {
    serde_json::from_str(&document(args, name, options, cancel)?)
        .map_err(|_| invalid("a diff manifest is not valid current JSON metadata"))
}
fn rows(
    args: &ArgMatches,
    name: &str,
    options: AuditOptions,
    cancel: &Cancel,
) -> Result<Vec<crate::AnyRow>, Error> {
    super::audit::lines(
        &document(args, name, options, cancel)?,
        options.max_cases.get(),
        analysis::decode_row,
    )
}
