use thinkthen::{Annotated, Engine, QuestionSet};

let tt = Engine::from_env()?;
let set = QuestionSet::load("form.json")?;
let reports = [
    "Steps: click Export. It is very slow.",
    "Steps: click Log in. Nobody gets in.",
    "The Pay button on billing is too blue.",
];
let areas = ["export", "login", "billing"];
let triage = tt.annotate(&set, reports);
for (form, want) in triage.zip(areas) {
    let area = form?.values()[1].value().clone();
    assert_eq!(area, Annotated::Choice(Some(want.into())));
}
