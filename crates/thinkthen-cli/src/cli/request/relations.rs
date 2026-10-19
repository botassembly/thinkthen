//! Command relation flags construct a supported public native definition.
use crate::{Relate, RelationRule, Threshold};
use crate::cli::args::RelateArguments;
use crate::failure::Failure;

const RULES: &str = "relate takes one or more distinct relation rules";
const CUT: &str = "the relate threshold is one cut above zero and at most one";

pub(super) fn inline(arguments: &RelateArguments) -> Result<Relate, Failure> {
    // Parse every rule before checking duplicates, as native inline admission did.
    let rules = arguments.relations.iter()
        .map(|text| RelationRule::parse_inline(text, arguments.either).map_err(Failure::from))
        .collect::<Result<Vec<_>, _>>()?;
    if rules.is_empty() { return Err(Failure::Usage(RULES)); }
    let mut builder = Relate::builder();
    for rule in rules {
        builder = builder.relation(rule).map_err(|_| Failure::Usage(RULES))?;
    }
    if let Some(rule) = &arguments.threshold {
        let value = rule.parse::<Threshold>().ok().and_then(Threshold::cut_value)
            .ok_or(Failure::Usage(CUT))?;
        builder = builder.threshold(value).map_err(|_| Failure::Usage(CUT))?;
    }
    let relation = builder.build().map_err(Failure::from)?;
    relation.record_fields(
        arguments.common.field.first().map_or("/name", String::as_str),
        arguments.kind_field.as_deref().unwrap_or("/kind"),
    ).map_err(Failure::from)
}

#[cfg(test)]
mod tests {
    //! Stays: command relation parsing must preserve saved definitions and typed override precedence.
    use super::*;
    use clap::Parser as _;
    use crate::cli::args::{Cli, Command};

    fn parse(words: &[&str]) -> RelateArguments {
        let cli = Cli::try_parse_from(words).expect("command");
        let Some(Command::Relate(arguments)) = cli.command else { panic!("relate command") };
        arguments
    }

    #[test]
    fn inline_flags_keep_native_identity_and_refusal_order() {
        let arguments = parse(&["thinkthen", "relate", "knows=person:person", "--either", "--threshold", "0.7", "--field", "/label"]);
        let relation = inline(&arguments).unwrap();
        let native = Relate::from_records_json(r#"{"version":1,"relate":{"relations":[{"name":"knows","source":"person","target":"person","reads":"knows","either":true}],"fields":{"name":"/label","kind":"/kind"}},"threshold":0.7}"#).unwrap();
        for mode in [crate::RelationIdentityMode::Records, crate::RelationIdentityMode::Lines] {
            assert_eq!(crate::RequestDefinition::from(relation.clone()).relation_sha256(mode).unwrap(),
                crate::RequestDefinition::from(native.clone()).relation_sha256(mode).unwrap());
        }
        for (words, expected) in [
            (vec!["thinkthen", "relate", "knows", "knows", "bad:rule"], "a relate relation is NAME=SOURCE_KIND:TARGET_KIND, or a bare NAME"),
            (vec!["thinkthen", "relate", "knows", "knows", "--threshold", "0"], RULES),
            (vec!["thinkthen", "relate", "knows", "--threshold", "0", "--field", "bad"], CUT),
        ] {
            let error = inline(&parse(&words)).unwrap_err();
            assert_eq!(crate::failure::sentence(&error), (2, expected.to_owned()));
        }
    }
}
