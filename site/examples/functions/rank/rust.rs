use thinkthen::{Engine, Question};

let tt = Engine::from_env()?;
let question = "Is this urgent?";
let urgent = Question::rank(question)?;
let inbox = [
    concat!(
        "Newsletter: our autumn catalog is here. ",
        "No reply needed.",
    ),
    "Our checkout page is down and customers cannot pay",
    "Reminder: your invoice is due in 30 days",
    "Please send the signed quote by 5 pm today",
];
let ranked = tt.rank(&urgent, inbox)?;
let order: Vec<&str> = ranked
    .iter()
    .map(|one| *one.input())
    .collect();
assert_eq!(order, [inbox[1], inbox[3], inbox[2], inbox[0]]);
