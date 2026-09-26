use thinkthen::{Engine, Question};

let tt = Engine::from_env()?;
let question = "Is this a complaint?";
let complaint = Question::decide(question)?.cut();
let reviews = [
    "Arrived a day early. Thank you!",
    "The zipper broke the first time I used it.",
    "Does this come in blue?",
    "The strap snapped on day two.",
];
let complaints = tt
    .filter(&complaint, reviews)
    .collect::<Result<Vec<_>, _>>()?;
assert_eq!(complaints, [reviews[1], reviews[3]]);
