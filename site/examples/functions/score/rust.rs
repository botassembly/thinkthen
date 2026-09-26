use thinkthen::{Engine, Question};

let tt = Engine::from_env()?;
let question = "How urgent is this?";
let urgency_scale = Question::score(question)?
    .level("Routine.", None)?
    .level("Soon.", None)?
    .level("Immediate.", None)?
    .build()?;
let texts = [
    "Please update my mailing address when you can.",
    "Can you send the signed contract by Friday?",
    "Nobody can log in to the site right now.",
];
let urgency = texts
    .iter()
    .map(|text| tt.score(&urgency_scale, text))
    .collect::<Result<Vec<_>, _>>()?;
assert_eq!(urgency, [0.06, 0.99, 2.0]);
