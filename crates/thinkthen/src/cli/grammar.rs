//! One grammar supplies parsing, help, completions and the packaged manual.
use clap::{Arg, ArgAction, Command};

fn option(name: &'static str, help: &'static str) -> Arg {
    Arg::new(name).long(name).help(help)
}
fn switch(name: &'static str, help: &'static str) -> Arg {
    option(name, help)
        .num_args(0..=1)
        .require_equals(true)
        .default_missing_value("true")
}

pub(super) fn command() -> Command {
    let mut command = Command::new("thinkthen")
        .about("Ask typed questions about evidence. Text answers or streaming JSON Lines.")
        .version(version())
        .subcommand_required(true)
        .arg_required_else_help(true);
    for (name, help) in [
        (
            "backend",
            "Backend name [THINKTHEN_BACKEND; default: systemone]",
        ),
        ("model", "Model name [THINKTHEN_MODEL]"),
        ("url", "Backend address"),
        ("batch", "Records per request [THINKTHEN_BATCH]"),
        (
            "from",
            "Input framing [THINKTHEN_FROM; default: inferred from FILE]",
        ),
        ("evidence", "Evidence JSON Pointer [THINKTHEN_EVIDENCE]"),
        ("format", "Output format [THINKTHEN_FORMAT; default: text]"),
        ("color", "Diagnostic color [THINKTHEN_COLOR; NO_COLOR]"),
    ] {
        let mut arg = option(name, help).global(true);
        arg = match name {
            "from" => arg.value_parser(["text", "lines", "jsonl", "csv", "tsv", "xlsx"]),
            "format" => arg.value_parser(["text", "json"]),
            "color" => arg.value_parser(["auto", "always", "never"]),
            _ => arg,
        };
        command = command.arg(arg);
    }
    for (name, short, help) in [
        ("json", None, "Write typed JSON Lines [THINKTHEN_JSON]"),
        (
            "quiet",
            Some('q'),
            "Hide notices and progress [THINKTHEN_QUIET]",
        ),
        (
            "verbose",
            Some('v'),
            "Print a bounded row summary [THINKTHEN_VERBOSE]",
        ),
    ] {
        let mut arg = switch(name, help).global(true);
        if let Some(short) = short {
            arg = arg.short(short);
        }
        command = command.arg(arg);
    }
    for function in crate::FunctionName::ALL {
        // The shared registry supplies the spelling and complete function set.
        let name = serde_json::to_value(function)
            .ok()
            .and_then(|v| v.as_str().map(str::to_owned));
        if let Some(name) = name {
            command = command.subcommand(data_command(&name));
        }
    }
    command
        .subcommand(data_command("run"))
        .subcommand(
            Command::new("completions")
                .about("Generate shell completions without a backend")
                .arg(
                    Arg::new("shell")
                        .required(true)
                        .value_parser(clap::value_parser!(clap_complete::Shell)),
                ),
        )
        .subcommand(
            Command::new("man")
                .hide(true)
                .about("Generate the package manual"),
        )
}

fn data_command(name: &str) -> Command {
    let mut command = Command::new(name.to_owned());
    if name != "run" {
        command = command.arg(option(
            "name",
            "Optional execution identity [THINKTHEN_NAME]",
        ));
    }
    if !matches!(name, "recognize" | "relate") {
        command = command.arg(Arg::new("question").required(true).value_name(
            if matches!(name, "annotate" | "run") {
                "TOOLBOX"
            } else {
                "QUESTION"
            },
        ));
    }
    command = command.arg(
        Arg::new("files")
            .value_name("FILE")
            .num_args(0..)
            .action(ArgAction::Append),
    );
    for (key, help) in [
        ("sheet", "Worksheet [THINKTHEN_SHEET]"),
        ("into", "Answer column [THINKTHEN_INTO]"),
        ("output", "Destination path [THINKTHEN_OUTPUT]"),
    ] {
        command = command.arg(option(key, help));
    }
    command = command
        .arg(
            option(
                "column",
                "Evidence column; repeat [THINKTHEN_COLUMNS: JSON array]",
            )
            .action(ArgAction::Append),
        )
        .arg(switch(
            "in-place",
            "Replace source with backup [THINKTHEN_IN_PLACE]",
        ));
    if matches!(
        name,
        "decide" | "filter" | "choose" | "tag" | "find" | "recognize" | "relate"
    ) {
        command = command.arg(option("threshold", "Probability cut [THINKTHEN_THRESHOLD]"));
    }
    if matches!(name, "decide" | "filter" | "choose") {
        command = command
            .arg(option("low", "Band lower bound [THINKTHEN_LOW]"))
            .arg(option("high", "Band upper bound [THINKTHEN_HIGH]"));
    }
    if matches!(name, "decide" | "filter" | "choose" | "find") {
        command = command.arg(switch(
            "check",
            "Return the verdict as an exit code [THINKTHEN_CHECK]",
        ));
    }
    let list = match name {
        "choose" => Some((
            "choice",
            "Choice; repeat in order [THINKTHEN_CHOICES: JSON array]",
        )),
        "tag" => Some((
            "label",
            "Label; repeat in order [THINKTHEN_LABELS: JSON array]",
        )),
        "rank" | "score" => Some((
            "level",
            "Scale level; repeat low to high [THINKTHEN_LEVELS: JSON array]",
        )),
        "recognize" => Some((
            "kind",
            "Kind; repeat in order [THINKTHEN_KINDS: JSON array]",
        )),
        _ => None,
    };
    if let Some((key, help)) = list {
        command = command.arg(option(key, help).action(ArgAction::Append));
    }
    if name == "rank" {
        command = command.arg(option("top", "Maximum answered ranks [THINKTHEN_TOP]"));
    }
    if name == "relate" {
        command = command
            .arg(
                option(
                    "rule",
                    "NAME SOURCE TARGET; repeat [THINKTHEN_RULES: JSON array]",
                )
                .num_args(3)
                .action(ArgAction::Append),
            )
            .arg(switch(
                "either",
                "Override every rule direction [THINKTHEN_EITHER]",
            ));
    }
    if name == "run" {
        command = command.arg(option("entry", "Saved entry name [THINKTHEN_ENTRY]"));
    }
    command
}

fn version() -> String {
    let backends = crate::BackendName::ALL
        .iter()
        .map(|backend| {
            if *backend == crate::BackendName::Stub {
                backend.as_str().to_owned()
            } else {
                format!("{} {}", backend.as_str(), backend.default_model())
            }
        })
        .collect::<Vec<_>>()
        .join(", ");
    format!(
        "{} (contract {}; backends {backends})",
        crate::VERSION,
        crate::VERSION
    )
}
