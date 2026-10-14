use thinkthen::{Annotated, Answer, Engine, QuestionSet};

let tt = Engine::from_env()?;
let set = QuestionSet::load("form.json")?;
let report = "Steps: click Log in. Nobody gets in.";
let triage = tt.annotate(&set, [report])?;
let fields = &triage.value()[0];
assert_eq!(
    fields[0].value(),
    &Annotated::Decision(Answer::Yes),
);
assert_eq!(
    fields[1].value(),
    &Annotated::Choice(Some("login".into())),
);
assert_eq!(fields[2].value(), &Annotated::Score(1.98));
