use thinkthen::{Engine, QuestionSet};

let tt = Engine::from_env()?;
let set = QuestionSet::load("form.json")?;
let report = "Steps: click Log in. Nobody gets in.";
let triage = tt
    .annotate(&set, [report])
    .map(|form| form.map(|one| one.value_json()))
    .collect::<Result<Vec<_>, _>>()?;
let row = r#"{"steps":true,"area":"login","impact":1.98}"#;
assert_eq!(triage, [row]);
