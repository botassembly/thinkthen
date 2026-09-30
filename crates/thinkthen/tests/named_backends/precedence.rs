//! ADR 0114 section 3's precedence, every pair of tiers with conflicting
//! values, through the real command. The builder's rows are in `builder.rs`.

use conformance_backend::Listener;

use crate::support::{Home, by_marker, listener, only, said};

/// One row: the typed, environment, and configuration tiers as
/// (backend, address) pairs, any extra flags, then the expected address,
/// key variable, marker place, and model. An address is a listener letter
/// `A` to `D`, or a whole base.
struct Row {
    typed: (Option<&'static str>, Option<&'static str>),
    environment: (Option<&'static str>, Option<&'static str>),
    configuration: (Option<&'static str>, Option<&'static str>),
    extra: &'static [&'static str],
    url: &'static str,
    key: (&'static str, usize),
    model: &'static str,
}

const LOCAL: (&str, usize) = ("LOCAL_D1_KEY", 4);
const LIQUID: (&str, usize) = ("LIQUIDAI_API_KEY", 1);
const PRIMARY: (&str, usize) = ("THINKTHEN_API_KEY", 3);
const LIQUID_BASE: &str = "https://api.liquid.ai/decisions/v1";

const ROWS: [Row; 19] = [
    // Rule 5: a lower tier's address never replaces a higher tier's backend.
    Row {
        typed: (Some("local-d1"), None),
        environment: (None, Some("B")),
        configuration: (None, None),
        extra: &[],
        url: "D",
        key: LOCAL,
        model: "local-model",
    },
    Row {
        typed: (Some("local-d1"), None),
        environment: (None, None),
        configuration: (None, Some("C")),
        extra: &[],
        url: "D",
        key: LOCAL,
        model: "local-model",
    },
    Row {
        typed: (None, None),
        environment: (Some("local-d1"), None),
        configuration: (None, Some("C")),
        extra: &[],
        url: "D",
        key: LOCAL,
        model: "local-model",
    },
    Row {
        typed: (Some("liquid"), None),
        environment: (None, Some("B")),
        configuration: (None, None),
        extra: &[],
        url: LIQUID_BASE,
        key: LIQUID,
        model: "d1:free",
    },
    // Rule 6: a higher tier's address outranks a lower tier's backend.
    Row {
        typed: (None, Some("A")),
        environment: (Some("local-d1"), None),
        configuration: (None, None),
        extra: &[],
        url: "A",
        key: PRIMARY,
        model: "config-model",
    },
    Row {
        typed: (None, Some("A")),
        environment: (None, None),
        configuration: (Some("liquid"), None),
        extra: &[],
        url: "A",
        key: PRIMARY,
        model: "config-model",
    },
    Row {
        typed: (None, None),
        environment: (None, Some("B")),
        configuration: (Some("liquid"), None),
        extra: &[],
        url: "B",
        key: PRIMARY,
        model: "config-model",
    },
    // Rule 4: one tier names both.
    Row {
        typed: (Some("liquid"), Some("A")),
        environment: (Some("local-d1"), Some("B")),
        configuration: (None, None),
        extra: &[],
        url: "A",
        key: LIQUID,
        model: "d1:free",
    },
    Row {
        typed: (None, None),
        environment: (Some("liquid"), Some("B")),
        configuration: (Some("local-d1"), Some("C")),
        extra: &[],
        url: "B",
        key: LIQUID,
        model: "d1:free",
    },
    Row {
        typed: (None, None),
        environment: (None, None),
        configuration: (Some("liquid"), Some("C")),
        extra: &[],
        url: "C",
        key: LIQUID,
        model: "d1:free",
    },
    // Backend against backend.
    Row {
        typed: (Some("local-d1"), None),
        environment: (Some("liquid"), None),
        configuration: (None, None),
        extra: &[],
        url: "D",
        key: LOCAL,
        model: "local-model",
    },
    Row {
        typed: (None, None),
        environment: (Some("local-d1"), None),
        configuration: (Some("liquid"), None),
        extra: &[],
        url: "D",
        key: LOCAL,
        model: "local-model",
    },
    Row {
        typed: (Some("local-d1"), None),
        environment: (None, None),
        configuration: (Some("liquid"), None),
        extra: &[],
        url: "D",
        key: LOCAL,
        model: "local-model",
    },
    // Address against address: today's order, kept.
    Row {
        typed: (None, Some("A")),
        environment: (None, Some("B")),
        configuration: (None, Some("C")),
        extra: &[],
        url: "A",
        key: PRIMARY,
        model: "config-model",
    },
    Row {
        typed: (None, None),
        environment: (None, Some("B")),
        configuration: (None, Some("C")),
        extra: &[],
        url: "B",
        key: PRIMARY,
        model: "config-model",
    },
    // Rule 7 and the configuration's backend alone.
    Row {
        typed: (None, None),
        environment: (None, None),
        configuration: (None, None),
        extra: &[],
        url: "https://api.typesafe.ai/v1",
        key: PRIMARY,
        model: "config-model",
    },
    Row {
        typed: (None, None),
        environment: (None, None),
        configuration: (Some("liquid"), None),
        extra: &[],
        url: LIQUID_BASE,
        key: LIQUID,
        model: "d1:free",
    },
    // The flag's model outranks the backend's model; the configuration's model does not.
    Row {
        typed: (Some("local-d1"), None),
        environment: (None, None),
        configuration: (None, None),
        extra: &["--model", "asked-model"],
        url: "D",
        key: LOCAL,
        model: "asked-model",
    },
    Row {
        typed: (None, Some("A")),
        environment: (None, None),
        configuration: (None, None),
        extra: &["--model", "asked-model"],
        url: "A",
        key: PRIMARY,
        model: "asked-model",
    },
];

