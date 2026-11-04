//! One grammar supplies parsing, help, completions and the packaged manual.
use crate::core::questions::command::ThresholdOption;
use clap::{Arg, ArgAction, Command};

fn option(name: &'static str, help: &'static str) -> Arg {
    Arg::new(name).long(name).help(help).value_name(match name {
        "backend" => "NAME".into(),
        _ => name.to_uppercase(),
    })
}
fn switch(name: &'static str, help: &'static str) -> Arg {
    option(name, help)
        .num_args(0..=1)
        .require_equals(true)
        .default_missing_value("true")
}

pub(super) fn command() -> Command {
    let mut command = Command::new("thinkthen")
        .about("Ask typed questions about records. Text answers or streaming JSON Lines.")
        .version(crate::VERSION)
        .after_help(environment_help())
        .subcommand_required(true)
        .arg_required_else_help(false);
    for (name, help) in [
        (
            "backend",
            "Backend name [THINKTHEN_BACKEND; default: systemone]",
        ),
        ("model", "Model name [THINKTHEN_MODEL]"),
        ("url", "Backend address [THINKTHEN_URL]"),
        ("batch", "Texts per request [THINKTHEN_BATCH]"),
        (
            "from",
            "Input framing [THINKTHEN_FROM; default: inferred from FILE]",
        ),
        (
            "field",
            "Field pointer; repeat for an object keyed by final names [THINKTHEN_FIELD]",
        ),
        ("format", "Output format [THINKTHEN_FORMAT; default: text]"),
        ("color", "Diagnostic color [THINKTHEN_COLOR; NO_COLOR]"),
    ] {
        let mut arg = option(name, help).global(true).display_order(0);
        if name == "field" {
            arg = arg.action(ArgAction::Append);
        }
        arg = match name {
            "from" => arg.value_parser(super::options::choices::<super::input::Format>()),
            "format" => arg.value_parser(super::options::choices::<super::options::OutputFormat>()),
            "color" => arg.value_parser(super::options::choices::<clap::ColorChoice>()),
            _ => arg,
        };
        command = command.arg(arg);
    }
    for (name, short, help) in [
        ("json", None, "Write typed JSON Lines [THINKTHEN_JSON]"),
        (
            "quiet",
            Some('q'),
            "Print no answers, notices or progress [THINKTHEN_QUIET]",
        ),
        ("verbose", Some('v'), "Print a summary [THINKTHEN_VERBOSE]"),
    ] {
        let mut arg = switch(name, help).global(true).display_order(0);
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
                        .value_name("SHELL")
                        .required(true)
                        .help("Shell to generate completions for")
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
    let spec = serde_json::from_value::<crate::FunctionName>(name.into())
        .ok()
        .map(crate::FunctionName::command);
    let mut command = Command::new(name.to_owned())
        .before_help(super::help::examples(name))
        .after_help(format!(
            "Reference: https://thinkthen.dev/functions/{name}/\n\n{}",
            environment_help()
        ));
    if let Some(spec) = &spec {
        command = command.about(spec.description);
    } else {
        command = command.about("Run a saved question or pipeline from a toolbox");
    }
    if name != "run" {
        command = command
            .arg(option(
                "name",
                "Optional execution identity [THINKTHEN_NAME]",
            ))
            .arg(
                option("text", "Judge this text instead of reading input")
                    .allow_hyphen_values(true)
                    .conflicts_with_all(["files", "from"]),
            );
    }
    if !matches!(name, "recognize" | "relate") {
        command = command.arg(
            Arg::new("question")
                .required(true)
                .help(if matches!(name, "annotate" | "run") {
                    "Toolbox path or saved entry name"
                } else {
                    "Question to ask"
                })
                .value_name(if matches!(name, "annotate" | "run") {
                    "TOOLBOX"
                } else {
                    "QUESTION"
                }),
        );
    }
    command = command.arg(
        Arg::new("files")
            .value_name("FILE")
            .help(if name == "run" {
                "Entry name after a toolbox file or folder, then input files; default: standard input"
            } else {
                "Input files; default: standard input"
            })
            .num_args(0..)
            .action(ArgAction::Append),
    );
    let into_help = if name == "run" {
        "Answer column"
    } else {
        "Answer field for JSON Lines; column for tables"
    };
    for (key, help) in [
        ("sheet", "Worksheet"),
        ("into", into_help),
        ("output", "Destination path"),
    ] {
        command = command.arg(
            option(key, help)
                .help_heading("Table options")
                .display_order(100),
        );
    }
    command = command
        .arg(
            option("column", "Field column; repeat; replaces a saved pointer")
                .action(ArgAction::Append)
                .help_heading("Table options")
                .display_order(100),
        )
        .arg(
            switch("in-place", "Replace source with backup")
                .help_heading("Table options")
                .display_order(100),
        )
        .arg(
            switch(
                "details",
                "Add status, probability, error, detail and source columns",
            )
            .help_heading("Table options")
            .display_order(100),
        );
    if let Some(spec) = spec {
        if !matches!(spec.threshold, ThresholdOption::None) {
            command = command.arg(option(
                "threshold",
                "Required probability [THINKTHEN_THRESHOLD]",
            ));
        }
        if matches!(spec.threshold, ThresholdOption::Band) {
            command = command
                .arg(option("low", "Low probability bar [THINKTHEN_LOW]"))
                .arg(option("high", "High probability bar [THINKTHEN_HIGH]"));
        }
        if spec.check {
            command = command.arg(switch(
                "check",
                "Return the verdict as an exit code [THINKTHEN_CHECK]",
            ));
        }
        if let Some((key, _, env, help)) = spec.list {
            command = command.arg(
                Arg::new(key)
                    .long(key)
                    .help(format!("{help} [{env}: JSON array]"))
                    .value_name(key.to_uppercase())
                    .action(ArgAction::Append),
            );
        }
    }
    if matches!(name, "decide" | "filter") {
        command = command
            .arg(option("true", "Meaning of a true answer [THINKTHEN_TRUE]").value_name("TEXT"))
            .arg(option("false", "Meaning of a false answer [THINKTHEN_FALSE]").value_name("TEXT"));
    }
    if matches!(name, "choose" | "tag") {
        command = command.arg(
            option("describe", "Description for a label; repeat")
                .value_name("LABEL=TEXT")
                .action(ArgAction::Append),
        );
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
                .value_names(["NAME", "SOURCE", "TARGET"])
                .action(ArgAction::Append),
            )
            .arg(switch(
                "either",
                "Override every rule direction [THINKTHEN_EITHER]",
            ));
    }
    if name == "run" {
        command = command
            .arg(option(
                "entry",
                "Saved entry name; same as NAME after TOOLBOX",
            ))
            .arg(
                option(
                    "allow-shell",
                    "Allow the toolbox to execute its declared shell steps",
                )
                .action(ArgAction::SetTrue),
            );
    }
    command
}

fn backends() -> String {
    let backends = crate::BackendName::ALL
        .iter()
        .filter(|backend| backend.advertised())
        .map(|backend| format!("{} {}", backend.as_str(), backend.default_model()))
        .collect::<Vec<_>>()
        .join(", ");
    format!("Backends: {backends}")
}

fn environment_help() -> String {
    format!(
        "{}\nCredentials: THINKTHEN_API_KEY (System One), OPENAI_API_KEY (OpenAI).\nAddress: THINKTHEN_URL (both backends), THINKTHEN_BASE_URL (System One only); certificate authority: THINKTHEN_CA_FILE.\nExits: 0 success; 1 no (--check); 2 usage; 3 unsure (--check); 4 backend failure; 5 local or internal failure; 6 failed records; 130 cancelled; 141 broken pipe.",
        backends()
    )
}
