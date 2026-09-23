//! Exact default rows beside the requests each record made.

use super::{ONE_AT_A_TIME, RECORDS, decide, printed, serving, states};

#[test]
fn each_framing_prints_each_record_beside_its_value_in_input_order() {
    let listener = serving(&["0.97", "0.02", "0.80"]).expect("a loopback listener");
    let output = decide(
        listener.base(),
        &[&["--jsonl", "--field", "/body"][..], &ONE_AT_A_TIME].concat(),
        RECORDS,
    )
    .expect("the compiled binary runs");

    assert_eq!(output.status.code(), Some(0));
    assert_eq!(
        printed(&output),
        concat!(
            r#"{"input":{"id":"R-1","body":"The payout failed again."},"value":true}"#,
            "\n",
            r#"{"input":{"id":"R-2","body":"Thanks for the quick fix."},"value":false}"#,
            "\n",
            r#"{"input":{"id":"R-3","body":"The card was refused at checkout."},"value":true}"#,
            "\n",
        )
    );
    assert_eq!(
        states(&listener),
        [
            r#""The payout failed again.""#,
            r#""Thanks for the quick fix.""#,
            r#""The card was refused at checkout.""#,
        ]
    );

    let listener = serving(&["0.97", "0.02"]).expect("a loopback listener");
    let output = decide(
        listener.base(),
        &[&["--lines"][..], &ONE_AT_A_TIME].concat(),
        "first line\nsecond line\n",
    )
    .expect("the compiled binary runs");

    assert_eq!(output.status.code(), Some(0));
    assert_eq!(
        printed(&output),
        concat!(
            r#"{"input":"first line","value":true}"#,
            "\n",
            r#"{"input":"second line","value":false}"#,
            "\n",
        )
    );
    assert_eq!(states(&listener), [r#""first line""#, r#""second line""#]);
}
