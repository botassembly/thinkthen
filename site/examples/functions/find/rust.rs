use thinkthen::{Engine, Question};

let tt = Engine::from_env()?;
let question = "Which line gives the refund deadline?";
let deadline = Question::find(question)?;
let policy = [
    "Returns need the original receipt.",
    "Refunds are issued within 30 days of purchase.",
    "Shipping is free on orders over $50.",
    "Gift cards cannot be exchanged for cash.",
];
let found = tt.find(&deadline, policy)?;
assert_eq!(found.selected(), Some(&policy[1]));
