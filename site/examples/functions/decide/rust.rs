use thinkthen::{Answer, Engine, Question};

let tt = Engine::from_env()?;
let question = "Does the customer ask for a refund?";
let refund = Question::decide(question)?.cut();
let broken = "Please refund my order. It arrived broken.";
let thanks = "Thanks for the quick help yesterday!";
let broken_is_refund = tt.decide(&refund, broken)?;
let thanks_is_refund = tt.decide(&refund, thanks)?;
assert_eq!(broken_is_refund, Answer::Yes);
assert_eq!(thanks_is_refund, Answer::No);
