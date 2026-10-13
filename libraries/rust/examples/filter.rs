//! `filter`: the records whose answer is yes, lazily, in input order.

use std::io::Write;

use thinkthen::{
    CallOptions, Engine, Question, RecordInput, Request, RequestArguments, RequestCall,
    RequestInput, RequestOptions, RequestQuestion, RequestRows,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let tt = Engine::from_env()?;
    let complaint = Question::decide("Is this a complaint?")?.cut();
    let reviews = [
        "Arrived a day early. Thank you!",
        "The zipper broke the first time I used it.",
    ];
    let request = Request::new(RequestCall::Filter(RequestArguments {
        question: RequestQuestion::Definition {
            value: complaint.into(),
        },
        input: RequestInput::Feed {
            name: "reviews".into(),
            framing: Default::default(),
            reading: Default::default(),
            images: Vec::new(),
        },
        options: RequestOptions::default(),
    }))
    .admit()?;
    let records = reviews.into_iter().map(|original| {
        Ok(RecordInput {
            original,
            context: None,
            options: None,
            examples: None,
            seed_spans: None,
        })
    });
    let RequestRows::Filtered(kept) =
        tt.execute_request_rows(&request, records, CallOptions::new())?
    else {
        return Err("filter returned another result kind".into());
    };
    let mut out = std::io::stdout().lock();
    for row in kept {
        let row = row?;
        if row.result().value() {
            writeln!(out, "{}", row.original())?;
        }
    }
    Ok(())
}
