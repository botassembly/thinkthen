use thinkthen::{Answer, Engine, Question};

let tt = Engine::from_env()?;
let question = "Does the customer ask for a refund?";
let refund = Question::decide(question)?.cut();
let broken = "Please refund my order. It arrived broken.";
let thanks = "Thanks for the quick help yesterday!";
assert_eq!(tt.decide(&refund, broken)?, Answer::Yes);
assert_eq!(tt.decide(&refund, thanks)?, Answer::No);
