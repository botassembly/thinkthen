//! Relation argument mapping through the shared engine and output.
use super::RelateArgs;
use thinkthen::{Cancel, Error, Relate, RelationRule};

pub(super) fn run(args: RelateArgs, cancel: Cancel) -> Result<u8, Error> {
    let question = Relate::new(
        args.rules
            .iter()
            .map(|rule| RelationRule::parse(rule).map(|rule| rule.either(args.either)))
            .collect::<Result<Vec<_>, _>>()?,
    )?;
    let question = match args.threshold {
        Some(text) => question.threshold(text.parse()?)?,
        None => question,
    };
    super::run(args.common, &question, cancel)
}