#[test]
fn every_pair_of_tiers_resolves_as_adr_0114_section_3_states() {
    for (index, row) in ROWS.iter().enumerate() {
        let listeners: Vec<Listener> = (0..4).map(|_| listener()).collect();
        let base = |letter: &'static str| match letter {
            "A" => listeners[0].base().to_owned(),
            "B" => listeners[1].base().to_owned(),
            "C" => listeners[2].base().to_owned(),
            "D" => listeners[3].base().to_owned(),
            whole => whole.to_owned(),
        };
        let home = Home::new("precedence");
        let mut config = format!(
            r#"{{"schema":"thinkthen.config/1","model":"config-model","backends":{{"local-d1":{{"url":"{}","key_env":"LOCAL_D1_KEY","model":"local-model"}}}}"#,
            base("D")
        );
        if let Some(name) = row.configuration.0 {
            config.push_str(&format!(r#","backend":"{name}""#));
        }
        if let Some(url) = row.configuration.1 {
            config.push_str(&format!(r#","url":"{}""#, base(url)));
        }
        home.config(&(config + "}"));
        let mut flags: Vec<String> = row.extra.iter().map(|&flag| flag.to_owned()).collect();
        if let Some(name) = row.typed.0 {
            flags.extend(["--backend".to_owned(), name.to_owned()]);
        }
        if let Some(url) = row.typed.1 {
            flags.extend(["--url".to_owned(), base(url)]);
        }
        let mut changes = vec![];
        if let Some(name) = row.environment.0 {
            changes.push(("THINKTHEN_BACKEND", name.to_owned()));
        }
        if let Some(url) = row.environment.1 {
            changes.push(("THINKTHEN_BASE_URL", base(url)));
        }
        let changes: Vec<(&str, &str)> = changes
            .iter()
            .map(|(name, value)| (*name, value.as_str()))
            .collect();
        let run = |more: &str| {
            let mut arguments = vec!["decide", "a refund?", "--lines", "--input"];
            let input = home.evidence(&["alpha", "beta"]);
            arguments.push(&input);
            arguments.extend(flags.iter().map(String::as_str));
            arguments.push(more);
            let output = home.run(
                &arguments,
                &[&[("THINKTHEN_BATCH", "1")], changes.as_slice()].concat(),
            );
            let (stdout, stderr) = said(&output);
            assert_eq!(output.status.code(), Some(0), "{index}: {stderr}");
            stdout
        };
        let plan = run("--plan");
        let expected = format!(
            r#"{{"url":"{}/systemone","model":"{}","key_env":"{}","#,
            base(row.url),
            row.model,
            row.key.0
        );
        assert!(plan.starts_with(&expected), "{index}: {plan}");
        if row.url.len() == 1 {
            run("--no-cache");
            assert_only(&listeners, row.url, row.key.1, index);
        }
        home.assert_no_marker_in_files();
    }
}

/// Assert the listener named by `letter` got two requests under the marker at
/// `place`, and every other listener got none.
fn assert_only(listeners: &[Listener], letter: &str, place: usize, index: usize) {
    for (name, listener) in ["A", "B", "C", "D"].iter().zip(listeners) {
        let wanted = if *name == letter {
            only(place, 2)
        } else {
            [0; 7]
        };
        assert_eq!(by_marker(listener), wanted, "{index} {name}");
    }
}
