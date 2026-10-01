use thinkthen::{Engine, QuestionSet};

let tt = Engine::from_env()?;
let set = QuestionSet::load("form.json")?;
let reports = [
    "Steps: click Export. It is very slow.",
    "Steps: click Log in. Nobody gets in.",
    "The Pay button on billing is too blue.",
];
for form in tt.annotate(&set, reports) {
    let triage = form?.value_json();
    println!("{triage}");
}
