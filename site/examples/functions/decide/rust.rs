use thinkthen::{Answer, Engine, Question};

let tt = Engine::from_env()?;
let question = "Does the customer ask for a refund?";
let refund = Question::decide(question)?.cut();
let broken = "Please refund my order. It arrived broken.";
let is_refund = tt.decide(&refund, broken)?.into_value();
assert_eq!(is_refund, Answer::Yes);
