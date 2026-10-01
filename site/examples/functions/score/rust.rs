use thinkthen::{Engine, Question};

let tt = Engine::from_env()?;
let question = "How urgent is this?";
let urgency_scale = Question::score(question)?
    .level("Routine.", None)?
    .level("Soon.", None)?
    .level("Immediate.", None)?
    .build()?;
let outage =
    "Our checkout page is down and customers cannot pay.\n";
let urgency = tt
    .score(&urgency_scale, outage)?
    .into_value();
assert_eq!(urgency, 2.0);
