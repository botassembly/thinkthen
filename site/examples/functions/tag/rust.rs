use thinkthen::{Engine, Question};

thinkthen::choices! {
    enum Label {
        Praise => "praise",
        Bug => "bug",
        Billing => "billing",
    }
}

let tt = Engine::from_env()?;
let question = "Which labels fit this message?";
let labels = Question::tag::<Label>(question)?
    .label(Label::Praise, None)?
    .label(Label::Bug, None)?
    .label(Label::Billing, None)?
    .cut()?;
let message = concat!(
    "Love the new dashboard, ",
    "but export crashes the app, ",
    "and I was charged twice.",
);
let fitting_labels = tt.tag(&labels, message)?.into_value();
let expected = [Label::Praise, Label::Bug, Label::Billing];
assert_eq!(fitting_labels, expected);
