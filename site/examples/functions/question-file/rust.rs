use thinkthen::{Answer, Engine, Question};

let tt = Engine::from_env()?;
let refund = Question::load("refund.json")?;
let money_back = concat!(
    "I would like to return this and get my money back.",
    "\n",
);
let is_refund = tt
    .decide(&refund, money_back)?
    .into_value();
assert_eq!(is_refund, Answer::Yes);
