use thinkthen::{Annotated, Engine, QuestionSet};

let tt = Engine::from_env()?;
let set = QuestionSet::load("form.json")?;
let reports = [
    "CSV export fails. Steps: click Export.",
    "The login page spins and nobody can sign in.",
    "The Pay button on the billing page is too blue.",
];
let areas = ["export", "login", "billing"];
let triage = tt.annotate(&set, reports);
for (form, want) in triage.zip(areas) {
    let area = form?.values()[1].value().clone();
    assert_eq!(area, Annotated::Choice(Some(want.into())));
}
