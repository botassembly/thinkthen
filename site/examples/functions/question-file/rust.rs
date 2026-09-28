use thinkthen::{Answer, Engine, Question};

let tt = Engine::from_env()?;
let question = "Does the customer ask for a refund?";
let refund = Question::decide(question)?.band(0.2, 0.8)?;
let send_back = "I want to send this back.";
let is_refund = tt.decide(&refund, send_back)?.into_value();
assert_eq!(is_refund, Answer::Unsure);
